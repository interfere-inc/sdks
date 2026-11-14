import { hasServerDigest } from "./server-digest.js";
import { describe, expect, test } from "vitest";

describe(hasServerDigest, () => {
  test.each([null, undefined, "error", 42, false, {}, new Error("plain")])(
    "does not identify a server error from %s",
    (value) => {
      expect(hasServerDigest(value)).toBe(false);
    },
  );

  test.each([undefined, null, 42, false, {}, []])(
    "rejects a non-string digest %s",
    (digest) => {
      expect(hasServerDigest({ digest })).toBe(false);
    },
  );

  test.each(["", "server-error-123"])("accepts digest %s", (digest) => {
    const error = Object.assign(new Error("server"), { digest });

    expect(hasServerDigest(error)).toBe(true);
  });

  test("accepts an inherited digest", () => {
    // oxlint-disable-next-line unicorn/custom-error-definition -- This fixture tests inherited digest lookup on native Error instances.
    class ServerError extends Error {
      // oxlint-disable-next-line typescript/class-literal-property-style, eslint/class-methods-use-this
      get digest() {
        return "inherited";
      }
    }

    expect(hasServerDigest(new ServerError("server"))).toBe(true);
  });
});
