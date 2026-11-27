import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  StackContextManager,
} from "@opentelemetry/sdk-trace";
import { sessionIdSchema } from "@interfere/types/data/session";

import { ROOT_CONTEXT, context, propagation, trace } from "@opentelemetry/api";

import { Kernel } from "./internal/kernel.js";
import { SessionTracker } from "./tracking/api.js";
import { span } from "./api.js";
import {
  registerKernel,
  unregisterKernel,
} from "./internal/kernel-registry.js";
import { buildOtelProvider } from "./internal/otel/provider.js";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, describe, expect, test } from "vitest";

import { worker } from "../test/msw.js";

const MILLIS_PER_SECOND = 1000;

const SESSION_ID = sessionIdSchema.parse(
  "01900000-0000-7000-8000-000000000001",
);

describe("span() async semantics", () => {
  let exporter: InMemorySpanExporter;
  let kernel: Kernel;

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
          releaseTokenHeader: { apiKey: "test-key" },
        },
      }),
    );

    context.disable();

    const handle = buildOtelProvider({
      additionalSpanProcessors: [new SimpleSpanProcessor({ exporter })],
      getDeviceId: () => kernel.device.getDeviceId(),
      getSessionId: () => SESSION_ID,
      getViewId: () => kernel.views.viewId(),
      clientOptions: {
        baseUrl: "https://sdk.invalid",
        releaseTokenHeader: { apiKey: "test-key" },
      },
      sdkStack: [],
    });

    if (!context.setGlobalContextManager(new StackContextManager().enable())) {
      throw new Error("another context manager is already registered");
    }

    kernel.attachOtel(handle, () => {
      context.disable();
    });

    registerKernel(kernel);
  });

  afterEach(async () => {
    try {
      unregisterKernel(kernel);

      await kernel.dispose();
    } finally {
      context.disable();

      trace.disable();

      propagation.disable();
    }
  });

  function named(name: string) {
    return exporter.getFinishedSpans().filter((s) => s.name === name);
  }

  function finished(name: string) {
    const [found] = named(name);

    if (!found) {
      throw new Error(`span ${name} did not end`);
    }

    return found;
  }

  function parentOf(name: string) {
    return finished(name).parentSpanContext?.spanId;
  }

  test("nests spans started synchronously inside fn", () => {
    span("outer", () => {
      span("inner", () => undefined);
    });

    expect(parentOf("inner")).toBe(finished("outer").spanContext().spanId);
  });

  test("nests spans started before the first await of an async fn", async () => {
    await span("outer", async () => {
      span("before-await", () => undefined);

      await Promise.resolve();
    });

    expect(parentOf("before-await")).toBe(
      finished("outer").spanContext().spanId,
    );
  });

  test("ends the span when the returned promise settles, including awaited work", async () => {
    const gate = Promise.withResolvers<undefined>();

    const pending = span("op", async () => {
      await gate.promise;

      return "done";
    });

    await Promise.resolve();

    expect(named("op")).toHaveLength(0);

    const releasedAt = Date.now();

    gate.resolve(undefined);

    await expect(pending).resolves.toBe("done");

    const [seconds, nanos] = finished("op").endTime;

    expect(named("op")).toHaveLength(1);

    expect(seconds * MILLIS_PER_SECOND + nanos / 1e6).toBeGreaterThanOrEqual(
      releasedAt - 1,
    );
  });

  test("starts work after an await as a new root carrying the same session and view", async () => {
    await span("op", async () => {
      await Promise.resolve();

      span("after-await", () => undefined);
    });

    const after = finished("after-await");

    expect(after.parentSpanContext).toBeUndefined();

    expect(after.attributes["interfere.view.id"]).toBe(kernel.views.viewId());

    expect(after.attributes["interfere.view.id"]).toBe(
      finished("op").attributes["interfere.view.id"],
    );
  });

  test("keeps an explicit ROOT_CONTEXT root and gives unrelated page work separate traces", () => {
    span("first", () => undefined);

    context.with(ROOT_CONTEXT, () => {
      span("rooted", () => undefined);
    });

    span("second", () => undefined);

    const traces = new Set(
      ["first", "rooted", "second"].map(
        (name) => finished(name).spanContext().traceId,
      ),
    );

    expect(traces.size).toBe(3);

    expect(finished("rooted").parentSpanContext).toBeUndefined();
  });

  test("never parents work after an await under the span that awaited, and keeps the session", async () => {
    await span("op", async () => {
      await Promise.resolve();

      span("after-await", () => undefined);
    });

    expect(parentOf("after-await")).not.toBe(
      finished("op").spanContext().spanId,
    );

    expect(finished("after-await").attributes["session.id"]).toBe(SESSION_ID);
  });

  test("keeps concurrent async chains from parenting each other", async () => {
    expect.hasAssertions();

    const gates = [
      Promise.withResolvers<undefined>(),
      Promise.withResolvers<undefined>(),
    ] as const;

    const chain = async (name: string, gate: Promise<undefined>) =>
      span(name, async () => {
        span(`${name}.sync`, () => undefined);

        await gate;

        span(`${name}.after`, () => undefined);
      });

    const a = chain("a", gates[0].promise);
    const b = chain("b", gates[1].promise);

    gates[1].resolve(undefined);

    await b;

    gates[0].resolve(undefined);

    await a;

    const id = (name: string) => finished(name).spanContext().spanId;

    expect(parentOf("a.sync")).toBe(id("a"));
    expect(parentOf("b.sync")).toBe(id("b"));

    for (const after of ["a.after", "b.after"]) {
      expect([id("a"), id("b")]).not.toContain(parentOf(after));
    }
  });

  test("records an error thrown after an await and ends the span once", async () => {
    const failure = new Error("late boom");

    await expect(
      span("op", async () => {
        await Promise.resolve();

        throw failure;
      }),
    ).rejects.toBe(failure);

    expect(named("op")).toHaveLength(1);
    expect(finished("op").status.code).toBe(2);

    expect(finished("op").events.map((e) => e.name)).toStrictEqual([
      "exception",
    ]);
  });

  test("records cancellation as an error and rethrows the abort reason", async () => {
    const controller = new AbortController();
    const reason = new Error("cancelled");

    const pending = span("op", async () => {
      await new Promise((_resolve, reject) => {
        controller.signal.addEventListener("abort", () => {
          reject(reason);
        });
      });
    });

    controller.abort(reason);

    await expect(pending).rejects.toBe(reason);

    expect(finished("op").status).toStrictEqual({
      code: 2,
      message: "cancelled",
    });
  });

  test("parents post-await work explicitly through context.with(trace.setSpan(...))", async () => {
    await span("op", async (s) => {
      await Promise.resolve();

      context.with(trace.setSpan(context.active(), s), () => {
        span("explicit-child", () => undefined);
      });
    });

    expect(parentOf("explicit-child")).toBe(
      finished("op").spanContext().spanId,
    );
  });

  test("starts an explicit root with { root: true } inside another span", () => {
    span("outer", () => {
      span("detached", () => undefined, { root: true });
    });

    expect(finished("detached").parentSpanContext).toBeUndefined();

    expect(finished("detached").spanContext().traceId).not.toBe(
      finished("outer").spanContext().traceId,
    );
  });
});
