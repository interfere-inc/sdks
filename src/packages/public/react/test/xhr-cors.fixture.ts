import { context, propagation } from "@opentelemetry/api";
import { W3CTraceContextPropagator } from "@opentelemetry/core";
import { StackContextManager, TracerProvider } from "@opentelemetry/sdk-trace";
import { registerBundledInstrumentations } from "../src/internal/otel/instrumentations/instrumentations.js";
import { w3cTraceHeaders } from "./trace-headers.test-utils.js";
import type { BundledInstrumentations } from "../src/internal/otel/instrumentations/instrumentations.js";

let instrumentations: BundledInstrumentations | undefined;

const fixture = {
  install(options: { backendHosts?: string[]; propagateTo?: string[] }) {
    instrumentations?.dispose();

    context.disable();

    context.setGlobalContextManager(new StackContextManager().enable());

    propagation.disable();

    propagation.setGlobalPropagator(new W3CTraceContextPropagator());

    instrumentations = registerBundledInstrumentations({
      ...options,
      ignoreUrls: [],
      traceHeaders: w3cTraceHeaders(),
      tracerProvider: new TracerProvider(),
    });
  },
  async request(
    url: string,
    options: { abort?: boolean; timeout?: number; traceHeader?: boolean } = {},
  ) {
    return new Promise<{ status: number; body: string; events: string[] }>(
      (resolve) => {
        const xhr = new XMLHttpRequest();
        const events: string[] = [];

        for (const event of [
          "readystatechange",
          "loadstart",
          "load",
          "error",
          "abort",
          "timeout",
          "loadend",
        ]) {
          xhr.addEventListener(event, () => {
            events.push(
              event === "readystatechange"
                ? `${event}:${xhr.readyState}`
                : event,
            );
          });
        }

        xhr.onloadend = () =>
          resolve({ status: xhr.status, body: xhr.responseText, events });

        xhr.open("POST", url);

        xhr.timeout = options.timeout ?? 0;

        if (options.traceHeader) {
          xhr.setRequestHeader(
            "traceparent",
            "00-12345678901234567890123456789012-1234567890123456-01",
          );
        }

        xhr.send("customer-order");

        if (options.abort) {
          xhr.abort();
        }
      },
    );
  },
};

declare global {
  var xhrCorsFixture: typeof fixture;
}

globalThis.xhrCorsFixture = fixture;
