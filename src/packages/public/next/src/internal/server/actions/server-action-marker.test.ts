import { markServerAction, readServerAction } from "./server-action-marker.js";
import { describe, expect, test } from "vitest";

describe("server action marker", () => {
  test("attaches a readable marker without exposing it during enumeration", () => {
    const error = new Error("action failed");
    // oxlint-disable-next-line anti-slop/no-shape-in-symbol-names -- The marker protocol names the privacy-safe argument structure argsShape.
    const info = { name: "checkout", argsShape: "[object]" };

    markServerAction(error, info);

    expect(readServerAction(error)).toStrictEqual(info);
    expect(Object.keys(error)).toStrictEqual([]);
  });

  test("allows a later action to replace the marker", () => {
    const error = new Error("action failed");

    markServerAction(error, { name: "first" });

    markServerAction(error, { name: "second" });

    expect(readServerAction(error)).toStrictEqual({ name: "second" });
  });

  test.each([null, undefined, "failure", 42, false])(
    "ignores thrown non-object value %s",
    (error) => {
      expect(() => markServerAction(error, { name: "checkout" })).not.toThrow();
      expect(readServerAction(error)).toBeUndefined();
    },
  );

  test.each([
    undefined,
    null,
    "marker",
    {},
    { name: 42 },
    // oxlint-disable-next-line anti-slop/no-shape-in-symbol-names -- This malformed value tests the existing argsShape protocol field.
    { name: "ok", argsShape: 42 },
  ])("rejects malformed marker %s", (marker) => {
    expect(
      readServerAction({ __interfere_server_action__: marker }),
    ).toBeUndefined();
  });

  // oxlint-disable-next-line anti-slop/no-shape-in-symbol-names -- The existing argsShape protocol distinguishes an undefined field from a string description.
  test.each([{}, { argsShape: undefined }])(
    "omits an absent or undefined argument description %s",
    (optional) => {
      expect(
        readServerAction({
          __interfere_server_action__: { name: "checkout", ...optional },
        }),
      ).toStrictEqual({ name: "checkout" });
    },
  );

  test("returns undefined when no marker property exists", () => {
    expect(readServerAction(new Error("ordinary"))).toBeUndefined();
  });

  test("reads an inherited marker", () => {
    // oxlint-disable-next-line unicorn/custom-error-definition -- This fixture tests inherited marker lookup on native Error instances.
    class ActionError extends Error {
      // oxlint-disable-next-line eslint/class-methods-use-this
      get __interfere_server_action__() {
        return { name: "inherited" };
      }
    }

    expect(readServerAction(new ActionError("action"))).toStrictEqual({
      name: "inherited",
    });
  });
});
