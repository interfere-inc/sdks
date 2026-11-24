import {
  TELEMETRY_EXPORT_TIMEOUT_MS,
  retryTelemetry,
} from "@interfere/helpers/retry-telemetry";
import { InterfereClient, InterfereError } from "@interfere/javascript";
import type { OtlpSignal } from "@interfere/constants/api";
import { toError } from "@interfere/types/sdk/errors";
import { isPlainRecord } from "@interfere/types/sdk/guards/json";

import type { ExportResult } from "@opentelemetry/core";
import { SpanKind } from "@opentelemetry/api";
import { ExportResultCode } from "@opentelemetry/core";
import { JsonTraceSerializer } from "@opentelemetry/otlp-transformer";
import type { ReadableSpan, SpanExporter } from "@opentelemetry/sdk-trace";

import { fetchIngest, stampSentAt } from "../ingest-backoff/ingest-backoff.js";
import {
  isPageHidden,
  reserveKeepaliveBody,
} from "../page-lifecycle/page-lifecycle.js";
import { isPageLeaving } from "../page-lifecycle/unloading.js";
import { currentSpanGate, noteLogsDecided } from "./transport/leave-gate.js";
import { withExportCarryover } from "./export/carryover.js";
import type { RetainedSend } from "./export/carryover.js";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { sendInKeepaliveChunks } from "./keepalive-chunks.js";
import { compressExportBody } from "./transport/compress-export.js";
import { PRODUCER_VERSION } from "../version.js";
import { IngestLogExporter } from "./exporter-logs.js";
import { createInFlightExports } from "./transport/in-flight-exports.js";
import { IngestMetricExporter } from "./exporter-metrics.js";
import type { GatedResourceAttributes } from "./exporter-metrics.js";

const MILLIS_PER_SECOND = 1000;

type ExportHooks = Omit<RetainedSend, "id"> & { id?: string };

export type SignalSender = ((
  payload: Uint8Array | undefined,
  resultCallback: (result: ExportResult) => void,
  abortSignal?: AbortSignal,
  hooks?: ExportHooks,
) => void) & { dispose?: () => void };

const CARRYOVER_HEADER = "x-interfere-carryover";

const HOOKS_HEADER = "x-interfere-export-hooks";

interface ExporterCommon {
  clientOptions: InterfereClient.Options;
  canPersistExports?: (() => boolean) | undefined;
}

const AUTH_INVALID_HALT_THRESHOLD = 3;

const AUTH_INVALID_HALT_COOLDOWN_MS = 5 * 60 * 1000;

type IngestHaltReason =
  | "environment_filtered"
  | "release_not_found"
  | "public_key_rejected";

const RPC_PERMISSION_DENIED = 7;

const HALT_MESSAGES: Record<IngestHaltReason, string> = {
  environment_filtered:
    "ingest halted: the ingest endpoint filters this page's environment",
  release_not_found:
    "ingest halted: the ingest endpoint does not recognize this build's release",
  public_key_rejected:
    "ingest paused: the ingest endpoint rejected this page's public key",
};

let haltReason: IngestHaltReason | null = null;

let authHaltExpiresAt = 0;

let consecutiveAuthRejections = 0;

/** Test-only reset of the ingest halt state. */
export function resetIngestHalt() {
  haltReason = null;

  authHaltExpiresAt = 0;

  consecutiveAuthRejections = 0;
}

function currentHaltReason() {
  if (haltReason === "public_key_rejected" && Date.now() >= authHaltExpiresAt) {
    haltReason = null;

    consecutiveAuthRejections = AUTH_INVALID_HALT_THRESHOLD - 1;
  }

  return haltReason;
}

export function isIngestHalted() {
  return currentHaltReason() !== null;
}

function detectUnacceptableExport(error: InterfereError) {
  if (
    currentHaltReason() !== null ||
    (error.statusCode !== 422 &&
      error.statusCode !== 401 &&
      error.statusCode !== 403)
  ) {
    return;
  }

  const body: unknown = error.body;

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Rejected ingest responses may contain arbitrary JSON; only objects can carry a machine-readable halt code.
  if (body === null || typeof body !== "object" || !("code" in body)) {
    return;
  }

  if (error.statusCode === 403 && body.code === RPC_PERMISSION_DENIED) {
    haltReason = "environment_filtered";

    const detail =
      // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Rejected ingest responses may contain arbitrary JSON; only a string message is worth repeating.
      "message" in body && typeof body.message === "string"
        ? ` ${body.message}`
        : "";

    console.warn(
      `[interfere] the ingest endpoint rejected this page's deployment environment. Telemetry export is disabled for this page load.${detail}`,
    );

    return;
  }

  if (error.statusCode === 422 && body.code === "RELEASE_NOT_FOUND") {
    haltReason = "release_not_found";

    console.warn(
      `[interfere] the ingest endpoint does not recognize this build's release. Telemetry export is disabled for this page load. Check that source-map upload and finalization succeeded and that build and runtime use the same surface key.`,
    );

    return;
  }

  if (error.statusCode === 401 && body.code === "AUTH_PK_INVALID") {
    consecutiveAuthRejections += 1;

    if (consecutiveAuthRejections < AUTH_INVALID_HALT_THRESHOLD) {
      return;
    }

    haltReason = "public_key_rejected";

    authHaltExpiresAt = Date.now() + AUTH_INVALID_HALT_COOLDOWN_MS;

    console.warn(
      `[interfere] the ingest endpoint rejected this page's public key ${consecutiveAuthRejections} times in a row — telemetry export is paused for ${AUTH_INVALID_HALT_COOLDOWN_MS / 60_000} minutes, then retried once. A revoked key stays rejected until the page is rebuilt; build with INTERFERE_API_KEY set so the build plugin bakes in a current release token.`,
    );
  }
}

