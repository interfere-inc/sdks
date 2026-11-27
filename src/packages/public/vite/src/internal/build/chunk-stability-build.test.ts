import { mkdtemp, readFile, readdir, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { deriveReleaseSlug } from "@interfere/types/releases/slug";
import { build } from "vite";
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
  RELEASE_API,
  resetRecordedRequests,
  serveReleaseToken,
  startReleaseApi,
  stopReleaseApi,
} from "./release-api.fixture";

const BROWSER_GLOBALS = fileURLToPath(
  new URL("../browser-globals.ts", import.meta.url),
);

const FIRST = "1111111111111111111111111111111111111111";

const SECOND = "2222222222222222222222222222222222222222";

beforeAll(() => {
  startReleaseApi();
});

afterEach(() => {
  vi.unstubAllEnvs();
});

afterAll(() => {
  stopReleaseApi();
});

test("a new commit renames only the chunk that applies the release identity", async () => {
  const root = await mkdtemp(path.join(tmpdir(), "vite-release-slug-build-"));

  onTestFinished(async () => rm(root, { recursive: true, force: true }));

  vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

  vi.stubEnv("INTERFERE_API_URL", RELEASE_API);

  vi.stubEnv("INTERFERE_ENVIRONMENT", "production");

  await writeFile(
    path.join(root, "app.js"),
    `import { applyBrowserGlobals } from ${JSON.stringify(BROWSER_GLOBALS)};\nimport { env } from "./shared.js";\napplyBrowserGlobals();\nconsole.log(env);`,
  );

  await writeFile(
    path.join(root, "widget.js"),
    `import { env } from "./shared.js";\nconsole.log(env);`,
  );

  await writeFile(
    path.join(root, "shared.js"),
    "export const env = { ...import.meta.env };",
  );

  async function compile(commitSha: string) {
    const outDir = path.join(root, commitSha);

    resetRecordedRequests();

    serveReleaseToken(`irt_${commitSha.slice(0, 8)}`);

    await build({
      configFile: false,
      root,
      envDir: root,
      logLevel: "silent",
      plugins: [interfere({ commitSha })],
      build: {
        outDir,
        minify: false,
        rolldownOptions: {
          input: {
            app: path.join(root, "app.js"),
            widget: path.join(root, "widget.js"),
          },
          output: {
            chunkFileNames: "[name]-[hash].js",
            entryFileNames: "[name]-[hash].js",
          },
        },
      },
    });

    const files = await readdir(outDir);

    return Object.fromEntries(
      await Promise.all(
        files.map(
          async (file) =>
            [file, await readFile(path.join(outDir, file), "utf8")] as const,
        ),
      ),
    );
  }

  const first = await compile(FIRST);

  const second = await compile(SECOND);

  const named = (chunks: Record<string, string>, prefix: string) =>
    Object.keys(chunks).filter((file) => file.startsWith(`${prefix}-`));

  const [app] = named(second, "app");

  expect(named(first, "shared")).toHaveLength(1);
  expect(named(second, "shared")).toStrictEqual(named(first, "shared"));
  expect(named(second, "widget")).toStrictEqual(named(first, "widget"));
  expect(app && second[app]).toContain(deriveReleaseSlug(SECOND));
  expect(app && second[app]).toContain(`irt_${SECOND.slice(0, 8)}`);
});
