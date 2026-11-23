import { bytesToHex } from "@interfere/helpers/hex";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { InterfereClient } from "@interfere/javascript";
import { INTERFERE_BROWSER_EVENTS_LOGGER_NAME } from "@interfere/types/data/browser-event-loggers";
import {
  INTERFERE_ACTION_ID_KEY,
  INTERFERE_VIEW_ID_KEY,
  PAGE_VIEW_EVENT_NAME,
  USER_ACTION_EVENT_NAME,
} from "@interfere/types/data/correlation-events";
import { isNonErrorException } from "@interfere/types/sdk/errors";
import type { NonErrorException } from "@interfere/types/sdk/errors";
import { exceptionSpanName } from "@interfere/types/sdk/throw-site";
import { createUrlRedactionSpanProcessor } from "@interfere/types/sdk/url-redaction";
import type { UrlRedactionPolicy } from "@interfere/types/sdk/url-redaction";

import { context, isSpanContextValid, trace } from "@opentelemetry/api";
import type {
  AnyValue,
  Attributes,
  Context,
  LogRecord,
} from "@opentelemetry/api";

import { identityAttributes } from "../../tracking/view.js";
import { SESSION_START_EVENT_NAME } from "../../tracking/session.js";
import { createLogger } from "../../util/log.js";
import { fetchIngest, stampSentAt } from "../ingest-backoff/ingest-backoff.js";
import { reserveKeepaliveBody } from "../page-lifecycle/page-lifecycle.js";
import { PRODUCER_VERSION } from "../version.js";
import type { TelemetrySink } from "./sink.js";

const MILLIS_PER_SECOND = 1000;

const log = createLogger("capture");

const EXCEPTION_SCOPE_NAME = "@interfere/react/exception";

const SPAN_KIND_INTERNAL = 1;

const STATUS_CODE_ERROR = 2;

const KEEPALIVE_MAX_BYTES = 32_000;

// WARNING: Keep this under KEEPALIVE_MAX_BYTES: a batch that reaches the reservation limit goes out without keepalive, which is what a drain during unload must never do.
/** Serialized body bytes one export may carry; a queue that exceeds this leaves as several keepalive-sized exports. */
export const MAX_EXPORT_BODY_BYTES = 30_000;

/** Spans waiting behind the one in-flight export; later exceptions are dropped until it drains. */
export const MAX_QUEUED_EXCEPTION_SPANS = 32;

const MAX_QUEUED_EVENT_RECORDS = 64;

const EVENT_EXPORT_DELAY_MS = 5000;

const EVENT_NAMES = new Set<string | undefined>([
  PAGE_VIEW_EVENT_NAME,
  SESSION_START_EVENT_NAME,
  USER_ACTION_EVENT_NAME,
]);

type Signal = "logs" | "traces";

const ENVELOPES = {
  logs: ["resourceLogs", "scopeLogs", "logRecords"],
  traces: ["resourceSpans", "scopeSpans", "spans"],
} as const;

const textEncoder = new TextEncoder();

export interface CaptureOnlySinkInput {
  clientOptions: InterfereClient.Options;
  getActionId: () => string | null;
  getDeviceId: () => string | null;
  getSessionId: () => string | null;
  getViewId: () => string | null;
  resource: Record<string, string>;
  urlRedaction: UrlRedactionPolicy;
}

/** When, and under which active context, a buffered exception was captured. */
export interface CaptureOrigin {
  readonly at: number;
  readonly context: Context;
}

function randomHex(bytes: number) {
  return bytesToHex(crypto.getRandomValues(new Uint8Array(bytes)));
}

function otlpValue(value: AnyValue) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- OTLP JSON encodes each attribute by its primitive kind.
  switch (typeof value) {
    case "boolean": {
      return { boolValue: value };
    }
    case "number": {
      return Number.isInteger(value)
        ? { intValue: `${value}` }
        : { doubleValue: value };
    }
    case "string": {
      return { stringValue: value };
    }
    default: {
      return { stringValue: JSON.stringify(value) };
    }
  }
}

