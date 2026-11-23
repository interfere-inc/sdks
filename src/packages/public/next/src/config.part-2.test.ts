/* oxlint-disable typescript/no-unsafe-assignment, typescript/no-unsafe-return */
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";
import { deriveReleaseSlug } from "@interfere/types/releases/slug";

import { existsSync, writeFileSync } from "node:fs";
import path from "node:path";
import type { NextConfig } from "next";
import type { RuleSetRule } from "webpack";
import { PHASE_DEVELOPMENT_SERVER } from "next/constants";

import { HttpResponse, http } from "msw";
import { releaseNetwork, withTempProject } from "../test/fixtures/build.js";
import { webpackContext } from "../test/fixtures/webpack.js";
import { withInterfere } from "./config.js";
import {
  afterEach,
  assert,
  beforeEach,
  describe,
  expect,
  test,
  vi,
} from "vitest";

function clearReleaseMetadataEnv() {
  for (const key of releaseSourceIdEnvKeys) {
    vi.stubEnv(key, undefined);
  }
}

async function injectionRule(input: NextConfig | Promise<NextConfig>) {
  const output = await input;

  const rules: RuleSetRule[] = [];

  output.webpack?.({ module: { rules } }, await webpackContext(process.cwd()));

  const [rule] = rules;

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Webpack conditions also accept strings and regular expressions; this test invokes the generated predicate variant.
  if (!rule || typeof rule.test !== "function" || !Array.isArray(rule.use)) {
    throw new Error(
      "Expected an instrumentation rule with a path condition and loader array",
    );
  }

  const [loader] = rule.use;

  if (
    loader === null || // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Webpack loader entries may be strings or descriptors; instrumentation requires a descriptor with options.
    typeof loader !== "object" ||
    !loader.loader
  ) {
    throw new Error("Expected instrumentation loader options");
  }

  assert.exists(loader.options, "Expected instrumentation loader options");

  // oxlint-disable-next-line anti-slop/no-runtime-typeof
  if (typeof loader.options !== "object") {
    throw new TypeError("Expected instrumentation loader options");
  }

  const { serializedValues } = loader.options;

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Webpack's open option dictionary does not guarantee the serialized JSON string produced by this loader.
  if (typeof serializedValues !== "string") {
    throw new TypeError("Expected serialized instrumentation values");
  }

  return { test: rule.test, loader: loader.loader, serializedValues };
}

async function injectedValues(rule: Awaited<ReturnType<typeof injectionRule>>) {
  return new Response(rule.serializedValues).json();
}

beforeEach(() => {
  releaseNetwork.use(
    http.get("*/v3/releases/config", () => HttpResponse.error()),
    http.post("*/v3/releases", () => HttpResponse.error()),
    http.get("*/v3/config", () => HttpResponse.json({})),
  );

  vi.stubEnv("OTEL_SERVICE_NAME", undefined);

  vi.stubEnv("VERCEL_ENV", undefined);

  vi.stubEnv("VERCEL_TARGET_ENV", undefined);

  vi.stubEnv("CLOUDFLARE_ENV", undefined);

  vi.stubEnv("INTERFERE_ENVIRONMENT", undefined);
});

afterEach(() => {
  vi.restoreAllMocks();

  vi.unstubAllEnvs();
});

function instrumentationOptions(output: NextConfig, resourceGlob: string) {
  const rule = output.turbopack?.rules?.[resourceGlob];

  if (!rule || Array.isArray(rule)) {
    throw new Error("Expected a Turbopack instrumentation rule");
  }

  const loader = rule.loaders?.[0];

  assert.exists(loader, "Expected a configured instrumentation loader");

  // oxlint-disable-next-line anti-slop/no-runtime-typeof
  if (typeof loader === "string") {
    throw new TypeError("Expected a configured instrumentation loader");
  }

  const serializedValues = loader.options?.["serializedValues"];

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Turbopack options accept JSON values; the generated serialized values must specifically be strings.
  if (typeof serializedValues !== "string") {
    throw new TypeError("Expected serialized instrumentation loader options");
  }

  return { serializedValues };
}

