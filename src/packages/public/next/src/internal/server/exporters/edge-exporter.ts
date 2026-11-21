import {
  TELEMETRY_EXPORT_TIMEOUT_MS,
  retryTelemetry,
} from "@interfere/helpers/retry-telemetry";
import { InterfereClient } from "@interfere/javascript";
/* oxlint-disable unicorn/no-useless-promise-resolve-reject */
import { ExportResultCode } from "@opentelemetry/core";
import type { ExportResult } from "@opentelemetry/core";
import { JsonTraceSerializer } from "@opentelemetry/otlp-transformer";
import type { ReadableSpan, SpanExporter } from "@opentelemetry/sdk-trace";
import { exportTracePayload } from "../tracing/trace-payload";

const NEXT_ORIGINAL_FETCH = "_nextOriginalFetch";

interface NextPatchedFetch {
  readonly [NEXT_ORIGINAL_FETCH]?: typeof fetch;
}

function untracedFetch() {
  const host: typeof fetch & NextPatchedFetch = globalThis.fetch;

  // WARNING: Next's patched fetch traces each call as AppRender.fetch on the global provider; exporting through it exports every export forever.
  return host[NEXT_ORIGINAL_FETCH] ?? host;
}

export class FetchTraceExporter implements SpanExporter {
  private readonly client: InterfereClient;

  constructor(options: InterfereClient.Options) {
    this.client = new InterfereClient({
      ...options,
      maxRetries: 0,
      timeoutInSeconds: TELEMETRY_EXPORT_TIMEOUT_MS / 1000,
      fetch: Object.assign(
        async (
          url: Parameters<typeof fetch>[0],
          init?: Parameters<typeof fetch>[1],
        ) =>
          retryTelemetry(
            async (signal) =>
              untracedFetch()(url, { ...init, keepalive: true, signal }),
            init?.signal,
          ),
        globalThis.fetch,
      ),
    });
  }

  export(
    spans: ReadableSpan[],
    resultCallback: (result: ExportResult) => void,
  ) {
    if (spans.length === 0) {
      resultCallback({ code: ExportResultCode.SUCCESS });

      return;
    }

    const payload = JsonTraceSerializer.serializeRequest(spans);

    exportTracePayload(payload, this.client, resultCallback);
  }

  async shutdown() {
    return Promise.resolve();
  }

  async forceFlush() {
    return Promise.resolve();
  }
}
