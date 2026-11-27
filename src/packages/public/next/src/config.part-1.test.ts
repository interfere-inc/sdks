import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";

import { writeFileSync } from "node:fs";
import path from "node:path";
import type { NextConfig } from "next";
import {
  PHASE_DEVELOPMENT_SERVER,
  PHASE_PRODUCTION_BUILD,
} from "next/constants";

import { HttpResponse, http } from "msw";
import {
  RELEASE_RESPONSE,
  releaseNetwork,
  sdkConfigRequests,
  withTempProject,
} from "../test/fixtures/build.js";
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

function registerReleaseWithToken() {
  releaseNetwork.use(
    http.post("*/v3/releases", () =>
      HttpResponse.json({ ...RELEASE_RESPONSE, token: "irt_next_build" }),
    ),
  );
}

function stubProductionPreflightEnv(projectDir: string) {
  registerReleaseWithToken();

  vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

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
  test("adds W3C client trace metadata when the browser SDK is configured", async () => {
    expect.assertions(1);

    await withTempProject("next-config-", async (projectDir) => {
      vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test_xxx");

      process.chdir(projectDir);

      writeFileSync(path.join(projectDir, "instrumentation-client.ts"), "");

      registerReleaseWithToken();

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

  test("a production build adds the Interfere client trace metadata keys whatever the surface stores", async () => {
    expect.assertions(2);

    await withTempProject("next-config-owner-", async (projectDir) => {
      stubProductionPreflightEnv(projectDir);

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
  });

  test("bakes the server globals from one surface snapshot", async () => {
    expect.assertions(4);

    await withTempProject("next-config-snapshot-", async (projectDir) => {
      stubProductionPreflightEnv(projectDir);

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
});
