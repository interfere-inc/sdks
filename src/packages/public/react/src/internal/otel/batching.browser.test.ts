import { buildOtelProvider } from "./provider.js";
import type { OtelProviderHandle } from "./provider.js";
import {
  afterEach,
  beforeEach,
  describe,
  expect,
  onTestFinished,
  test,
  vi,
} from "vitest";
import { HttpResponse } from "msw";

import {
  CLIENT_OPTIONS,
  collector,
  exportedLogCount,
  exportedSpanCount,
  gate,
  ok,
  resetSent,
  sent,
  sentSignal,
} from "../../../test/fixtures/export-collector.js";
import { worker } from "../../../test/msw.js";

beforeEach(() => {
  resetSent();
});

afterEach(() => {
  vi.useRealTimers();
});

describe("B01-B04 batch processor cadence", () => {
  let handle: OtelProviderHandle | null = null;

  function buildProvider() {
    handle = buildOtelProvider({
      clientOptions: CLIENT_OPTIONS,
      getDeviceId: () => null,
      getSessionId: () => null,
      sdkStack: ["@interfere/react@test"],
    });

    return handle;
  }

  afterEach(async () => {
    await handle?.shutdown();

    handle?.contextManager.disable();

    handle = null;
  });

  test("a trace export request carries at most 200 spans (B01)", async () => {
    expect.hasAssertions();

    worker.use(collector(ok));

    const provider = buildProvider();

    const tracer = provider.tracerProvider.getTracer("b01");

    for (let index = 0; index < 450; index += 1) {
      tracer.startSpan(`span ${index}`).end();
    }

    await provider.flush();

    const traces = sent.filter((entry) => entry.signal === "traces");

    expect(exportedSpanCount(traces)).toBe(450);

    for (const entry of traces) {
      expect(exportedSpanCount([entry])).toBeLessThanOrEqual(200);
    }
  });

  test("an ended span is exported once the 60 s trace batch delay passes, with no flush (B02)", async () => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });

    worker.use(collector(ok));

    const provider = buildProvider();

    provider.tracerProvider.getTracer("b02").startSpan("lonely").end();

    await vi.advanceTimersByTimeAsync(59_900);

    expect(sent.filter((entry) => entry.signal === "traces")).toHaveLength(0);

    await vi.advanceTimersByTimeAsync(100);

    await sentSignal.reached(1);

    expect(sent.map((entry) => entry.signal)).toStrictEqual(["traces"]);
  });

  test("an emitted log record still leaves after the 5 s log batch delay (B02b)", async () => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });

    worker.use(collector(ok));

    const provider = buildProvider();

    provider.loggerProvider.getLogger("b02b").emit({ body: "lonely" });

    await vi.advanceTimersByTimeAsync(4900);

    expect(sent.filter((entry) => entry.signal === "logs")).toHaveLength(0);

    await vi.advanceTimersByTimeAsync(100);

    await sentSignal.reached(1);

    expect(sent.map((entry) => entry.signal)).toStrictEqual(["logs"]);
  });

  test("a busy tab ending 30 spans/s for the whole 60 s delay loses no span and posts at most 200 per request (B03b)", async () => {
    expect.hasAssertions();

    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });

    worker.use(collector(ok));

    const provider = buildProvider();

    const tracer = provider.tracerProvider.getTracer("b03b");

    const produced = { ended: 0 };

    for (let second = 0; second < 60; second += 1) {
      setTimeout(() => {
        for (let index = 0; index < 30; index += 1) {
          tracer.startSpan(`busy ${second}.${index}`).end();

          produced.ended += 1;
        }
      }, second * 1000);
    }

    await vi.advanceTimersByTimeAsync(60_000);

    expect(produced.ended).toBe(1800);

    await provider.flush();

    await sentSignal.reached(9);

    const traces = sent.filter((entry) => entry.signal === "traces");

    expect(exportedSpanCount(traces)).toBe(produced.ended);

    for (const entry of traces) {
      expect(exportedSpanCount([entry])).toBeLessThanOrEqual(200);
    }
  });

  test("a burst beyond the 2048-span queue is dropped while an export is in flight (B03)", async () => {
    const held = gate();

    onTestFinished(held.open);

    worker.use(
      collector(async () => {
        await held.wait();

        return new HttpResponse(null);
      }),
    );

    const provider = buildProvider();

    const tracer = provider.tracerProvider.getTracer("b03");

    for (let index = 0; index < 3000; index += 1) {
      tracer.startSpan(`burst ${index}`).end();
    }

    await sentSignal.reached(1);

    held.open();

    await provider.flush();

    const exported = exportedSpanCount(
      sent.filter((entry) => entry.signal === "traces"),
    );

    expect(exported).toBeGreaterThanOrEqual(2048);
    expect(exported).toBeLessThanOrEqual(2048 + 200);
  });

  test("log records leave in requests of at most 200 records (B04)", async () => {
    expect.hasAssertions();

    worker.use(collector(ok));

    const provider = buildProvider();

    const logger = provider.loggerProvider.getLogger("b04");

    for (let index = 0; index < 450; index += 1) {
      logger.emit({ body: `log ${index}` });
    }

    await provider.flush();

    const logs = sent.filter((entry) => entry.signal === "logs");

    expect(exportedLogCount(logs)).toBe(450);

    for (const entry of logs) {
      expect(exportedLogCount([entry])).toBeLessThanOrEqual(200);
    }
  });

  test("metrics export on the 75 s interval with delta temporality (B05)", async () => {
    vi.useFakeTimers({
      toFake: ["setTimeout", "clearTimeout", "setInterval", "clearInterval"],
    });

    worker.use(collector(ok));

    const provider = buildProvider();

    provider.meterProvider.getMeter("b05").createCounter("clicks").add(1);

    await vi.advanceTimersByTimeAsync(74_000);

    expect(sent.filter((entry) => entry.signal === "metrics")).toHaveLength(0);

    await vi.advanceTimersByTimeAsync(1000);

    await sentSignal.reached(1);

    const metrics = sent.find((entry) => entry.signal === "metrics");

    expect(metrics?.text).toContain('"aggregationTemporality":1');
  });

  test("browser spans are always sampled; there is no head sampler (B49)", () => {
    worker.use(collector(ok));

    const span = buildProvider()
      .tracerProvider.getTracer("b49")
      .startSpan("sampled");

    expect(span.isRecording()).toBe(true);
    // oxlint-disable-next-line no-bitwise -- traceFlags is a bit field; bit 0 is sampled.
    expect(span.spanContext().traceFlags & 1).toBe(1);

    span.end();
  });
});
