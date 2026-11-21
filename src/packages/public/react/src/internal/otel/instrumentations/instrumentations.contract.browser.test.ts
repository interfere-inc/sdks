/* oxlint-disable unicorn/no-useless-promise-resolve-reject */
import { isWrapped } from "@opentelemetry/instrumentation";
import type {
  LogRecordProcessor,
  ReadWriteLogRecord,
} from "@opentelemetry/sdk-logs";
import { LoggerProvider } from "@opentelemetry/sdk-logs";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";

import { registerBundledInstrumentations } from "./instrumentations.js";
import { afterEach, describe, expect, test, vi } from "vitest";

interface Harness {
  dispose: () => void;
  exporter: InMemorySpanExporter;
  loggerProvider: LoggerProvider;
  logs: ReadWriteLogRecord[];
  provider: TracerProvider;
  setPageEventsEnabled: (enabled: boolean) => void;
}

function setupBundle(opts?: { pageEvents?: boolean }) {
  const exporter = new InMemorySpanExporter();

  const provider = new TracerProvider({
    spanProcessors: [new SimpleSpanProcessor({ exporter })],
  });

  const logs: ReadWriteLogRecord[] = [];

  const logProcessor: LogRecordProcessor = {
    async forceFlush() {
      return Promise.resolve();
    },
    onEmit(logRecord) {
      logs.push(logRecord);
    },
    async shutdown() {
      return Promise.resolve();
    },
  };

  const loggerProvider = new LoggerProvider({ processors: [logProcessor] });

  const handle = registerBundledInstrumentations({
    ignoreUrls: [],
    loggerProvider,
    tracerProvider: provider,
  });

  handle.setPageEventsEnabled(opts?.pageEvents ?? true);

  return {
    dispose: handle.dispose,
    exporter,
    loggerProvider,
    logs,
    provider,
    setPageEventsEnabled: handle.setPageEventsEnabled,
  } satisfies Harness;
}

function isLongtaskObserveOpts(
  opts: Parameters<PerformanceObserver["observe"]>[0],
) {
  return (
    opts?.type === "longtask" || opts?.entryTypes?.includes("longtask") === true
  );
}

describe("structural: long-task and document-load instrumentations patched after registration", () => {
  let harness: Harness | null = null;

  afterEach(async () => {
    harness?.dispose();

    await harness?.provider.shutdown();

    await harness?.loggerProvider.shutdown();

    harness = null;
  });

  test("leaves listener registration and history unpatched; the ViewTracker and user actions own those", () => {
    harness = setupBundle();

    // oxlint-disable-next-line typescript/unbound-method
    expect(isWrapped(HTMLElement.prototype.addEventListener)).toBe(false);
    // oxlint-disable-next-line typescript/unbound-method
    expect(isWrapped(history.pushState)).toBe(false);
    // oxlint-disable-next-line typescript/unbound-method
    expect(isWrapped(history.replaceState)).toBe(false);
  });

  test("subscribes a PerformanceObserver for longtask on enable", () => {
    const observeSpy = vi.spyOn(PerformanceObserver.prototype, "observe");

    try {
      harness = setupBundle();

      const longtaskCalls = observeSpy.mock.calls.filter((args) =>
        isLongtaskObserveOpts(args[0]),
      );

      expect(
        longtaskCalls.length,
        "expected PerformanceObserver.observe called with longtask entry type",
      ).toBeGreaterThanOrEqual(1);
    } finally {
      observeSpy.mockRestore();
    }
  });

  test("emits a documentLoad span on enable when the page is already complete", () => {
    expect(document.readyState).toBe("complete");

    vi.useFakeTimers({ toFake: ["setTimeout"] });

    try {
      harness = setupBundle();

      vi.runAllTimers();

      const spans = harness.exporter.getFinishedSpans();

      const docSpan = spans.find(
        (s) => s.name === "documentLoad" || s.name === "documentFetch",
      );

      expect(
        docSpan,
        `expected a documentLoad/documentFetch span from DocumentLoadInstrumentation; observed span names: ${spans.map((s) => s.name).join(", ") || "(none)"}`,
      ).toBeDefined();
    } finally {
      vi.useRealTimers();
    }
  });
});

function documentLoadSpans(h: Harness) {
  return h.exporter
    .getFinishedSpans()
    .filter((s) => s.name === "documentLoad" || s.name === "documentFetch");
}

describe("pageEvents gating", () => {
  let harness: Harness | null = null;

  afterEach(async () => {
    harness?.dispose();

    await harness?.provider.shutdown();

    await harness?.loggerProvider.shutdown();

    harness = null;
  });

  test("emits no document-load spans until pageEvents is enabled", () => {
    vi.useFakeTimers({ toFake: ["setTimeout"] });

    try {
      harness = setupBundle({ pageEvents: false });

      vi.runAllTimers();

      expect(documentLoadSpans(harness)).toHaveLength(0);

      harness.setPageEventsEnabled(true);

      vi.runAllTimers();

      expect(documentLoadSpans(harness)).not.toHaveLength(0);
    } finally {
      vi.useRealTimers();
    }
  });
});

describe("characterization: history.pushState emits nothing from the bundle", () => {
  let harness: Harness | null = null;
  const originalPath = globalThis.location.pathname;

  afterEach(async () => {
    harness?.dispose();

    await harness?.provider.shutdown();

    await harness?.loggerProvider.shutdown();

    harness = null;

    history.replaceState({}, "", originalPath);
  });

  test("one pushState emits no span and no log; page views come from the ViewTracker", () => {
    harness = setupBundle();

    const spansBefore = harness.exporter.getFinishedSpans().length;
    const logsBefore = harness.logs.length;

    history.pushState({}, "", "/contract-test-push");

    const spansEmitted =
      harness.exporter.getFinishedSpans().length - spansBefore;

    const logsEmitted = harness.logs.length - logsBefore;

    expect({ logsEmitted, spansEmitted }).toStrictEqual({
      logsEmitted: 0,
      spansEmitted: 0,
    });
  });
});
