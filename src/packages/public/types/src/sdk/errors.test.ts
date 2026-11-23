import type { ExceptionValue } from "./plugins/payload/errors";

import {
  isNonErrorException,
  parseReactComponentStack,
  shouldDropBrowserExtensionNoise,
  shouldDropUnresolvableStack,
  toError,
  toException,
  toExceptions,
} from "./errors.ts";
import { EXCEPTION_LIMITS } from "./errors/truncate.ts";
import { describe, expect, test } from "vitest";

function asErrorVariant(e: ExceptionValue | undefined) {
  if (!e || e.kind === "non-error") {
    throw new Error("expected error variant exception");
  }

  return e;
}

function createErrorChain(length: number) {
  let tail: Error | undefined;

  for (let i = length - 1; i >= 0; i -= 1) {
    const current = new Error(`error-${i}`);

    current.name = `Error${i}`;

    if (tail) {
      current.cause = tail;
    }

    tail = current;
  }

  return tail ?? new Error("root");
}

describe("sdk errors utilities", () => {
  test("toError returns Error values unchanged", () => {
    const error = new Error("boom");

    expect(toError(error)).toBe(error);
  });

  test("toError wraps non-Error inputs", () => {
    expect(toError("boom")).toBeInstanceOf(Error);
    expect(toError(123).message).toBe("123");
  });

  test("toError unwraps a nested Error on plain objects", () => {
    const inner = new Error("inner");

    expect(toError({ error: inner })).toBe(inner);
  });

  test("toError prefers `cause` over other Error-valued keys", () => {
    const cause = new Error("cause");
    const other = new Error("other");

    expect(toError({ cause, other })).toBe(cause);
  });

  test("toError preserves the original shape as JSON when no nested Error exists", () => {
    const result = toError({ code: 42, error: "boom" });

    expect(result).toBeInstanceOf(Error);
    expect(result.name).toBe("NonError");

    expect(JSON.parse(result.message)).toStrictEqual({
      code: 42,
      error: "boom",
    });
  });

  test("toError handles null without throwing", () => {
    expect(toError(null)).toBeInstanceOf(Error);
  });

  test("toException returns Error instances unchanged", () => {
    const error = new Error("boom");

    expect(toException(error)).toBe(error);
  });

  test("toException maps strings to Error (string IS Error-shaped)", () => {
    const result = toException("boom");

    expect(result).toBeInstanceOf(Error);
    expect(result).toHaveProperty("message", "boom");
  });

  test("toException unwraps a nested Error on plain objects", () => {
    const inner = new Error("inner");

    expect(toException({ error: inner })).toBe(inner);
  });

  test("toException prefers `cause` over other Error-valued keys", () => {
    const cause = new Error("cause");
    const other = new Error("other");

    expect(toException({ cause, other })).toBe(cause);
  });

  test("toException preserves plain-object rejection shapes", () => {
    const result = toException({ body: "forbidden", code: 401 });

    expect(isNonErrorException(result)).toBe(true);

    if (!isNonErrorException(result)) {
      throw new Error("unreachable");
    }

    expect(result.kind).toBe("non-error");
    expect(result.type).toBe("PlainObject");
    expect(result.value).toBe("Non-Error captured with keys: body, code");
    expect(result.serialized).toStrictEqual({ body: "forbidden", code: 401 });
  });

  test.each([
    {
      name: "element target",
      target: Object.assign(new EventTarget(), {
        id: "gtm",
        src: "https://cdn.app.test/gtm.js",
        tagName: "SCRIPT",
      }),
      value: "Event 'error' on script#gtm https://cdn.app.test/gtm.js",
      serialized: {
        target: {
          id: "gtm",
          tag: "script",
          url: "https://cdn.app.test/gtm.js",
        },
        type: "error",
      },
    },
    {
      name: "non-element target",
      target: new EventTarget(),
      value: "Event 'error' on EventTarget",
      serialized: { target: { kind: "EventTarget" }, type: "error" },
    },
    {
      name: "stylesheet target without id",
      target: Object.assign(new EventTarget(), {
        href: "https://cdn.app.test/a.css",
        id: "",
        tagName: "LINK",
      }),
      value: "Event 'error' on link https://cdn.app.test/a.css",
      serialized: {
        target: { tag: "link", url: "https://cdn.app.test/a.css" },
        type: "error",
      },
    },
    {
      name: "image target preferring currentSrc",
      target: Object.assign(new EventTarget(), {
        currentSrc: "https://cdn.app.test/2x.png",
        src: "https://cdn.app.test/1x.png",
        tagName: "IMG",
      }),
      value: "Event 'error' on img https://cdn.app.test/2x.png",
      serialized: {
        target: { tag: "img", url: "https://cdn.app.test/2x.png" },
        type: "error",
      },
    },
    {
      name: "element target without a URL",
      target: Object.assign(new EventTarget(), { tagName: "DIV" }),
      value: "Event 'error' on div",
      serialized: { target: { tag: "div" }, type: "error" },
    },
    {
      name: "undispatched event",
      target: null,
      value: "Event 'error'",
      serialized: { target: null, type: "error" },
    },
  ])(
    "toException describes a DOM Event with its type and $name",
    ({ target, value, serialized }) => {
      const event = new Event("error");

      target?.dispatchEvent(event);

      expect(toException(event)).toStrictEqual({
        kind: "non-error",
        serialized,
        type: "Event",
        value,
      });
    },
  );

  test("toException names Event subclasses and ignores events without a type", () => {
    expect(toException(new CustomEvent("load-failed"))).toMatchObject({
      type: "CustomEvent",
      value: "CustomEvent 'load-failed'",
    });

    expect(toException(new Event(""))).toMatchObject({
      type: "Event",
      value: "Non-Error captured: empty object",
    });
  });

  test("toException preserves number rejections", () => {
    const result = toException(42);

    expect(isNonErrorException(result)).toBe(true);

    if (!isNonErrorException(result)) {
      throw new Error("unreachable");
    }

    expect(result.type).toBe("number");
    expect(result.value).toBe("Non-Error captured: 42");
    expect(result.serialized).toBe(42);
  });

  test("toException tags arrays distinctly from plain objects", () => {
    const result = toException([1, 2, 3]);

    expect(isNonErrorException(result)).toBe(true);

    if (!isNonErrorException(result)) {
      throw new Error("unreachable");
    }

    expect(result.type).toBe("Array");
    expect(result.serialized).toStrictEqual([1, 2, 3]);
  });

  test("toException handles null and undefined", () => {
    const fromNull = toException(null);

    expect(isNonErrorException(fromNull)).toBe(true);

    if (!isNonErrorException(fromNull)) {
      throw new Error("unreachable");
    }

    expect(fromNull.type).toBe("null");
    expect(fromNull.serialized).toBeNull();

    const fromUndef = toException(undefined);

    expect(isNonErrorException(fromUndef)).toBe(true);

    if (!isNonErrorException(fromUndef)) {
      throw new Error("unreachable");
    }

    expect(fromUndef.type).toBe("undefined");
  });

  test("toException survives circular references via depth-limited walk", () => {
    const a = { name: "a" };

    Object.assign(a, { self: a });

    const result = toException(a);

    expect(isNonErrorException(result)).toBe(true);

    if (!isNonErrorException(result)) {
      throw new Error("unreachable");
    }

    expect(result.serialized).toStrictEqual({ name: "a", self: "[circular]" });
  });

  test("serializes non-JSON leaves without dropping undefined object fields", () => {
    const callback = () => "callback";

    const result = toException({
      callback,
      count: 12n,
      marker: Symbol("marker"),
      missing: undefined,
      values: [false, null, 0],
    });

    expect(result).toMatchObject({ kind: "non-error" });

    if (!isNonErrorException(result)) {
      throw new Error("expected a non-error exception");
    }

    expect(result.serialized).toStrictEqual({
      callback: callback.toString(),
      count: "12",
      marker: "Symbol(marker)",
      missing: undefined,
      values: [false, null, 0],
    });
  });

  test("toExceptions returns a single non-error entry for NonErrorException", () => {
    const captured = toException({ code: 500 });

    const exceptions = toExceptions(captured, {
      handled: false,
      type: "test",
    });

    expect(exceptions).toHaveLength(1);
    expect(exceptions[0]?.kind).toBe("non-error");

    expect(exceptions[0]?.mechanism).toStrictEqual({
      handled: false,
      type: "test",
    });
  });

  test("toExceptions ships raw stack strings, not parsed frames", () => {
    const err = new Error("boom");

    err.stack = "Error: boom\n    at fn (file.js:1:1)";

    const [exception] = toExceptions(err, { handled: false, type: "test" });
    const errVariant = asErrorVariant(exception);

    expect(errVariant.stack).toBe(err.stack);
    expect(errVariant).not.toHaveProperty("frames");
  });

  test("toExceptions emits empty stack string when Error.stack is missing", () => {
    class StacklessError extends Error {
      override name = "StacklessError";
      override stack = "";
    }

    const err = new StacklessError("boom");
    const [exception] = toExceptions(err, { handled: false, type: "test" });

    expect(asErrorVariant(exception).stack).toBe("");
  });

  test("toExceptions applies mechanism only to top-level exception", () => {
    const root = createErrorChain(3);
    const exceptions = toExceptions(root, { handled: false, type: "test" });

    expect(exceptions).toHaveLength(3);

    expect(exceptions[0]?.mechanism).toStrictEqual({
      handled: false,
      type: "test",
    });

    expect(exceptions[1]?.mechanism).toBeUndefined();
    expect(exceptions[2]?.mechanism).toBeUndefined();
  });

  test("toExceptions caps cause chain depth at five", () => {
    const root = createErrorChain(8);
    const exceptions = toExceptions(root, { handled: false, type: "test" });

    expect(exceptions).toHaveLength(5);
    expect(exceptions[0]?.type).toBe("Error0");
    expect(exceptions[4]?.type).toBe("Error4");
  });
});

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

