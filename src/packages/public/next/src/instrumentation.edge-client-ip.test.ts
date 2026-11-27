import { HttpResponse, http } from "msw";
import { registeredTracerProvider } from "@interfere/server-core/ownership";
import { trace } from "@opentelemetry/api";
import { TracerProvider } from "@interfere/server-core/otel/sdk/trace/provider";
import { afterEach, beforeAll, beforeEach, expect, test, vi } from "vitest";
import { delivered, network, requests } from "../test/edge/network.fixture";

beforeAll(async () => {
  await import("./instrumentation.edge.js");
});

beforeEach(() => {
  vi.resetModules();

  trace.disable();

  vi.stubEnv("INTERFERE_API_KEY", "interfere_secret_us_test");

  vi.stubEnv("INTERFERE_API_URL", "https://edge.invalid");

  vi.stubEnv("INTERFERE_SOURCE_ID", "a".repeat(40));

  vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

  network.use(
    http.post(
      "https://edge.invalid/drain/v1/traces",
      () => new HttpResponse(null, { status: 204 }),
    ),
  );
});

afterEach(async () => {
  const provider = registeredTracerProvider();

  if (provider instanceof TracerProvider) {
    await provider.shutdown();
  }

  trace.disable();

  vi.unstubAllEnvs();
});

test.each([
  ["managed", undefined],
  ["managed", true],
  ["managed", false],
  ["application", undefined],
  ["application", false],
] as const)(
  "%s Edge capture respects clientIp=%s",
  async (tracing, clientIp) => {
    const { register, onRequestError } =
      await import("./instrumentation.edge.js");

    await register({
      tracing,
      privacy: clientIp === undefined ? {} : { clientIp },
    });

    await onRequestError(
      new Error("edge request failed"),
      {
        method: "GET",
        path: "/orders",
        headers: {
          "x-forwarded-for": "203.0.113.7, 10.0.0.1",
          "x-forwarded-host": "app.example.com",
          authorization: "secret-credential",
        },
      },
      { routePath: "/orders", routeType: "route", routerKind: "App Router" },
    );

    await delivered();

    expect(requests).toHaveLength(1);

    const body = await requests[0]?.text();

    expect(body).toContain("edge request failed");
    expect(body).toContain("app.example.com");
    expect(body).not.toContain("secret-credential");
    expect(body?.includes("203.0.113.7, 10.0.0.1")).toBe(clientIp !== false);
  },
);
