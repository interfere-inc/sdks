import { createContext, runInContext } from "node:vm";

import loader from "./value-injection-loader";
import type { ValueInjectionLoaderOptions } from "./value-injection-loader";
import { describe, expect, test } from "vitest";

function invoke(options: ValueInjectionLoaderOptions, code = "export {};") {
  return loader.call({ getOptions: () => options }, code);
}

describe("config emission", () => {
  test("emits nested JSON and special global-name assignment syntax", () => {
    const serializedValues =
      '{"__proto__":{"safe":true},"items":[null,false,42,"text",{"nested":true}],"empty":"","zero":0}';

    expect(invoke({ serializedValues })).toBe(
      'globalThis["__proto__"] = {"safe":true};\nglobalThis["items"] = [null,false,42,"text",{"nested":true}];\nglobalThis["empty"] = "";\nglobalThis["zero"] = 0;\nexport {};',
    );
  });

  test("initializes JSON globals before running the original module body", () => {
    const values = {
      items: [null, false, 42, "text", { nested: true }],
      empty: "",
      zero: 0,
      __INTERFERE_SERVER_TRACING__: "drain",
    };

    const output = invoke(
      { serializedValues: JSON.stringify(values) },
      "globalThis.observed = JSON.stringify({ items: globalThis.items, empty: globalThis.empty, zero: globalThis.zero, __INTERFERE_SERVER_TRACING__: globalThis.__INTERFERE_SERVER_TRACING__ });",
    );

    const scope = { observed: "" };

    runInContext(output, createContext(scope), { timeout: 1000 });

    expect(JSON.parse(scope.observed)).toStrictEqual(values);
  });

  test.each([
    {},
    { serializedValues: "{bad" },
    { serializedValues: "null" },
    { serializedValues: "[]" },
  ])("ignores malformed static values %s", (options) => {
    expect(invoke(options)).toBe("export {};");
  });
});
