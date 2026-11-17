import {
  chmod,
  mkdir,
  mkdtemp,
  readFile,
  readdir,
  rm,
  utimes,
  writeFile,
} from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { build, createBuilder, createLogger, resolveConfig } from "vite";
import type { Logger, Plugin } from "vite";
import {
  afterAll,
  afterEach,
  beforeAll,
  expect,
  onTestFinished,
  test,
  vi,
} from "vitest";

import { interfere } from "../../plugin";
import {
  finalized,
  serveSurfaceProviders,
  startReleaseApi,
  stopReleaseApi,
  stubReleaseEnv,
} from "./release-api.fixture";

beforeAll(() => {
  startReleaseApi();
});

afterEach(() => {
  vi.unstubAllEnvs();
});

afterAll(() => {
  stopReleaseApi();
});

function recordingLogger() {
  const warnings: string[] = [];

  const logger: Logger = {
    ...createLogger("silent"),
    warn(message) {
      warnings.push(message);
    },
  };

  return { logger, warnings };
}

async function project(prefix: string) {
  const root = await mkdtemp(path.join(tmpdir(), prefix));

  onTestFinished(async () => rm(root, { recursive: true, force: true }));

  await writeFile(path.join(root, "entry.js"), 'console.log("entry");');

  return root;
}

async function builtFiles(root: string) {
  const distDir = path.join(root, "dist");

  const files = await readdir(distDir, { recursive: true });

  const chunks = await Promise.all(
    files
      .filter((file) => file.endsWith(".js"))
      .map(async (file) => readFile(path.join(distDir, file), "utf8")),
  );

  return { chunks, maps: files.filter((file) => file.endsWith(".map")) };
}

async function buildWith(root: string, logger: Logger, plugins: Plugin[] = []) {
  await build({
    configFile: false,
    root,
    envDir: root,
    customLogger: logger,
    plugins: [interfere({ commitSha: "deadbeef12345678" }), ...plugins],
    build: { rolldownOptions: { input: path.join(root, "entry.js") } },
  });
}

test("uploads source maps when the build has no public key to check the surface with first", async () => {
  expect.hasAssertions();

  const root = await project("vite-no-public-key-");

  stubReleaseEnv();

  vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "");

  await buildWith(root, recordingLogger().logger);

  expect(finalized).toHaveLength(1);

  await expect(builtFiles(root)).resolves.toMatchObject({ maps: [] });
});

test.each([
  {
    providers: { sourceProvider: null, destinationProvider: "cloudflare" },
    setting: "version control provider",
  },
  {
    providers: { sourceProvider: "github", destinationProvider: null },
    setting: "deployment target",
  },
])(
  "keeps Interfere on and uploads source maps when the App has no $setting",
  async ({ providers }) => {
    expect.hasAssertions();

    const root = await project("vite-surface-unconfigured-");

    stubReleaseEnv();

    vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "");

    serveSurfaceProviders(providers);

    const { logger, warnings } = recordingLogger();

    await buildWith(root, logger);

    expect(warnings).not.toContainEqual(
      expect.stringMatching(/disabled|release skipped/u),
    );

    expect(finalized).toHaveLength(1);

    await expect(builtFiles(root)).resolves.toMatchObject({ maps: [] });
  },
);

test("names the output file that cleanup could not rewrite", async () => {
  expect.hasAssertions();

  const root = await project("vite-cleanup-failure-");

  stubReleaseEnv();

  const { logger, warnings } = recordingLogger();

  const locked: string[] = [];

  onTestFinished(async () => {
    await Promise.all(locked.map(async (file) => chmod(file, 0o644)));
  });

  await buildWith(root, logger, [
    {
      name: "read-only-output",
      async writeBundle(output, bundle) {
        const chunk = Object.keys(bundle).find((file) => file.endsWith(".js"));

        if (!output.dir || !chunk) {
          throw new Error("Built chunk missing");
        }

        locked.push(path.join(output.dir, chunk));

        await chmod(path.join(output.dir, chunk), 0o444);
      },
    },
  ]);

  expect(finalized).toHaveLength(1);

  expect(warnings).toStrictEqual(
    expect.arrayContaining([
      `[interfere] cleanup left ${locked[0]} on disk: EACCES`,
    ]),
  );
});

test("publishes client chunks at their own path when the client writes to the top-level outDir", async () => {
  expect.hasAssertions();

  const root = await project("vite-client-top-level-");

  stubReleaseEnv();

  const builder = await createBuilder({
    configFile: false,
    root,
    envDir: root,
    logLevel: "silent",
    plugins: [
      interfere({ commitSha: "deadbeef12345678" }),
      { name: "tanstack-start" },
    ],
    environments: {
      client: {
        build: {
          outDir: "dist",
          rolldownOptions: { input: path.join(root, "entry.js") },
        },
      },
      ssr: {
        build: {
          outDir: "dist/server",
          rolldownOptions: { input: path.join(root, "entry.js") },
        },
      },
    },
  });

  await builder.buildApp();

  expect(finalized.map((file) => file.path).toSorted()).toStrictEqual([
    expect.stringMatching(/^assets\/entry-[\w-]+\.js\.map$/u),
    expect.stringMatching(/^server\/entry\.m?js\.map$/u),
  ]);
});

test("waits for fresh server maps, looking through nested folders and past stale or non-map files", async () => {
  expect.hasAssertions();

  const root = await project("vite-server-wait-");

  stubReleaseEnv();

  await mkdir(path.join(root, "dist"));

  await writeFile(
    path.join(root, "dist/app.js"),
    "console.log('app');\n//# sourceMappingURL=app.js.map",
  );

  await writeFile(
    path.join(root, "dist/app.js.map"),
    JSON.stringify({
      version: 3,
      sources: ["app.ts"],
      names: [],
      mappings: "AAAA",
    }),
  );

  const serverDir = path.join(root, "server-output");

  const plugin = interfere({
    commitSha: "deadbeef12345678",
    serverSourcemapsDir: "server-output",
  });

  await resolveConfig(
    {
      configFile: false,
      root,
      envDir: root,
      logLevel: "silent",
      plugins: [plugin, { name: "tanstack-start" }],
    },
    "build",
  );

  const closeBundle = plugin.closeBundle as () => Promise<void>;

  await closeBundle();

  expect(finalized).toStrictEqual([]);

  const nested = path.join(serverDir, "chunks/nested");

  await mkdir(nested, { recursive: true });

  await writeFile(path.join(serverDir, "index.js"), "export {};");

  await writeFile(path.join(nested, "stale.js.map"), "{}");

  const stale = new Date(Date.now() - 60_000);

  await utimes(path.join(nested, "stale.js.map"), stale, stale);

  await closeBundle();

  expect(finalized).toStrictEqual([]);

  await writeFile(path.join(nested, "fresh.js.map"), "{}");

  await closeBundle();

  expect(finalized.map((file) => file.path)).toStrictEqual(["app.js.map"]);
});