// WARNING: The host page's own beacons and keepalive fetches share the browser's 64 KiB keepalive quota and cannot see this SDK's reservations; telemetry exports leave half of it free so a hide-time flush does not push the page over and get rejected before it is sent.
const EXPORT_KEEPALIVE_MAX_BYTES = 32_000;

function makeSender(input: ExporterCommon, signal: OtlpSignal) {
  const keepaliveHooks = new Map<string, ExportHooks>();

  const client = new InterfereClient({
    ...input.clientOptions,
    maxRetries: 0,
    timeoutInSeconds: TELEMETRY_EXPORT_TIMEOUT_MS / MILLIS_PER_SECOND,
    fetch: Object.assign(
      async (
        url: Parameters<typeof fetch>[0],
        init?: Parameters<typeof fetch>[1],
      ) =>
        fetchIngest(url, async () => {
          const prepared =
            // oxlint-disable-next-line anti-slop/no-runtime-typeof -- The generated client serializes JSON bodies to strings; only those can be measured and compressed before dispatch.
            typeof init?.body === "string"
              ? await compressExportBody(init.body)
              : { body: init?.body ?? null, bytes: 0, encoding: null };

          const { bytes } = prepared;

          const headers = new Headers(init?.headers);

          if (prepared.encoding) {
            headers.set("content-encoding", prepared.encoding);
          }

          const retained = keepaliveHooks.get(headers.get(HOOKS_HEADER) ?? "");

          headers.delete(HOOKS_HEADER);

          const dispatch = async (
            abortSignal: AbortSignal,
            keepalive: boolean,
          ) => {
            if (keepalive) {
              retained?.onKeepalive();
            }

            return globalThis.fetch(url, {
              ...init,
              body: prepared.body,
              headers: stampSentAt(new Headers(headers)),
              credentials: "omit",
              keepalive,
              signal: abortSignal,
            });
          };

          return retryTelemetry(async (abortSignal) => {
            const release = reserveKeepaliveBody(
              bytes,
              EXPORT_KEEPALIVE_MAX_BYTES,
            );

            if (signal === "logs") {
              noteLogsDecided();
            }

            if (release === null) {
              // WARNING: a leave-time flush can run while the page still looks visible (beforeunload); an ordinary fetch started then is cancelled by the navigation, so an over-budget body must be carried over instead.
              if (isPageHidden() || isPageLeaving()) {
                retained?.onBudgetExhausted();

                throw new Error("Keepalive budget exhausted");
              }

              return dispatch(abortSignal, false);
            }

            try {
              return await dispatch(abortSignal, true);
            } catch (error) {
              if (!(error instanceof TypeError) || abortSignal.aborted) {
                throw error;
              }
            } finally {
              release();
            }

            // WARNING: Chromium rejects an unloading page's keepalive fetch after sending it, in the task that fires pagehide, and this catch can run before the lifecycle's pagehide listener; wait one task so a resend never duplicates a delivered body.
            await new Promise((resolve) => {
              setTimeout(resolve, 0);
            });

            if (isPageHidden() || isPageLeaving()) {
              throw new Error("Keepalive request rejected");
            }

            return dispatch(abortSignal, false);
          }, init?.signal);
        }),
      globalThis.fetch,
    ),
  });

  const send = ((payload, resultCallback, abortSignal, hooks) => {
    if (!payload || payload.byteLength === 0) {
      resultCallback({ code: ExportResultCode.SUCCESS });

      return;
    }

    const reason = currentHaltReason();

    if (reason !== null) {
      resultCallback({
        code: ExportResultCode.FAILED,
        error: new Error(HALT_MESSAGES[reason]),
      });

      return;
    }

    const hooksKey = hooks ? crypto.randomUUID() : undefined;

    void (async () => {
      if (hooks && hooksKey) {
        keepaliveHooks.set(hooksKey, hooks);
      }

      try {
        const body: unknown = JSON.parse(new TextDecoder().decode(payload));

        if (!isPlainRecord(body)) {
          throw new TypeError("OTLP export body is not a JSON object");
        }

        await client.drain.otlpExport(
          { signal, body },
          {
            abortSignal,
            headers: omitUndefined({
              "x-interfere-producer-version": PRODUCER_VERSION,
              [CARRYOVER_HEADER]: hooks?.id,
              [HOOKS_HEADER]: hooksKey,
            }),
          },
        );

        consecutiveAuthRejections = 0;

        resultCallback({ code: ExportResultCode.SUCCESS });
      } catch (error) {
        if (error instanceof InterfereError) {
          detectUnacceptableExport(error);
        }

        resultCallback({
          code: ExportResultCode.FAILED,
          error: toError(error),
        });
      } finally {
        if (hooksKey) {
          keepaliveHooks.delete(hooksKey);
        }
      }
    })();
  }) satisfies SignalSender;

  const { baseUrl } = input.clientOptions;
  const publicKey = input.clientOptions.publicKeyHeader?.apiKey;

  if (
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Supplier options are not persistable resolved destinations.
    typeof baseUrl !== "string" ||
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Supplier options are not persistable public keys.
    typeof publicKey !== "string" ||
    input.clientOptions.auth !== undefined
  ) {
    return send;
  }

  const carryover = withExportCarryover(
    JSON.stringify([baseUrl, publicKey, signal]),
    send,
    input.canPersistExports ?? (() => true),
  );

  // WARNING: a beforeunload flush runs while the page still looks visible and the navigation tears it down before navigator.locks grants a drain, so it must deliver (retain and dispatch at once) rather than send.
  const leaveAware = ((payload, resultCallback, abortSignal, hooks) => {
    if (isPageHidden()) {
      carryover.send(payload, resultCallback);

      return;
    }

    if (isPageLeaving()) {
      carryover.deliver(payload, resultCallback);

      return;
    }

    send(payload, resultCallback, abortSignal, hooks);
  }) satisfies SignalSender;

  return Object.assign(leaveAware, { dispose: () => carryover.dispose() });
}

