import {
  parseReactComponentStack,
  shouldDropBrowserExtensionNoise,
  shouldDropUnresolvableStack,
  toExceptions,
} from "./errors.ts";
import { describe, expect, test } from "vitest";

describe("shouldDropBrowserExtensionNoise (regex over raw stack)", () => {
  test("drops extension-only stacks (every frame is an extension URL)", () => {
    const stack = [
      "TypeError: Cannot redefine property: ethereum",
      "    at defineProp (chrome-extension://nkbihfbeogaeaoehlefnkodbefgpgknn/inpage.js:1:2345)",
      "    at init (chrome-extension://nkbihfbeogaeaoehlefnkodbefgpgknn/inpage.js:2:100)",
    ].join("\n");

    expect(shouldDropBrowserExtensionNoise([stack])).toBe(true);
  });

  test("drops mixed stacks with at least one extension frame (intercept pattern)", () => {
    const stack = [
      "TypeError: Failed to fetch",
      "    at n (chrome-extension://hoklmmgfnpapgjgcpechhaamimifchmp/frame_ant.js:1:1)",
      "    at l.send (https://oklch.fyi/_next/static/chunks/chunk.js:10:5)",
    ].join("\n");

    expect(shouldDropBrowserExtensionNoise([stack])).toBe(true);
  });

  test("keeps an app error whose extension frames sit below the top app frame", () => {
    const stack = [
      "Error: Minified React error #418",
      "    at rX (https://oklch.fyi/_next/static/chunks/main.js:1:1)",
      "    at w (chrome-extension://egjidjbpglichdcondbcbdnbeeppgdph/inpage.js:9:1)",
    ].join("\n");

    expect(shouldDropBrowserExtensionNoise([stack])).toBe(false);
  });

  test("keeps a Gecko app stack that calls through an extension frame", () => {
    const stack = [
      "render@https://shop.example/assets/app.js:12:5",
      "wrapped@moz-extension://abc-def/content.js:3:9",
      "@https://shop.example/assets/app.js:1:1",
    ].join("\n");

    expect(shouldDropBrowserExtensionNoise([stack])).toBe(false);
  });

  test("drops when the top located frame is an extension after native frames", () => {
    const stack = [
      "TypeError: Cannot read properties of undefined",
      "    at new Promise (<anonymous>)",
      "    at Array.forEach (native)",
      "    at inject (chrome-extension://abc/content.js:4:2)",
      "    at main (https://shop.example/app.js:1:1)",
    ].join("\n");

    expect(shouldDropBrowserExtensionNoise([stack])).toBe(true);
  });

  test("ignores extension URLs in the message line", () => {
    const stack = [
      "Error: failed to load chrome-extension://abc/inpage.js",
      "    at load (https://shop.example/app.js:1:1)",
    ].join("\n");

    expect(shouldDropBrowserExtensionNoise([stack])).toBe(false);
  });

  test("drops when an extension frame appears only in the cause-chain stack", () => {
    const outer = [
      "Error: outer",
      "    at fn (https://customer.com/_next/static/chunks/app.js:10:1)",
    ].join("\n");

    const inner = [
      "TypeError: inner",
      "    at handler (chrome-extension://abc/inpage.js:1:1)",
    ].join("\n");

    expect(shouldDropBrowserExtensionNoise([outer, inner])).toBe(true);
  });

  test("drops Safari ITP webkit-masked-url:// frames", () => {
    const stack =
      "TypeError: x\n    at fn (webkit-masked-url://hidden/script.js:1:1)";

    expect(shouldDropBrowserExtensionNoise([stack])).toBe(true);
  });

  test("drops moz-extension and ms-browser-extension schemes", () => {
    expect(
      shouldDropBrowserExtensionNoise([
        "Error\n    at f (moz-extension://abc-def/bg.js:1:1)",
      ]),
    ).toBe(true);

    expect(
      shouldDropBrowserExtensionNoise([
        "Error\n    at f (ms-browser-extension://xyz/x.js:1:1)",
      ]),
    ).toBe(true);
  });

  test("returns false for https-only stacks", () => {
    const stack = "Error: x\n    at fn (https://cdn.example.com/app.js:1:1)";

    expect(shouldDropBrowserExtensionNoise([stack])).toBe(false);
  });

  test("returns false for empty stacks array", () => {
    expect(shouldDropBrowserExtensionNoise([])).toBe(false);
  });

  test("returns false for empty-string stacks", () => {
    expect(shouldDropBrowserExtensionNoise([""])).toBe(false);
  });
});

