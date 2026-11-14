import { context, propagation } from "@opentelemetry/api";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";

import { createError } from "../../../test/fixtures/errors.js";
import { worker } from "../../../test/msw.js";
import { loadPlugin } from "../../plugins/lib/loader.js";
import type { PluginCleanup } from "../../plugins/lib/types.js";
import { SessionTracker } from "../../tracking/api.js";
import { Kernel } from "../kernel.js";
import { buildOtelProvider } from "../otel/provider.js";
import { PluginRuntime } from "./plugin-runtime.js";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, test } from "vitest";

let kernel: Kernel;

let runtime: PluginRuntime;

let exporter: InMemorySpanExporter;

let cleanup: PluginCleanup | null;

beforeEach(() => {
  worker.use(
    http.post(
      "https://plugins.invalid/drain/v1/traces",
      () => new HttpResponse(null, { status: 204 }),
    ),
  );

  kernel = new Kernel(
    new SessionTracker({
      target: {
        baseUrl: "https://plugins.invalid",
        publicKeyHeader: { apiKey: "test-key" },
      },
    }),
  );

  runtime = new PluginRuntime(kernel);

  kernel.attachRuntime(runtime);

  exporter = new InMemorySpanExporter();

  cleanup = null;

  const otel = buildOtelProvider({
    additionalSpanProcessors: [new SimpleSpanProcessor(exporter)],

    getDeviceId: () => kernel.device.getDeviceId(),
    getSessionId: () => kernel.session.getId(),
    clientOptions: {
      baseUrl: "https://plugins.invalid",
      publicKeyHeader: { apiKey: "test-key" },
    },
    sdkStack: [],
  });

  kernel.attachOtel(otel, () => otel.contextManager.disable());
});

afterEach(async () => {
  try {
    cleanup?.();

    await kernel.dispose();
  } finally {
    context.disable();

    propagation.disable();
  }
});

test("captures errors immediately after runtime start without an async wait", () => {
  runtime.start();

  globalThis.onerror?.(
    "startup",
    "https://app.invalid/main.js",
    4,
    2,
    createError("TypeError", "startup"),
  );

  const spans = exporter.getFinishedSpans();

  expect(spans).toHaveLength(1);

  expect(spans[0]?.events[0]).toMatchObject({
    name: "exception",
    attributes: {
      "exception.message": "startup",
      "interfere.exception.mechanism": "auto.browser.global_handlers.onerror",
      "interfere.exception.handled": false,
    },
  });
});

test("restores browser error handling when a started runtime is disposed", async () => {
  const original = globalThis.onerror;
  const limit = Error.stackTraceLimit;

  runtime.start();

  const installed = globalThis.onerror;

  expect(installed).not.toBe(original);

  await runtime.dispose();

  expect(globalThis.onerror).toBe(original);
  expect(Error.stackTraceLimit).toBe(limit);
  expect(exporter.getFinishedSpans()).toStrictEqual([]);
});

test("loads the errors plugin and its cleanup through the real dispatcher", async () => {
  const original = globalThis.onerror;

  cleanup = await loadPlugin("errors", kernel);

  expect(cleanup).toBeTypeOf("function");

  globalThis.onerror?.(
    "loader",
    "https://app.invalid/main.js",
    4,
    2,
    createError("Error", "loader"),
  );

  expect(
    exporter.getFinishedSpans()[0]?.events[0]?.attributes?.[
      "exception.message"
    ],
  ).toBe("loader");

  cleanup?.();

  cleanup = null;

  expect(globalThis.onerror).toBe(original);
});
