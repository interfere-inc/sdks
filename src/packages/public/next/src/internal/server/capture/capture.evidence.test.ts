import { runInNewContext } from "node:vm";
import { trace } from "@opentelemetry/api";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

import { captureError, onRequestError } from "./capture.js";

let exporter: InMemorySpanExporter;
let provider: BasicTracerProvider;

beforeEach(() => {
  vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

  vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_test_123");

  exporter = new InMemorySpanExporter();

  provider = new BasicTracerProvider({
    spanProcessors: [new SimpleSpanProcessor(exporter)],
  });

  trace.disable();

  trace.setGlobalTracerProvider(provider);
});

afterEach(async () => {
  trace.disable();

  await provider.shutdown();

  vi.unstubAllEnvs();
});

function attributes() {
  const spans = exporter.getFinishedSpans();

  expect(spans).toHaveLength(1);

  return spans[0]?.events[0]?.attributes;
}

test("retains nested causes with their own messages and stacks", () => {
  const root = new TypeError("database failed");

  root.stack =
    "TypeError: database failed\n at query (/srv/customer/database.ts:7:2)";

  const error = new Error("request failed", { cause: root });

  error.stack =
    "Error: request failed\n at route (/srv/customer/route.ts:18:4)";

  captureError(error);

  expect(
    JSON.parse(String(attributes()?.["interfere.exception.chain"])),
  ).toStrictEqual([
    {
      kind: "error",
      mechanism: { handled: true, type: "auto.function.nextjs.capture_error" },
      stack: error.stack,
      type: "Error",
      value: "request failed",
    },
    {
      kind: "error",
      stack: root.stack,
      type: "TypeError",
      value: "database failed",
    },
  ]);
});

test("bounds cyclic causes without dropping the exception", () => {
  const error = new Error("cyclic failure");

  error.cause = error;

  captureError(error);

  expect(
    JSON.parse(String(attributes()?.["interfere.exception.chain"])),
  ).toHaveLength(1);
});

test("caps deep cause chains before export", () => {
  let error = new Error("oldest cause");

  for (let index = 0; index < 8; index += 1) {
    error = new Error(`cause ${index}`, { cause: error });
  }

  captureError(error);

  const serialized = String(attributes()?.["interfere.exception.chain"]);

  expect(JSON.parse(serialized)).toHaveLength(5);
  expect(serialized).toContain("cause 7");
  expect(serialized).not.toContain("oldest cause");
});

test("preserves structured throws with circular references and non-JSON leaves", () => {
  const value = { code: "QUOTA", detail: { remaining: 0 }, count: 12n };

  Object.assign(value, { self: value });

  captureError(value);

  const attrs = attributes();

  expect(attrs?.["interfere.exception.kind"]).toBe("non-error");
  expect(attrs).not.toHaveProperty("exception.stacktrace");

  expect(
    JSON.parse(String(attrs?.["interfere.exception.serialized"])),
  ).toStrictEqual({
    code: "QUOTA",
    detail: { remaining: 0 },
    count: "12",
    self: "[circular]",
  });
});

test("retains automatic Edge errors crossing Next's VM boundary", async () => {
  const error = runInNewContext(
    'new Error("qualification: edge error")',
  ) as Error;

  expect(error).not.toBeInstanceOf(Error);

  await onRequestError(
    error,
    { method: "GET", path: "/api/edge" },
    {
      routerKind: "App Router",
      routeType: "route",
      routePath: "/api/edge",
    },
  );

  expect(attributes()).toMatchObject({
    "exception.message": "qualification: edge error",
    "exception.stacktrace": error.stack,
    "interfere.exception.kind": "error",
    "interfere.exception.handled": false,
    "interfere.exception.mechanism": "auto.function.nextjs.route_handler",
  });
});
