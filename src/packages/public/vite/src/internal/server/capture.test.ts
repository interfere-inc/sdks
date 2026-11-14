import { trace } from "@opentelemetry/api";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";

import { captureError } from "./capture.js";
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  expect,
  test,
  vi,
} from "vitest";

const exporter = new InMemorySpanExporter();

const provider = new BasicTracerProvider({
  spanProcessors: [new SimpleSpanProcessor(exporter)],
});

beforeAll(() => {
  trace.disable();

  trace.setGlobalTracerProvider(provider);
});

beforeEach(() => {
  exporter.reset();

  vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "interfere_pk_test_fake");
});

afterEach(() => vi.unstubAllEnvs());

afterAll(async () => {
  trace.disable();

  await provider.shutdown();
});

test("records request and stack attributes on exception events", () => {
  const error = new Error("request failed");

  error.stack =
    "Error: request failed\n    at createOrder (/srv/app/orders.ts:18:9)";

  captureError(error, { request: { method: "POST", path: "/orders" } });

  const [span] = exporter.getFinishedSpans();

  expect(span?.name).toBe("createOrder (orders.ts:18)");

  expect(span?.attributes).toMatchObject({
    "interfere.capture.kind": "exception",
  });

  expect(span?.events[0]?.attributes).toMatchObject({
    "exception.type": "Error",
    "exception.message": "request failed",
    "exception.stacktrace": error.stack,
    "http.request.method": "POST",
    "url.path": "/orders",
    "interfere.exception.handled": true,
  });
});

test("omits stack and request attributes when unavailable", () => {
  const error = new Error("without stack");

  delete error.stack;

  captureError(error);

  const [span] = exporter.getFinishedSpans();
  const attributes = span?.events[0]?.attributes;

  expect(span?.name).toBe("Error");

  expect(attributes).toMatchObject({ "exception.message": "without stack" });
  expect(attributes).not.toHaveProperty("exception.stacktrace");
  expect(attributes).not.toHaveProperty("http.request.method");
  expect(attributes).not.toHaveProperty("url.path");
});

test("retains non-error rejection classification and request context", () => {
  captureError(42, { request: { method: "GET", path: "/status" } });

  const [span] = exporter.getFinishedSpans();

  expect(span?.name).toBe("number");

  expect(span?.events[0]?.attributes).toMatchObject({
    "exception.type": "number",
    "interfere.exception.kind": "non-error",
    "http.request.method": "GET",
    "url.path": "/status",
  });
});

test.each([null, undefined, 42, false, "failure", { detail: "failure" }])(
  "captures repeated arbitrary rejection value %j independently",
  (value) => {
    captureError(value);

    captureError(value);

    expect(exporter.getFinishedSpans()).toHaveLength(2);

    expect(
      exporter.getFinishedSpans()[0]?.events[0]?.attributes?.[
        "exception.message"
      ],
    ).toStrictEqual(expect.any(String));
  },
);
