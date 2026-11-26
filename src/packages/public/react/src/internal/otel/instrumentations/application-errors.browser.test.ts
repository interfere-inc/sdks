import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";
import { http } from "msw";
import { afterEach, beforeEach, expect, test, vi } from "vitest";
import { worker } from "../../../../test/msw.js";
import { requestSpans } from "../../../../test/request-spans.test-utils.js";
import { registerBundledInstrumentations } from "./instrumentations.js";
import { applyFetchSpanAttrs } from "./request/outcome.js";
import type { BundledInstrumentations } from "./instrumentations.js";
import type { CaptureFailedRequestsOption } from "./failure-policy.js";
import { response } from "./failed-response.fixture.js";
import type { ResponseEvidence } from "@interfere/types/sdk/response/evidence";

let exporter: InMemorySpanExporter;
let provider: TracerProvider;
let handle: BundledInstrumentations;
const base = `${globalThis.location.origin}/application-errors`;

function install(captureFailedRequests?: CaptureFailedRequestsOption) {
  handle = registerBundledInstrumentations({
    ignoreUrls: [],
    tracerProvider: provider,
    captureFailedRequests: captureFailedRequests ?? { applicationErrors: true },
  });
}

beforeEach(() => {
  exporter = new InMemorySpanExporter();

  provider = new TracerProvider({
    spanProcessors: [new SimpleSpanProcessor({ exporter })],
  });

  install();
});

afterEach(async () => {
  handle.dispose();

  vi.restoreAllMocks();

  await provider.shutdown();
});

async function xhrRequest(url: string) {
  const xhr = new XMLHttpRequest();

  await new Promise<void>((resolve, reject) => {
    xhr.open("GET", url);

    xhr.addEventListener("load", () => resolve(), { once: true });

    xhr.addEventListener("error", () => reject(new Error("XHR failed")), {
      once: true,
    });

    xhr.send();
  });

  return xhr;
}

test.each([
  { code: "INVITE_SECRET_123" },
  { code: "INVITE_SECRET_123", errorCode: "0" },
  { code: "INVITE_SECRET_123", error: { code: "none" } },
  { code: "INVITE_SECRET_123", data: { result: true }, errors: [] },
  {
    code: "INVITE_SECRET_123",
    data: { result: true },
    errors: [{ extensions: { code: "PARTIAL" } }],
  },
])(
  "fetch and XHR successes never emit code secrets or capture attributes: %j",
  async (body) => {
    expect.hasAssertions();

    worker.use(
      http.get(`${base}/success`, () =>
        response(JSON.stringify(body), {}, 200),
      ),
    );

    await expect(
      (await fetch(`${base}/success`)).json(),
    ).resolves.toStrictEqual(body);

    expect((await xhrRequest(`${base}/success`)).responseText).toBe(
      JSON.stringify(body),
    );

    await expect.poll(() => requestSpans(exporter).length).toBe(2);

    for (const span of requestSpans(exporter)) {
      expect(
        span.events.filter((event) => event.name === "exception"),
      ).toStrictEqual([]);

      expect(span.attributes["http.response.error_codes"]).toBeUndefined();
      expect(span.attributes["interfere.http.capture_status"]).toBeUndefined();

      expect(JSON.stringify(span.attributes)).not.toContain(
        "INVITE_SECRET_123",
      );
    }
  },
);

test("opted-in HTTP-200 failures retain only error-specific codes", async () => {
  const body = {
    code: "OAUTH_SECRET_123",
    error: { code: "DENIED", message: "private" },
  };

  worker.use(
    http.get(`${base}/failure`, () => response(JSON.stringify(body), {}, 200)),
  );

  await fetch(`${base}/failure`);

  await xhrRequest(`${base}/failure`);

  await expect
    .poll(
      () =>
        requestSpans(exporter)
          .flatMap((span) => span.events)
          .filter((event) => event.name === "exception").length,
    )
    .toBe(2);

  const spans = requestSpans(exporter);

  const errors = spans
    .flatMap((span) => span.events)
    .filter((event) => event.name === "exception");

  expect(
    errors.map((event) => event.attributes?.["http.response.error_codes"]),
  ).toStrictEqual([["DENIED"], ["DENIED"]]);

  expect(
    JSON.stringify(
      spans.map((span) => ({
        attributes: span.attributes,
        events: span.events,
      })),
    ),
  ).not.toMatch(/OAUTH_SECRET_123|private/u);
});

test.each([
  true,
  false,
  { statusCodes: [422] },
  { applicationErrors: false },
] satisfies CaptureFailedRequestsOption[])(
  "application error classification requires an explicit opt-in: %j",
  async (option) => {
    handle.dispose();

    install(option);

    worker.use(
      http.get(`${base}/disabled`, () =>
        response('{"errorCode":"DENIED"}', {}, 200),
      ),
    );

    await fetch(`${base}/disabled`);

    await xhrRequest(`${base}/disabled`);

    await expect.poll(() => requestSpans(exporter).length).toBe(2);

    expect(
      requestSpans(exporter)
        .flatMap((span) => span.events)
        .filter((event) => event.name === "exception"),
    ).toStrictEqual([]);

    expect(
      requestSpans(exporter).map(
        (span) => span.attributes["interfere.http.capture_status"],
      ),
    ).toStrictEqual([undefined, undefined]);
  },
);

