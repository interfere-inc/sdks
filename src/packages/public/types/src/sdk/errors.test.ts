import type { ExceptionValue } from "./plugins/payload/errors";

import {
  isNonErrorException,
  toError,
  toException,
  toExceptions,
} from "./errors.ts";
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
