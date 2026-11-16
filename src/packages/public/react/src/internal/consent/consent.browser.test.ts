import {
  DEFAULT_CONSENT,
  PENDING_CONSENT,
  hasConsentChanged,
  isConsentAllowed,
  resolveConsentDecision,
} from "./consent.js";
import { describe, expect, test } from "vitest";

describe("consent policy", () => {
  describe(isConsentAllowed, () => {
    test("allows necessary category regardless of consent state", () => {
      expect(isConsentAllowed("necessary", PENDING_CONSENT)).toBe(true);
    });

    test("allows every category by default", () => {
      expect(isConsentAllowed("analytics", DEFAULT_CONSENT)).toBe(true);
      expect(isConsentAllowed("replay", DEFAULT_CONSENT)).toBe(true);
    });

    test("blocks every gated category while consent is pending", () => {
      expect(isConsentAllowed("analytics", PENDING_CONSENT)).toBe(false);
      expect(isConsentAllowed("replay", PENDING_CONSENT)).toBe(false);
    });

    test("blocks only the denied category", () => {
      const state = resolveConsentDecision({ replay: false });

      expect(isConsentAllowed("analytics", state)).toBe(true);
      expect(isConsentAllowed("replay", state)).toBe(false);
    });
  });

  describe(resolveConsentDecision, () => {
    test.each([
      [true, { analytics: true, replay: true }],
      [false, { analytics: false, replay: false }],
      [{ replay: false }, { analytics: true, replay: false }],
      [{ replay: true }, { analytics: true, replay: true }],
    ] as const)("resolves %j", (decision, expected) => {
      expect(resolveConsentDecision(decision)).toStrictEqual(expected);
    });
  });

  describe(hasConsentChanged, () => {
    test("returns false for identical states", () => {
      expect(
        hasConsentChanged(DEFAULT_CONSENT, resolveConsentDecision(true)),
      ).toBe(false);
    });

    test("returns true when analytics changes", () => {
      expect(hasConsentChanged(DEFAULT_CONSENT, PENDING_CONSENT)).toBe(true);
    });

    test("returns true when replay changes", () => {
      expect(
        hasConsentChanged(
          DEFAULT_CONSENT,
          resolveConsentDecision({ replay: false }),
        ),
      ).toBe(true);
    });
  });
});