function leavePriority(span: ReadableSpan) {
  if (span.events.some((event) => event.name === "exception")) {
    return 0;
  }

  return span.kind === SpanKind.CLIENT ? 1 : 2;
}

// WARNING: A leave-time export larger than the keepalive budget is split and the tail is retained for the next page; exception spans, then request spans, go first so a bounced visit still delivers them.
function leaveOrder(spans: ReadableSpan[]) {
  return [0, 1, 2].flatMap((priority) =>
    spans.filter((span) => leavePriority(span) === priority),
  );
}

const serialize = (chunk: ReadableSpan[]) =>
  JsonTraceSerializer.serializeRequest(chunk);

class IngestTraceExporter implements SpanExporter {
  private readonly send: SignalSender;

  private readonly inFlight = createInFlightExports();

  constructor(send: SignalSender) {
    this.send = send;
  }

  export(
    spans: ReadableSpan[],
    resultCallback: (result: ExportResult) => void,
  ) {
    if (spans.length === 0) {
      resultCallback({ code: ExportResultCode.SUCCESS });

      return;
    }

    const gate = currentSpanGate();

    if (!gate || !(isPageHidden() || isPageLeaving())) {
      sendInKeepaliveChunks(
        spans,
        serialize,
        this.send,
        this.inFlight.track(resultCallback),
        leaveOrder,
      );

      return;
    }

    const exceptions = spans.filter((span) => leavePriority(span) === 0);
    const rest = spans.filter((span) => leavePriority(span) !== 0);
    const done = this.inFlight.track(resultCallback);

    let pending = (exceptions.length > 0 ? 1 : 0) + (rest.length > 0 ? 1 : 0);
    let failure: ExportResult | undefined;

    const settle = (result: ExportResult) => {
      pending -= 1;

      if (result.code !== ExportResultCode.SUCCESS) {
        failure ??= result;
      }

      if (pending === 0) {
        done(failure ?? { code: ExportResultCode.SUCCESS });
      }
    };

    if (exceptions.length > 0) {
      sendInKeepaliveChunks(
        exceptions,
        serialize,
        this.send,
        settle,
        leaveOrder,
      );
    }

    if (rest.length > 0) {
      void (async () => {
        await gate.wait();

        sendInKeepaliveChunks(rest, serialize, this.send, settle, leaveOrder);
      })();
    }
  }

  async shutdown() {
    await this.inFlight.settled();

    this.send.dispose?.();
  }

  async forceFlush() {
    await this.inFlight.settled();
  }
}

export function createTraceExporter(input: ExporterCommon) {
  return new IngestTraceExporter(
    makeSender(input, "traces"),
  ) satisfies SpanExporter;
}

export function createMetricExporter(
  input: ExporterCommon,
  gatedResourceAttributes?: GatedResourceAttributes,
) {
  return new IngestMetricExporter(
    makeSender(input, "metrics"),
    gatedResourceAttributes,
  ) satisfies IngestMetricExporter;
}

export function createLogExporter(input: ExporterCommon) {
  return new IngestLogExporter(
    makeSender(input, "logs"),
  ) satisfies IngestLogExporter;
}