test("a pending opted-in body never holds or stamps the successful request span", async () => {
  const { promise, resolve } = Promise.withResolvers<ResponseEvidence>();
  const tracer = provider.getTracer("test");
  const span = tracer.startSpan("fetch");
  // oxlint-disable-next-line typescript/unbound-method -- Compare method identity without invoking it unbound.
  const { end } = span;

  void applyFetchSpanAttrs(
    span,
    `${base}/pending`,
    new Response(null, { status: 200 }),
    [[400, 599]],
    { dispose: () => undefined, leftSince: () => false },
    () => true,
    undefined,
    undefined,
    promise,
    true,
    tracer,
  );

  // oxlint-disable-next-line typescript/unbound-method -- Compare method identity without invoking it unbound.
  expect(span.end).toBe(end);

  span.end();

  expect(exporter.getFinishedSpans()).toHaveLength(1);

  expect(
    exporter.getFinishedSpans()[0]?.attributes["interfere.http.capture_status"],
  ).toBeUndefined();

  resolve({ status: "captured", codes: [], applicationError: false });

  await promise;

  expect(exporter.getFinishedSpans()).toHaveLength(1);
});

test("default XHR response text is never read unless HTTP-200 inspection is enabled", async () => {
  handle.dispose();

  install(true);

  const responseText = vi.spyOn(
    XMLHttpRequest.prototype,
    "responseText",
    "get",
  );

  worker.use(
    http.get(`${base}/xhr`, () => response('{"errorCode":"DENIED"}', {}, 200)),
  );

  const xhr = await xhrRequest(`${base}/xhr`);

  await expect.poll(() => requestSpans(exporter).length).toBe(1);

  expect(
    responseText.mock.contexts.filter((context) => context === xhr),
  ).toHaveLength(0);

  expect(
    requestSpans(exporter)[0]?.attributes["interfere.http.capture_status"],
  ).toBeUndefined();
});

test("disposed opt-in capture adds no response clone beyond the instrumentation's own body-completion clone after default reinstallation", async () => {
  handle.dispose();

  install(true);

  const clone = vi.spyOn(Response.prototype, "clone");

  worker.use(
    http.get(`${base}/no-inspection`, () =>
      response('{"errorCode":"DENIED"}', {}, 200),
    ),
  );

  const result = await fetch(`${base}/no-inspection`);

  await expect(result.text()).resolves.toBe('{"errorCode":"DENIED"}');

  await expect.poll(() => requestSpans(exporter).length).toBe(1);

  expect(
    clone.mock.contexts.filter((context) => context === result),
  ).toHaveLength(1);

  expect(
    requestSpans(exporter)[0]?.events.filter(
      (event) => event.name === "exception",
    ),
  ).toStrictEqual([]);
});

test.each([422, 503])(
  "preserves legacy fetch and XHR error labels for HTTP %s",
  async (status) => {
    expect.hasAssertions();

    const body = {
      code: "PAYMENT_DECLINED",
      error_code: "0",
      errorCode: "none",
      error: { code: "false" },
      data: { result: true },
      errors: ["ok", "success", "null", "PARTIAL"].map((code) => ({
        extensions: { code },
      })),
    };

    const expected = [
      "PAYMENT_DECLINED",
      "0",
      "none",
      "false",
      "ok",
      "success",
      "null",
      "PARTIAL",
    ];

    worker.use(
      http.get(`${base}/legacy`, () =>
        response(JSON.stringify(body), {}, status),
      ),
    );

    await expect((await fetch(`${base}/legacy`)).json()).resolves.toStrictEqual(
      body,
    );

    const xhr = new XMLHttpRequest();

    await new Promise<void>((resolve, reject) => {
      xhr.open("GET", `${base}/legacy`);

      xhr.addEventListener("load", () => resolve(), { once: true });

      xhr.addEventListener("error", () => reject(new Error("XHR failed")), {
        once: true,
      });

      xhr.send();
    });

    expect(xhr.responseText).toBe(JSON.stringify(body));

    await expect.poll(() => requestSpans(exporter).length).toBe(2);

    for (const span of requestSpans(exporter)) {
      expect(span.attributes["http.response.error_codes"]).toStrictEqual(
        expected,
      );

      const events = span.events.filter((event) => event.name === "exception");

      expect(events).toHaveLength(1);

      expect(
        events[0]?.attributes?.["http.response.error_codes"],
      ).toStrictEqual(expected);

      expect(events[0]?.attributes?.["http.response.status_code"]).toBe(status);

      expect(
        span.attributes["interfere.http.application_error"],
      ).toBeUndefined();
    }
  },
);