describe("exception payload bounds", () => {
  const huge = "x".repeat(5 * 1024 * 1024);

  test("toExceptions bounds every entry's value and stack", () => {
    expect.hasAssertions();

    const error = new Error(huge, { cause: new Error(huge) });

    const exceptions = toExceptions(error, { handled: true, type: "test" });

    expect(exceptions).toHaveLength(2);

    for (const entry of exceptions) {
      const { stack, value } = asErrorVariant(entry);

      expect(value.length).toBeLessThanOrEqual(EXCEPTION_LIMITS.messageLength);
      expect(stack.length).toBeLessThanOrEqual(EXCEPTION_LIMITS.stackLength);
    }
  });

  test("toException bounds serialized payloads by breadth and length, not only depth", () => {
    const wide = Object.fromEntries(
      Array.from({ length: 10_000 }, (_, index) => [
        `key${index}`,
        huge.slice(0, 500),
      ]),
    );

    const result = toException(wide);

    if (!isNonErrorException(result)) {
      throw new Error("expected a non-error exception");
    }

    expect(JSON.stringify(result.serialized).length).toBeLessThanOrEqual(
      EXCEPTION_LIMITS.serializedLength,
    );

    expect(result.value.length).toBeLessThanOrEqual(
      EXCEPTION_LIMITS.messageLength,
    );

    expect(result.serialized).toHaveProperty("key0");
  });

  test("toException bounds a long thrown string leaf", () => {
    const result = toException({ body: huge });

    if (!isNonErrorException(result)) {
      throw new Error("expected a non-error exception");
    }

    expect(JSON.stringify(result.serialized).length).toBeLessThanOrEqual(
      EXCEPTION_LIMITS.serializedLength,
    );
  });
});
