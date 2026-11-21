/* oxlint-disable typescript/no-unsafe-assignment, typescript/no-unsafe-return, typescript/require-await */
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";
import { deriveReleaseSlug } from "@interfere/types/releases/slug";
import { SERVICE_NAME_GLOBAL } from "@interfere/types/sdk/service-name";

import { existsSync, writeFileSync } from "node:fs";
import path from "node:path";
import type { NextConfig } from "next";
import type { RuleSetRule } from "webpack";
import {
  PHASE_DEVELOPMENT_SERVER,
  PHASE_PRODUCTION_BUILD,
} from "next/constants";

import { HttpResponse, http } from "msw";
import {
  releaseNetwork,
  sdkConfigRequests,
  withTempProject,
} from "../test/fixtures/build.js";
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

const configFn = (phase: string) => ({
  reactStrictMode: phase === PHASE_PRODUCTION_BUILD,
});

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

  test("adds W3C client trace metadata when the browser SDK is configured", async () => {
    expect.assertions(1);

    await withTempProject("next-config-", async (projectDir) => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

      vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_next");

      process.chdir(projectDir);

      writeFileSync(path.join(projectDir, "instrumentation-client.ts"), "");

      const output = await withInterfere({
        experimental: {
          clientTraceMetadata: ["custom-trace-key"],
        },
      })(PHASE_DEVELOPMENT_SERVER);

      expect(output.experimental?.clientTraceMetadata).toStrictEqual([
        "custom-trace-key",
        "traceparent",
        "tracestate",
        "baggage",
      ]);
    });
  });

  const WITH_INTERFERE_KEYS = [
    "custom-trace-key",
    "traceparent",
    "tracestate",
    "baggage",
  ];

  test.each(["release", "public-key-only"] as const)(
    "a production build with %s credentials adds the Interfere client trace metadata keys whatever the surface stores",
    async (credentials) => {
      expect.assertions(2);

      await withTempProject("next-config-owner-", async (projectDir) => {
        stubProductionPreflightEnv(projectDir, credentials);

        releaseNetwork.use(
          http.get("*/v3/config", () =>
            HttpResponse.json({ tracingOwnership: "application" }),
          ),
        );

        const output = await withInterfere({
          experimental: { clientTraceMetadata: ["custom-trace-key"] },
        })(PHASE_PRODUCTION_BUILD);

        expect(output.experimental?.clientTraceMetadata).toStrictEqual(
          WITH_INTERFERE_KEYS,
        );

        expect(sdkConfigRequests).toHaveLength(1);
      });
    },
  );

  test("bakes the server globals from one surface snapshot", async () => {
    expect.assertions(4);

    await withTempProject("next-config-snapshot-", async (projectDir) => {
      stubProductionPreflightEnv(projectDir, "public-key-only");

      writeFileSync(
        path.join(projectDir, "instrumentation.ts"),
        "export {};\n",
      );

      const responses = [{ serverTracing: "drain" }, { serverTracing: "full" }];

      releaseNetwork.use(
        http.get("*/v3/config", () => HttpResponse.json(responses.shift())),
      );

      const output = await withInterfere()(PHASE_PRODUCTION_BUILD);

      expect(output.experimental?.clientTraceMetadata).toStrictEqual([
        "traceparent",
        "tracestate",
        "baggage",
      ]);

      expect(
        JSON.parse(
          instrumentationOptions(output, "**/instrumentation.ts")
            .serializedValues,
        ),
      ).toMatchObject({ __INTERFERE_SERVER_TRACING__: "drain" });

      expect(responses).toHaveLength(1);
      expect(sdkConfigRequests).toHaveLength(1);
    });
  });

  test("keeps client trace metadata on when the surface configuration cannot be fetched", async () => {
    expect.assertions(1);

    await withTempProject("next-config-owner-outage-", async (projectDir) => {
      stubProductionPreflightEnv(projectDir);

      releaseNetwork.use(http.get("*/v3/config", () => HttpResponse.error()));

      const output = await withInterfere()(PHASE_PRODUCTION_BUILD);

      expect(output.experimental?.clientTraceMetadata).toStrictEqual([
        "traceparent",
        "tracestate",
        "baggage",
      ]);
    });
  });

  test("does not add client trace metadata when the browser SDK is not configured", async () => {
    expect.assertions(1);

    await withTempProject("next-config-", async (projectDir) => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

      process.chdir(projectDir);

      const output = await withInterfere()(PHASE_DEVELOPMENT_SERVER);

      expect(output.experimental?.clientTraceMetadata).toBeUndefined();
    });
  });

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

function stubProductionRemoteConfigEnv(projectDir: string) {
  vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

  vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_next");

  vi.stubEnv("INTERFERE_API_URL", "https://in.example.com");

  vi.stubEnv("INTERFERE_ENVIRONMENT", "production");

  clearReleaseMetadataEnv();

  process.chdir(projectDir);

  writeFileSync(
    path.join(projectDir, "instrumentation-client.ts"),
    "export {};\n",
  );

  writeFileSync(path.join(projectDir, "instrumentation.ts"), "export {};\n");
}