describe("shouldDropUnresolvableStack (regex over raw stack)", () => {
  test("drops <anonymous>-only inline-script stacks", () => {
    const stack = [
      "ReferenceError: TrackerStorageType is not defined",
      "    at <anonymous>:14:42",
    ].join("\n");

    expect(shouldDropUnresolvableStack([stack])).toBe(true);
  });

  test("drops eval-only stacks (no source location anchor)", () => {
    const stack = ["SyntaxError: boom", "    at eval (<anonymous>)"].join("\n");

    expect(shouldDropUnresolvableStack([stack])).toBe(true);
  });

  test("drops when no stack in the chain has a source-location anchor", () => {
    expect(
      shouldDropUnresolvableStack([
        "Error: a\n    at <anonymous>",
        "Error: b\n    at <anonymous>",
      ]),
    ).toBe(true);
  });

  test("keeps stacks with at least one https:// frame", () => {
    const stack =
      "Error: real\n    at fn (https://customer.com/_next/static/chunks/app.js:42:7)";

    expect(shouldDropUnresolvableStack([stack])).toBe(false);
  });

  test("keeps stacks with a webpack-internal:// frame", () => {
    const stack =
      "Error: r\n    at f (webpack-internal:///./src/app/page.tsx:5:5)";

    expect(shouldDropUnresolvableStack([stack])).toBe(false);
  });

  test("keeps stacks with a V8 `at fn (path:line:col)` frame even without a URL scheme", () => {
    const stack = [
      "Error: native",
      "    at handler (/var/task/.next/server/chunks/page.js:42:7)",
    ].join("\n");

    expect(shouldDropUnresolvableStack([stack])).toBe(false);
  });

  test("keeps stacks whose anchor is only in the cause chain", () => {
    const outer = "Error: outer\n    at <anonymous>";

    const inner =
      "TypeError: inner\n    at fn (https://customer.com/app.js:1:1)";

    expect(shouldDropUnresolvableStack([outer, inner])).toBe(false);
  });

  test("returns false for empty stacks array", () => {
    expect(shouldDropUnresolvableStack([])).toBe(false);
  });
});

