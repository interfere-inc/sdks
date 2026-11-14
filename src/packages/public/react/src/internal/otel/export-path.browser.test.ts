import { createLogExporter, createMetricExporter } from "./exporter.js";
import { clearExportCarryover } from "./export-carryover.js";
import { isPageHidden } from "../page-lifecycle/page-lifecycle.js";
import {
  afterEach,
  beforeEach,
  describe,
  expect,
  onTestFinished,
  test,
  vi,
} from "vitest";
import { HttpResponse, http } from "msw";

import {
  CLIENT_OPTIONS,
  bulkySpans,
  collector,
  exportSpans,
  exportedLogCount,
  exportedSpanCount,
  gate,
  hidePage,
  makeLogs,
  makeResourceMetrics,
  makeSpan,
  ok,
  resetSent,
  resultCallback,
  sent,
  sentSignal,
  showPage,
} from "../../../test/fixtures/export-collector.js";
import { worker } from "../../../test/msw.js";

beforeEach(() => {
  isPageHidden();

  clearExportCarryover();

  resetSent();
});

afterEach(() => {
  showPage();

  vi.useRealTimers();
});

describe("B12 bfcache restore", () => {
  test("after pageshow a large batch leaves as one request again (B12)", async () => {
    worker.use(collector(ok));

    hidePage();

    showPage();

    const result = exportSpans(bulkySpans(40));

    await expect(result.called).resolves.toStrictEqual({ code: 0 });

    expect(sent).toHaveLength(1);
  });
});

describe("B14 split hidden export result", () => {
  test("a hidden export reports one FAILED result when any chunk fails (B14)", async () => {
    worker.use(
      collector((index) =>
        index === 1
          ? new HttpResponse(null, { status: 400 })
          : new HttpResponse(null),
      ),
    );

    hidePage();

    const result = exportSpans(bulkySpans(40));

    const first = await result.called;

    expect(sent.length).toBeGreaterThan(2);
    expect(result.results).toHaveLength(1);
    expect(first.code).toBe(1);
  });
});

describe("B15 hidden log export", () => {
  test("while hidden a large log batch leaves as keepalive-sized requests (B15)", async () => {
    expect.hasAssertions();

    worker.use(collector(ok));

    hidePage();

    const result = resultCallback();

    createLogExporter({ clientOptions: CLIENT_OPTIONS }).export(
      makeLogs(40),
      result.callback,
    );

    await expect(result.called).resolves.toStrictEqual({ code: 0 });

    expect(sent.length).toBeGreaterThan(1);

    for (const entry of sent) {
      expect(entry.bytes).toBeLessThanOrEqual(16_000);
    }

    expect(exportedLogCount(sent)).toBe(40);
  });
});

describe("hide-time flush over the keepalive share", () => {
  test("every chunk of an 80 KB hide-time flush is retained before sending with keepalive", async () => {
    const held = gate();

    onTestFinished(held.open);

    worker.use(
      collector(async () => {
        await held.wait();

        return new HttpResponse(null);
      }),
    );

    hidePage();

    const result = exportSpans(bulkySpans(40));

    await sentSignal.reached(1);

    expect(
      Array.from(
        { length: localStorage.length },
        (_, index) => localStorage.key(index) ?? "",
      ).filter((key) => key.startsWith("interfere:export:v1:")).length,
    ).toBeGreaterThan(2);

    held.open();

    await result.called;

    expect(sent.map((entry) => entry.keepalive)).not.toContain(false);
    expect(exportedSpanCount(sent)).toBe(40);
  });
});