describe(withInterfere, () => {
  test("injects __INTERFERE_RELEASE_SLUG__ deterministically derived from the commit SHA", async () => {
    expect.assertions(5);

    await withTempProject("next-config-", async (projectDir) => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

      vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_next");

      vi.stubEnv("INTERFERE_ENVIRONMENT", undefined);

      vi.stubEnv("VERCEL_ENV", undefined);

      clearReleaseMetadataEnv();

      vi.stubEnv(
        "VERCEL_GIT_COMMIT_SHA",
        "abc0123456789def0123456789abcdef01234567",
      );

      const filePath = path.join(projectDir, "instrumentation-client.ts");

      writeFileSync(filePath, "export {};\n");

      process.chdir(projectDir);

      const output = await withInterfere({
        webpack: (webpackConfig) => webpackConfig,
      })(PHASE_DEVELOPMENT_SERVER);

      const injectedRule = await injectionRule(output);

      expect(injectedRule.test(filePath)).toBe(true);

      expect(injectedRule.loader).toContain("value-injection-loader");

      expect(existsSync(injectedRule.loader)).toBe(true);

      const expectedSlug = deriveReleaseSlug(
        "abc0123456789def0123456789abcdef01234567",
      );

      await expect(injectedValues(injectedRule)).resolves.toStrictEqual({
        __INTERFERE_BUILD__: true,
        __INTERFERE_PUBLIC_KEY__: "interfere_pk_next",
        __INTERFERE_RELEASE_SLUG__: expectedSlug,
      });

      const turbopackOptions = instrumentationOptions(
        output,
        "**/instrumentation-client.ts",
      );

      expect(JSON.parse(turbopackOptions.serializedValues)).toStrictEqual({
        __INTERFERE_BUILD__: true,
        __INTERFERE_PUBLIC_KEY__: "interfere_pk_next",
        __INTERFERE_RELEASE_SLUG__: expectedSlug,
      });
    });
  });

  test("bakes __INTERFERE_BUILD__ and applies value injection even without a public key", async () => {
    expect.assertions(4);

    await withTempProject("next-config-no-key-", async (projectDir) => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

      vi.stubEnv("INTERFERE_PUBLIC_KEY", undefined);

      clearReleaseMetadataEnv();

      vi.stubEnv(
        "VERCEL_GIT_COMMIT_SHA",
        "abc0123456789def0123456789abcdef01234567",
      );

      process.chdir(projectDir);

      const filePath = path.join(projectDir, "instrumentation-client.ts");

      writeFileSync(filePath, "export {};\n");

      const injectedRule = await injectionRule(
        withInterfere()(PHASE_DEVELOPMENT_SERVER),
      );

      expect(injectedRule).toBeDefined();
      expect(injectedRule.test(filePath)).toBe(true);

      const values = await injectedValues(injectedRule);

      expect(values).toMatchObject({ __INTERFERE_BUILD__: true });
      expect(values).not.toHaveProperty("__INTERFERE_PUBLIC_KEY__");
    });
  });

  test("preserves existing webpack and turbopack config when nothing applies", async () => {
    expect.assertions(2);

    await withTempProject("next-config-", async (projectDir) => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

      clearReleaseMetadataEnv();

      process.chdir(projectDir);

      const existingWebpack: NonNullable<NextConfig["webpack"]> = (
        webpackConfig,
      ) => webpackConfig;

      const output = await withInterfere({
        turbopack: { rules: { "**/*.ts": { loaders: [] } } },
        webpack: existingWebpack,
      })(PHASE_DEVELOPMENT_SERVER);

      expect(output.webpack).toBe(existingWebpack);

      expect(output.turbopack).toStrictEqual({
        debugIds: true,
        rules: { "**/*.ts": { loaders: [] } },
      });
    });
  });

  test("ignores the removed NEXT_PUBLIC_INTERFERE_FORCE_ENABLE environment variable", async () => {
    expect.assertions(1);

    await withTempProject("next-config-", async (projectDir) => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

      clearReleaseMetadataEnv();

      vi.stubEnv(
        "VERCEL_GIT_COMMIT_SHA",
        "abc0123456789def0123456789abcdef01234567",
      );

      vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

      process.chdir(projectDir);

      writeFileSync(
        path.join(projectDir, "instrumentation-client.ts"),
        "export {};\n",
      );

      const injectedRule = await injectionRule(
        withInterfere()(PHASE_DEVELOPMENT_SERVER),
      );

      await expect(injectedValues(injectedRule)).resolves.not.toHaveProperty(
        "__INTERFERE_FORCE_ENABLE__",
      );
    });
  });
});
