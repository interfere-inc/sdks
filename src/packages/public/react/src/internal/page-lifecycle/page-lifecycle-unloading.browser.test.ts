import { ExportResultCode } from "@opentelemetry/core";

import { sendInKeepaliveChunks } from "../otel/keepalive-chunks.js";
import {
  isPageHidden,
  onPageUnloading,
  resetPageLifecycleForTests,
} from "./page-lifecycle.js";
import { afterEach, beforeEach, expect, test } from "vitest";

const encoder = new TextEncoder();

beforeEach(() => {
  resetPageLifecycleForTests();
});

afterEach(() => {
  resetPageLifecycleForTests();
});

test("an export started by an unloading subscriber is sent directly in keepalive-sized chunks", () => {
  const items = Array.from({ length: 400 }, (_, index) =>
    `${index}:`.padEnd(200, "x"),
  );

  const sent: { bytes: number; hidden: boolean }[] = [];

  onPageUnloading(() => {
    sendInKeepaliveChunks(
      items,
      (chunk) => encoder.encode(JSON.stringify(chunk)),
      (payload, callback) => {
        sent.push({ bytes: payload?.byteLength ?? 0, hidden: isPageHidden() });

        callback({ code: ExportResultCode.SUCCESS });
      },
      () => undefined,
    );
  });

  globalThis.dispatchEvent(
    new PageTransitionEvent("pagehide", { persisted: false }),
  );

  expect(sent.length).toBeGreaterThan(1);

  expect(sent.every((request) => request.bytes <= 16_000)).toBe(true);

  expect(sent.every((request) => !request.hidden)).toBe(true);
});

test("a leave-time export is packed into the fewest keepalive-sized chunks, in order", () => {
  const items = Array.from({ length: 44 }, (_, index) => index);

  const sent: number[][] = [];
  const chunkOf = new Map<Uint8Array, number[]>();

  onPageUnloading(() => {
    sendInKeepaliveChunks(
      items,
      (chunk) => {
        const payload = encoder.encode("x".repeat(1200 + chunk.length * 1800));

        chunkOf.set(payload, chunk);

        return payload;
      },
      (payload, callback) => {
        sent.push(chunkOf.get(payload ?? new Uint8Array()) ?? []);

        callback({ code: ExportResultCode.SUCCESS });
      },
      () => undefined,
    );
  });

  globalThis.dispatchEvent(
    new PageTransitionEvent("pagehide", { persisted: false }),
  );

  expect(sent.length).toBeLessThanOrEqual(
    Math.ceil((44 * 1800 + 1200) / 16_000) + 1,
  );

  expect(sent.flat()).toStrictEqual(items);
});
