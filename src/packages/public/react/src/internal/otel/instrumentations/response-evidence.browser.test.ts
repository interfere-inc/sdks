import { failedSpan, response } from "./failed-response.fixture.js";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { WebTracerProvider } from "@opentelemetry/sdk-trace-web";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, test } from "vitest";
import { worker } from "../../../../test/msw.js";
import { requestSpans } from "../../../../test/request-spans.test-utils.js";
import { registerBundledInstrumentations } from "./instrumentations.js";
import type { BundledInstrumentations } from "./instrumentations.js";

let exporter: InMemorySpanExporter;
let provider: WebTracerProvider;
let handle: BundledInstrumentations;
const base = `${globalThis.location.origin}/failed-request-evidence`;

beforeEach(() => {
  exporter = new InMemorySpanExporter();

  provider = new WebTracerProvider({
    spanProcessors: [new SimpleSpanProcessor(exporter)],
  });

  handle = registerBundledInstrumentations({
    ignoreUrls: [],
    tracerProvider: provider,
  });
});

afterEach(async () => {
  handle.dispose();

  await provider.shutdown();
});

test("captures callsite, status and header presence while preserving the application response", async () => {
  expect.hasAssertions();

  const body = JSON.stringify({
    code: "PAYMENT_DECLINED",
    error_code: "ACCOUNT_BLOCKED",
    errorCode: "LIMIT",
    error: { code: "NESTED", message: "secret@example.com" },
    errors: [{ extensions: { code: "GRAPHQL" } }],
    message: "secret",
    type: "PrivateType",
  });

  let requests = 0;

  worker.use(
    http.post(`${base}/failure`, () => {
      requests += 1;

      return response(body);
    }),
  );

  async function customerAction() {
    return fetch(`${base}/failure`, {
      method: "POST",
      headers: { "Next-Action": "secret-action-id" },
      body: "private request",
    });
  }

  const result = await customerAction();

  expect(result.status).toBe(422);

  await expect(result.text()).resolves.toBe(body);

  const span = await failedSpan(exporter);

  const event = span.events.find((item) => item.name === "exception");

  expect(event?.attributes).toMatchObject({
    "interfere.http.capture_status": "captured",
    "http.response.status_code": 422,
    "http.request.next_action": true,
    "http.response.error_codes": [
      "PAYMENT_DECLINED",
      "ACCOUNT_BLOCKED",
      "LIMIT",
      "NESTED",
      "GRAPHQL",
    ],
  });

  expect(event?.attributes?.["exception.stacktrace"]).toContain(
    "customerAction",
  );

  expect(span.attributes["http.response.error_codes"]).toStrictEqual(
    event?.attributes?.["http.response.error_codes"],
  );

  expect(JSON.stringify(span.events)).not.toMatch(
    /secret|private request|PrivateType/u,
  );

  expect(requests).toBe(1);
});

test.each([
  [
    "missing-length",
    '{"code":"NO_LENGTH"}',
    { "content-length": "" },
    "skipped",
    "no-content-length",
  ],
  [
    "declared-large",
    '{"code":"TOO_LARGE"}',
    { "content-length": "65537" },
    "skipped",
    "over-64KiB",
  ],
  [
    "actual-large",
    JSON.stringify({ code: "TOO_LARGE", padding: "x".repeat(66_000) }),
    { "content-length": "12" },
    "skipped",
    "over-64KiB",
  ],
  [
    "html",
    '{"code":"HTML"}',
    { "content-type": "text/html" },
    "skipped",
    "non-json",
  ],
  ["invalid", '{"code":', {}, "skipped", "invalid-json"],
  [
    "freeform",
    JSON.stringify({
      error: "SECRET",
      type: "SECRET",
      message: "SECRET",
      code: "contains personal details",
      error_code: "secret@example.com",
    }),
    {},
    "captured",
    undefined,
  ],
])(
  "does not capture ineligible response details: %s",
  async (path, body, headers, status, reason) => {
    expect.hasAssertions();

    worker.use(http.get(`${base}/${path}`, () => response(body, headers)));

    const result = await fetch(`${base}/${path}`);

    await expect(result.text()).resolves.toBe(body);

    const span = await failedSpan(exporter);

    expect(span.attributes["http.response.error_codes"]).toBeUndefined();

    expect(span.attributes["interfere.http.capture_status"]).toBe(status);
    expect(span.attributes["interfere.http.capture_skip_reason"]).toBe(reason);

    expect(
      span.events.find((item) => item.name === "exception")?.attributes?.[
        "http.request.next_action"
      ],
    ).toBe(false);
  },
);

