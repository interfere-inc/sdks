/* oxlint-disable unicorn/no-useless-promise-resolve-reject */

import { context, propagation, trace } from "@opentelemetry/api";
import type {
  LogRecordProcessor,
  ReadableLogRecord,
} from "@opentelemetry/sdk-logs";
import type { TracerProvider } from "@opentelemetry/sdk-trace";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace";

import { ingestHandlers } from "../../test/fixtures/kernel.js";
import { getLogLevel, setLogLevel } from "../util/log.js";
import { createKernel } from "./kernel.js";
import type { KernelOptions } from "./kernel.js";
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

describe(createKernel, () => {
  test("runs public processors before wrapper processors and owns their lifecycle", async () => {
    const exporter = new InMemorySpanExporter();

    const processor = new SimpleSpanProcessor({
      exporter: new InMemorySpanExporter(),
    });

    const wrapper = new SimpleSpanProcessor({ exporter });

    const onStart = vi.spyOn(processor, "onStart");

    const publicEnd = vi.spyOn(processor, "onEnd");
    const wrapperEnd = vi.spyOn(wrapper, "onEnd");

    const forceFlush = vi.spyOn(processor, "forceFlush");

    const shutdown = vi.spyOn(processor, "shutdown");

    const opts = {
      tracing: { spanProcessors: [processor] },
    } satisfies KernelOptions;

    const kernel = await createKernel({
      opts: {
        ...opts,
        _internalAdditionalSpanProcessors: [wrapper],
      },
    });

    ownedKernels.add(kernel);

    kernel
      .getTracer("public-processors")
      .startSpan("custom", {
        attributes: { "url.full": "https://example.com/secret" },
      })
      .end();

    await kernel.flush();

    expect(
      exporter.getFinishedSpans().find((span) => span.name === "custom")
        ?.attributes["url.full"],
    ).toBe("https://example.com/secret");

    expect(publicEnd.mock.invocationCallOrder[0]).toBeLessThan(
      wrapperEnd.mock.invocationCallOrder[0] ?? 0,
    );

    expect(onStart).toHaveBeenCalledWith(
      expect.objectContaining({ name: "custom" }),
      expect.anything(),
    );

    expect(publicEnd).toHaveBeenCalledWith(
      expect.objectContaining({ name: "custom" }),
    );

    expect(forceFlush).toHaveBeenCalledWith();

    await kernel.dispose();

    expect(shutdown).toHaveBeenCalledTimes(1);
  });

  test("tracing false leaves wrapper processors inactive", async () => {
    const processor = new SimpleSpanProcessor({
      exporter: new InMemorySpanExporter(),
    });

    const onStart = vi.spyOn(processor, "onStart");

    const onEnd = vi.spyOn(processor, "onEnd");

    const forceFlush = vi.spyOn(processor, "forceFlush");

    const shutdown = vi.spyOn(processor, "shutdown");

    const kernel = await createKernel({
      opts: { tracing: false, _internalAdditionalSpanProcessors: [processor] },
    });

    ownedKernels.add(kernel);

    kernel.getTracer("disabled").startSpan("ignored").end();

    await kernel.flush();

    await kernel.dispose();

    expect(onStart).not.toHaveBeenCalled();
    expect(onEnd).not.toHaveBeenCalled();
    expect(forceFlush).not.toHaveBeenCalled();
    expect(shutdown).not.toHaveBeenCalled();
  });

  test("never emits the initial navigation log when analytics consent is denied", async () => {
    const logs: ReadableLogRecord[] = [];

    const logProcessor = {
      forceFlush: async () => Promise.resolve(),
      onEmit: (record) => {
        const navigationType = Boolean(
          record.attributes["browser.navigation.type"],
        );

        if (navigationType) {
          logs.push(record);
        }
      },
      shutdown: async () => Promise.resolve(),
    } satisfies LogRecordProcessor;

    const kernel = await createKernel({
      opts: { _internalAdditionalLogRecordProcessors: [logProcessor] },
    });

    ownedKernels.add(kernel);

    kernel.consent.set(false);

    kernel.syncProviderConfig({});

    await kernel.dispose();

    expect(logs).toHaveLength(0);
  });
});
