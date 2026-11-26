import { SpanKind, context } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";
import { HttpResponse, http } from "msw";
import { setupServer } from "msw/node";
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  expect,
  test,
} from "vitest";

import {
  ERROR_RESPONSE_EVENT,
  installFetchErrorEvidence,
} from "./fetch-error-evidence.js";

const ORDERS = "https://cqs.example.test/v2/orders";

const server = setupServer();

const exporter = new InMemorySpanExporter();

const tracer = new TracerProvider({
  spanProcessors: [new SimpleSpanProcessor({ exporter })],
}).getTracer("test");

let hostFetch: typeof fetch;

let restore: () => void;

beforeAll(() => {
  server.listen({ onUnhandledFrame: "error" });
});

beforeEach(() => {
  context.setGlobalContextManager(
    new AsyncLocalStorageContextManager().enable(),
  );

  hostFetch = globalThis.fetch;

  restore = installFetchErrorEvidence();
});

afterEach(() => {
  restore();

  context.disable();

  exporter.reset();

  server.resetHandlers();
});

afterAll(() => {
  server.close();
});

async function inSpan<T>(kind: SpanKind, run: () => Promise<T>) {
  return tracer.startActiveSpan("trpc.procedure", { kind }, async (span) => {
    try {
      return await run();
    } finally {
      span.end();
    }
  });
}

function finishedSpan() {
  const [span] = exporter.getFinishedSpans();

  return { attributes: span?.attributes, events: span?.events ?? [] };
}

test("a JSON 4xx records its codes and redacted message on the caller's span, and the app still reads the body", async () => {
  expect.assertions(2);

  server.use(
    http.post(ORDERS, () =>
      HttpResponse.json(
        {
          code: "VALIDATION_FAILED",
          message: "jane@example.com is not allowed",
        },
        { status: 422 },
      ),
    ),
  );

  const body = await inSpan(SpanKind.SERVER, async () => {
    const response = await fetch(ORDERS, { method: "POST" });

    return response.text();
  });

  expect(body).toBe(
    JSON.stringify({
      code: "VALIDATION_FAILED",
      message: "jane@example.com is not allowed",
    }),
  );

  expect(finishedSpan().events).toMatchObject([
    {
      name: ERROR_RESPONSE_EVENT,
      attributes: {
        "url.full": ORDERS,
        "http.response.status_code": 422,
        "http.response.error_codes": ["VALIDATION_FAILED"],
        "http.response.error_detail": "[email] is not allowed",
      },
    },
  ]);
});

test("on a client span the evidence becomes span attributes", async () => {
  expect.assertions(2);

  server.use(
    http.get(ORDERS, () =>
      HttpResponse.json({ detail: "Upstream timed out" }, { status: 503 }),
    ),
  );

  await inSpan(SpanKind.CLIENT, async () => fetch(ORDERS));

  expect(finishedSpan().events).toStrictEqual([]);

  expect(finishedSpan().attributes).toMatchObject({
    "http.response.status_code": 503,
    "http.response.error_detail": "Upstream timed out",
  });
});

test.each([
  ["a success", () => HttpResponse.json({ message: "ok" })],
  [
    "a non-JSON error",
    () => HttpResponse.html("<h1>Bad Gateway</h1>", { status: 502 }),
  ],
  [
    "a message-less error",
    () => HttpResponse.json({ data: null }, { status: 500 }),
  ],
])("%s records nothing and leaves the body unread", async (_name, reply) => {
  expect.assertions(2);

  server.use(http.get(ORDERS, reply));

  const response = await inSpan(SpanKind.SERVER, async () => fetch(ORDERS));

  expect(response.bodyUsed).toBe(false);

  expect(finishedSpan().events).toStrictEqual([]);
});

test("network failures reach the caller unchanged", async () => {
  expect.assertions(1);

  server.use(http.get(ORDERS, () => HttpResponse.error()));

  await expect(
    inSpan(SpanKind.SERVER, async () => fetch(ORDERS)),
  ).rejects.toThrow(TypeError);
});

test("installing twice wraps once, and restore puts back the host fetch", () => {
  expect.assertions(2);

  const wrapped = globalThis.fetch;

  installFetchErrorEvidence()();

  expect(globalThis.fetch).toBe(wrapped);

  restore();

  expect(globalThis.fetch).toBe(hostFetch);
});

test("outside any span an error response passes through untouched", async () => {
  expect.assertions(2);

  server.use(
    http.get(ORDERS, () =>
      HttpResponse.json({ message: "nope" }, { status: 400 }),
    ),
  );

  const response = await fetch(ORDERS);

  expect(response.bodyUsed).toBe(false);

  expect(exporter.getFinishedSpans()).toStrictEqual([]);
});
