import { expect, test } from "vitest";

import {
  currentSpanGate,
  holdSpansForLogs,
  noteLogsDecided,
  noteLogsExporting,
} from "./leave-gate.js";

test("a log decision releases every held gate, including one a later flush replaced", async () => {
  expect.hasAssertions();

  holdSpansForLogs();

  const first = currentSpanGate();

  holdSpansForLogs();

  const second = currentSpanGate();

  noteLogsExporting();

  for (let tick = 0; tick < 200; tick += 1) {
    // oxlint-disable-next-line no-await-in-loop -- Lets every fallback loop finish while the log export is still pending.
    await Promise.resolve();
  }

  noteLogsDecided();

  await expect(
    Promise.all([first?.wait(), second?.wait()]),
  ).resolves.toStrictEqual([true, true]);

  expect(currentSpanGate()).toBeUndefined();
});

test("with no logs to export the held spans are released after a few microtasks", async () => {
  expect.hasAssertions();

  holdSpansForLogs();

  await expect(currentSpanGate()?.wait()).resolves.toBe(true);
});
