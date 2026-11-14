import { runInNewContext } from "node:vm";
import { z } from "zod";
import {
  debugIdRuntimeSnippet,
  injectDebugIdIntoJs,
  readDebugIdFromJs,
} from "./source-maps";
import { describe, expect, test } from "vitest";

const debugRegistrySchema = z.object({
  _debugIds: z.record(z.string(), z.string()),
});

describe(debugIdRuntimeSnippet, () => {
  test("registers the stack under globalThis._debugIds", () => {
    const sandbox = {};
    const debugId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";

    runInNewContext(debugIdRuntimeSnippet(debugId), sandbox);

    const { _debugIds: registry } = debugRegistrySchema.parse(sandbox);

    expect(registry).toBeDefined();
    expect(Object.values(registry)).toContain(debugId);
  });

  test("rejects debug IDs that cannot be embedded as a JS string literal", () => {
    expect(() => debugIdRuntimeSnippet('abc";throw new Error();')).toThrow(
      "debugId must be hex and dashes only",
    );
  });
});

describe(readDebugIdFromJs, () => {
  test("reads the last spec debugId comment", () => {
    expect(
      readDebugIdFromJs(
        [
          "console.log('first');",
          "//# debugId=11111111-1111-1111-1111-111111111111",
          "console.log('second');",
          "//# debugId=22222222-2222-2222-2222-222222222222",
        ].join("\n"),
      ),
    ).toBe("22222222-2222-2222-2222-222222222222");
  });

  test("returns null when the chunk has no spec debugId comment", () => {
    expect(readDebugIdFromJs("console.log('chunk');")).toBeNull();
  });
});

describe(injectDebugIdIntoJs, () => {
  const debugId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";

  test("injects the runtime registry snippet and debugId before sourceMappingURL", () => {
    const output = injectDebugIdIntoJs(
      "console.log('chunk');\n//# sourceMappingURL=app.js.map",
      debugId,
    );

    const lines = output.split("\n");

    expect(lines.at(-1)).toBe("//# sourceMappingURL=app.js.map");
    expect(output).toContain("new e.Error");
    expect(output).toContain(`_debugIds[n]="${debugId}"`);
    expect(lines.indexOf(`//# debugId=${debugId}`)).toBe(lines.length - 2);
  });

  test("registers the injected debugId at runtime without moving sourceMappingURL", () => {
    const sandbox = {};

    const output = injectDebugIdIntoJs(
      "void 0;\n//# sourceMappingURL=app.js.map",
      debugId,
    );

    runInNewContext(output, sandbox);

    const { _debugIds: registry } = debugRegistrySchema.parse(sandbox);

    expect(output.split("\n").at(-1)).toBe("//# sourceMappingURL=app.js.map");
    expect(Object.values(registry)).toContain(debugId);
  });

  test("replaces an existing debugId comment instead of duplicating it", () => {
    const output = injectDebugIdIntoJs(
      [
        "console.log('chunk');",
        "//# debugId=11111111-1111-1111-1111-111111111111",
        "//# sourceMappingURL=app.js.map",
      ].join("\n"),
      debugId,
    );

    expect(output.match(/\/\/# debugId=/gu)).toHaveLength(1);
    expect(readDebugIdFromJs(output)).toBe(debugId);
    expect(output).not.toContain("_debugIds[n]=");
  });

  test("appends the runtime registry when sourceMappingURL is absent", () => {
    const output = injectDebugIdIntoJs("console.log('chunk');", debugId);

    expect(output).toContain("console.log('chunk');");
    expect(output).toContain(`_debugIds[n]="${debugId}"`);
    expect(output.endsWith(`//# debugId=${debugId}`)).toBe(true);
  });

  test("re-syncs its own runtime carrier when the debugId is overwritten", () => {
    const idB = "11111111-2222-3333-4444-555555555555";

    const once = injectDebugIdIntoJs(
      "void 0;\n//# sourceMappingURL=app.js.map",
      debugId,
    );

    const twice = injectDebugIdIntoJs(once, idB);

    expect(twice.match(/\/\/# debugId=/gu)).toHaveLength(1);
    expect(readDebugIdFromJs(twice)).toBe(idB);
    expect(twice).toContain(`_debugIds[n]="${idB}"`);
    expect(twice).not.toContain(debugId);
  });

  test("registers the overwritten id, not the original, at runtime", () => {
    const idB = "11111111-2222-3333-4444-555555555555";
    const sandbox = {};

    const once = injectDebugIdIntoJs(
      "void 0;\n//# sourceMappingURL=app.js.map",
      debugId,
    );

    const twice = injectDebugIdIntoJs(once, idB);

    runInNewContext(twice, sandbox);

    const { _debugIds: registry } = debugRegistrySchema.parse(sandbox);

    expect(Object.values(registry)).toContain(idB);
    expect(Object.values(registry)).not.toContain(debugId);
  });
});
