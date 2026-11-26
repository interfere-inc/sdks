import { once } from "node:events";
import { createServer } from "node:http";
import type { AddressInfo } from "node:net";
import { attribute, exportedSpans } from "@interfere/test-utils/vitest/otlp";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { context, logs, propagation, trace } from "@opentelemetry/api";
import type { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import { W3CTraceContextPropagator } from "@opentelemetry/core";
import type { TracerProvider } from "@opentelemetry/sdk-trace";
import type { LoggerProvider } from "@opentelemetry/sdk-logs";
import {
  afterEach,
  beforeEach,
  expect,
  onTestFinished,
  test,
  vi,
} from "vitest";

import { startNodeRuntime } from "./node.js";
import type { NodeRuntimeInput } from "./node.js";
import { ServerRuntime } from "./runtime.js";

let collector: Awaited<ReturnType<typeof receiver>>;
let runtime: ServerRuntime;
let input: NodeRuntimeInput;
let host: TracerProvider | undefined;
let manager: AsyncLocalStorageContextManager | undefined;
let hostLogger: LoggerProvider | undefined;

beforeEach(async () => {
  collector = await receiver();

  runtime = new ServerRuntime("core-contract");

  input = {
    name: "core-contract",
    version: "1.0.0",
    apiUrl: collector.url,
    credential: { apiKey: "core-contract" },
    resourceAttributes: {},
    options: { consoleBridge: false },
    drain: false,
    logDrain: false,
    vercel: false,
    propagator: new W3CTraceContextPropagator(),
  };
});

afterEach(async () => {
  await runtime.shutdown();

  await host?.shutdown();

  await hostLogger?.shutdown();

  hostLogger = undefined;

  host = undefined;

  manager?.disable();

  manager = undefined;

  trace.disable();

  context.disable();

  propagation.disable();

  logs.disable();

  await collector.close();

  vi.restoreAllMocks();

  vi.unstubAllEnvs();
});

test("a multi-megabyte exception is truncated so its batch stays under the gateway cap with its siblings", async () => {
  await runtime.register(async () => startNodeRuntime(input));

  const tracer = trace.getTracer("server");

  tracer.startActiveSpan("GET /route", (request) => {
    tracer.startSpan("sibling", { attributes: { ok: true } }).end();

    runtime.recordException(new Error("x".repeat(9 * 1024 * 1024)), {
      type: "auto.node.request_error",
      handled: false,
    });

    request.end();
  });

  await runtime.flush();

  const traces = collector.requests.filter((request) =>
    request.url.startsWith("/drain/v1/traces"),
  );

  expect(traces.every((request) => request.body.length < 8 * 1024 * 1024)).toBe(
    true,
  );

  const spans = exportedSpans(collector.requests);

  expect(spans.map((span) => span.name).toSorted()).toStrictEqual([
    "GET /route",
    "sibling",
  ]);

  const message = attribute(
    spans.find((span) => span.name === "GET /route")?.events[0]?.attributes ??
      [],
    "exception.message",
  );

  expect(String(message)).toMatch(/^x{1000,65536}$/u);
});

test("a 429 with Retry-After is retried and the batch is delivered exactly once", async () => {
  const statuses: number[] = [];

  const bodies: string[] = [];

  const server = createServer((request, response) => {
    const chunks: Buffer[] = [];

    request.on("data", (chunk: Buffer) => chunks.push(chunk));

    request.on("end", () => {
      const status = statuses.length === 0 ? 429 : 200;

      statuses.push(status);

      bodies.push(Buffer.concat(chunks).toString());

      response.writeHead(
        status,
        status === 429
          ? { "content-type": "application/json", "retry-after": "1" }
          : { "content-type": "application/json" },
      );

      response.end("{}");
    });
  });

  server.listen(0, "127.0.0.1");

  await once(server, "listening");

  onTestFinished(async () => {
    server.closeAllConnections();

    await new Promise((resolve) => {
      server.close(resolve);
    });
  });

  const { port } = server.address() as AddressInfo;

  await runtime.register(async () =>
    startNodeRuntime({
      ...input,
      apiUrl: `http://127.0.0.1:${port}`,
    }),
  );

  runtime.recordException(new Error("throttled once"), {
    type: "manual.api.capture",
    handled: true,
  });

  await runtime.flush();

  expect(statuses).toStrictEqual([429, 200]);

  expect(bodies.filter((body) => body.includes("throttled once"))).toHaveLength(
    2,
  );
});

test("a flush waits for an export an earlier flush already has in flight", async () => {
  const held: (() => void)[] = [];

  const received = Promise.withResolvers<undefined>();

  const bodies: string[] = [];

  const server = createServer((request, response) => {
    const chunks: Buffer[] = [];

    request.on("data", (chunk: Buffer) => chunks.push(chunk));

    request.on("end", () => {
      bodies.push(Buffer.concat(chunks).toString());

      held.push(() => {
        response.writeHead(200, { "content-type": "application/json" });

        response.end("{}");
      });

      received.resolve(undefined);
    });
  });

  server.listen(0, "127.0.0.1");

  await once(server, "listening");

  onTestFinished(async () => {
    server.closeAllConnections();

    await new Promise((resolve) => {
      server.close(resolve);
    });
  });

  const { port } = server.address() as AddressInfo;

  await runtime.register(async () =>
    startNodeRuntime({
      ...input,
      apiUrl: `http://127.0.0.1:${port}`,
    }),
  );

  runtime.recordException(new Error("delivered in the background"), {
    type: "manual.api.capture",
    handled: true,
  });

  let backgroundDone = false;

  const background = (async () => {
    await runtime.flush();

    backgroundDone = true;
  })();

  await received.promise;

  const explicit = runtime.flush();

  for (const release of held) {
    release();
  }

  await explicit;

  expect(backgroundDone).toBe(true);

  await background;

  expect(
    bodies.filter((body) => body.includes("delivered in the background")),
  ).toHaveLength(1);
});
