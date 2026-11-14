import {
  classifyReplayEvent,
  splitReplayEvents,
  toReplayChunkPayload,
  utf8ByteLength,
} from "./chunking.js";
import type { BufferedReplayEvent } from "./chunking.js";
import { describe, expect, test } from "vitest";

const UPLOAD_LIMIT_BYTES = 8 * 1024 * 1024;

const TARGET_CHUNK_BYTES = 6 * 1024 * 1024;

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Classification tests deliberately serialize incomplete or malformed replay events without asserting rrweb validity.
function buffered(event: unknown, ts: number) {
  const json = JSON.stringify(event);
  const { isActivity, isFullSnapshot } = classifyReplayEvent(event);

  return {
    bytes: utf8ByteLength(json),
    isActivity,
    isFullSnapshot,
    json,
    ts,
  } satisfies BufferedReplayEvent;
}

function sized(bytes: number, ts: number) {
  return {
    bytes,
    isActivity: false,
    isFullSnapshot: false,
    json: `{"ts":${ts}}`,
    ts,
  } satisfies BufferedReplayEvent;
}

function parseJsonl(body: string) {
  return body
    .split("\n")
    .filter((line) => line.length > 0)
    .map((line) => {
      const value: unknown = JSON.parse(line);

      return value;
    });
}

