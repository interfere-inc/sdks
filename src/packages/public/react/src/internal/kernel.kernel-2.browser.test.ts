import {
  SpanStatusCode,
  context,
  propagation,
  trace,
} from "@opentelemetry/api";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  StackContextManager,
  TracerProvider,
} from "@opentelemetry/sdk-trace";

import { buildKernel, ingestHandlers } from "../../test/fixtures/kernel.js";
import { getLogLevel, setLogLevel } from "../util/log.js";
import type { createKernel } from "./kernel.js";
import { buildOtelProvider } from "./otel/provider.js";
import { HttpResponse, http } from "msw";
import {
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  test,
  vi,
} from "vitest";

import { addBaselineHandlers } from "../../test/msw.js";

beforeAll(() => {
  addBaselineHandlers(
    http.get("https://api.interfere.com/v3/config", () =>
      HttpResponse.json({ plugins: {} }),
    ),
    ...ingestHandlers("https://api.interfere.com"),
    ...ingestHandlers("https://in.example.com"),
  );
});

const ownedKernels = new Set<Awaited<ReturnType<typeof createKernel>>>();

const providers = new Set<TracerProvider>();

let priorUrl: string;

let priorHistory: unknown;

let priorCookie: Awaited<ReturnType<CookieStore["get"]>>;

let priorLogLevel: ReturnType<typeof getLogLevel>;

beforeEach(async () => {
  priorLogLevel = getLogLevel();

  priorUrl = globalThis.location.href;

  priorHistory = globalThis.history.state;

  priorCookie = await cookieStore.get("interfere_did");
});

afterEach(async () => {
  try {
    const results = await Promise.allSettled([
      ...Array.from(ownedKernels, async (kernel) => kernel.dispose()),
      ...Array.from(providers, async (provider) => provider.shutdown()),
    ]);

    const failures = results.filter((result) => result.status === "rejected");

    if (failures.length > 0) {
      throw new AggregateError(
        failures.map((result) => {
          const decodedValue: unknown = result.reason;

          return decodedValue;
        }),
        "Kernel cleanup failed",
      );
    }
  } finally {
    ownedKernels.clear();

    providers.clear();

    context.disable();

    trace.disable();

    propagation.disable();

    setLogLevel(priorLogLevel);

    globalThis.history.replaceState(priorHistory, "", priorUrl);

    await cookieStore.delete("interfere_did");

    if (priorCookie?.name && priorCookie.value !== undefined) {
      await cookieStore.set({
        ...priorCookie,
        name: priorCookie.name,
        value: priorCookie.value,
      });
    }

    vi.unstubAllGlobals();
  }
});

describe("Kernel", () => {
  describe("recordException", () => {
    test("synthesises a private-provider span when no active span exists", async () => {
      const privateExporter = new InMemorySpanExporter();
      const globalExporter = new InMemorySpanExporter();

      const globalProvider = new TracerProvider({
        spanProcessors: [new SimpleSpanProcessor({ exporter: globalExporter })],
      });

      const { kernel, tracker } = buildKernel();

      ownedKernels.add(kernel);

      trace.disable();

      context.disable();

      providers.add(globalProvider);

      trace.setGlobalTracerProvider(globalProvider);

      context.setGlobalContextManager(new StackContextManager().enable());

      try {
        tracker.start();

        kernel.attachOtel(
          buildOtelProvider({
            additionalSpanProcessors: [
              new SimpleSpanProcessor({ exporter: privateExporter }),
            ],

            getDeviceId: () => tracker.getDeviceId(),
            getSessionId: () => tracker.sessionId(),
            clientOptions: {
              baseUrl: "https://in.example.com",
              releaseTokenHeader: { apiKey: "test-key" },
            },
            sdkStack: [],
          }),
          vi.fn(),
        );

        const boom = new Error("boom");

        boom.stack =
          "Error: boom\n    at Array.map (<anonymous>)\n    at submitOrder (https://shop.example/assets/checkout-a1b2.js:1:4821)";

        kernel.recordException(boom, {
          mechanism: { handled: false, type: "browser.onerror" },
        });

        const stackless = new TypeError("stackless");

        delete stackless.stack;

        kernel.recordException(stackless, {
          mechanism: { handled: true, type: "browser.console.error" },
        });

        kernel.recordException(
          { kind: "non-error", serialized: 42, type: "number", value: "42" },
          {
            mechanism: { handled: false, type: "browser.onunhandledrejection" },
          },
        );

        const finished = privateExporter.getFinishedSpans();

        expect(finished).toHaveLength(3);
        expect(globalExporter.getFinishedSpans()).toHaveLength(0);

        expect(finished.map((span) => span.name)).toStrictEqual([
          "submitOrder (checkout-a1b2.js:1)",
          "TypeError",
          "number",
        ]);

        expect(finished.map((span) => span.status.code)).toStrictEqual([
          SpanStatusCode.ERROR,
          SpanStatusCode.UNSET,
          SpanStatusCode.ERROR,
        ]);

        expect(finished[0]?.attributes).toMatchObject({
          "interfere.capture.kind": "exception",
        });

        const exceptionEvent = finished[0]?.events.find(
          (e) => e.name === "exception",
        );

        expect(exceptionEvent?.attributes).toMatchObject({
          "exception.message": "boom",
          "interfere.exception.handled": false,
          "interfere.exception.mechanism": "browser.onerror",
        });
      } finally {
        await kernel.dispose();

        await globalProvider.shutdown();

        context.disable();

        trace.disable();
      }
    });
  });
});
