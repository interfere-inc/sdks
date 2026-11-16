import { datadogRum } from "@datadog/browser-rum";
import {
  InMemoryLogRecordExporter,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { correlateRequestTrace } from "../src/request-trace.js";
import { createKernel } from "../src/internal/kernel.js";

interface Options {
  order: "datadog-first" | "interfere-first";
  baggage: boolean;
  sampled: boolean;
  injectAll?: boolean;
  holdBridge?: boolean;
}

const spans = new InMemorySpanExporter();
const logs = new InMemoryLogRecordExporter();
let kernel: Awaited<ReturnType<typeof createKernel>>;
let resourceEvents = 0;
const pending: (() => void)[] = [];
let disposed = false;

function initializeDatadog(options: Options) {
  datadogRum.init({
    applicationId: "00000000-0000-4000-8000-000000000001",
    clientToken: "fixture",
    site: "datadoghq.com",
    service: "interfere-compatibility",
    sessionSampleRate: 100,
    sessionReplaySampleRate: 0,
    traceSampleRate: options.sampled ? 100 : 0,
    traceContextInjection: options.injectAll ? "all" : "sampled",
    trackResources: true,
    propagateTraceBaggage: false,
    allowedTracingUrls: [
      { match: location.origin, propagatorTypes: ["tracecontext"] },
    ],
    proxy: `${location.origin}/datadog`,
    beforeSend(event, context) {
      if (
        event.type === "resource" &&
        "response" in context &&
        context.response
      ) {
        const { response } = context;

        const headers =
          context.requestInit?.headers ??
          (context.requestInput instanceof Request
            ? context.requestInput.headers
            : {});

        if (options.holdBridge) {
          pending.push(() => correlateRequestTrace(response, headers));
        } else {
          correlateRequestTrace(response, headers);
        }
      }

      if (event.type === "resource" && event.resource.url.endsWith("/echo")) {
        resourceEvents += 1;
      }

      return false;
    },
  });
}

async function install(options: Options) {
  Object.assign(globalThis, {
    __INTERFERE_PUBLIC_KEY__: "interfere_pk_test",
    __INTERFERE_FORCE_ENABLE__: true,
  });

  if (options.order === "datadog-first") {
    initializeDatadog(options);
  }

  kernel = await createKernel({
    onCreated: (created) => {
      created.consent.set({ replay: false });
    },
    opts: {
      apiHost: location.origin,
      _internalAdditionalSpanProcessors: [new SimpleSpanProcessor(spans)],
      _internalAdditionalLogRecordProcessors: [
        new SimpleLogRecordProcessor({ exporter: logs }),
      ],
      tracing: {
        propagation: "external",
        propagateTraceBaggage: options.baggage,
      },
    },
  });

  if (options.order === "interfere-first") {
    initializeDatadog(options);
  }
}

async function request(kind: "fetch" | "request" | "xhr") {
  const url = `${location.origin}/echo`;

  const init = {
    method: "POST",
    body: "{}",
    headers: { "content-type": "application/json" },
  };

  if (kind === "xhr") {
    return new Promise<{
      traceparent: string | null;
      baggage: string | null;
      status: number;
    }>((resolve, reject) => {
      const xhr = new XMLHttpRequest();

      xhr.open("POST", url);

      xhr.setRequestHeader("content-type", "application/json");

      xhr.addEventListener(
        "load",
        () =>
          resolve({
            traceparent: xhr.getResponseHeader("x-wire-traceparent"),
            baggage: xhr.getResponseHeader("x-wire-baggage"),
            status: xhr.status,
          }),
        { once: true },
      );

      xhr.addEventListener("error", () => reject(new Error("XHR failed")), {
        once: true,
      });

      xhr.send("{}");
    });
  }

  const response = await (kind === "fetch"
    ? fetch(url, init)
    : fetch(new Request(url, init)));

  await response.text();

  return {
    traceparent: response.headers.get("x-wire-traceparent"),
    baggage: response.headers.get("x-wire-baggage"),
    status: response.status,
  };
}

const REQUEST_SPAN_END_MS = 400;

async function snapshot() {
  await new Promise((resolve) => {
    setTimeout(resolve, REQUEST_SPAN_END_MS);
  });

  if (!disposed) {
    await kernel.flush();
  }

  return [
    ...logs.getFinishedLogRecords().map((record) => ({
      traceId: record.spanContext?.traceId,
      attributes: record.attributes,
    })),
    ...spans.getFinishedSpans().map((span) => ({
      traceId: span.spanContext().traceId,
      attributes: span.attributes,
    })),
  ].filter(
    (record) => record.attributes["url.full"] === `${location.origin}/echo`,
  );
}

const fixture = {
  install,
  request,
  snapshot,
  ready: () => Boolean(datadogRum.getInternalContext()?.session_id),
  resourceEvents: () => resourceEvents,
  release: () => {
    for (const deliver of pending.splice(0)) {
      deliver();
    }
  },
  revoke: () => kernel.consent.set(false),
  revokeRegrant: () => {
    kernel.consent.set(false);

    kernel.consent.set({ replay: false });
  },
  rotate: () => kernel.identity.clear(),
  session: () => kernel.session.getId(),
  dispose: async () => {
    await kernel.dispose();

    disposed = true;
  },
};

Object.assign(globalThis, { datadogFixture: fixture });

declare global {
  var datadogFixture: typeof fixture;
}