describe("remote-config baking", () => {
  test("bakes the surface snapshot and __INTERFERE_ENVIRONMENT__ into the server instrumentation rule at a production build", async () => {
    expect.assertions(1);

    await withTempProject("next-config-eng2888-prod-", async (projectDir) => {
      stubProductionRemoteConfigEnv(projectDir);

      releaseNetwork.use(
        http.get("*/v3/config", () =>
          HttpResponse.json({
            backendHosts: ["api.customer.test"],
            logSourcing: "drain",
            platform: "vercel",
            plugins: { errors: false },
            serverTracing: "drain",
          }),
        ),
      );

      const options = instrumentationOptions(
        await withInterfere()(PHASE_PRODUCTION_BUILD),
        "**/instrumentation.ts",
      );

      expect(JSON.parse(options.serializedValues)).toMatchObject({
        __INTERFERE_BACKEND_HOSTS__: ["api.customer.test"],
        __INTERFERE_ENVIRONMENT__: "production",
        __INTERFERE_LOG_SOURCING__: "drain",
        __INTERFERE_PLATFORM__: "vercel",
        __INTERFERE_PLUGINS__: { errors: false },
        __INTERFERE_SERVER_TRACING__: "drain",
      });
    });
  });

  test("omits the surface snapshot and __INTERFERE_ENVIRONMENT__ from the server instrumentation rule at the dev server", async () => {
    expect.assertions(2);

    await withTempProject("next-config-eng2888-dev-", async (projectDir) => {
      stubProductionRemoteConfigEnv(projectDir);

      const options = instrumentationOptions(
        await withInterfere()(PHASE_DEVELOPMENT_SERVER),
        "**/instrumentation.ts",
      );

      expect(JSON.parse(options.serializedValues)).not.toHaveProperty(
        "__INTERFERE_SERVER_TRACING__",
      );

      expect(JSON.parse(options.serializedValues)).not.toHaveProperty(
        "__INTERFERE_ENVIRONMENT__",
      );
    });
  });
});

function stubProdRemoteConfigEnv(projectDir: string) {
  vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

  vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_next");

  vi.stubEnv("INTERFERE_API_URL", "https://in.example.com");

  vi.stubEnv("INTERFERE_ENVIRONMENT", "production");

  clearReleaseMetadataEnv();

  process.chdir(projectDir);

  writeFileSync(
    path.join(projectDir, "instrumentation-client.ts"),
    "export {};\n",
  );

  writeFileSync(path.join(projectDir, "instrumentation.ts"), "export {};\n");
}

describe("withInterfere function-form config", () => {
  test("calls a sync function config and composes its result with interfere's layer", async () => {
    expect.assertions(2);

    await withTempProject("next-config-fn-sync-", async (projectDir) => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

      clearReleaseMetadataEnv();

      process.chdir(projectDir);

      const resolved = await withInterfere((_phase) => ({
        reactStrictMode: true,
      }))(PHASE_DEVELOPMENT_SERVER);

      expect(resolved.reactStrictMode).toBe(true);

      expect(resolved.compiler?.runAfterProductionCompile).toBeTypeOf(
        "function",
      );
    });
  });

  test("passes the resolved phase to the inner function and gates baking by phase", async () => {
    expect.assertions(4);

    await withTempProject("next-config-fn-prod-", async (projectDir) => {
      stubProdRemoteConfigEnv(projectDir);

      const resolved = await withInterfere(configFn)(PHASE_PRODUCTION_BUILD);

      const options = instrumentationOptions(resolved, "**/instrumentation.ts");

      expect(resolved.reactStrictMode).toBe(true);

      expect(JSON.parse(options.serializedValues)).toMatchObject({
        __INTERFERE_ENVIRONMENT__: "production",
      });
    });

    await withTempProject("next-config-fn-dev-", async (projectDir) => {
      stubProdRemoteConfigEnv(projectDir);

      const resolved = await withInterfere(configFn)(PHASE_DEVELOPMENT_SERVER);

      const options = instrumentationOptions(resolved, "**/instrumentation.ts");

      expect(resolved.reactStrictMode).toBe(false);

      expect(JSON.parse(options.serializedValues)).not.toHaveProperty(
        "__INTERFERE_ENVIRONMENT__",
      );
    });
  });

  test("composes the (settings, fn) overload: fn config survives and settings stay out of the Next config", async () => {
    expect.assertions(3);

    await withTempProject("next-config-fn-settings-", async (projectDir) => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

      clearReleaseMetadataEnv();

      process.chdir(projectDir);

      const resolved = await withInterfere(
        { sourceMaps: { purge: false } },
        (_phase) => ({ reactStrictMode: true }),
      )(PHASE_DEVELOPMENT_SERVER);

      expect(resolved.reactStrictMode).toBe(true);
      expect("sourceMaps" in resolved).toBe(false);

      expect(resolved.compiler?.runAfterProductionCompile).toBeTypeOf(
        "function",
      );
    });
  });

  test("gracefully awaits an async config function and composes interfere's layer on top", async () => {
    expect.assertions(2);

    await withTempProject("next-config-fn-async-", async (projectDir) => {
      stubProdRemoteConfigEnv(projectDir);

      const resolved = await withInterfere(async (_phase) => ({
        reactStrictMode: true,
      }))(PHASE_PRODUCTION_BUILD);

      expect(resolved.reactStrictMode).toBe(true);

      const options = instrumentationOptions(resolved, "**/instrumentation.ts");

      expect(JSON.parse(options.serializedValues)).toMatchObject({
        __INTERFERE_ENVIRONMENT__: "production",
      });
    });
  });
});
