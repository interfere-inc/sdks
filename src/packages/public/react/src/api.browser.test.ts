/* oxlint-disable typescript/only-throw-error, typescript/require-await */
import { context, propagation, trace } from "@opentelemetry/api";
import { StackContextManager } from "@opentelemetry/sdk-trace-web";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";

import { Kernel } from "./internal/kernel.js";
import { SessionTracker } from "./tracking/api.js";
import { capture, span } from "./api.js";
import {
  activeKernel,
  registerKernel,
  unregisterKernel,
} from "./internal/kernel-registry.js";
import { buildOtelProvider } from "./internal/otel/provider.js";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

import { worker } from "../test/msw.js";

describe("api", () => {
  let exporter: InMemorySpanExporter;
  let kernel: Kernel;
  const owned: Kernel[] = [];

  beforeEach(() => {
    exporter = new InMemorySpanExporter();

    worker.use(
      ...[
        "v3/session",
        "drain/v1/traces",
        "drain/v1/logs",
        "drain/v1/metrics",
      ].map((path) =>
        http.post(
          `https://sdk.invalid/${path}`,
          () => new HttpResponse(null, { status: 204 }),
        ),
      ),
    );

    kernel = new Kernel(
      new SessionTracker({
        target: {
          baseUrl: "https://sdk.invalid",
          publicKeyHeader: { apiKey: "test-key" },
        },
      }),
    );

    owned.push(kernel);

    context.setGlobalContextManager(new StackContextManager().enable());
  });

  afterEach(async () => {
    try {
      const results = await Promise.allSettled(
        owned.splice(0).map(async (entry) => {
          unregisterKernel(entry);

          await entry.dispose();
        }),
      );

      const failures = results.filter((result) => result.status === "rejected");

      if (failures.length > 0) {
        throw new AggregateError(
          failures.map((result) => {
            const decodedValue: unknown = result.reason;

            return decodedValue;
          }),
          "SDK cleanup failed",
        );
      }
    } finally {
      context.disable();

      trace.disable();

      propagation.disable();

      vi.restoreAllMocks();
    }
  });

  describe(span, () => {
    beforeEach(() => {
      const handle = buildOtelProvider({
        additionalSpanProcessors: [new SimpleSpanProcessor(exporter)],

        getDeviceId: () => kernel.device.getDeviceId(),
        getSessionId: () => kernel.session.getId(),
        clientOptions: {
          baseUrl: "https://sdk.invalid",
          publicKeyHeader: { apiKey: "test-key" },
        },
        sdkStack: [],
      });

      kernel.attachOtel(handle, () => undefined);

      registerKernel(kernel);
    });

    test("records through the kernel's private provider with no global tracer provider", () => {
      const result = span("op", () => 42);

      expect(result).toBe(42);

      expect(exporter.getFinishedSpans().map((s) => s.name)).toStrictEqual([
        "op",
      ]);

      expect(exporter.getFinishedSpans()[0]?.spanContext().traceId).not.toBe(
        "00000000000000000000000000000000",
      );
    });

    test("runs the function without recording when no kernel is registered", () => {
      unregisterKernel(kernel);

      const result = span("op", () => 7);

      expect(result).toBe(7);
      expect(exporter.getFinishedSpans()).toHaveLength(0);
    });

    test("falls back to the global registry when OTel is not wired", () => {
      const bare = new Kernel(
        new SessionTracker({
          target: {
            baseUrl: "https://sdk.invalid",
            publicKeyHeader: { apiKey: "test-key" },
          },
        }),
      );

      owned.push(bare);

      registerKernel(bare);

      const result = span("op", () => "ok");

      expect(result).toBe("ok");
      expect(exporter.getFinishedSpans()).toHaveLength(0);

      unregisterKernel(bare);

      registerKernel(kernel);
    });

    test("wraps an async function and ends the span on resolve", async () => {
      const result = await span("op", async () => {
        await Promise.resolve();

        return "ok";
      });

      expect(result).toBe("ok");
      expect(exporter.getFinishedSpans()).toHaveLength(1);
    });

    test("records an exception and re-throws on sync throw", () => {
      expect(() =>
        span("op", () => {
          throw new Error("boom");
        }),
      ).toThrow("boom");

      const finished = exporter.getFinishedSpans();

      expect(finished[0]?.events.map((e) => e.name)).toContain("exception");
      expect(finished[0]?.status.code).toBe(2);
    });

    test("records an exception and re-throws on async reject", async () => {
      await expect(
        span("op", async () => {
          throw new Error("async boom");
        }),
      ).rejects.toThrow("async boom");

      const finished = exporter.getFinishedSpans();

      expect(finished[0]?.events.map((e) => e.name)).toContain("exception");
    });

    test.each(["rejected", null, 0, { reason: "rejected" }])(
      "preserves non-Error rejection identity for %j",
      async (reason) => {
        await expect(
          span("op", async () => {
            throw reason;
          }),
        ).rejects.toBe(reason);

        const finished = exporter.getFinishedSpans();

        expect(finished).toHaveLength(1);

        expect(finished[0]?.events.map((event) => event.name)).toContain(
          "exception",
        );

        expect(finished[0]?.status.code).toBe(2);
      },
    );

    test.each([undefined, null, false, 0, "result", { value: "result" }])(
      "preserves synchronous result identity for %j",
      (result) => {
        expect(span("op", () => result)).toBe(result);
        expect(exporter.getFinishedSpans()).toHaveLength(1);
      },
    );
  });

  describe(capture, () => {
    test("is a no-op before init", () => {
      expect(activeKernel()).toBeNull();

      expect(() => {
        capture(new Error("boom"));
      }).not.toThrow();
    });

    test("records via kernel.recordException when init is registered", () => {
      registerKernel(kernel);

      const spy = vi.spyOn(kernel, "recordException");
      const err = new Error("boom");

      capture(err);

      expect(spy).toHaveBeenCalledWith(
        err,
        expect.objectContaining({
          mechanism: { handled: true, type: "manual.api.capture" },
        }),
      );
    });

    test("wraps non-Error throws", () => {
      registerKernel(kernel);

      const spy = vi.spyOn(kernel, "recordException");

      capture("plain string failure");

      expect(spy).toHaveBeenCalledTimes(1);

      const [forwarded] = spy.mock.calls[0] ?? [];

      expect(forwarded).toBeInstanceOf(Error);

      expect(forwarded).toHaveProperty(
        "message",
        expect.stringContaining("plain string failure"),
      );
    });
  });
});
