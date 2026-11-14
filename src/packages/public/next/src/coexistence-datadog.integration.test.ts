import { afterAll, beforeAll, describe, expect, test } from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { exceptions } from "@interfere/test-utils/vitest/otlp";

let collector: Awaited<ReturnType<typeof receiver>>;
let app: Awaited<ReturnType<typeof application>>;
const cleanups: (() => Promise<void>)[] = [];

function baseEnv() {
  return {
    INTERFERE_PUBLIC_KEY: "interfere_public_us_acceptance",
    INTERFERE_API_KEY: "interfere_secret_us_acceptance",
    INTERFERE_API_URL: collector.url,
    INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
    INTERFERE_ENVIRONMENT: "preview",
    DD_TRACE_ENABLED: "false",
  };
}

beforeAll(async () => {
  collector = await receiver();

  cleanups.push(async () => collector.close());

  app = await application(`${import.meta.dirname}/..`, [
    "@interfere/next",
    "dd-trace",
    "next",
    "react",
    "react-dom",
    "typescript",
    "@types/node",
    "@types/react",
    "@types/react-dom",
  ]);

  cleanups.push(async () => app.close());

  await app.copyFixture("acceptance");

  await app.copyFixture("coexistence-datadog");

  await app.buildNext(baseEnv(), "turbopack");
}, 240_000);

afterAll(async () => {
  for (const cleanup of cleanups.toReversed()) {
    // oxlint-disable-next-line eslint/no-await-in-loop -- Resources close in reverse order of creation.
    await cleanup();
  }
});

describe.each([
  {
    otel: "true",
    warning: "[interfere] Adopted the OpenTelemetry tracer provider",
  },
  {
    otel: "false",
    warning: "[interfere] dd-trace is also tracing this process",
  },
])("real dd-trace with DD_TRACE_OTEL_ENABLED=$otel", ({ otel, warning }) => {
  let agent: Awaited<ReturnType<typeof receiver>>;
  let outbound: Awaited<ReturnType<typeof receiver>>;
  let host: Awaited<ReturnType<typeof app.start>>;

  function interfereWarnings() {
    return host
      .output()
      .split("\n")
      .filter((line) => line.includes("[interfere]"));
  }

  function datadogTraces(text: string) {
    return agent.requests.filter(
      (request) =>
        /^\/v0\.[45]\/traces/u.test(request.url) && request.body.includes(text),
    );
  }

  beforeAll(async () => {
    agent = await receiver();

    outbound = await receiver();

    collector.reset();

    host = await app.start("serve.ts", {
      ...baseEnv(),
      DD_TRACE_ENABLED: "true",
      DD_TRACE_OTEL_ENABLED: otel,
      DD_TRACE_AGENT_URL: agent.url,
      DD_SERVICE: "coexistence",
      DD_TRACE_FLUSH_INTERVAL: "100",
      DD_TRACE_STARTUP_LOGS: "false",
      DD_INSTRUMENTATION_TELEMETRY_ENABLED: "false",
      DD_REMOTE_CONFIGURATION_ENABLED: "false",
      DD_RUNTIME_METRICS_ENABLED: "false",
      DD_PROFILING_ENABLED: "false",
      DD_CRASHTRACKING_ENABLED: "false",
      OUTBOUND_URL: outbound.url,
    });
  }, 60_000);

  afterAll(async () => {
    await host.close();

    await outbound.close();

    await agent.close();
  });

  test("routes stay healthy and Interfere and Datadog each capture the route error once", async () => {
    expect.hasAssertions();

    for (const path of ["/", "/api/healthz"]) {
      // oxlint-disable-next-line eslint/no-await-in-loop -- Exercise each route before the intentional failure.
      const response = await fetch(host.url + path);

      expect(response.status).toBe(200);

      // oxlint-disable-next-line eslint/no-await-in-loop -- Consume each response before continuing.
      await response.text();
    }

    const failure = await fetch(`${host.url}/api/failure`);

    expect(failure.status).toBe(500);

    await failure.text();

    await expect
      .poll(() => datadogTraces("next-route-error").length, {
        timeout: 15_000,
      })
      .toBeGreaterThan(0);

    await expect
      .poll(() => exceptions(collector.requests, "next-route-error").length, {
        timeout: 15_000,
      })
      .toBeGreaterThan(0);

    const healthz = await fetch(`${host.url}/api/healthz`);

    expect(healthz.status).toBe(200);

    await healthz.text();

    expect(exceptions(collector.requests, "next-route-error")).toHaveLength(1);

    expect(interfereWarnings()).toStrictEqual([
      expect.stringContaining(warning),
    ]);
  });

  test("an outbound fetch carries exactly one traceparent", async () => {
    expect.hasAssertions();

    for (const attempt of [1, 2]) {
      // oxlint-disable-next-line eslint/no-await-in-loop -- Each outbound call is observed before the next.
      const response = await fetch(`${host.url}/api/outbound`);

      expect(response.status).toBe(200);

      // oxlint-disable-next-line eslint/no-await-in-loop -- Consume each response before continuing.
      await expect(response.json()).resolves.toStrictEqual({ status: 200 });

      expect(
        outbound.requests.filter((request) => request.url === "/outbound"),
      ).toHaveLength(attempt);
    }

    for (const request of outbound.requests) {
      expect(request.headers.get("traceparent")).toMatch(
        /^00-[\da-f]{32}-[\da-f]{16}-0[01]$/u,
      );
    }

    expect(interfereWarnings()).toStrictEqual([
      expect.stringContaining(warning),
    ]);
  });
});
