import { describe, expect, test } from "vitest";

import { isTransportFailureMessage } from "./network.ts";

describe(isTransportFailureMessage, () => {
  test.each([
    ["Chromium fetch", "Failed to fetch"],
    ["Chromium fetch with the host", "Failed to fetch (api.example.com)"],
    ["WebKit fetch", "Load failed"],
    ["WebKit fetch with the host", "Load failed (api.example.com)"],
    ["Gecko fetch", "NetworkError when attempting to fetch resource."],
    ["undici fetch", "fetch failed"],
    ["React Native fetch", "Network request failed"],
    ["WebKit lost connection", "The network connection was lost."],
    ["WebKit offline", "The Internet connection appears to be offline."],
    [
      "Zero's relayed zero-cache fetch failure",
      "Fetch from API server threw error: fetch failed",
    ],
    ["a library that prefixes the error name", "TypeError: Failed to fetch"],
  ])("matches %s", (_, message) => {
    expect(isTransportFailureMessage(message)).toBe(true);
  });

  test.each([
    ["an application error", "Cannot read properties of undefined"],
    ["an HTTP status failure", "HTTP 503 GET /api/items"],
    ["a dynamic import failure", "Failed to fetch dynamically imported module"],
    [
      "a dynamic import failure with a URL",
      "Failed to fetch dynamically imported module: https://app.example.com/chunk.js",
    ],
    ["a message that only mentions fetch", "fetch failed to parse the body"],
    ["an empty message", ""],
    ["no message", null],
  ])("does not match %s", (_, message) => {
    expect(isTransportFailureMessage(message)).toBe(false);
  });
});
