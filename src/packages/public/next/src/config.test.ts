/* oxlint-disable typescript/no-unsafe-assignment, typescript/no-unsafe-return, typescript/require-await */
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";
import { SERVICE_NAME_GLOBAL } from "@interfere/types/sdk/service-name";

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

const REMOVED_BUILD_ID_RE = /Removed: interfere\.buildId/u;

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

function stubProductionPreflightEnv(
  projectDir: string,
  credentials: "release" | "public-key-only" = "release",
) {
  vi.stubEnv(
    "INTERFERE_API_KEY",
    credentials === "release" ? "interfere_ak_test_xxx" : undefined,
  );

  vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_next");

  vi.stubEnv("INTERFERE_API_URL", "https://in.example.com");

  clearReleaseMetadataEnv();

  vi.stubEnv(
    "VERCEL_GIT_COMMIT_SHA",
    "abc0123456789def0123456789abcdef01234567",
  );

  process.chdir(projectDir);

  writeFileSync(path.join(projectDir, "instrumentation-client.ts"), "");
}

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
  test.each([
    { upload: true, configured: undefined, expected: true },
    { upload: true, configured: false, expected: true },
    { upload: false, configured: undefined, expected: undefined },
    { upload: false, configured: false, expected: false },
    { upload: false, configured: true, expected: true },
  ])(
    "enables server maps for publication: $upload/$configured",
    async ({ upload, configured, expected }) => {
      expect.hasAssertions();

      await withTempProject("next-server-maps-", async (projectDir) => {
        stubProductionPreflightEnv(projectDir);

        const output = await withInterfere(
          { sourceMaps: { upload } },
          {
            experimental:
              configured === undefined ? {} : { serverSourceMaps: configured },
          },
        )(PHASE_PRODUCTION_BUILD, { defaultConfig: {} });

        expect(output.experimental?.serverSourceMaps).toBe(expected);
      });
    },
  );

  test("injects the shared inferred name into browser and server entry points", async () => {
    expect.assertions(5);

    await withTempProject("next-service-name-", async (projectDir) => {
      process.chdir(projectDir);

      vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_next");

      writeFileSync(
        path.join(projectDir, "package.json"),
        JSON.stringify({ name: "@acme/platform" }),
      );

      writeFileSync(
        path.join(projectDir, "instrumentation-client.ts"),
        "export {};\n",
      );

      writeFileSync(
        path.join(projectDir, "instrumentation.ts"),
        "export {};\n",
      );

      const output = await withInterfere({})(PHASE_DEVELOPMENT_SERVER);

      const rule = await injectionRule(output);

      const values: { [SERVICE_NAME_GLOBAL]?: string } =
        await injectedValues(rule);

      expect(values[SERVICE_NAME_GLOBAL]).toBe("platform");

      for (const filename of [
        "instrumentation-client.ts",
        "instrumentation.ts",
      ]) {
        expect(rule.test(path.join(projectDir, filename))).toBe(true);

        const options = instrumentationOptions(output, `**/${filename}`);

        const turbopackValues: { [SERVICE_NAME_GLOBAL]?: string } = JSON.parse(
          options.serializedValues,
        );

        expect(turbopackValues[SERVICE_NAME_GLOBAL]).toBe("platform");
      }
    });
  });

  test("returns a config object without the interfere field and without exposing buildId in env", async () => {
    expect.assertions(3);

    await withTempProject("next-config-", async (projectDir) => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

      process.chdir(projectDir);

      writeFileSync(path.join(projectDir, "instrumentation-client.ts"), "");

      const output = await withInterfere({
        reactStrictMode: true,
      })(PHASE_DEVELOPMENT_SERVER);

      expect(output.reactStrictMode).toBe(true);
      expect("interfere" in output).toBe(false);

      expect(output.env?.["NEXT_PUBLIC_INTERFERE_BUILD_ID"]).toBeUndefined();
    });
  });

  test("rejects the legacy interfere.buildId knob with a clear error", async () => {
    expect.assertions(1);

    await withTempProject("next-config-", async (projectDir) => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

      vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_test_xxx");

      process.chdir(projectDir);

      writeFileSync(path.join(projectDir, "instrumentation-client.ts"), "");

      expect(() => {
        void withInterfere({ interfere: { buildId: "build_123" } as never })(
          PHASE_DEVELOPMENT_SERVER,
        );
      }).toThrow(REMOVED_BUILD_ID_RE);
    });
  });

  test("warns instead of failing when a public key is set but no instrumentation client exists", async () => {
    expect.assertions(2);

    await withTempProject("next-config-", async (projectDir) => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

      vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_next");

      process.chdir(projectDir);

      const warn = vi.spyOn(log, "warn");

      expect(() => {
        void withInterfere()(PHASE_DEVELOPMENT_SERVER);
      }).not.toThrow();

      expect(warn).toHaveBeenCalledWith(
        "Missing instrumentation client",
        expect.any(Array),
      );
    });
  });

  test("passes user env through unchanged", async () => {
    expect.assertions(1);

    await withTempProject("next-config-", async (projectDir) => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

      process.chdir(projectDir);

      writeFileSync(path.join(projectDir, "instrumentation-client.ts"), "");

      const output = await withInterfere({
        env: {
          MY_CUSTOMER_VAR: "hello",
        },
      })(PHASE_DEVELOPMENT_SERVER);

      expect(output.env?.["MY_CUSTOMER_VAR"]).toBe("hello");
    });
  });
});
