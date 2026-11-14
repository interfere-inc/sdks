import { mkdtemp, readFile, readdir, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { build, createLogger } from "vite";
import type { Logger } from "vite";
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
  startReleaseApi,
  stopReleaseApi,
  stubReleaseEnv,
  uploads,
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

test("never leaves forced source maps in the output when one emitted map is malformed", async () => {
  expect.hasAssertions();

  const root = await mkdtemp(path.join(tmpdir(), "vite-malformed-map-build-"));

  onTestFinished(async () => rm(root, { recursive: true, force: true }));

  stubReleaseEnv();

  await writeFile(
    path.join(root, "entry.js"),
    'import("./lazy.js").then((lazy) => lazy.run());',
  );

  await writeFile(
    path.join(root, "lazy.js"),
    'export const run = () => console.log("lazy");',
  );

  await build({
    configFile: false,
    root,
    envDir: root,
    logLevel: "silent",
    plugins: [
      interfere({ commitSha: "deadbeef12345678" }),
      {
        name: "corrupt-one-map",
        async writeBundle(output, bundle) {
          const lazyMap = Object.keys(bundle).find(
            (file) => file.includes("lazy") && file.endsWith(".map"),
          );

          if (!output.dir || !lazyMap) {
            throw new Error("Lazy chunk map missing");
          }

          await writeFile(path.join(output.dir, lazyMap), "{");
        },
      },
    ],
    build: {
      sourcemap: "hidden",
      rolldownOptions: { input: path.join(root, "entry.js") },
    },
  });

  const distDir = path.join(root, "dist");

  const files = await readdir(distDir, { recursive: true });

  expect(files.filter((file) => file.endsWith(".map"))).toStrictEqual([]);

  const chunks = files.filter((file) => file.endsWith(".js"));

  expect(chunks.length).toBeGreaterThanOrEqual(2);

  for (const chunk of chunks) {
    // oxlint-disable-next-line eslint/no-await-in-loop
    await expect(
      readFile(path.join(distDir, chunk), "utf8"),
    ).resolves.not.toContain("sourceMappingURL");
  }

  expect(uploads.size).toBe(0);
});

test("uploads and removes source maps on every vite build --watch rebuild", async () => {
  expect.hasAssertions();

  const root = await mkdtemp(path.join(tmpdir(), "vite-watch-build-"));

  onTestFinished(async () => rm(root, { recursive: true, force: true }));

  stubReleaseEnv();

  const entry = path.join(root, "entry.js");

  await writeFile(entry, 'console.log("first");');

  let confirmed = Promise.withResolvers<string>();

  const logger: Logger = {
    ...createLogger("silent"),
    info(message) {
      if (message.includes("confirmed")) {
        confirmed.resolve(message);
      }
    },
  };

  async function nextOutput(trigger: () => Promise<void>) {
    confirmed = Promise.withResolvers<string>();

    await trigger();

    await confirmed.promise;

    const files = await readdir(distDir);

    expect(files.filter((file) => file.endsWith(".map"))).toStrictEqual([]);

    const chunk = files.find((file) => file.endsWith(".js"));

    if (!chunk) {
      throw new Error("Built chunk missing");
    }

    return readFile(path.join(distDir, chunk), "utf8");
  }

  const distDir = path.join(root, "dist");

  const first = await nextOutput(async () => {
    const watcher = await build({
      configFile: false,
      root,
      envDir: root,
      customLogger: logger,
      plugins: [interfere({ commitSha: "deadbeef12345678" })],
      build: {
        watch: {},
        rolldownOptions: {
          input: entry,
          output: { entryFileNames: "[name].js" },
        },
      },
    });

    if (!("close" in watcher)) {
      throw new Error("Expected a watcher");
    }

    onTestFinished(async () => watcher.close());
  });

  expect(first).not.toContain("sourceMappingURL");

  const second = await nextOutput(async () =>
    writeFile(entry, 'console.log("second");'),
  );

  expect(second).toContain("second");
  expect(second).not.toContain("sourceMappingURL");
  expect(finalized).toHaveLength(2);
});

test.each([
  { options: {}, injected: false },
  { options: { sourceMaps: { upload: true } }, injected: true },
])(
  "leaves library output untouched unless source-map upload is opted in: %j",
  async ({ options, injected }) => {
    expect.hasAssertions();

    const root = await mkdtemp(path.join(tmpdir(), "vite-lib-build-"));

    onTestFinished(async () => rm(root, { recursive: true, force: true }));

    stubReleaseEnv();

    await writeFile(
      path.join(root, "index.js"),
      "export const add = (a, b) => a + b;",
    );

    await build({
      configFile: false,
      root,
      envDir: root,
      logLevel: "silent",
      plugins: [interfere({ commitSha: "deadbeef12345678", ...options })],
      build: {
        lib: {
          entry: path.join(root, "index.js"),
          formats: ["es"],
          fileName: () => "index.js",
        },
      },
    });

    const distDir = path.join(root, "dist");

    const files = await readdir(distDir);

    expect(files.filter((file) => file.endsWith(".map"))).toStrictEqual([]);

    const code = await readFile(path.join(distDir, "index.js"), "utf8");

    expect(code.includes("_debugIds")).toBe(injected);

    expect(finalized.length > 0).toBe(injected);
  },
);

test("uploads and removes maps for a client build followed by a separate vite build --ssr", async () => {
  expect.hasAssertions();

  const root = await mkdtemp(path.join(tmpdir(), "vite-two-process-ssr-"));

  onTestFinished(async () => rm(root, { recursive: true, force: true }));

  stubReleaseEnv();

  await writeFile(path.join(root, "client.js"), 'console.log("client");');

  await writeFile(
    path.join(root, "server.js"),
    'export const render = () => "server";',
  );

  const shared = {
    configFile: false,
    root,
    envDir: root,
    logLevel: "silent",
  } as const;

  await build({
    ...shared,
    plugins: [interfere({ commitSha: "deadbeef12345678" })],
    build: {
      outDir: "dist/client",
      rolldownOptions: { input: path.join(root, "client.js") },
    },
  });

  const clientUploads = finalized.length;

  await build({
    ...shared,
    plugins: [interfere({ commitSha: "deadbeef12345678" })],
    build: { outDir: "dist/server", ssr: path.join(root, "server.js") },
  });

  const files = await readdir(path.join(root, "dist"), { recursive: true });

  expect(files.filter((file) => file.endsWith(".map"))).toStrictEqual([]);

  expect(clientUploads).toBeGreaterThan(0);

  expect(finalized.length).toBeGreaterThan(clientUploads);
});

test("injects a debug ID only into chunks whose source map is published", async () => {
  expect.hasAssertions();

  const root = await mkdtemp(path.join(tmpdir(), "vite-unmapped-chunk-"));

  onTestFinished(async () => rm(root, { recursive: true, force: true }));

  stubReleaseEnv();

  await writeFile(
    path.join(root, "entry.js"),
    'import("virtual:generated").then((generated) => console.log(generated.value));',
  );

  await build({
    configFile: false,
    root,
    envDir: root,
    logLevel: "silent",
    plugins: [
      interfere({ commitSha: "deadbeef12345678" }),
      {
        name: "generated-without-map",
        resolveId(id) {
          return id === "virtual:generated" ? "\0virtual:generated" : null;
        },
        load(id) {
          return id === "\0virtual:generated"
            ? 'export const value = "generated";'
            : null;
        },
      },
    ],
    build: {
      minify: false,
      rolldownOptions: { input: path.join(root, "entry.js") },
    },
  });

  const distDir = path.join(root, "dist");

  const chunks = (await readdir(distDir, { recursive: true })).filter((file) =>
    file.endsWith(".js"),
  );

  const published = new Set(
    finalized.flatMap((file) => [file.path.replace(/\.map$/u, "")]),
  );

  const marked = [];

  for (const chunk of chunks) {
    // oxlint-disable-next-line eslint/no-await-in-loop
    const code = await readFile(path.join(distDir, chunk), "utf8");

    if (code.includes("_debugIds")) {
      marked.push(chunk);
    }
  }

  expect(chunks.length).toBeGreaterThanOrEqual(2);
  expect(marked.toSorted()).toStrictEqual([...published].toSorted());
});