describe(parseReactComponentStack, () => {
  test("parses the React 16–18 legacy `in ComponentName` format", () => {
    const stack = [
      "    in BuyPageComponent",
      "    in Suspense",
      "    in Router",
      "    in App",
    ].join("\n");

    expect(parseReactComponentStack(stack)).toStrictEqual([
      { fn: "BuyPageComponent", id: "react-0" },
      { fn: "Suspense", id: "react-1" },
      { fn: "Router", id: "react-2" },
      { fn: "App", id: "react-3" },
    ]);
  });

  test("ignores trailing `(created by ...)` annotations", () => {
    const stack = [
      "    in BuyPageComponent (created by BuyPage)",
      "    in BuyPage (created by App)",
    ].join("\n");

    expect(parseReactComponentStack(stack)).toStrictEqual([
      { fn: "BuyPageComponent", id: "react-0" },
      { fn: "BuyPage", id: "react-1" },
    ]);
  });

  test("parses React 19 owner stack positions", () => {
    const stack = [
      "    at BuyPageComponent (components.client.tsx:766:13)",
      "    at Suspense",
      "    at Router (router.tsx:42:5)",
    ].join("\n");

    expect(parseReactComponentStack(stack)).toStrictEqual([
      {
        column: 13,
        file: "components.client.tsx",
        fn: "BuyPageComponent",
        id: "react-0",
        line: 766,
      },
      { fn: "Suspense", id: "react-1" },
      {
        column: 5,
        file: "router.tsx",
        fn: "Router",
        id: "react-2",
        line: 42,
      },
    ]);
  });

  test("keeps the browser URL of React 19 production frames in V8 form", () => {
    const stack = [
      "    at d (https://assets.interfere.com/app-assets/route-session-B1x.js:1:2345)",
      "    at https://assets.interfere.com/assets/Match-BUdeJLcb.js:1:3575",
      "    at Suspense (<anonymous>)",
      "    at main (<anonymous>)",
    ].join("\n");

    expect(parseReactComponentStack(stack)).toStrictEqual([
      {
        column: 2345,
        file: "https://assets.interfere.com/app-assets/route-session-B1x.js",
        fn: "d",
        id: "react-0",
        line: 1,
      },
      {
        column: 3575,
        file: "https://assets.interfere.com/assets/Match-BUdeJLcb.js",
        id: "react-1",
        line: 1,
      },
      { fn: "Suspense", id: "react-2" },
      { fn: "main", id: "react-3" },
    ]);
  });

  test("keeps the browser URL of React 19 production frames in Gecko and JavaScriptCore form", () => {
    const stack = [
      "d@https://assets.interfere.com/app-assets/route-session-B1x.js:1:2345",
      "@https://assets.interfere.com/assets/Match-BUdeJLcb.js:1:3575",
      "Suspense@unknown:0:0",
    ].join("\n");

    expect(parseReactComponentStack(stack)).toStrictEqual([
      {
        column: 2345,
        file: "https://assets.interfere.com/app-assets/route-session-B1x.js",
        fn: "d",
        id: "react-0",
        line: 1,
      },
      {
        column: 3575,
        file: "https://assets.interfere.com/assets/Match-BUdeJLcb.js",
        id: "react-1",
        line: 1,
      },
      { fn: "Suspense", id: "react-2" },
    ]);
  });

  test("returns an empty array for empty / blank input", () => {
    expect(parseReactComponentStack("")).toStrictEqual([]);
    expect(parseReactComponentStack("\n\n   \n")).toStrictEqual([]);
  });

  test("skips lines that match neither format", () => {
    const stack = [
      "garbage prefix",
      "    in GoodComponent",
      "also garbage",
    ].join("\n");

    expect(parseReactComponentStack(stack)).toStrictEqual([
      { fn: "GoodComponent", id: "react-0" },
    ]);
  });
});

describe("toExceptions with AggregateError", () => {
  const mechanism = { handled: false, type: "test" };

  test("lists each aggregated error after the AggregateError, then its cause", () => {
    const aggregate = new AggregateError(
      [new TypeError("first"), new RangeError("second")],
      "all failed",
      { cause: new Error("root cause") },
    );

    expect(
      toExceptions(aggregate, mechanism).map(({ type, value }) => [
        type,
        value,
      ]),
    ).toStrictEqual([
      ["AggregateError", "all failed"],
      ["TypeError", "first"],
      ["RangeError", "second"],
      ["Error", "root cause"],
    ]);
  });

  test("walks aggregated errors' own causes and stops at five entries", () => {
    const nested = new Error("nested", { cause: new Error("nested cause") });

    const aggregate = new AggregateError(
      [nested, new Error("b"), new Error("c"), new Error("d")],
      "many",
    );

    expect(
      toExceptions(aggregate, mechanism).map(({ value }) => value),
    ).toStrictEqual(["many", "nested", "nested cause", "b", "c"]);
  });

  test("does not revisit an AggregateError that contains itself", () => {
    const errors: Error[] = [];
    const aggregate = new AggregateError(errors, "loop");

    errors.push(aggregate, new Error("leaf"));

    Object.defineProperty(aggregate, "errors", { value: errors });

    expect(
      toExceptions(aggregate, mechanism).map(({ value }) => value),
    ).toStrictEqual(["loop", "leaf"]);
  });
});
