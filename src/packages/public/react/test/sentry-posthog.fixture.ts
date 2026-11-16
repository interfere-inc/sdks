import {
  breadcrumbsIntegration,
  browserTracingIntegration,
  init as initializeSentry,
} from "@sentry/browser";
import { posthog } from "posthog-js/dist/module.full.no-external";
import { z } from "zod";
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

export interface Options {
  vendor: "sentry" | "posthog";
  order: "vendor-first" | "interfere-first";
}

const fetchHint = z.object({
  response: z.instanceof(Response),
  input: z.custom<Parameters<typeof fetch>>(Array.isArray),
});

const spans = new InMemorySpanExporter();
const logs = new InMemoryLogRecordExporter();
let kernel: Awaited<ReturnType<typeof createKernel>>;
let breadcrumbs = 0;

function initializeVendor(vendor: Options["vendor"]) {
  if (vendor === "sentry") {
    initializeSentry({
      dsn: `http://fixture@${location.host}/1`,
      tunnel: `${location.origin}/sentry`,
      defaultIntegrations: false,
      integrations: [
        breadcrumbsIntegration({
          console: false,
          dom: false,
          history: false,
        }),
        browserTracingIntegration(),
      ],
      tracesSampleRate: 1,
      propagateTraceparent: true,
      tracePropagationTargets: [location.origin],
      beforeSend: () => null,
      beforeSendTransaction: () => null,
      beforeBreadcrumb(breadcrumb, hint) {
        if (breadcrumb.category === "fetch") {
          const parsed = fetchHint.safeParse(hint);

          if (parsed.success) {
            const [input, init] = parsed.data.input;

            correlateRequestTrace(
              parsed.data.response,
              init?.headers ?? (input instanceof Request ? input.headers : {}),
            );

            breadcrumbs += 1;
          }
        }

        return breadcrumb;
      },
    });

    return;
  }

  posthog.init("phc_fixture", {
    api_host: location.origin,
    ui_host: location.origin,
    autocapture: false,
    capture_pageview: false,
    capture_pageleave: false,
    disable_session_recording: true,
    disable_surveys: true,
    advanced_disable_flags: true,
    person_profiles: "never",
    tracing_headers: [location.hostname],
  });
}

async function install(options: Options) {
  Object.assign(globalThis, {
    __INTERFERE_PUBLIC_KEY__: "interfere_pk_test",
    __INTERFERE_FORCE_ENABLE__: true,
  });

  if (options.order === "vendor-first") {
    initializeVendor(options.vendor);
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
        propagation: options.vendor === "sentry" ? "external" : "interfere",
        propagateTraceBaggage: false,
      },
    },
  });

  if (options.order === "interfere-first") {
    initializeVendor(options.vendor);
  }
}

async function request(kind: "fetch" | "request" | "xhr") {
  const url = `${location.origin}/echo`;

  const init = {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: "{}",
  };

  if (kind === "xhr") {
    return new Promise<{
      kind: typeof kind;
      traceparent: string | null;
      session: string | null;
      status: number;
    }>((resolve, reject) => {
      const xhr = new XMLHttpRequest();

      xhr.open("POST", url);

      xhr.setRequestHeader("content-type", "application/json");

      xhr.addEventListener(
        "load",
        () =>
          resolve({
            kind,
            traceparent: xhr.getResponseHeader("x-wire-traceparent"),
            session: xhr.getResponseHeader("x-wire-posthog-session"),
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
    kind,
    traceparent: response.headers.get("x-wire-traceparent"),
    session: response.headers.get("x-wire-posthog-session"),
    status: response.status,
  };
}

const REQUEST_SPAN_END_MS = 400;

async function snapshot() {
  await new Promise((resolve) => {
    setTimeout(resolve, REQUEST_SPAN_END_MS);
  });

  await kernel.flush();

  return [
    ...logs
      .getFinishedLogRecords()
      .map((record) => ({ attributes: record.attributes })),
    ...spans
      .getFinishedSpans()
      .map((span) => ({ attributes: span.attributes })),
  ].filter(
    (record) => record.attributes["url.full"] === `${location.origin}/echo`,
  );
}

const fixture = {
  install,
  installVendorOnly: initializeVendor,
  request,
  snapshot,
  breadcrumbs: () => breadcrumbs,
  posthogSession: () => posthog.get_session_id(),
};

Object.assign(globalThis, { sentryPosthogFixture: fixture });

declare global {
  var sentryPosthogFixture: typeof fixture;
}
