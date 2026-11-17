import { toReleaseSlug } from "./slug-guard.ts";
import { deriveReleaseSlug, releaseSlugSchema } from "./slug.ts";
import { describe, expect, test } from "vitest";

const SLUG_RE = /^rel_[0-9a-f]{16}$/u;

const SHA_A = "abc0123456789def0123456789abcdef01234567";

const SHA_B = "def0123456789abc0123456789abcdef89abcdef";

describe(deriveReleaseSlug, () => {
  test("produces the rel_<16 hex chars> shape", () => {
    expect(deriveReleaseSlug(SHA_A)).toMatch(SLUG_RE);
  });

  test("is deterministic for the same commit SHA", () => {
    expect(deriveReleaseSlug(SHA_A)).toBe(deriveReleaseSlug(SHA_A));
  });

  test("differs when the commit SHA differs", () => {
    expect(deriveReleaseSlug(SHA_A)).not.toBe(deriveReleaseSlug(SHA_B));
  });

  test("takes the leading 16 hex of the commit SHA", () => {
    expect(deriveReleaseSlug(SHA_A)).toBe(`rel_${SHA_A.slice(0, 16)}`);
  });

  test("normalizes uppercase hex to lowercase", () => {
    expect(deriveReleaseSlug(SHA_A.toUpperCase())).toBe(
      deriveReleaseSlug(SHA_A),
    );
  });
});

test.each([
  "rel_0123456789abcdef",
  "rel_0123456789ABCDEF",
  "rel_0123456789abcde",
  "rel_0123456789abcdef0",
  "0123456789abcdef",
  "",
])("toReleaseSlug agrees with releaseSlugSchema for %j", (value) => {
  let minted: boolean;

  try {
    minted = toReleaseSlug(value) === value;
  } catch {
    minted = false;
  }

  expect(minted).toBe(releaseSlugSchema.safeParse(value).success);
});

test("toReleaseSlug rejects a malformed slug", () => {
  expect(toReleaseSlug("rel_0123456789abcdef")).toBe("rel_0123456789abcdef");
  expect(() => toReleaseSlug("rel_nothex")).toThrow(TypeError);
});