describe("B21-B23 keepalive budget across failures and retries", () => {
  test("a failed export releases its keepalive budget (B21)", async () => {
    worker.use(
      collector((index) =>
        index === 0
          ? new HttpResponse(null, { status: 400 })
          : new HttpResponse(null),
      ),
    );

    const failed = exportSpans(bulkySpans(1, 25_000));

    await expect(failed.called).resolves.toHaveProperty("code", 1);

    const next = exportSpans(bulkySpans(1, 25_000));

    await expect(next.called).resolves.toStrictEqual({ code: 0 });

    expect(sent.map((entry) => entry.keepalive)).toStrictEqual([true, true]);
  });

  test("a Retry-After wait does not hold keepalive budget for a request no longer in flight (B22)", async () => {
    const answered = Promise.withResolvers<null>();

    worker.use(
      collector((index) => {
        if (index !== 0) {
          return new HttpResponse(null);
        }

        setTimeout(() => {
          answered.resolve(null);
        }, 300);

        return new HttpResponse(null, {
          status: 429,
          headers: { "Retry-After": "3" },
        });
      }),
    );

    const throttled = exportSpans(bulkySpans(1, 25_000));

    await answered.promise;

    const next = exportSpans(bulkySpans(1, 25_000));

    await expect(next.called).resolves.toStrictEqual({ code: 0 });

    expect(sent[1]?.keepalive).toBe(true);

    await expect(throttled.called).resolves.toStrictEqual({ code: 0 });

    expect(sent).toHaveLength(3);
  });

  test("a keepalive rejected by the browser quota is resent without keepalive and does not back off other signals (B23)", async () => {
    const hostBeacon = gate();

    onTestFinished(hostBeacon.open);

    worker.use(
      http.post(`${location.origin}/host-beacon`, async () => {
        await hostBeacon.wait();

        return new HttpResponse(null);
      }),
      collector(ok),
    );

    const beacon = fetch("/host-beacon", {
      body: "h".repeat(40_000),
      keepalive: true,
      method: "POST",
    });

    const traces = exportSpans(bulkySpans(1, 36_000));

    await expect(traces.called).resolves.toStrictEqual({ code: 0 });

    expect(sent.map((entry) => entry.keepalive)).toStrictEqual([false]);

    const logs = resultCallback();

    createLogExporter({ clientOptions: CLIENT_OPTIONS }).export(
      makeLogs(1, 10),
      logs.callback,
    );

    await expect(logs.called).resolves.toStrictEqual({ code: 0 });

    hostBeacon.open();

    await beacon;
  });
});

describe("B29 shared network backoff", () => {
  test("a real network failure backs off traces, logs and metrics together (B29)", async () => {
    worker.use(collector(() => HttpResponse.error()));

    const traces = exportSpans([makeSpan()]);

    await expect(traces.called).resolves.toHaveProperty("code", 1);

    const attempted = sent.length;

    const logs = resultCallback();

    createLogExporter({ clientOptions: CLIENT_OPTIONS }).export(
      makeLogs(1, 10),
      logs.callback,
    );

    const metrics = resultCallback();

    createMetricExporter({ clientOptions: CLIENT_OPTIONS }).export(
      makeResourceMetrics(),
      metrics.callback,
    );

    await expect(logs.called).resolves.toHaveProperty("code", 1);

    await expect(metrics.called).resolves.toHaveProperty("code", 1);

    expect(sent).toHaveLength(attempted);
  });
});

describe("transient gateway errors", () => {
  test.each([500, 502, 504])(
    "retries a transient %s and delivers the batch",
    async (status) => {
      worker.use(
        collector((index) =>
          index === 0
            ? new HttpResponse(null, {
                status,
                headers: { "Retry-After": "0" },
              })
            : new HttpResponse(null),
        ),
      );

      const result = exportSpans([makeSpan()]);

      await expect(result.called).resolves.toStrictEqual({ code: 0 });

      expect(sent).toHaveLength(2);
    },
  );
});

describe("gateway body cap", () => {
  test("no request exceeds the gateway's 8 MiB OTLP body cap", async () => {
    expect.hasAssertions();

    worker.use(collector(ok));

    const result = exportSpans(bulkySpans(200, 45_000));

    await expect(result.called).resolves.toStrictEqual({ code: 0 });

    for (const entry of sent) {
      expect(entry.bytes).toBeLessThanOrEqual(8 * 1024 * 1024);
    }

    expect(exportedSpanCount(sent)).toBe(200);
  });
});
