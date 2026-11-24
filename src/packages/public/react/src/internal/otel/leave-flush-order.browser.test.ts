import { SpanKind } from "@opentelemetry/api";
import { afterEach, beforeEach, expect, onTestFinished, test } from "vitest";

import {
  CLIENT_OPTIONS,
  collector,
  gate,
  hidePage,
  ok,
  resetSent,
  sent,
  sentSignal,
  showPage,
} from "../../../test/fixtures/export-collector.js";
import { worker } from "../../../test/msw.js";
import { reserveKeepaliveBody } from "../page-lifecycle/page-lifecycle.js";
import { clearExportCarryover } from "./export/carryover.js";
import { buildOtelProvider } from "./provider.js";

const prefix = "interfere:export:v1:";

function retained() {
  return Array.from({ length: localStorage.length }, (_, index) =>
    localStorage.key(index),
  ).filter((key): key is string => key?.startsWith(prefix) ?? false);
}

beforeEach(() => {
  clearExportCarryover();

  resetSent();
});

afterEach(() => {
  clearExportCarryover();

  showPage();
});

test("a leave-time flush that cannot fit everything sends logs before spans and retains the spans", async () => {
  expect.hasAssertions();

  const held = gate();

  onTestFinished(held.open);

  worker.use(
    collector(async () => {
      await held.wait();

      return ok();
    }),
  );

  const handle = buildOtelProvider({
    clientOptions: CLIENT_OPTIONS,
    getDeviceId: () => null,
    getSessionId: () => null,
    sdkStack: ["test"],
  });

  onTestFinished(async () => handle.shutdown());

  const tracer = handle.tracerProvider.getTracer("leave-order");

  for (let index = 0; index < 8; index += 1) {
    tracer
      .startSpan(`GET /api/${index}`, {
        attributes: { padding: "y".repeat(1400) },
        kind: SpanKind.CLIENT,
      })
      .end();
  }

  const logger = handle.loggerProvider.getLogger("leave-order");

  for (let index = 0; index < 5; index += 1) {
    logger.emit({ body: "x".repeat(1000) });
  }

  const taken = reserveKeepaliveBody(14_000);

  onTestFinished(() => taken?.());

  hidePage();

  const flushing = handle.flush();

  await sentSignal.reached(1);

  expect(sent.find((entry) => entry.signal === "logs")?.keepalive).toBe(true);

  expect(sent.filter((entry) => entry.signal === "traces")).toHaveLength(0);

  expect(retained().length).toBeGreaterThan(0);

  held.open();

  await flushing;
});
