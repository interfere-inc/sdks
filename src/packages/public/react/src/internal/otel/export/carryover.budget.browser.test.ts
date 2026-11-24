import { ExportResultCode } from "@opentelemetry/core";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, onTestFinished, test } from "vitest";

import {
  CLIENT_OPTIONS,
  INGEST,
  collector,
  exportedSpanCount,
  gate,
  hidePage,
  makeSpan,
  resetSent,
  resultCallback,
  sent,
  sentSignal,
  showPage,
} from "../../../../test/fixtures/export-collector.js";
import { worker } from "../../../../test/msw.js";
import { createReplayCarryover } from "../../../plugins/replay/upload/carryover.js";
import {
  isPageHidden,
  reserveKeepaliveBody,
} from "../../page-lifecycle/page-lifecycle.js";
import { clearExportCarryover, withExportCarryover } from "./carryover.js";
import { createTraceExporter } from "../exporter.js";

const prefix = "interfere:export:v1:";
const scope = JSON.stringify([INGEST, "test-key", "traces"]);

function retained() {
  return Array.from({ length: localStorage.length }, (_, index) =>
    localStorage.key(index),
  ).filter((key): key is string => key?.startsWith(prefix) ?? false);
}

beforeEach(() => {
  isPageHidden();

  clearExportCarryover();

  resetSent();
});

afterEach(() => {
  clearExportCarryover();

  showPage();
});

test("a hidden trace batch drains when a preceding replay upload releases its keepalive reservation", async () => {
  expect.hasAssertions();

  const held = gate();
  const arrived = gate();
  const replayRequests: { bytes: number; keepalive: boolean }[] = [];

  onTestFinished(held.open);

  worker.use(
    http.post(`${INGEST}/v3/replay/upload/:id`, async ({ request }) => {
      replayRequests.push({
        bytes: (await request.arrayBuffer()).byteLength,
        keepalive: request.keepalive,
      });

      arrived.open();

      await held.wait();

      return HttpResponse.json({ objectKey: "replay/hidden" });
    }),
    collector(() => new HttpResponse(null)),
  );

  const replay = createReplayCarryover(
    { ...CLIENT_OPTIONS, headers: {}, maxRetries: 0, timeoutInSeconds: 30 },
    () => true,
  );

  const exporter = createTraceExporter({ clientOptions: CLIENT_OPTIONS });

  onTestFinished(() => replay.dispose());

  onTestFinished(async () => exporter.shutdown());

  hidePage();

  const uploaded = Promise.allSettled([
    replay.send(new Uint8Array(24_000), {
      sessionId: "hidden-session",
      meta: "",
      encoding: null,
    }),
  ]);

  await arrived.wait();

  const result = resultCallback();

  exporter.export([makeSpan({ payload: "x".repeat(12_000) })], result.callback);

  await expect(result.called).resolves.toStrictEqual({
    code: ExportResultCode.SUCCESS,
  });

  await navigator.locks.request(prefix + scope, () => {
    expect(sent).toHaveLength(0);
    expect(retained()).toHaveLength(2);
  });

  expect(replayRequests).toStrictEqual([{ bytes: 24_000, keepalive: true }]);

  held.open();

  await expect(uploaded).resolves.toMatchObject([
    { status: "fulfilled", value: { ok: true } },
  ]);

  await sentSignal.reached(1);

  await navigator.locks.request(prefix + scope, () => {
    expect(retained()).toHaveLength(0);
  });

  expect(isPageHidden()).toBe(true);
  expect(sent).toHaveLength(1);
  expect(sent[0]?.keepalive).toBe(true);
  expect(exportedSpanCount(sent)).toBe(1);
  expect(result.results).toHaveLength(1);
});

test("a release arriving before the blocked drain settles is remembered", async () => {
  const release = reserveKeepaliveBody(24_000);

  let attempts = 0;

  const carryover = withExportCarryover(
    "release-race",
    (_payload, callback, _signal, retainedSend) => {
      attempts += 1;

      if (attempts === 1) {
        retainedSend?.onBudgetExhausted();

        release?.();

        callback({
          code: ExportResultCode.FAILED,
          error: new Error("Keepalive budget exhausted"),
        });

        return;
      }

      callback({ code: ExportResultCode.SUCCESS });
    },
    () => true,
  );

  onTestFinished(() => {
    carryover.dispose();

    release?.();
  });

  hidePage();

  const result = resultCallback();

  carryover.send(new TextEncoder().encode("retained"), result.callback);

  await expect.poll(retained).toHaveLength(0);

  expect(attempts).toBe(2);
  expect(result.results).toStrictEqual([{ code: ExportResultCode.SUCCESS }]);
  expect(isPageHidden()).toBe(true);
});

