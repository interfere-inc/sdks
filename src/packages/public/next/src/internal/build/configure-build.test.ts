/* oxlint-disable typescript/no-unsafe-assignment, typescript/no-unsafe-return */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { normalize, resolve } from "node:path";
import type { RuleSetRule } from "webpack";

import { webpackContext } from "../../../test/fixtures/webpack.js";

import { configureBuild } from "./configure-build.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

const VALUES = { __INTERFERE_RELEASE_SLUG__: "rel_0123456789abcdef" };

let projectDir: string;

let clientPath: string;

let serverPath: string;

beforeEach(() => {
  projectDir = mkdtempSync(resolve(tmpdir(), "next-configure-build-"));

  clientPath = resolve(projectDir, "instrumentation-client.ts");

  serverPath = resolve(projectDir, "instrumentation.ts");

  writeFileSync(clientPath, "export {};\n");

  writeFileSync(serverPath, "export {};\n");
});

async function webpackRules(opts: {
  values?: { readonly __INTERFERE_RELEASE_SLUG__: string | null };
}) {
  const rules: RuleSetRule[] = [];

  const { webpack } = configureBuild({
    existingTurbopack: undefined,
    existingWebpack: undefined,
    nextMajor: 16,
    projectDir,
    values: opts.values ?? VALUES,
  });

  webpack?.({ module: { rules } }, await webpackContext(projectDir));

  return rules;
}

function turbopackRules() {
  const { turbopack } = configureBuild({
    existingTurbopack: undefined,
    existingWebpack: undefined,
    nextMajor: 16,
    projectDir,
    values: VALUES,
  });

  return turbopack.rules ?? {};
}

afterEach(() => {
  vi.clearAllMocks();

  rmSync(projectDir, { recursive: true, force: true });
});

describe("configureBuild value injection targets", () => {
  test.each([
    {},
    { module: null },
    { module: {} },
    { module: { rules: null } },
  ])("initializes absent webpack rules in %j", async (config) => {
    const { webpack } = configureBuild({
      existingTurbopack: undefined,
      existingWebpack: () => config,
      nextMajor: 16,
      projectDir,
      values: VALUES,
    });

    expect(webpack?.({}, await webpackContext(projectDir))).toBe(config);

    expect(config).toMatchObject({ module: { rules: [expect.any(Object)] } });
  });

  test.each([
    null,
    [],
    42,
    { module: [] },
    { module: true },
    { module: { rules: {} } },
  ])("rejects invalid webpack hook output %j", async (config) => {
    const { webpack } = configureBuild({
      existingTurbopack: undefined,
      existingWebpack: () => config,
      nextMajor: 16,
      projectDir,
      values: VALUES,
    });

    const context = await webpackContext(projectDir);

    expect(() => webpack?.({}, context)).toThrow(TypeError);
  });

  test.each([
    ["existing-loader"],
    [{ condition: "browser" as const, loaders: ["existing-loader"] }],
  ])("preserves existing array-form Turbopack rules %j", (...rules) => {
    const original = structuredClone(rules);

    const { turbopack } = configureBuild({
      existingTurbopack: { rules: { "**/instrumentation-client.ts": rules } },
      existingWebpack: undefined,
      nextMajor: 16,
      projectDir,
      values: VALUES,
    });

    expect(turbopack.rules?.["**/instrumentation-client.ts"]).toStrictEqual([
      ...original,
      {
        loaders: [
          expect.objectContaining({
            loader: expect.stringContaining("value-injection-loader"),
          }),
        ],
      },
    ]);

    expect(rules).toStrictEqual(original);
  });

  test("webpack rule matches both the client and the server instrumentation entry", async () => {
    const [rule] = await webpackRules({});

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Webpack permits string and RegExp conditions; this test must call the generated predicate variant.
    if (typeof rule?.test !== "function") {
      throw new TypeError("Expected an instrumentation path condition");
    }

    expect(rule.test(normalize(clientPath))).toBe(true);
    expect(rule.test(normalize(serverPath))).toBe(true);

    expect(rule.test(normalize(resolve(projectDir, "app/page.tsx")))).toBe(
      false,
    );
  });

  test("turbopack adds an injection rule for both the client and the server entry", () => {
    const rules = turbopackRules();

    expect(rules).toHaveProperty("**/instrumentation-client.ts");
    expect(rules).toHaveProperty("**/instrumentation.ts");
  });
});

describe("configureBuild loader gate is independent of the client instrumentation file", () => {
  test("injects the server instrumentation entry when only a server file exists", async () => {
    rmSync(clientPath);

    const { clientInstrumentationApplies, webpack } = configureBuild({
      existingTurbopack: undefined,
      existingWebpack: undefined,
      nextMajor: 16,
      projectDir,
      values: VALUES,
    });

    expect(clientInstrumentationApplies).toBe(false);
    expect(webpack).toBeTypeOf("function");

    const rules = turbopackRules();

    expect(rules).toHaveProperty("**/instrumentation.ts");
    expect(rules).not.toHaveProperty("**/instrumentation-client.ts");

    expect(rules["**/instrumentation.ts"]).toMatchObject({
      loaders: [{ options: { serializedValues: JSON.stringify(VALUES) } }],
    });

    const [webpackRule] = await webpackRules({});

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Webpack permits string and RegExp conditions; this test must call the generated predicate variant.
    if (typeof webpackRule?.test !== "function") {
      throw new TypeError("Expected an instrumentation path condition");
    }

    expect(webpackRule.test(normalize(serverPath))).toBe(true);
    expect(webpackRule.test(normalize(clientPath))).toBe(false);
  });

  test("performs no injection when no instrumentation file exists at all", () => {
    rmSync(clientPath);

    rmSync(serverPath);

    const { clientInstrumentationApplies, webpack, turbopack } = configureBuild(
      {
        existingTurbopack: undefined,
        existingWebpack: undefined,
        nextMajor: 16,
        projectDir,
        values: VALUES,
      },
    );

    expect(clientInstrumentationApplies).toBe(false);
    expect(webpack).toBeUndefined();
    expect(turbopack).not.toHaveProperty("rules");
  });
});

describe("configureBuild turbopack debugIds gating", () => {
  test.each([
    { name: "nextMajor 16", nextMajor: 16 },
    { name: "unknown nextMajor", nextMajor: null },
  ])("includes turbopack debugIds: true for $name", ({ nextMajor }) => {
    const { turbopack } = configureBuild({
      existingTurbopack: undefined,
      existingWebpack: undefined,
      nextMajor,
      projectDir,
      values: VALUES,
    });

    expect(turbopack).toHaveProperty("debugIds", true);
  });

  test("omits turbopack debugIds on Next 15 while still emitting injection rules", () => {
    const { turbopack } = configureBuild({
      existingTurbopack: undefined,
      existingWebpack: undefined,
      nextMajor: 15,
      projectDir,
      values: VALUES,
    });

    expect(turbopack).not.toHaveProperty("debugIds");

    const { rules } = turbopack;

    expect(rules).toHaveProperty("**/instrumentation-client.ts");
    expect(rules).toHaveProperty("**/instrumentation.ts");
  });

  test("preserves the user's explicit turbopack debugIds: false on Next 15", () => {
    const { turbopack } = configureBuild({
      existingTurbopack: { debugIds: false },
      existingWebpack: undefined,
      nextMajor: 15,
      projectDir,
      values: VALUES,
    });

    expect(turbopack).toHaveProperty("debugIds", false);
  });
});
