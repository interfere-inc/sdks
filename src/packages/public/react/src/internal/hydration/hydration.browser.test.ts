import { isHydrationError } from "./hydration.js";
import { describe, expect, test } from "vitest";

const HYDRATION_MESSAGES = [
  "Hydration failed because the initial UI does not match what was rendered on the server.",
  "Hydration failed because the server rendered HTML didn't match the client. As a result this tree will be regenerated on the client.",
  "Text content does not match server-rendered HTML.",
  "There was an error while hydrating. Because the error happened outside of a Suspense boundary, the entire root will switch to client rendering.",
  "A tree hydrated but some attributes of the server rendered HTML didn't match the client properties.",
  "Minified React error #418; visit https://react.dev/errors/418 for the full message.",
  "Minified React error #423;",
  "Minified React error #425;",
  "Something failed, visit https://react.dev/errors/425?args[]=foo for details",
];

const NON_HYDRATION_MESSAGES = [
  "Cannot read properties of undefined (reading 'foo')",
  "Network request failed",
  "Minified React error #310; visit https://react.dev/errors/310",
  "ChunkLoadError: Loading chunk 5 failed",
  "Maximum update depth exceeded",
];

describe(isHydrationError, () => {
  test.each(HYDRATION_MESSAGES)("flags hydration error: %s", (message) => {
    expect(isHydrationError(new Error(message))).toBe(true);
  });

  test.each(NON_HYDRATION_MESSAGES)("ignores non-hydration: %s", (message) => {
    expect(isHydrationError(new Error(message))).toBe(false);
  });

  test("returns false for non-Error values", () => {
    expect(isHydrationError("Hydration failed because")).toBe(false);
    expect(isHydrationError(null)).toBe(false);
    expect(isHydrationError(undefined)).toBe(false);
  });
});
