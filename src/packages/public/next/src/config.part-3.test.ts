/* oxlint-disable typescript/no-unsafe-return */
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";

import { writeFileSync } from "node:fs";
import path from "node:path";
import type { NextConfig } from "next";
import type { RuleSetRule } from "webpack";
import {
  PHASE_DEVELOPMENT_SERVER,
  PHASE_PRODUCTION_BUILD,
} from "next/constants";

import { HttpResponse, http } from "msw";
import { releaseNetwork, withTempProject } from "../test/fixtures/build.js";
import { webpackContext } from "../test/fixtures/webpack.js";
import { withInterfere } from "./config.js";
import { log } from "./internal/logger.js";
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

describe(withInterfere, () => {
  test("injects __INTERFERE_ENVIRONMENT__ from INTERFERE_ENVIRONMENT, preferring it over VERCEL_ENV", async () => {
    expect.assertions(1);

    await withTempProject("next-config-", async (projectDir) => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

      clearReleaseMetadataEnv();

      vi.stubEnv(
        "VERCEL_GIT_COMMIT_SHA",
        "abc0123456789def0123456789abcdef01234567",
      );

      vi.stubEnv("INTERFERE_ENVIRONMENT", "staging");

      vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_next");

      vi.stubEnv("VERCEL_ENV", "preview");

      process.chdir(projectDir);

      writeFileSync(
        path.join(projectDir, "instrumentation-client.ts"),
        "export {};\n",
      );

      const injectedRule = await injectionRule(
        withInterfere()(PHASE_PRODUCTION_BUILD),
      );

      await expect(injectedValues(injectedRule)).resolves.toMatchObject({
        __INTERFERE_ENVIRONMENT__: "staging",
      });
    });
  });

  test("falls back to VERCEL_ENV for __INTERFERE_ENVIRONMENT__, and builds without one when neither is set", async () => {
    expect.assertions(2);

    await withTempProject("next-config-", async (projectDir) => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

      clearReleaseMetadataEnv();

      vi.stubEnv(
        "VERCEL_GIT_COMMIT_SHA",
        "abc0123456789def0123456789abcdef01234567",
      );

      vi.stubEnv("INTERFERE_ENVIRONMENT", undefined);

      vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_next");

      vi.stubEnv("VERCEL_ENV", "preview");

      process.chdir(projectDir);

      writeFileSync(
        path.join(projectDir, "instrumentation-client.ts"),
        "export {};\n",
      );

      const injectedRule = await injectionRule(
        withInterfere()(PHASE_PRODUCTION_BUILD),
      );

      await expect(injectedValues(injectedRule)).resolves.toMatchObject({
        __INTERFERE_ENVIRONMENT__: "preview",
      });

      vi.stubEnv("VERCEL_ENV", undefined);

      await expect(
        withInterfere()(PHASE_PRODUCTION_BUILD),
      ).resolves.toBeDefined();
    });
  });

  test("warns without failing when a production build has no browser credential", async () => {
    expect.assertions(1);

    await withTempProject("next-config-", async (projectDir) => {
      vi.stubEnv("INTERFERE_PUBLIC_KEY", undefined);

      vi.stubEnv("INTERFERE_API_KEY", undefined);

      process.chdir(projectDir);

      const warn = vi.spyOn(log, "warn");

      await (
        await withInterfere()(PHASE_PRODUCTION_BUILD)
      ).compiler?.runAfterProductionCompile?.({ distDir: ".next", projectDir });

      expect(warn).toHaveBeenCalledWith(
        "No browser credential",
        expect.any(Array),
      );
    });
  });

  test("omits __INTERFERE_FORCE_ENABLE__ when NEXT_PUBLIC_INTERFERE_FORCE_ENABLE is not set", async () => {
    expect.assertions(1);

    await withTempProject("next-config-", async (projectDir) => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

      clearReleaseMetadataEnv();

      vi.stubEnv(
        "VERCEL_GIT_COMMIT_SHA",
        "def0123456789abc0123456789abcdef89abcdef",
      );

      vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", undefined);

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
