import {
  hasConsentChanged,
  isConsentAllowed,
  resolveGrantedConsent,
} from "./consent.js";
import { describe, expect, test } from "vitest";

describe("consent policy", () => {
  describe(isConsentAllowed, () => {
    test("allows necessary category regardless of consent state", () => {
      expect(isConsentAllowed("necessary", null)).toBe(true);

      expect(
        isConsentAllowed("necessary", { analytics: false, replay: false }),
      ).toBe(true);
    });

    test("allows all categories when consent state is null (no gating)", () => {
      expect(isConsentAllowed("analytics", null)).toBe(true);
      expect(isConsentAllowed("replay", null)).toBe(true);
    });

    test("allows a category when explicitly granted", () => {
      expect(
        isConsentAllowed("analytics", { analytics: true, replay: false }),
      ).toBe(true);
    });

    test("blocks a category when explicitly denied", () => {
      expect(
        isConsentAllowed("analytics", { analytics: false, replay: true }),
      ).toBe(false);
    });

    test("blocks a category when not present in consent state", () => {
      expect(isConsentAllowed("replay", { analytics: true })).toBe(false);
    });
  });

  describe(resolveGrantedConsent, () => {
    test.each([
      [true, { analytics: true, replay: true }],
      [false, { analytics: false, replay: false }],
    ] as const)(
      "resolves %s into current category permissions",
      (input, expected) => {
        expect(resolveGrantedConsent(input)).toStrictEqual(expected);
      },
    );

    test("returns all-granted when called with no argument", () => {
      expect(resolveGrantedConsent()).toStrictEqual({
        analytics: true,
        replay: true,
      });
    });

    test("merges a partial consent state over the defaults", () => {
      expect(resolveGrantedConsent({ analytics: false })).toStrictEqual({
        analytics: false,
        replay: true,
      });
    });

    test("keeps explicit values when merging over the defaults", () => {
      expect(
        resolveGrantedConsent({ analytics: false, replay: false }),
      ).toStrictEqual({ analytics: false, replay: false });
    });
  });

  describe(hasConsentChanged, () => {
    test("returns false for identical states", () => {
      const state = { analytics: true, replay: false };

      expect(hasConsentChanged(state, { analytics: true, replay: false })).toBe(
        false,
      );
    });

    test("returns true when analytics changes", () => {
      expect(
        hasConsentChanged(
          { analytics: true, replay: false },
          { analytics: false, replay: false },
        ),
      ).toBe(true);
    });

    test("returns true when replay changes", () => {
      expect(
        hasConsentChanged(
          { analytics: true, replay: false },
          { analytics: true, replay: true },
        ),
      ).toBe(true);
    });

    test("returns true when transitioning from null to a state", () => {
      expect(hasConsentChanged(null, { analytics: true, replay: true })).toBe(
        true,
      );
    });

    test("returns true when transitioning from a state to null", () => {
      expect(hasConsentChanged({ analytics: true, replay: true }, null)).toBe(
        true,
      );
    });

    test("returns false for null to null", () => {
      expect(hasConsentChanged(null, null)).toBe(false);
    });
  });
});
