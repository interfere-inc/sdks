import assert from "node:assert/strict";
import { HttpResponse, http } from "msw";
import { delivered, network, requests } from "../test/edge-network.fixture";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";
import { LOCAL_DEV_RELEASE_SLUG } from "@interfere/types/releases/slug";
import { RELEASE_SLUG_GLOBAL } from "@interfere/types/sdk/injected-globals";
import { SERVER_TRACING_GLOBAL } from "@interfere/types/sdk/server-tracing";
import { SERVICE_NAME_GLOBAL } from "@interfere/types/sdk/service-name";
import { trace } from "@opentelemetry/api";
import { BasicTracerProvider } from "@opentelemetry/sdk-trace-base";
import type { MockInstance } from "vitest";
import { PRODUCER_VERSION } from "./internal/version.js";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

let registration: MockInstance<typeof trace.setGlobalTracerProvider>;

beforeEach(() => {
  vi.resetModules();

  trace.disable();

  registration = vi.spyOn(trace, "setGlobalTracerProvider");

  vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_test");

  vi.stubEnv("INTERFERE_API_URL", "https://edge.invalid");

  vi.stubEnv("INTERFERE_ENVIRONMENT", "preview");

  vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "");

  setInjectedConfiguration(SERVER_TRACING_GLOBAL, undefined);

  setInjectedConfiguration(RELEASE_SLUG_GLOBAL, undefined);

  setInjectedConfiguration(SERVICE_NAME_GLOBAL, undefined);

  vi.stubEnv("OTEL_SERVICE_NAME", undefined);

  for (const key of releaseSourceIdEnvKeys) {
    vi.stubEnv(key, undefined);
  }

  vi.spyOn(console, "warn");

  vi.spyOn(console, "info");

  network.use(
    http.post(
      "https://edge.invalid/drain/v1/traces",
      () => new HttpResponse(null, { status: 204 }),
    ),
  );
});

afterEach(async () => {
  try {
    const providers = registration.mock.calls.flatMap(([provider]) =>
      provider instanceof BasicTracerProvider ? [provider] : [],
    );

    await Promise.all(providers.map(async (provider) => provider.shutdown()));

    for (const request of requests) {
      assert.equal(request.method, "POST");

      assert.equal(request.url, "https://edge.invalid/drain/v1/traces");

      assert.equal(
        request.headers.get("x-interfere-public-key"),
        "interfere_pk_test",
      );
    }
  } finally {
    trace.disable();

    vi.unstubAllGlobals();

    vi.unstubAllEnvs();

    vi.restoreAllMocks();
  }
});

test.each([
  {
    force: "",
    expectedRelease: undefined,
    expectedWarnings: [[expect.stringContaining("no release.slug")]],
    serviceName: undefined,
    buildName: undefined,
    environmentName: undefined,
    expectedName: `unknown_service:${process.argv0}`,
  },
  {
    force: "1",
    expectedRelease: LOCAL_DEV_RELEASE_SLUG,
    expectedWarnings: [],
    serviceName: undefined,
    buildName: undefined,
    environmentName: undefined,
    expectedName: `unknown_service:${process.argv0}`,
  },
  {
    force: "1",
    expectedRelease: LOCAL_DEV_RELEASE_SLUG,
    expectedWarnings: [],
    serviceName: "customer-edge",
    buildName: "platform",
    environmentName: "environment-edge",
    expectedName: "customer-edge",
  },
  {
    force: "1",
    expectedRelease: LOCAL_DEV_RELEASE_SLUG,
    expectedWarnings: [],
    serviceName: undefined,
    buildName: "platform",
    environmentName: undefined,
    expectedName: "platform",
  },
  {
    force: "1",
    expectedRelease: LOCAL_DEV_RELEASE_SLUG,
    expectedWarnings: [],
    serviceName: undefined,
    buildName: "platform",
    environmentName: "environment-edge",
    expectedName: "environment-edge",
  },
])(
  "exports native global spans for $expectedName, force=$force",
  async ({
    force,
    serviceName,
    buildName,
    environmentName,
    expectedName,
    expectedRelease,
    expectedWarnings,
  }) => {
    setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", Boolean(force));

    vi.stubEnv("OTEL_SERVICE_NAME", environmentName);

    setInjectedConfiguration(SERVICE_NAME_GLOBAL, buildName);

    const { register } = await import("./instrumentation.edge.js");

    const { flushServerTelemetry } =
      await import("./internal/server/exporters/server-flush.js");

    await register(serviceName ? { serviceName } : {});

    const span = trace
      .getTracer("edge-consumer")
      .startSpan("consumer-operation");

    expect(span.isRecording()).toBe(true);

    const spanContext = span.spanContext();

    span.end();

    await flushServerTelemetry();

    await delivered();

    expect(registration.mock.results[0]?.value).toBe(true);
    expect(requests).toHaveLength(1);

    const [request] = requests;

    expect(request?.url).toBe("https://edge.invalid/drain/v1/traces");

    expect(request?.method).toBe("POST");
    expect(request?.keepalive).toBe(true);
    expect(request?.headers.get("content-type")).toBe("application/json");

    expect(request?.headers.get("x-interfere-producer-version")).toBe(
      PRODUCER_VERSION,
    );

    const payload: unknown = await request?.json();

    expect(payload).toMatchObject({
      resourceSpans: [
        {
          resource: {
            attributes: Object.entries(
              omitUndefined({
                "deployment.environment.name": "preview",
                "interfere.sdk.name": "@interfere/next",
                "interfere.sdk.version": PRODUCER_VERSION,
                "service.name": expectedName,
                "telemetry.sdk.language": "nodejs",
                "release.slug": expectedRelease,
              }),
            ).map(([key, value]) => ({ key, value: { stringValue: value } })),
          },
          scopeSpans: [
            {
              scope: { name: "edge-consumer" },
              spans: [
                {
                  name: "consumer-operation",
                  traceId: spanContext.traceId,
                  spanId: spanContext.spanId,
                },
              ],
            },
          ],
        },
      ],
    });

    expect(vi.mocked(console.warn).mock.calls).toStrictEqual(expectedWarnings);
  },
);

test("exports the build-baked release identity on production Edge", async () => {
  setInjectedConfiguration(RELEASE_SLUG_GLOBAL, "rel_0123456789abcdef");

  const { register } = await import("./instrumentation.edge.js");

  const { flushServerTelemetry } =
    await import("./internal/server/exporters/server-flush.js");

  await register();

  trace.getTracer("edge-consumer").startSpan("production-edge").end();

  await flushServerTelemetry();

  await delivered();

  await expect(requests[0]?.json()).resolves.toMatchObject({
    resourceSpans: [
      {
        resource: {
          // oxlint-disable-next-line typescript/no-unsafe-assignment -- Vitest asymmetric matchers intentionally accept the remaining resource attributes.
          attributes: expect.arrayContaining([
            {
              key: "release.slug",
              value: { stringValue: "rel_0123456789abcdef" },
            },
          ]),
        },
      },
    ],
  });

  expect(console.warn).not.toHaveBeenCalled();
});