test("a later network failure does not keep waiting on a previous budget failure", async () => {
  const release = reserveKeepaliveBody(32_000);

  let attempts = 0;

  const carryover = withExportCarryover(
    "network-after-budget",
    (_payload, callback, _signal, retainedSend) => {
      attempts += 1;

      if (attempts === 1) {
        retainedSend?.onBudgetExhausted();
      }

      callback({
        code: ExportResultCode.FAILED,
        error: new TypeError("Failed to fetch"),
      });
    },
    () => true,
  );

  onTestFinished(() => {
    carryover.dispose();

    release?.();
  });

  hidePage();

  const result = resultCallback();

  carryover.send(new TextEncoder().encode("retained"), result.callback);

  await result.called;

  await navigator.locks.request(`${prefix}network-after-budget`, () => null);

  showPage();

  await navigator.locks.request(`${prefix}network-after-budget`, () => null);

  release?.();

  await navigator.locks.request(`${prefix}network-after-budget`, () => null);

  expect(attempts).toBe(2);
  expect(retained()).toHaveLength(1);
  expect(result.results).toHaveLength(1);
});

test.each(["dispose", "clear", "consent"] as const)(
  "a parked batch stays unsent after %s when capacity is released",
  async (stop) => {
    const release = reserveKeepaliveBody(32_000);

    let allowed = true;

    const exporter = createTraceExporter({
      clientOptions: CLIENT_OPTIONS,
      canPersistExports: () => allowed,
    });

    onTestFinished(async () => {
      await exporter.shutdown();

      release?.();
    });

    worker.use(collector(() => new HttpResponse(null)));

    hidePage();

    const result = resultCallback();

    exporter.export([makeSpan()], result.callback);

    await result.called;

    await navigator.locks.request(prefix + scope, () => null);

    expect(retained()).toHaveLength(1);

    if (stop === "dispose") {
      await exporter.shutdown();
    } else if (stop === "clear") {
      clearExportCarryover();
    } else {
      allowed = false;
    }

    release?.();

    await navigator.locks.request(prefix + scope, () => null);

    expect(sent).toHaveLength(0);
    expect(result.results).toHaveLength(1);
  },
);

test("a send begun on a visible page that fails after the page hides is retained and delivered", async () => {
  expect.hasAssertions();

  showPage();

  let attempts = 0;

  const carryover = withExportCarryover(
    "hide-in-flight",
    (_payload, callback) => {
      attempts += 1;

      if (attempts === 1) {
        hidePage();

        callback({
          code: ExportResultCode.FAILED,
          error: new Error("Keepalive budget exhausted"),
        });

        return;
      }

      callback({ code: ExportResultCode.SUCCESS });
    },
    () => true,
  );

  onTestFinished(() => carryover.dispose());

  const result = resultCallback();

  carryover.send(new TextEncoder().encode("late-hide"), result.callback);

  await expect(result.called).resolves.toStrictEqual({
    code: ExportResultCode.SUCCESS,
  });

  await expect.poll(retained).toHaveLength(0);

  expect(attempts).toBe(2);
  expect(result.results).toStrictEqual([{ code: ExportResultCode.SUCCESS }]);
});

test("a failure on a page that stays visible is reported, not retained", async () => {
  expect.hasAssertions();

  showPage();

  const carryover = withExportCarryover(
    "stays-visible",
    (_payload, callback) => {
      callback({
        code: ExportResultCode.FAILED,
        error: new TypeError("Failed to fetch"),
      });
    },
    () => true,
  );

  onTestFinished(() => carryover.dispose());

  const result = resultCallback();

  carryover.send(new TextEncoder().encode("stays-visible"), result.callback);

  await expect(result.called).resolves.toMatchObject({
    code: ExportResultCode.FAILED,
  });

  expect(retained()).toHaveLength(0);
});
