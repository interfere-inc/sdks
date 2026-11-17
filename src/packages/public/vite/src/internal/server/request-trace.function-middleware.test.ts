/* oxlint-disable typescript/require-await */
import { SpanStatusCode, context, trace } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

import { interfereFunctionMiddleware } from "../../tanstack-start";
import { withRequestSpan } from "./request-trace.js";

let exporter: InMemorySpanExporter;

let provider: BasicTracerProvider;

beforeEach(() => {
  vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

  exporter = new InMemorySpanExporter();

  provider = new BasicTracerProvider({
    spanProcessors: [new SimpleSpanProcessor(exporter)],
  });

  trace.setGlobalTracerProvider(provider);

  context.setGlobalContextManager(
    new AsyncLocalStorageContextManager().enable(),
  );
});

afterEach(async () => {
  context.disable();

  trace.disable();

  await provider.shutdown();

  vi.unstubAllEnvs();
});

test("a server function throw is recorded on the request span and rethrown", async () => {
  expect.hasAssertions();

  const { server } = interfereFunctionMiddleware.options;

  await withRequestSpan(
    new Request("https://app.test/_serverFn/fail", { method: "POST" }),
    async () => {
      await expect(
        server?.({
          context: undefined,
          data: undefined,
          method: "POST",
          next: async () => {
            throw new Error("server function boom");
          },
          serverFnMeta: { filename: "server-fn.js", id: "fail", name: "fail" },
          signal: new AbortController().signal,
        } as never),
      ).rejects.toThrow("server function boom");

      return new Response(null, { status: 500 });
    },
  );

  const [span] = exporter.getFinishedSpans();

  expect(span?.status.code).toBe(SpanStatusCode.ERROR);

  expect(
    span?.events
      .filter((event) => event.name === "exception")
      .map((event) => event.attributes?.["exception.message"]),
  ).toStrictEqual(["server function boom"]);
});

test.each([
  {
    name: "redirect",
    thrown: Response.redirect("https://app.test/login", 307),
  },
  { name: "notFound", thrown: { isNotFound: true } },
])(
  "a server function $name is router control flow, not an error",
  async ({ thrown }) => {
    expect.hasAssertions();

    const { server } = interfereFunctionMiddleware.options;

    await withRequestSpan(
      new Request("https://app.test/_serverFn/redirect", { method: "POST" }),
      async () => {
        await expect(
          server?.({
            context: undefined,
            data: undefined,
            method: "POST",
            next: async () => {
              // oxlint-disable-next-line typescript/only-throw-error -- Start redirect() and notFound() throw non-Error values.
              throw thrown;
            },
            serverFnMeta: { filename: "server-fn.js", id: "go", name: "go" },
            signal: new AbortController().signal,
          } as never),
        ).rejects.toBe(thrown);

        return new Response(null, { status: 307 });
      },
    );

    const [span] = exporter.getFinishedSpans();

    expect(span?.status.code).toBe(SpanStatusCode.UNSET);
    expect(span?.events).toStrictEqual([]);
  },
);
