import { CounterIdGenerator } from "./counter-id-generator";
import { PrerenderSafeIdGenerator } from "./id-generator.js";
import { describe, expect, test } from "vitest";

const TRACE_ID_RE = /^[0-9a-f]{32}$/u;

const SPAN_ID_RE = /^[0-9a-f]{16}$/u;

const INVALID_TRACE_ID = "0".repeat(32);

const INVALID_SPAN_ID = "0".repeat(16);

const SPAN_COUNTER_WRAP_AT = 2 ** 16;

describe(PrerenderSafeIdGenerator, () => {
  describe("normal path (Math.random available)", () => {
    test("delegates to OTel's RandomIdGenerator for trace IDs", () => {
      const gen = new PrerenderSafeIdGenerator();

      expect(gen.generateTraceId()).toMatch(TRACE_ID_RE);
    });

    test("delegates to OTel's RandomIdGenerator for span IDs", () => {
      const gen = new PrerenderSafeIdGenerator();

      expect(gen.generateSpanId()).toMatch(SPAN_ID_RE);
    });

    test("produces fully-random IDs (no shared prefix across calls)", () => {
      const gen = new PrerenderSafeIdGenerator();
      const traceIds = Array.from({ length: 50 }, () => gen.generateTraceId());
      const spanIds = Array.from({ length: 50 }, () => gen.generateSpanId());

      const uniqueTracePrefixes = new Set(
        traceIds.map((id) => id.slice(0, 24)),
      );

      const uniqueSpanPrefixes = new Set(spanIds.map((id) => id.slice(0, 12)));

      expect(uniqueTracePrefixes.size).toBeGreaterThan(40);
      expect(uniqueSpanPrefixes.size).toBeGreaterThan(40);
    });
  });

  describe("counter fallback", () => {
    test("returns valid IDs at the W3C widths", () => {
      expect.assertions(2);

      const gen = new CounterIdGenerator();

      expect(gen.generateTraceId()).toMatch(TRACE_ID_RE);
      expect(gen.generateSpanId()).toMatch(SPAN_ID_RE);
    });

    test("never produces the invalid-context sentinels", () => {
      expect.hasAssertions();

      const gen = new CounterIdGenerator();

      for (let i = 0; i < 100; i += 1) {
        expect(gen.generateTraceId()).not.toBe(INVALID_TRACE_ID);
        expect(gen.generateSpanId()).not.toBe(INVALID_SPAN_ID);
      }
    });

    test("produces unique trace IDs within a process", () => {
      expect.hasAssertions();

      const gen = new CounterIdGenerator();
      const seen = new Set<string>();

      for (let i = 0; i < 1000; i += 1) {
        seen.add(gen.generateTraceId());
      }

      expect(seen.size).toBe(1000);
    });

    test("produces unique span IDs within a process", () => {
      expect.hasAssertions();

      const gen = new CounterIdGenerator();
      const seen = new Set<string>();

      for (let i = 0; i < 1000; i += 1) {
        seen.add(gen.generateSpanId());
      }

      expect(seen.size).toBe(1000);
    });

    test("uses a fresh random prefix per generator instance", () => {
      expect.assertions(2);

      const a = new CounterIdGenerator();
      const b = new CounterIdGenerator();

      expect(a.generateTraceId()).not.toBe(b.generateTraceId());
      expect(a.generateSpanId()).not.toBe(b.generateSpanId());
    });

    test("keeps IDs at the spec width even when the counter wraps", () => {
      expect.hasAssertions();

      const gen = new CounterIdGenerator();

      for (let i = 0; i < SPAN_COUNTER_WRAP_AT - 2; i += 1) {
        gen.generateSpanId();
      }

      for (let i = 0; i < 4; i += 1) {
        const id = gen.generateSpanId();

        expect(id).toMatch(SPAN_ID_RE);
      }
    });
  });
});
