import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { PLUGIN_MANIFEST } from "@interfere/types/sdk/plugins/manifest";
import { PLUGINS_GLOBAL } from "@interfere/types/sdk/remote-config";
import { setupServer } from "msw/node";
import { HttpResponse, http } from "msw";
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  test,
  vi,
} from "vitest";

const API = "https://next-remote-config.test";

const server = setupServer();

const requests: Request[] = [];

function serve(response: () => Response) {
  server.use(
    http.get(`${API}/v3/config`, ({ request }) => {
      requests.push(request);

      return response();
    }),
  );
}

async function load() {
  return import("./remote-config.js");
}

beforeAll(() => {
  server.listen({ onUnhandledRequest: "error" });
});

beforeEach(() => {
  vi.resetModules();

  requests.length = 0;

  vi.stubEnv("NODE_ENV", "production");

  vi.stubEnv("INTERFERE_API_URL", API);

  vi.stubEnv("INTERFERE_API_KEY", "interfere_secret_us_test");

  vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_test");

  setInjectedConfiguration(PLUGINS_GLOBAL, undefined);
});

afterEach(() => {
  server.resetHandlers();

  vi.unstubAllEnvs();
});

afterAll(() => {
  server.close();
});

describe("N-RC-01 the server fails open before any config is known", () => {
  test("every plugin reads enabled before a fetch", async () => {
    const { isPluginEnabled } = await load();

    expect(isPluginEnabled("replay")).toBe(true);
    expect(isPluginEnabled("errors")).toBe(true);
  });
});

describe("N-RC-02 baked plugins win and the server never fetches", () => {
  test("baked replay false is honoured with no /v3/config request", async () => {
    serve(() => HttpResponse.json({ plugins: { replay: true } }));

    setInjectedConfiguration(
      PLUGINS_GLOBAL,
      Object.fromEntries(
        PLUGIN_MANIFEST.map((p) => [p.name, p.name !== "replay"]),
      ),
    );

    const { fetchAndCacheRemoteConfig, isPluginEnabled } = await load();

    await fetchAndCacheRemoteConfig();

    expect(requests).toHaveLength(0);
    expect(isPluginEnabled("replay")).toBe(false);
    expect(isPluginEnabled("logs")).toBe(true);
  });
});

describe("N-RC-03 the fetched config is cached and a remote false disables on the server", () => {
  test("logs false from /v3/config disables logs after one fetch", async () => {
    serve(() => HttpResponse.json({ plugins: { logs: false } }));

    const { fetchAndCacheRemoteConfig, isPluginEnabled } = await load();

    await fetchAndCacheRemoteConfig();

    expect(requests).toHaveLength(1);

    expect([...(requests[0]?.headers.values() ?? [])]).toContain(
      "interfere_pk_test",
    );

    expect(isPluginEnabled("logs")).toBe(false);
    expect(isPluginEnabled("errors")).toBe(true);
  });
});

describe("N-RC-04 a failed server fetch keeps defaults", () => {
  test.each([
    ["a 503", () => new HttpResponse("down", { status: 503 })],
    ["a network error", () => HttpResponse.error()],
    ["a malformed body", () => HttpResponse.json({ plugins: { logs: "no" } })],
  ])("%s leaves every plugin enabled", async (_case, response) => {
    serve(response);

    const { fetchAndCacheRemoteConfig, isPluginEnabled } = await load();

    await expect(fetchAndCacheRemoteConfig()).resolves.toBeUndefined();

    expect(requests).toHaveLength(1);
    expect(isPluginEnabled("logs")).toBe(true);
  });

  test("a later failure keeps the last good config", async () => {
    serve(() => HttpResponse.json({ plugins: { logs: false } }));

    const { fetchAndCacheRemoteConfig, isPluginEnabled } = await load();

    await fetchAndCacheRemoteConfig();

    server.resetHandlers();

    serve(() => HttpResponse.error());

    await fetchAndCacheRemoteConfig();

    expect(isPluginEnabled("logs")).toBe(false);
  });
});

describe("N-RC-05 the server fetches in development with a public key", () => {
  test.each([
    { force: undefined, expected: 1 },
    { force: "1", expected: 1 },
  ])(
    "with NODE_ENV development and FORCE_ENABLE=$force makes $expected requests",
    async ({ force, expected }) => {
      vi.stubEnv("NODE_ENV", "development");

      vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", force);

      serve(() => HttpResponse.json({ plugins: {} }));

      const { fetchAndCacheRemoteConfig } = await load();

      await fetchAndCacheRemoteConfig();

      expect(requests).toHaveLength(expected);
    },
  );
});

describe("N-RC-06 the server does not fetch without a public key", () => {
  test("no INTERFERE_PUBLIC_KEY means no request", async () => {
    vi.stubEnv("INTERFERE_PUBLIC_KEY", undefined);

    serve(() => HttpResponse.json({ plugins: {} }));

    const { fetchAndCacheRemoteConfig } = await load();

    await fetchAndCacheRemoteConfig();

    expect(requests).toHaveLength(0);
  });
});

test("selects server config with the service identity", async () => {
  serve(() => HttpResponse.json({ plugins: {} }));

  const { fetchAndCacheRemoteConfig } = await load();

  await fetchAndCacheRemoteConfig({
    serviceName: "web",
    serviceNamespace: "shop",
  });

  expect(requests[0]?.headers.get("x-interfere-service")).toBe("web");

  expect(requests[0]?.headers.get("x-interfere-service-namespace")).toBe(
    "shop",
  );
});
