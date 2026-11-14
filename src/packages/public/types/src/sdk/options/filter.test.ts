import { describe, expect, test } from "vitest";

import {
  matchesDeniedUrl,
  matchesIgnoredError,
  topFrameUrl,
} from "./filter.ts";

describe(topFrameUrl, () => {
  test.each([
    {
      engine: "V8 named frame",
      stack:
        "Error: mail a@b.example failed\n    at send (https://cdn.example/widget.js:10:5)\n    at https://app.example/main.js:1:1",
      url: "https://cdn.example/widget.js",
    },
    {
      engine: "V8 anonymous frame",
      stack: "TypeError: x\n    at https://app.example/main.js:1:1",
      url: "https://app.example/main.js",
    },
    {
      engine: "V8 native frame first",
      stack:
        "Error: x\n    at Array.map (<anonymous>)\n    at run (https://app.example/a.js:2:3)",
      url: "https://app.example/a.js",
    },
    {
      engine: "Firefox",
      stack:
        "send@https://cdn.example/widget.js:10:5\n@https://app.example/main.js:1:1\n",
      url: "https://cdn.example/widget.js",
    },
    {
      engine: "Safari global code",
      stack: "global code@https://app.example/main.js:4:9",
      url: "https://app.example/main.js",
    },
    { engine: "no frames", stack: "Error: x", url: null },
  ])("$engine", ({ stack, url }) => {
    expect(topFrameUrl(stack)).toBe(url);
  });
});

describe(matchesIgnoredError, () => {
  test("matches substrings, type-prefixed messages and global regexes repeatedly", () => {
    const global = /loop/gu;

    expect(
      matchesIgnoredError(new Error("ResizeObserver loop"), [global]),
    ).toBe(true);

    expect(
      matchesIgnoredError(new Error("ResizeObserver loop"), [global]),
    ).toBe(true);

    expect(matchesIgnoredError(new TypeError("x"), ["TypeError: x"])).toBe(
      true,
    );

    expect(
      matchesIgnoredError(
        { kind: "non-error", serialized: 1, type: "number", value: "1" },
        ["number: 1"],
      ),
    ).toBe(true);

    expect(matchesIgnoredError(new Error("kept"), ["other"])).toBe(false);
  });
});

describe(matchesDeniedUrl, () => {
  test("never matches an unknown URL", () => {
    expect(matchesDeniedUrl(null, [/.*/u])).toBe(false);
  });
});