describe("plugins/replay/chunking", () => {
  describe(utf8ByteLength, () => {
    test("counts ascii bytes the same as string length", () => {
      expect(utf8ByteLength("abc")).toBe(3);
    });

    test("counts multibyte payloads by utf-8 bytes, not code units", () => {
      const emoji = "😀";
      const cjk = "中";

      expect(emoji).toHaveLength(2);
      expect(utf8ByteLength(emoji)).toBe(4);
      expect(cjk).toHaveLength(1);
      expect(utf8ByteLength(cjk)).toBe(3);
    });

    test("matches TextEncoder accounting for mixed content", () => {
      const value = String.raw`a"😀\中`;

      expect(utf8ByteLength(value)).toBe(
        new TextEncoder().encode(value).length,
      );
    });
  });

  describe(toReplayChunkPayload, () => {
    test("emits newline-delimited JSON objects, not strings", () => {
      const events = [
        { data: { source: 2 }, timestamp: 10, type: 3 },
        { data: {}, timestamp: 20, type: 2 },
      ];

      const payload = toReplayChunkPayload(
        events.map((event, index) => buffered(event, 100 + index)),
      );

      const parsed = parseJsonl(payload.bodyJson);

      expect(parsed).toStrictEqual(events);
    });

    test("preserves escaped strings and round-trips losslessly", () => {
      const events = [
        { data: { tag: String.raw`he said "hi"\back` }, timestamp: 1, type: 5 },
        {
          data: { nl: "a\nb", text: "emoji 😀 and 中文" },
          timestamp: 2,
          type: 5,
        },
      ];

      const payload = toReplayChunkPayload(
        events.map((event, index) => buffered(event, 200 + index)),
      );

      expect(parseJsonl(payload.bodyJson)).toStrictEqual(events);
      expect(payload.eventCount).toBe(2);
      expect(payload.firstTs).toBe(200);
      expect(payload.lastTs).toBe(201);
    });

    test("reports null timestamps for an empty group", () => {
      const payload = toReplayChunkPayload([]);

      expect(payload.bodyJson).toBe("");
      expect(payload.eventCount).toBe(0);
      expect(payload.firstTs).toBeNull();
      expect(payload.lastTs).toBeNull();
      expect(payload.hasFullSnapshot).toBe(false);
      expect(payload.activityCount).toBe(0);
    });

    test("flags hasFullSnapshot when a full snapshot event is present", () => {
      const withSnapshot = toReplayChunkPayload([
        buffered({ data: { source: 2 }, timestamp: 1, type: 3 }, 0),
        buffered({ data: {}, timestamp: 2, type: 2 }, 1),
      ]);

      expect(withSnapshot.hasFullSnapshot).toBe(true);

      const withoutSnapshot = toReplayChunkPayload([
        buffered({ data: { source: 2 }, timestamp: 1, type: 3 }, 0),
      ]);

      expect(withoutSnapshot.hasFullSnapshot).toBe(false);
    });

    test("counts activity events in the chunk", () => {
      const payload = toReplayChunkPayload([
        buffered({ data: { source: 1 }, timestamp: 1, type: 3 }, 1000),
        buffered({ data: { source: 1 }, timestamp: 2, type: 3 }, 1200),
        buffered({ data: { source: 3 }, timestamp: 3, type: 3 }, 2500),
        buffered({ data: {}, timestamp: 4, type: 2 }, 2600),
      ]);

      expect(payload.activityCount).toBe(3);
    });
  });

  describe(classifyReplayEvent, () => {
    test("flags full snapshots", () => {
      expect(classifyReplayEvent({ data: {}, type: 2 })).toStrictEqual({
        isActivity: false,
        isFullSnapshot: true,
      });
    });

    test("flags activity sources on incremental snapshots", () => {
      expect(
        classifyReplayEvent({ data: { source: 2 }, type: 3 }),
      ).toStrictEqual({
        isActivity: true,
        isFullSnapshot: false,
      });
    });

    test("ignores non-activity incremental sources", () => {
      expect(
        classifyReplayEvent({ data: { source: 0 }, type: 3 }),
      ).toStrictEqual({
        isActivity: false,
        isFullSnapshot: false,
      });
    });

    test("is safe for malformed events", () => {
      expect(classifyReplayEvent(null)).toStrictEqual({
        isActivity: false,
        isFullSnapshot: false,
      });

      expect(classifyReplayEvent({ type: "click" })).toStrictEqual({
        isActivity: false,
        isFullSnapshot: false,
      });
    });

    test.each([
      undefined,
      3,
      "event",
      {},
      { type: "3", data: { source: 2 } },
      { type: 3, data: null },
      { type: 3, data: "input" },
      { type: 3, data: {} },
      { type: 3, data: { source: "2" } },
      { type: 3, data: { source: Number.NaN } },
    ])("ignores malformed discriminator or activity data %j", (event) => {
      expect(classifyReplayEvent(event)).toStrictEqual({
        isActivity: false,
        isFullSnapshot: false,
      });
    });
  });

  describe(splitReplayEvents, () => {
    test("keeps small events in a single chunk", () => {
      const events = [buffered({ a: 1 }, 1), buffered({ b: 2 }, 2)];
      const { chunks, droppedEventBytes } = splitReplayEvents(events);

      expect(droppedEventBytes).toStrictEqual([]);
      expect(chunks).toHaveLength(1);
      expect(chunks[0]).toStrictEqual(events);
    });

    test("drops a single event that exceeds the upload limit", () => {
      const oversized = sized(UPLOAD_LIMIT_BYTES, 1);

      const { chunks, droppedEventBytes } = splitReplayEvents([
        buffered({ ok: true }, 0),
        oversized,
      ]);

      expect(chunks).toHaveLength(1);
      expect(chunks[0]).toHaveLength(1);
      expect(droppedEventBytes).toStrictEqual([UPLOAD_LIMIT_BYTES + 1]);
    });

    test("splits at the target boundary while preserving event order", () => {
      expect.hasAssertions();

      const halfish = Math.floor(TARGET_CHUNK_BYTES * 0.4);

      const events = [
        sized(halfish, 1),
        sized(halfish, 2),
        sized(halfish, 3),
        sized(halfish, 4),
      ];

      const { chunks, droppedEventBytes } = splitReplayEvents(events);

      expect(droppedEventBytes).toStrictEqual([]);
      expect(chunks).toHaveLength(2);
      expect(chunks.flat()).toStrictEqual(events);

      for (const chunk of chunks) {
        const chunkBytes = chunk.reduce((sum, entry) => sum + entry.bytes, 0);

        expect(chunkBytes).toBeLessThanOrEqual(TARGET_CHUNK_BYTES);
      }
    });
  });
});
