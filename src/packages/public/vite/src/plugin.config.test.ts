import { setupServer } from "msw/node";
import { HttpResponse, http } from "msw";

import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { build, resolveConfig } from "vite";

import { interfere } from "./plugin";
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  expect,
  test,
  vi,
} from "vitest";

const server = setupServer();

let root: string;

beforeEach(async () => {
  root = await mkdtemp(path.join(tmpdir(), "vite-build-config-"));

  vi.stubEnv("VITEST", "");

  vi.stubEnv("VITEST_WORKER_ID", "");

  vi.stubEnv("INTERFERE_ENVIRONMENT", "production");

  vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

  vi.stubEnv("INTERFERE_API_URL", "https://release.test");

  server.use(
    http.get("https://release.test/v3/releases/config", () =>
      HttpResponse.json({
        org: { id: "org_1", slug: "acme" },
        surface: {
          destinationProvider: null,
          environmentFiltering: false,
          sdkConfig: null,
          id: "surface_1",
          slug: "vite",
          sourceProvider: null,
        },
      }),
    ),
    http.post("https://release.test/v3/releases", () =>
      HttpResponse.json({
        build: { hash: "a".repeat(40) },
        destination: {
          environment: "production",
          id: "release_1",
          slug: "rel_aaaaaaaaaaaaaaaa",
        },
        org: { id: "org_1", slug: "acme" },
        source: { id: "source_1" },
        surface: { id: "surface_1", slug: "vite" },
        token: "irt_config_test",
      }),
    ),
  );
});

afterEach(async () => {
  vi.unstubAllEnvs();

  await rm(root, { force: true, recursive: true });
});

const options = {
  apiHost: "https://config.test",
  commitSha: "a".repeat(40),
  sourceMaps: { upload: false },
};

async function configure(plugin: ReturnType<typeof interfere>) {
  return resolveConfig(
    { root, plugins: [plugin], configFile: false, logLevel: "silent" },
    "build",
    "production",
  );
}

beforeAll(() => {
  server.listen({ onUnhandledFrame: "error" });
});

// oxlint-disable-next-line vitest/no-duplicate-hooks -- Independent teardown must still run after another cleanup fails, in native reverse registration order.
afterEach(() => {
  server.resetHandlers();
});

afterAll(() => {
  server.close();
});

test("embeds all remote settings into a production SSR build", async () => {
  const calls: Request[] = [];

  server.use(
    http.get("https://config.test/v3/config", ({ request }) => {
      calls.push(request);

      return HttpResponse.json(
        {
          ingestUrl: "https://ingest.test",
          logSourcing: "drain",
          platform: "vercel",
          plugins: { errors: false },
          serverTracing: "drain",
        },
        { status: 200 },
      );
    }),
  );

  const entry = path.join(root, "entry.js");

  await writeFile(
    entry,
    `export const config = {
    plugins: import.meta.env.VITE_INTERFERE_PLUGINS,
    ingestUrl: import.meta.env.VITE_INTERFERE_INGEST_URL,
    serverTracing: import.meta.env?.VITE_INTERFERE_SERVER_TRACING,
    logSourcing: import.meta.env?.VITE_INTERFERE_LOG_SOURCING,
    platform: import.meta.env?.VITE_INTERFERE_PLATFORM,
  };`,
  );

  const plugin = interfere(options);

  const config = await configure(plugin);

  expect(config.ssr.noExternal).toContain("@interfere/vite");

  await build({
    build: {
      minify: false,
      outDir: "dist",
      rolldownOptions: { output: { entryFileNames: "entry.js" } },
      ssr: entry,
    },
    configFile: false,
    logLevel: "silent",
    plugins: [plugin],
    root,
  });

  const output = await readFile(path.join(root, "dist/entry.js"), "utf8");

  expect(output).toContain('serverTracing: "drain"');
  expect(output).toContain('logSourcing: "drain"');
  expect(output).toContain('platform: "vercel"');
  expect(output).toContain('ingestUrl: "https://ingest.test"');
  expect(output).toContain(JSON.stringify(JSON.stringify({ errors: false })));
  expect(output).not.toContain("import.meta.env");
  expect(calls).toHaveLength(1);
});

test("leaves malformed or unavailable settings to runtime fallbacks", async () => {
  server.use(
    http.get("https://config.test/v3/config", () =>
      HttpResponse.json(
        {
          logSourcing: 1,
          plugins: [],
          serverTracing: "invalid",
        },
        { status: 200 },
      ),
    ),
  );

  const config = await configure(interfere(options));

  expect(config.define).not.toHaveProperty(
    "import.meta.env.VITE_INTERFERE_PLUGINS",
  );

  expect(config.define).not.toHaveProperty(
    "import.meta.env?.VITE_INTERFERE_SERVER_TRACING",
  );

  expect(config.define).not.toHaveProperty(
    "import.meta.env?.VITE_INTERFERE_LOG_SOURCING",
  );

  server.use(
    http.get("https://config.test/v3/config", () =>
      HttpResponse.json({}, { status: 503 }),
    ),
  );

  const unavailable = await configure(interfere(options));

  const {
    "import.meta.env.VITE_INTERFERE_PLATFORM": fetchedPlatform,
    ...fetched
  } = config.define ?? {};

  const {
    "import.meta.env.VITE_INTERFERE_PLATFORM": unavailablePlatform,
    ...rest
  } = unavailable.define ?? {};

  expect(rest).toStrictEqual(fetched);

  expect(
    unavailable.define?.["import.meta.env.VITE_INTERFERE_SERVER_TRACING"],
  ).toBe("null");

  expect(fetchedPlatform).toBe("null");
  expect(unavailablePlatform).toBe("undefined");
});

test("uses the hosted config endpoint when apiHost is omitted", async () => {
  const calls: Request[] = [];

  server.use(
    http.get("https://api.interfere.com/v3/config", ({ request }) => {
      calls.push(request);

      return HttpResponse.json({ serverTracing: "drain" }, { status: 200 });
    }),
  );

  const config = await configure(
    interfere({ commitSha: options.commitSha, sourceMaps: { upload: false } }),
  );

  expect(config.define?.["import.meta.env.VITE_INTERFERE_SERVER_TRACING"]).toBe(
    JSON.stringify("drain"),
  );

  expect(calls).toHaveLength(1);
});
