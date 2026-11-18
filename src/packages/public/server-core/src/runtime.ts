import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { isNonErrorException, toExceptions } from "@interfere/types/sdk/errors";
import type { NonErrorException } from "@interfere/types/sdk/errors";
import type { ErrorMechanism } from "@interfere/types/sdk/plugins/payload/errors";
import type { TracingOwnership } from "@interfere/types/sdk/remote-config/tracing-ownership";
import {
  EXCEPTION_DELIVERY,
  EXCEPTION_DELIVERY_ATTRIBUTE,
} from "@interfere/types/sdk/exception-delivery";
import { exceptionSpanName } from "@interfere/types/sdk/throw-site";
import {
  createUrlRedactionSpanProcessor,
  resolveUrlRedaction,
} from "@interfere/types/sdk/url-redaction";
import type { UrlRedactionPolicy } from "@interfere/types/sdk/url-redaction";
import {
  ROOT_CONTEXT,
  SpanKind,
  SpanStatusCode,
  isSpanContextValid,
  trace,
} from "@opentelemetry/api";
import type { Attributes, TracerProvider } from "@opentelemetry/api";

import { contain, containAsync } from "./contain.js";
import { CAPTURE_KIND_ATTRIBUTE } from "./sampler.js";

/** Longest span attribute or status message a managed pipeline exports. */
export const ATTRIBUTE_VALUE_LENGTH_LIMIT = 65_536;

export interface CapturePolicy {
  errors: "host" | "interfere";
  recordOnHostSpan: boolean;
  urlRedaction: UrlRedactionPolicy;
  /** Capture forwarded client IPs by default; false omits them from exception events. */
  clientIp?: boolean;
}

export interface ServerPipeline {
  mode: TracingOwnership;
  provider: TracerProvider;
  isolated: boolean;
  capture: CapturePolicy;
  flush: () => Promise<void>;
  shutdown: () => Promise<void>;
}

/** Whether this isolate is workerd, which settles a promise only inside the request that created it. */
export function onWorkerd() {
  return (
    "navigator" in globalThis &&
    globalThis.navigator.userAgent === "Cloudflare-Workers"
  );
}

/** Owns one adapter's initialization, error delivery and provider lifetime. */
export class ServerRuntime {
  private readonly name: string;
  private pending: Promise<void> | null = null;
  private readonly flushes = new Set<Promise<unknown>>();
  private pipeline: ServerPipeline | null = null;
  private isolatedProvider: TracerProvider | null = null;
  private policy: CapturePolicy = {
    errors: "interfere",
    recordOnHostSpan: true,
    urlRedaction: resolveUrlRedaction(),
  };

  constructor(name: string) {
    this.name = name;
  }

  get mode() {
    return (this.pipeline?.mode ?? "managed") satisfies TracingOwnership;
  }

  async register(start: () => Promise<ServerPipeline | null>) {
    if (this.pending) {
      // WARNING: on workerd another request may own `pending` and end before it settles; awaiting it there rejects and strands this request.
      return onWorkerd() ? undefined : this.pending;
    }

    if (this.pipeline) {
      return;
    }

    this.pending = (async () => {
      await containAsync("register()", async () => {
        const pipeline = await start();

        this.pipeline = pipeline;

        if (pipeline) {
          this.policy = pipeline.capture;
          this.isolatedProvider = pipeline.isolated ? pipeline.provider : null;
        }
      });

      this.pending = null;
    })();

    return this.pending;
  }

  configureCapture(provider: TracerProvider | null, policy: CapturePolicy) {
    this.isolatedProvider = provider;
    this.policy = policy;
  }

  async flush() {
    // WARNING: workerd settles a promise only in the request that created it; a flush there must await only its own export, never `pending` or `flushes`.
    if (onWorkerd()) {
      const { pipeline } = this;

      await containAsync("flush()", async () => pipeline?.flush());

      return;
    }

    await this.pending;

    const { pipeline } = this;

    const flushing = containAsync("flush()", async () => pipeline?.flush());

    this.flushes.add(flushing);

    try {
      await Promise.all(this.flushes);
    } finally {
      this.flushes.delete(flushing);
    }
  }

  async shutdown() {
    await this.pending;

    const { pipeline } = this;

    this.pipeline = null;
    this.isolatedProvider = null;

    await containAsync("shutdown()", async () => pipeline?.shutdown());
  }

  recordException(
    value: Error | NonErrorException,
    mechanism: ErrorMechanism,
    attributes: Attributes = {},
  ) {
    contain("captureError()", () => {
      const attrs: Attributes = {
        ...attributes,
        "interfere.exception.handled": mechanism.handled,
        "interfere.exception.mechanism": mechanism.type,
        "exception.type": isNonErrorException(value) ? value.type : value.name,
        "exception.message": isNonErrorException(value)
          ? value.value
          : value.message,
        "interfere.exception.kind": isNonErrorException(value)
          ? "non-error"
          : "error",
      };

      if (this.policy.clientIp === false) {
        delete attrs["http.request.header.x-forwarded-for"];
      }

      if (isNonErrorException(value)) {
        const serialized =
          value.serialized === undefined
            ? "undefined"
            : JSON.stringify(value.serialized);

        attrs["interfere.exception.serialized"] =
          // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Normalized string throws retain their literal value.
          typeof value.serialized === "string" ? value.serialized : serialized;
      } else {
        attrs["interfere.exception.chain"] = JSON.stringify(
          toExceptions(value, mechanism),
        );

        if (value.stack) {
          attrs["exception.stacktrace"] = value.stack;
        }
      }

      createUrlRedactionSpanProcessor(this.policy.urlRedaction).onEnd({
        attributes: {},
        events: [{ name: "exception", attributes: attrs }],
      });

      const statusMessage = String(attrs["exception.message"]).slice(
        0,
        ATTRIBUTE_VALUE_LENGTH_LIMIT,
      );

      const host = trace.getActiveSpan();

      const active =
        this.isolatedProvider && !this.policy.recordOnHostSpan
          ? undefined
          : host;

      const timestamp = Date.now();

      const separate =
        this.isolatedProvider !== null && this.policy.errors === "interfere";

      if (active?.isRecording()) {
        active.addEvent(
          "exception",
          separate
            ? {
                ...attrs,
                [EXCEPTION_DELIVERY_ATTRIBUTE]: EXCEPTION_DELIVERY.interfere,
              }
            : attrs,
          timestamp,
        );

        if (!mechanism.handled) {
          active.setStatus({
            code: SpanStatusCode.ERROR,
            message: statusMessage,
          });
        }

        if (!separate) {
          return;
        }
      }

      const tracer = (
        this.isolatedProvider ?? trace.getTracerProvider()
      ).getTracer(this.name);

      const parent = host?.spanContext();

      const span = tracer.startSpan(
        exceptionSpanName(value),
        omitUndefined({
          attributes: { [CAPTURE_KIND_ATTRIBUTE]: "exception" },
          kind: SpanKind.INTERNAL,
          links:
            this.isolatedProvider && parent && isSpanContextValid(parent)
              ? [{ context: parent }]
              : undefined,
        }),
        this.isolatedProvider ? ROOT_CONTEXT : undefined,
      );

      span.addEvent("exception", attrs, timestamp);

      if (!mechanism.handled) {
        span.setStatus({
          code: SpanStatusCode.ERROR,
          message: statusMessage,
        });
      }

      span.end();
    });
  }
}