function keyValues(attributes: Attributes) {
  return Object.entries(attributes).flatMap(([key, value]) =>
    value === undefined ? [] : [{ key, value: otlpValue(value) }],
  );
}

/**
 * The sink for `tracing: false`: each captured exception becomes one
 * hand-encoded, URL-redacted OTLP/JSON span whose view and action ids are
 * the exception event's, so span and event agree even for exceptions
 * buffered before startup finished. Page-view and user-action log records
 * are hand-encoded the same way, stamped with the current identity, and
 * every other log record is dropped. One export is in flight at a time;
 * records captured meanwhile queue (bounded) and leave together, exceptions
 * first.
 */
export function createCaptureOnlySink(input: CaptureOnlySinkInput) {
  const client = new InterfereClient({
    ...input.clientOptions,
    maxRetries: 0,
    timeoutInSeconds: 10,
    fetch: Object.assign(
      async (
        url: Parameters<typeof fetch>[0],
        init?: Parameters<typeof fetch>[1],
      ) =>
        fetchIngest(url, async () => {
          const body = init?.body;

          const release = reserveKeepaliveBody(
            // oxlint-disable-next-line anti-slop/no-runtime-typeof -- The generated client serializes JSON bodies to strings; only those carry a byte length before dispatch.
            typeof body === "string" ? textEncoder.encode(body).byteLength : 0,
            KEEPALIVE_MAX_BYTES,
          );

          try {
            return await globalThis.fetch(url, {
              ...init,
              headers: stampSentAt(new Headers(init?.headers)),
              credentials: "omit",
              keepalive: release !== null,
            });
          } finally {
            release?.();
          }
        }),
      globalThis.fetch,
    ),
  });

  const resource = { attributes: keyValues(input.resource) };

  const redaction = createUrlRedactionSpanProcessor(input.urlRedaction);

  const encode = (
    value: Error | NonErrorException,
    raw: Attributes,
    handled: boolean,
    origin: CaptureOrigin,
  ) => {
    const at = `${Math.round(origin.at * MILLIS_PER_SECOND)}000`;
    const host = trace.getSpanContext(origin.context);
    const event = { name: "exception", attributes: { ...raw } };

    let name = exceptionSpanName(value);

    let status = {
      code: STATUS_CODE_ERROR,
      message: isNonErrorException(value) ? value.value : value.message,
    };

    redaction.onEnding({
      name,
      status,
      attributes: {},
      events: [event],
      updateName: (redacted) => {
        name = redacted;
      },
      setStatus: (redacted) => {
        status = redacted;
      },
    });

    return {
      traceId: randomHex(16),
      spanId: randomHex(8),
      name,
      kind: SPAN_KIND_INTERNAL,
      startTimeUnixNano: at,
      endTimeUnixNano: at,
      attributes: keyValues({
        "interfere.capture.kind": "exception",
        "session.id": input.getSessionId() ?? undefined,
        "device.id": input.getDeviceId() ?? undefined,
        [INTERFERE_VIEW_ID_KEY]: raw[INTERFERE_VIEW_ID_KEY],
        [INTERFERE_ACTION_ID_KEY]: raw[INTERFERE_ACTION_ID_KEY],
      }),
      events: [
        {
          name: event.name,
          timeUnixNano: at,
          attributes: keyValues(event.attributes),
        },
      ],
      links:
        host && isSpanContextValid(host)
          ? [
              {
                traceId: host.traceId,
                spanId: host.spanId,
                flags: host.traceFlags,
              },
            ]
          : [],
      status: handled ? {} : status,
    };
  };

  const encodeLog = (record: LogRecord) => {
    const at = `${Math.round((performance.timeOrigin + performance.now()) * 1e3)}000`;
    const host = record.context && trace.getSpanContext(record.context);
    const link = host && isSpanContextValid(host) ? host : undefined;

    return omitUndefined({
      timeUnixNano: at,
      observedTimeUnixNano: at,
      severityNumber: record.severityNumber ?? 0,
      eventName: record.eventName,
      attributes: keyValues({
        ...record.attributes,
        ...identityAttributes(input),
      }),
      traceId: link?.traceId,
      spanId: link?.spanId,
      flags: link?.traceFlags,
    });
  };

  type Encoded = ReturnType<typeof encode> | ReturnType<typeof encodeLog>;

  const envelope = (signal: Signal, records: Encoded[]) => {
    const [root, scopes, items] = ENVELOPES[signal];

    const name =
      signal === "traces"
        ? EXCEPTION_SCOPE_NAME
        : INTERFERE_BROWSER_EVENTS_LOGGER_NAME;

    return {
      [root]: [{ resource, [scopes]: [{ scope: { name }, [items]: records }] }],
    };
  };

  const byteLength = (value: Encoded | ReturnType<typeof envelope>) =>
    textEncoder.encode(JSON.stringify(value)).byteLength;

  const queues: Record<Signal, { bytes: number; record: Encoded }[]> = {
    logs: [],
    traces: [],
  };

  const dropped = { logs: 0, traces: 0 };

  let draining: Promise<void> | null = null;

  let scheduled: ReturnType<typeof setTimeout> | null = null;

  let closed = false;

  const nextBatch = (signal: Signal) => {
    const queue = queues[signal];

    let bytes = byteLength(envelope(signal, []));
    let count = 0;

    for (const entry of queue) {
      bytes += entry.bytes + (count > 0 ? 1 : 0);

      if (bytes > MAX_EXPORT_BODY_BYTES && count > 0) {
        break;
      }

      count += 1;
    }

    return queue.splice(0, count).map((entry) => entry.record);
  };

  const drain = async () => {
    while (queues.traces.length > 0 || queues.logs.length > 0) {
      const signal: Signal = queues.traces.length > 0 ? "traces" : "logs";

      try {
        // oxlint-disable-next-line no-await-in-loop -- One export is in flight at a time; the next batch is cut from the queue only after this one settles.
        await client.drain.otlpExport(
          { signal, body: envelope(signal, nextBatch(signal)) },
          { headers: { "x-interfere-producer-version": PRODUCER_VERSION } },
        );
      } catch {
        log.warn("%s export failed", signal);
      }
    }

    draining = null;
  };

  const drainNow = () => {
    if (scheduled !== null) {
      clearTimeout(scheduled);

      scheduled = null;
    }

    if (queues.traces.length > 0 || queues.logs.length > 0) {
      draining ??= drain();
    }
  };

  const enqueue = (
    signal: Signal,
    limit: number,
    encodeRecord: () => Encoded,
  ) => {
    if (closed) {
      return false;
    }

    if (queues[signal].length >= limit) {
      dropped[signal] += 1;

      if (dropped[signal] === 1) {
        log.warn("%s export is backed up; dropping new records", signal);
      }

      return false;
    }

    const record = encodeRecord();

    queues[signal].push({ bytes: byteLength(record), record });

    return true;
  };

  return {
    recordException: (
      value: Error | NonErrorException,
      attributes: Attributes,
      handled: boolean,
      origin?: CaptureOrigin,
    ) => {
      if (
        enqueue("traces", MAX_QUEUED_EXCEPTION_SPANS, () =>
          encode(
            value,
            attributes,
            handled,
            origin ?? {
              at: performance.timeOrigin + performance.now(),
              context: context.active(),
            },
          ),
        )
      ) {
        drainNow();
      }
    },
    emitLog: (record) => {
      if (
        EVENT_NAMES.has(record.eventName) &&
        enqueue("logs", MAX_QUEUED_EVENT_RECORDS, () => encodeLog(record)) &&
        draining === null
      ) {
        scheduled ??= setTimeout(drainNow, EVENT_EXPORT_DELAY_MS);
      }
    },
    flush: async () => {
      drainNow();

      await draining;
    },
    shutdown: async () => {
      closed = true;

      drainNow();

      await draining;
    },
  } satisfies TelemetrySink;
}
