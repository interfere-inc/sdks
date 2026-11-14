import { setupServer } from "msw/node";
import { HttpResponse, http } from "msw";

import { createBuildConfigResolver } from "./config";
import { afterAll, afterEach, beforeAll, describe, expect, test } from "vitest";

const server = setupServer();

const url = "https://config.test";

describe("build config", () => {
  test("shares concurrent reads within one build and refreshes for a new build", async () => {
    const calls: Request[] = [];

    server.use(
      http.get("https://config.test/v3/config", ({ request }) => {
        calls.push(request);

        return HttpResponse.json(
          {
            ingestUrl: "https://ingest.test",
            logSourcing: "drain",
            plugins: { errors: false },
            serverTracing: "drain",
          },
          { status: 200 },
        );
      }),
    );

    const resolve = createBuildConfigResolver();

    const results = await Promise.all([
      resolve(url, "first"),
      resolve(url, "first"),
    ]);

    expect(results[0]).toStrictEqual({
      ingestUrl: "https://ingest.test",
      logSourcing: "drain",
      platform: null,
      plugins: { errors: false },
      serverTracing: "drain",
    });

    expect(results[1]).toStrictEqual(results[0]);
    expect(calls).toHaveLength(1);

    await resolve(url, "second");

    expect(calls).toHaveLength(2);

    await createBuildConfigResolver()(url, "first");

    expect(calls).toHaveLength(3);

    await resolve(url, "first", "another-build");

    expect(calls).toHaveLength(4);
  });

  test.each([
    {
      body: { logSourcing: "sdk", serverTracing: "full" },
      expected: { logSourcing: "sdk", platform: null, serverTracing: "full" },
    },
    {
      body: { plugins: { errors: false, unknownPlugin: true } },
      expected: { platform: null, plugins: { errors: false } },
    },
    {
      body: {
        ingestUrl: "bad",
        logSourcing: "invalid",
        platform: "heroku",
        plugins: { errors: "yes" },
        serverTracing: "invalid",
      },
      expected: {
        ingestUrl: undefined,
        logSourcing: undefined,
        platform: null,
        plugins: undefined,
        serverTracing: undefined,
      },
    },
    { body: { plugins: {} }, expected: { platform: null, plugins: {} } },
    {
      body: { platform: "vercel", plugins: {} },
      expected: { platform: "vercel", plugins: {} },
    },
    {
      body: { plugins: [], serverTracing: "drain" },
      expected: { platform: null, plugins: undefined, serverTracing: "drain" },
    },
    { body: null, expected: { platform: null } },
    { body: [], expected: { platform: null } },
  ])("validates fields independently: $body", async ({ body, expected }) => {
    server.use(
      http.get("https://config.test/v3/config", () =>
        HttpResponse.json(body, { status: 200 }),
      ),
    );

    await expect(
      createBuildConfigResolver()(url, "first"),
    ).resolves.toStrictEqual(expected);
  });

  test("preserves runtime fallbacks when config is unavailable", async () => {
    server.use(
      http.get("https://config.test/v3/config", () =>
        HttpResponse.json({}, { status: 503 }),
      ),
    );

    await expect(
      createBuildConfigResolver()(url, "first"),
    ).resolves.toStrictEqual({});

    server.use(
      http.get("https://config.test/v3/config", () => HttpResponse.error()),
    );

    await expect(
      createBuildConfigResolver()(url, "first"),
    ).resolves.toStrictEqual({});
  });
});

beforeAll(() => {
  server.listen({ onUnhandledRequest: "error" });
});

afterEach(() => {
  server.resetHandlers();
});

afterAll(() => {
  server.close();
});

test("isolates workspace config by service name and namespace", async () => {
  const calls: Request[] = [];

  server.use(
    http.get("https://config.test/v3/config", ({ request }) => {
      calls.push(request);

      return HttpResponse.json({
        plugins: {
          errors: request.headers.get("x-interfere-service") === "web",
        },
      });
    }),
  );

  const resolve = createBuildConfigResolver();

  const [web, api] = await Promise.all([
    resolve(url, "workspace", "build", {
      serviceName: "web",
      serviceNamespace: "shop",
    }),
    resolve(url, "workspace", "build", {
      serviceName: "api",
      serviceNamespace: "shop",
    }),
  ]);

  await resolve(url, "workspace", "build", {
    serviceName: "web",
    serviceNamespace: "shop",
  });

  await resolve(url, "workspace", "build", {
    serviceName: "web",
    serviceNamespace: "admin",
  });

  expect(web.plugins?.errors).toBe(true);
  expect(api.plugins?.errors).toBe(false);

  expect(
    calls.map((request) => [
      request.headers.get("x-interfere-public-key"),
      request.headers.get("x-interfere-service"),
      request.headers.get("x-interfere-service-namespace"),
    ]),
  ).toStrictEqual([
    ["workspace", "web", "shop"],
    ["workspace", "api", "shop"],
    ["workspace", "web", "admin"],
  ]);
});
