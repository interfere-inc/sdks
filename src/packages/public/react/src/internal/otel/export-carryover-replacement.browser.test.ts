import { ExportResultCode } from "@opentelemetry/core";
import type { ExportResult } from "@opentelemetry/core";
import { expect, onTestFinished, test } from "vitest";

import {
  clearExportCarryover,
  withExportCarryover,
} from "./export-carryover.js";
import type { SignalSender } from "./exporter.js";

test("a replacement exporter drains the same scope while its disposed predecessor waits for the lock", async () => {
  const scope = "replacement-exporter";
  const lockName = `interfere:export:v1:${scope}`;
  const acquired = Promise.withResolvers<null>();
  const release = Promise.withResolvers<null>();
  const bodies: string[] = [];
  const results: ExportResult[] = [];

  const send: SignalSender = (payload, callback) => {
    bodies.push(new TextDecoder().decode(payload));

    callback({ code: ExportResultCode.SUCCESS });
  };

  clearExportCarryover();

  onTestFinished(() => clearExportCarryover());

  const lock = navigator.locks.request(lockName, async () => {
    acquired.resolve(null);

    await release.promise;
  });

  onTestFinished(() => release.resolve(null));

  await acquired.promise;

  const predecessor = withExportCarryover(
    scope,
    send,
    () => true,
    () => true,
  );

  onTestFinished(() => predecessor.dispose());

  await expect
    .poll(async () =>
      (await navigator.locks.query()).pending?.filter(
        ({ name }) => name === lockName,
      ),
    )
    .toHaveLength(1);

  predecessor.dispose();

  const replacement = withExportCarryover(
    scope,
    send,
    () => true,
    () => true,
  );

  const result = Promise.withResolvers<ExportResult>();

  onTestFinished(() => replacement.dispose());

  replacement.send(new TextEncoder().encode("replacement"), (value) => {
    results.push(value);

    result.resolve(value);
  });

  await expect
    .poll(async () =>
      (await navigator.locks.query()).pending?.filter(
        ({ name }) => name === lockName,
      ),
    )
    .toHaveLength(2);

  release.resolve(null);

  await lock;

  await expect(result.promise).resolves.toStrictEqual({
    code: ExportResultCode.SUCCESS,
  });

  expect(bodies).toStrictEqual(["replacement"]);
  expect(localStorage).toHaveLength(0);
  expect(results).toHaveLength(1);
});
