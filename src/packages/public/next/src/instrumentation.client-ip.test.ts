import { attribute, exportedSpans } from "@interfere/test-utils/vitest/otlp";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { PLUGINS_GLOBAL } from "@interfere/types/sdk/remote-config/plugins";
import { expect, onTestFinished, test, vi } from "vitest";

import { onRequestError, register } from "./instrumentation.js";
import { runtime } from "./internal/server/runtime.js";
import { uninstallGlobalHandlers } from "./internal/server/capture/global-handlers.js";

test.each([
  [undefined, "203.0.113.7, 10.0.0.1"],
  [true, "203.0.113.7, 10.0.0.1"],
  [false, undefined],
] as const)(
  "node registration applies clientIp=%s to request errors",
  async (clientIp, expectedIp) => {
    const collector = await receiver();

    vi.stubEnv("INTERFERE_API_KEY", "interfere_secret_us_test");

    vi.stubEnv("INTERFERE_API_URL", collector.url);

    vi.stubEnv("INTERFERE_SOURCE_ID", "a".repeat(40));

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

    setInjectedConfiguration(PLUGINS_GLOBAL, { errors: true });

    onTestFinished(async () => {
      uninstallGlobalHandlers();

      await runtime.shutdown();

      await collector.close();

      vi.unstubAllEnvs();
    });

    await register({
      consoleBridge: false,
      captureUncaughtException: false,
      captureUnhandledRejection: false,
      privacy: clientIp === undefined ? {} : { clientIp },
    });

    await onRequestError(
      new Error("node request failed"),
      {
        method: "GET",
        path: "/orders",
        headers: {
          "x-forwarded-for": ["203.0.113.7", "10.0.0.1"],
          authorization: "secret-credential",
        },
      },
      { routePath: "/orders", routeType: "route", routerKind: "App Router" },
    );

    await runtime.flush();

    const spans = exportedSpans(collector.requests);

    expect(spans).toHaveLength(1);
    expect(spans[0]?.events).toHaveLength(1);

    expect(
      attribute(
        spans[0]?.events[0]?.attributes ?? [],
        "http.request.header.x-forwarded-for",
      ),
    ).toBe(expectedIp);

    expect(JSON.stringify(spans)).not.toContain("secret-credential");
  },
);
