import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import {
  SeverityNumber,
  context,
  propagation,
  trace,
} from "@opentelemetry/api";
import { omitUndefined } from "@interfere/helpers/omit-undefined";

import {
  InMemoryLogRecordExporter,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace";

import { DISABLE_ALL_PLUGINS } from "../../test/fixtures/disable-all-plugins.js";
import { createKernel } from "./kernel.js";
import type { Kernel } from "./kernel.js";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

import { worker } from "../../test/msw.js";

let ownedKernel: Kernel | undefined;

beforeEach(() => {
  ownedKernel = undefined;

  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", "interfere_pk_test");

  setInjectedConfiguration("__INTERFERE_INGEST_URL__", undefined);

  trace.disable();

  context.disable();

  propagation.disable();
});

afterEach(async () => {
  try {
    await ownedKernel?.dispose();
  } finally {
    trace.disable();

    context.disable();

    propagation.disable();

    vi.unstubAllGlobals();
  }
});

function recordTracingRequests(requests: Request[]) {
  worker.use(
    http.get(`${location.origin}/tracing-test`, ({ request }) =>
      HttpResponse.json({ baggage: request.headers.get("baggage") }),
    ),
    http.get("https://kernel.invalid/v3/config", () =>
      HttpResponse.json({ plugins: DISABLE_ALL_PLUGINS }),
    ),
    http.post(
      "https://kernel.invalid/v3/session",
      () => new HttpResponse(null, { status: 204 }),
    ),
    http.all("https://kernel.invalid/*", ({ request }) => {
      requests.push(request);

      return new HttpResponse(null, { status: 204 });
    }),
  );
}

test("exports SDK spans and logs when tracing uses its default", async () => {
  expect.hasAssertions();

  const tracing = undefined;

  const spans = new InMemorySpanExporter();
  const logs = new InMemoryLogRecordExporter();
  const requests: Request[] = [];

  recordTracingRequests(requests);

  const kernel = await createKernel({
    opts: omitUndefined({
      apiHost: "https://kernel.invalid",
      _internalAdditionalSpanProcessors: [
        new SimpleSpanProcessor({ exporter: spans }),
      ],
      _internalAdditionalLogRecordProcessors: [
        new SimpleLogRecordProcessor({ exporter: logs }),
      ],
      tracing,
      serviceName: "storefront",
      serviceNamespace: "shop",
    }),
  });

  ownedKernel = kernel;

  const response = await fetch(`${location.origin}/tracing-test`);

  await expect(response.json()).resolves.toStrictEqual({
    baggage: `interfere.session.id=${kernel.session.getId()}`,
  });

  const span = kernel.getTracer("bootstrap-test").startSpan("bootstrap-work");

  expect(span.isRecording()).toBe(true);

  span.setAttribute("test.operation", "bootstrap");

  span.end();

  kernel.recordLog({
    severityNumber: SeverityNumber.INFO,
    severityText: "INFO",
    body: "bootstrap-log",
  });

  await kernel.flush();

  const finished = spans
    .getFinishedSpans()
    .filter((record) => record.name === "bootstrap-work");

  const emitted = logs.getFinishedLogRecords();

  expect(finished).toHaveLength(1);
  expect(emitted).toHaveLength(1);

  expect(finished.map((record) => record.attributes)).toMatchObject([
    {
      "test.operation": "bootstrap",
      "session.id": kernel.session.getId(),
    },
  ]);

  expect(finished.map((record) => record.resource.attributes)).toMatchObject([
    { "service.name": "storefront", "service.namespace": "shop" },
  ]);

  expect(emitted.map((record) => record.resource.attributes)).toMatchObject([
    { "service.name": "storefront", "service.namespace": "shop" },
  ]);

  expect(emitted).toMatchObject([
    {
      body: "bootstrap-log",
      severityText: "INFO",
      severityNumber: SeverityNumber.INFO,
      attributes: { "session.id": kernel.session.getId() },
    },
  ]);

  expect(requests.length).toBeGreaterThan(0);

  expect(
    requests.some(
      (request) => request.url === "https://kernel.invalid/drain/v1/traces",
    ),
  ).toBe(true);

  expect(
    requests.some(
      (request) => request.url === "https://kernel.invalid/drain/v1/logs",
    ),
  ).toBe(true);

  await expect(kernel.dispose()).resolves.toBeUndefined();

  expect(kernel.session.getId()).toBeNull();

  for (const request of requests) {
    expect(request.method).toBe("POST");

    expect([
      "https://kernel.invalid/drain/v1/traces",
      "https://kernel.invalid/drain/v1/logs",
      "https://kernel.invalid/drain/v1/metrics",
    ]).toContain(request.url);
  }
});

test("does not export SDK spans or logs when tracing is disabled", async () => {
  expect.hasAssertions();

  const tracing = false as const;

  const spans = new InMemorySpanExporter();
  const logs = new InMemoryLogRecordExporter();
  const requests: Request[] = [];

  recordTracingRequests(requests);

  const kernel = await createKernel({
    opts: omitUndefined({
      apiHost: "https://kernel.invalid",
      _internalAdditionalSpanProcessors: [
        new SimpleSpanProcessor({ exporter: spans }),
      ],
      _internalAdditionalLogRecordProcessors: [
        new SimpleLogRecordProcessor({ exporter: logs }),
      ],
      tracing,
      serviceName: "storefront",
      serviceNamespace: "shop",
    }),
  });

  ownedKernel = kernel;

  const response = await fetch(`${location.origin}/tracing-test`);

  await expect(response.json()).resolves.toStrictEqual({
    baggage: null,
  });

  const span = kernel.getTracer("bootstrap-test").startSpan("bootstrap-work");

  expect(span.isRecording()).toBe(false);

  span.setAttribute("test.operation", "bootstrap");

  span.end();

  kernel.recordLog({
    severityNumber: SeverityNumber.INFO,
    severityText: "INFO",
    body: "bootstrap-log",
  });

  await kernel.flush();

  const finished = spans
    .getFinishedSpans()
    .filter((record) => record.name === "bootstrap-work");

  const emitted = logs.getFinishedLogRecords();

  expect(finished).toHaveLength(0);
  expect(emitted).toHaveLength(0);

  expect(finished.map((record) => record.attributes)).toMatchObject([]);

  expect(finished.map((record) => record.resource.attributes)).toMatchObject(
    [],
  );

  expect(emitted.map((record) => record.resource.attributes)).toMatchObject([]);

  expect(emitted).toMatchObject([]);

  expect(requests).toHaveLength(0);

  expect(
    requests.some(
      (request) => request.url === "https://kernel.invalid/drain/v1/traces",
    ),
  ).toBe(false);

  expect(
    requests.some(
      (request) => request.url === "https://kernel.invalid/drain/v1/logs",
    ),
  ).toBe(false);

  await expect(kernel.dispose()).resolves.toBeUndefined();

  expect(kernel.session.getId()).toBeNull();

  for (const request of requests) {
    expect(request.method).toBe("POST");

    expect([
      "https://kernel.invalid/drain/v1/traces",
      "https://kernel.invalid/drain/v1/logs",
      "https://kernel.invalid/drain/v1/metrics",
    ]).toContain(request.url);
  }
});

async function sendXhr(url: string) {
  return new Promise<void>((resolve, reject) => {
    const xhr = new XMLHttpRequest();

    xhr.open("GET", url);

    xhr.addEventListener("load", () => resolve(), { once: true });

    xhr.addEventListener("error", () => reject(new Error("XHR failed")), {
      once: true,
    });

    xhr.send();
  });
}

test.each([
  ["fetch", async (url: string) => fetch(url)],
  ["XHR", sendXhr],
] as const)(
  "%s propagates sessions to application hosts and follows consent changes",
  async (_transport, send) => {
    expect.hasAssertions();

    const headers = new Map<
      string,
      { baggage: string | null; traceparent: string | null }
    >();

    const sameOriginUrl = `${location.origin}/session-baggage-test`;
    const apiUrl = "https://api.example.test/graphql";
    const thirdPartyUrl = "https://unrelated.example.test/resource";

    worker.use(
      http.get("https://kernel.invalid/v3/config", () =>
        HttpResponse.json({ plugins: DISABLE_ALL_PLUGINS }),
      ),
      http.all(
        "https://kernel.invalid/*",
        () => new HttpResponse(null, { status: 204 }),
      ),
      ...[sameOriginUrl, apiUrl, thirdPartyUrl].map((url) =>
        http.get(url, ({ request }) => {
          headers.set(url, {
            baggage: request.headers.get("baggage"),
            traceparent: request.headers.get("traceparent"),
          });

          return new HttpResponse(null, { status: 204 });
        }),
      ),
    );

    const kernel = await createKernel({
      onCreated: (created) => {
        created.consent.set(true);
      },
      opts: {
        apiHost: "https://kernel.invalid",
        tracing: { propagateTo: ["https://api.example.test"] },
      },
    });

    ownedKernel = kernel;

    const sessionHeader = `interfere.session.id=${kernel.session.getId()}`;
    const traceHeader = /^00-[0-9a-f]{32}-[0-9a-f]{16}-01$/u;

    await Promise.all(
      [sameOriginUrl, apiUrl, thirdPartyUrl].map(async (url) => send(url)),
    );

    for (const url of [sameOriginUrl, apiUrl]) {
      expect(headers.get(url)?.baggage).toBe(sessionHeader);
      expect(headers.get(url)?.traceparent).toMatch(traceHeader);
    }

    expect(headers.get(thirdPartyUrl)).toStrictEqual({
      baggage: null,
      traceparent: null,
    });

    kernel.consent.set(false);

    await send(sameOriginUrl);

    const pageHeader = `interfere.session.id=${String(kernel.session.getId())}`;

    expect(pageHeader).toMatch(/[=]page_/u);
    expect(headers.get(sameOriginUrl)?.baggage).toBe(pageHeader);
    expect(headers.get(sameOriginUrl)?.traceparent).toMatch(traceHeader);

    kernel.consent.set(true);

    await send(sameOriginUrl);

    expect(headers.get(sameOriginUrl)?.baggage).toBe(
      `interfere.session.id=${String(kernel.session.getId())}`,
    );

    expect(headers.get(sameOriginUrl)?.baggage).not.toBe(pageHeader);
  },
);
