/* oxlint-disable typescript/require-await */
import { bytesToHex } from "@interfere/helpers/hex";
import {
  SpanStatusCode,
  context,
  propagation,
  trace,
} from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@interfere/server-core/otel/context-async-hooks";
import {
  CompositePropagator,
  W3CBaggagePropagator,
  W3CTraceContextPropagator,
} from "@interfere/server-core/otel/core/propagation";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@interfere/server-core/otel/sdk/trace/processing";
import { TracerProvider } from "@interfere/server-core/otel/sdk/trace/provider";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

import { traceMeta, withRequestSpan } from "./trace.js";

let exporter: InMemorySpanExporter;

let provider: TracerProvider;

beforeEach(() => {
  vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

  exporter = new InMemorySpanExporter();

  provider = new TracerProvider({
    spanProcessors: [new SimpleSpanProcessor({ exporter })],
  });

  trace.setGlobalTracerProvider(provider);

  context.setGlobalContextManager(
    new AsyncLocalStorageContextManager().enable(),
  );

  propagation.setGlobalPropagator(
    new CompositePropagator({
      propagators: [
        new W3CTraceContextPropagator(),
        new W3CBaggagePropagator(),
      ],
    }),
  );
});

afterEach(async () => {
  context.disable();

  trace.disable();

  propagation.disable();

  await provider.shutdown();

  vi.unstubAllEnvs();
});

function streamOf(chunks: string[], failure?: Error) {
  const encoder = new TextEncoder();
  const pending = [...chunks];

  return new ReadableStream<Uint8Array>({
    pull(controller) {
      const chunk = pending.shift();

      if (chunk !== undefined) {
        controller.enqueue(encoder.encode(chunk));

        return;
      }

      if (failure) {
        controller.error(failure);

        return;
      }

      controller.close();
    },
  });
}

async function tick(ms: number) {
  await new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

test("keeps the SERVER span open until a streamed body ends", async () => {
  const response = await withRequestSpan(
    new Request("https://app.test/page"),
    async () =>
      new Response(streamOf(["<html>", "</html>"]), {
        headers: { "content-type": "text/html" },
        status: 201,
      }),
  );

  expect(exporter.getFinishedSpans()).toHaveLength(0);
  expect(response.status).toBe(201);
  expect(response.headers.get("content-type")).toBe("text/html");

  await expect(response.text()).resolves.toBe("<html></html>");

  const [span] = exporter.getFinishedSpans();

  expect(span?.status.code).toBe(SpanStatusCode.UNSET);
});

test("marks the SERVER span as errored when a streamed body fails after the first chunk", async () => {
  const result = await withRequestSpan(
    new Request("https://app.test/page"),
    async () => ({
      pathname: "/page",
      response: new Response(streamOf(["<html>"], new Error("stream boom"))),
    }),
  );

  expect(result.pathname).toBe("/page");
  expect(exporter.getFinishedSpans()).toHaveLength(0);

  await expect(result.response.text()).rejects.toThrow("stream boom");

  const [span] = exporter.getFinishedSpans();

  expect(span?.status.code).toBe(SpanStatusCode.ERROR);

  expect(
    span?.events
      .filter((event) => event.name === "exception")
      .map((event) => event.attributes?.["exception.message"]),
  ).toStrictEqual(["stream boom"]);
});

test("ends the SERVER span when the client cancels a streamed body", async () => {
  const response = await withRequestSpan(
    new Request("https://app.test/page"),
    async () => new Response(streamOf(["a", "b", "c"])),
  );

  const reader = response.body?.getReader();

  await reader?.read();

  expect(exporter.getFinishedSpans()).toHaveLength(0);

  await reader?.cancel("client went away");

  const [span] = exporter.getFinishedSpans();

  expect(span?.status.code).toBe(SpanStatusCode.UNSET);
});

test("keeps trace ids separate across 50 interleaved requests", async () => {
  expect.hasAssertions();

  const traceIds = Array.from(
    { length: 50 },
    (_, index) => `${bytesToHex(Uint8Array.of(index + 1))}${"a".repeat(30)}`,
  );

  const observed = await Promise.all(
    traceIds.map(async (traceId, index) =>
      withRequestSpan(
        new Request(`https://app.test/page/${index}`, {
          headers: { traceparent: `00-${traceId}-b7ad6b7169203331-01` },
        }),
        async () => {
          await tick((index * 7) % 5);

          const before = traceMeta();

          trace
            .getTracer("child")
            .startActiveSpan(`child-${index}`, (child) => {
              child.end();
            });

          await tick((index * 3) % 4);

          return { after: traceMeta(), before };
        },
      ),
    ),
  );

  const spans = exporter.getFinishedSpans();

  for (const [index, traceId] of traceIds.entries()) {
    for (const meta of [observed[index]?.before, observed[index]?.after]) {
      expect(
        meta?.find(({ name }) => name === "traceparent")?.content,
      ).toContain(`00-${traceId}-`);
    }

    expect(
      spans.find(({ name }) => name === `child-${index}`)?.spanContext()
        .traceId,
    ).toBe(traceId);
  }
});

test("runs the request and emits no meta when a foreign propagator throws", async () => {
  propagation.disable();

  propagation.setGlobalPropagator({
    extract: () => {
      throw new Error("extract boom");
    },
    fields: () => [],
    inject: () => {
      throw new Error("inject boom");
    },
  });

  let metaInside: ReturnType<typeof traceMeta> | undefined;

  const result = await withRequestSpan(
    new Request("https://app.test/page", {
      headers: {
        traceparent: "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01",
      },
    }),
    async () => {
      metaInside = traceMeta();

      return "ok";
    },
  );

  expect(result).toBe("ok");
  expect(metaInside).toStrictEqual([]);
  expect(exporter.getFinishedSpans()).toHaveLength(1);
});
