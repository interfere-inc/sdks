import { toException } from "./errors.ts";
import { exceptionSpanName } from "./throw-site.ts";
import { describe, expect, test } from "vitest";

type HostStack =
  | string
  | number
  | null
  | undefined
  | readonly string[]
  | { readonly stack: string };

function withStack(stack: HostStack) {
  const error = new TypeError("boom");

  Object.defineProperty(error, "stack", {
    configurable: true,
    value: stack,
    writable: true,
  });

  return error;
}

describe("exceptionSpanName throw-site parsing", () => {
  test.each([
    {
      expected: "handleClick (app.js:42)",
      format: "V8 named frame",
      stack:
        "TypeError: boom\n    at handleClick (https://example.com/assets/app.js:42:12)\n    at onClick (https://example.com/assets/app.js:10:5)",
    },
    {
      expected: "app.js:42",
      format: "V8 anonymous frame",
      stack: "Error: boom\n    at https://example.com/assets/app.js:42:12",
    },
    {
      expected: "app.js:42",
      format: "V8 module scope",
      stack: "Error: boom\n    at Object.<anonymous> (/srv/app/app.js:42:12)",
    },
    {
      expected: "load (loader.ts:7)",
      format: "V8 async frame",
      stack: "Error: boom\n    at async load (file:///srv/app/loader.ts:7:3)",
    },
    {
      expected: "Checkout (checkout.ts:3)",
      format: "V8 constructor frame",
      stack: "Error: boom\n    at new Checkout (/srv/app/checkout.ts:3:9)",
    },
    {
      expected: "submit (form.js:12)",
      format: "Firefox frame",
      stack:
        "submit@https://example.com/form.js:12:4\nonClick@https://example.com/form.js:3:1",
    },
    {
      expected: "form.js:12",
      format: "Safari anonymous frame",
      stack:
        "@https://example.com/form.js:12:4\nglobal code@https://example.com/form.js:1:1",
    },
    {
      expected: "render (widget.js:9)",
      format: "node_modules and native frames above the app frame",
      stack:
        "Error: boom\n    at Array.map (<anonymous>)\n    at dispatch (https://example.com/node_modules/react-dom/index.js:5:1)\n    at render (https://example.com/widget.js:9:2)",
    },
    {
      expected: "loadInvoice (page.tsx:14)",
      format: "webpack-internal path with a parenthesised segment",
      stack:
        "Error: boom\n    at loadInvoice (webpack-internal:///(rsc)/./app/invoices/page.tsx:14:11)",
    },
    {
      expected: "render (widget.js:9)",
      format: "eval frames skipped",
      stack:
        "Error: boom\n    at run (eval at compile (https://example.com/app.js:3:1), <anonymous>:1:2)\n    at render (https://example.com/widget.js:9:2)",
    },
    {
      expected: "app.js:42",
      format: "query string and hash stripped",
      stack: "Error: boom\n    at https://example.com/app.js?v=3#x:42:12",
    },
  ])("names the top in-app frame ($format)", ({ expected, stack }) => {
    expect(exceptionSpanName(withStack(stack))).toBe(expected);
  });

  test.each([
    undefined,
    "",
    "Error: boom",
    "Error: boom\n    at <anonymous>",
    42,
    null,
    { stack: "nope" },
    ["at x (a.js:1:1)"],
  ])("falls back to the error name when the stack is %j", (stack) => {
    expect(exceptionSpanName(withStack(stack))).toBe("TypeError");
  });
});

describe(exceptionSpanName, () => {
  test("uses the throw site when the stack has an in-app frame", () => {
    const error = new Error("boom");

    error.stack = "Error: boom\n    at handleClick (app.js:42:12)";

    expect(exceptionSpanName(error)).toBe("handleClick (app.js:42)");
  });

  test("falls back to the error name without a usable frame", () => {
    const error = new TypeError("boom");

    error.stack = "TypeError: boom";

    expect(exceptionSpanName(error)).toBe("TypeError");

    delete error.stack;

    expect(exceptionSpanName(error)).toBe("TypeError");
  });

  test("uses the structural type for non-error rejections", () => {
    expect(exceptionSpanName(toException({ code: 401 }))).toBe("PlainObject");
  });
});