test("captures XHR protocol codes without changing its response", async () => {
  expect.hasAssertions();

  const body = JSON.stringify({
    code: "XHR_CODE",
    message: "private",
  });

  worker.use(http.get(`${base}/xhr`, () => response(body)));

  const xhr = new XMLHttpRequest();

  await new Promise<void>((resolve, reject) => {
    xhr.open("GET", `${base}/xhr`);

    xhr.responseType = "json";

    xhr.addEventListener("load", () => resolve());

    xhr.addEventListener("error", () => reject(new Error("XHR failed")));

    xhr.send();
  });

  expect(xhr.response).toStrictEqual({
    code: "XHR_CODE",
    message: "private",
  });

  const span = await failedSpan(exporter);

  expect(span.attributes["http.response.error_codes"]).toStrictEqual([
    "XHR_CODE",
  ]);

  expect(JSON.stringify(span.events)).not.toContain("private");
});

test("captureFailedRequests false leaves failure details unrecorded", async () => {
  expect.hasAssertions();

  handle.dispose();

  handle = registerBundledInstrumentations({
    ignoreUrls: [],
    tracerProvider: provider,
    captureFailedRequests: false,
  });

  const body = '{"code":"DISABLED"}';

  worker.use(http.get(`${base}/disabled`, () => response(body)));

  const result = await fetch(`${base}/disabled`);

  await expect(result.text()).resolves.toBe(body);

  await expect.poll(() => requestSpans(exporter).length).toBeGreaterThan(0);

  expect(
    requestSpans(exporter)
      .flatMap((span) => span.events)
      .filter((event) => event.name === "exception"),
  ).toStrictEqual([]);

  expect(
    requestSpans(exporter).some(
      (span) => span.attributes["http.response.error_codes"] !== undefined,
    ),
  ).toBe(false);
});

test("successful JSON responses do not produce failure evidence", async () => {
  expect.hasAssertions();

  worker.use(
    http.get(`${base}/success`, () => response('{"code":"SUCCESS"}', {}, 200)),
  );

  await expect((await fetch(`${base}/success`)).json()).resolves.toStrictEqual({
    code: "SUCCESS",
  });

  await expect.poll(() => requestSpans(exporter).length).toBeGreaterThan(0);

  expect(
    requestSpans(exporter)
      .flatMap((span) => span.events)
      .filter((event) => event.name === "exception"),
  ).toStrictEqual([]);
});

test("stops inspecting a slow body while the application still receives it", async () => {
  expect.hasAssertions();

  const body = '{"code":"LATE"}';

  worker.use(
    http.get(
      `${base}/slow`,
      () =>
        new HttpResponse(
          new ReadableStream({
            start(controller) {
              controller.enqueue(new TextEncoder().encode(body.slice(0, 5)));

              setTimeout(() => {
                controller.enqueue(new TextEncoder().encode(body.slice(5)));

                controller.close();
              }, 1200);
            },
          }),
          {
            status: 422,
            headers: {
              "content-type": "application/json",
              "content-length": String(body.length),
            },
          },
        ),
    ),
  );

  const result = await fetch(`${base}/slow`);

  await expect(result.text()).resolves.toBe(body);

  const span = await failedSpan(exporter);

  expect(span.attributes["http.response.error_codes"]).toBeUndefined();
  expect(span.attributes["interfere.http.capture_skip_reason"]).toBe("timeout");
});

test("preserves a caller's abort reason without retrying", async () => {
  expect.hasAssertions();

  let requests = 0;

  worker.use(
    http.get(`${base}/abort`, () => {
      requests += 1;

      return response('{"code":"ABORT"}');
    }),
  );

  const controller = new AbortController();
  const reason = new DOMException("Caller cancelled", "AbortError");

  controller.abort(reason);

  await expect(
    fetch(`${base}/abort`, { signal: controller.signal }),
  ).rejects.toBe(reason);

  expect(requests).toBe(0);
});

test("bounds XHR text by UTF8 bytes even when content-length is incorrect", async () => {
  expect.hasAssertions();

  const body = JSON.stringify({
    code: "OVERSIZED",
    padding: "界".repeat(24_000),
  });

  worker.use(
    http.get(`${base}/xhr-unicode`, () =>
      response(body, { "content-length": "24" }),
    ),
  );

  const xhr = new XMLHttpRequest();

  await new Promise<void>((resolve, reject) => {
    xhr.open("GET", `${base}/xhr-unicode`);

    xhr.responseType = "text";

    xhr.addEventListener("load", () => resolve());

    xhr.addEventListener("error", () => reject(new Error("XHR failed")));

    xhr.send();
  });

  expect(xhr.responseText).toBe(body);

  const span = await failedSpan(exporter);

  expect(span.attributes["http.response.error_codes"]).toBeUndefined();
});
