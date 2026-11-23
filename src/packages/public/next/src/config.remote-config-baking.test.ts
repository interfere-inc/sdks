/* oxlint-disable typescript/require-await */
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";

import { writeFileSync } from "node:fs";
import path from "node:path";
import type { NextConfig } from "next";
import {
  PHASE_DEVELOPMENT_SERVER,
  PHASE_PRODUCTION_BUILD,
} from "next/constants";

import { HttpResponse, http } from "msw";
import { releaseNetwork, withTempProject } from "../test/fixtures/build.js";
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

const configFn = (phase: string) => ({
  reactStrictMode: phase === PHASE_PRODUCTION_BUILD,
});

function clearReleaseMetadataEnv() {
  for (const key of releaseSourceIdEnvKeys) {
    vi.stubEnv(key, undefined);
  }
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
