import { readFile, readdir } from "node:fs/promises";
import { join } from "node:path";
import { TraceMap, originalPositionFor } from "@jridgewell/trace-mapping";
import { Miniflare, convertV4MiniflareOptions } from "miniflare";
import { HttpResponse } from "msw";
import { z } from "zod";
import { afterAll, beforeAll, expect, test } from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";

const MILLIS_PER_SECOND = 1000;

const WORKER_ENVIRONMENT = "packed_worker";

const FRAME_RE =
  /at explode \((?<file>[^\s()]+):(?<line>\d+):(?<column>\d+)\)/u;

const finalizeSchema = z.object({
  files: z.array(
    z.object({
      chunkUrl: z.string(),
      debugId: z.string(),
      path: z.string(),
    }),
  ),
});

const signSchema = z.object({ files: z.array(z.object({ path: z.string() })) });

const workerConfigSchema = z.object({
  compatibility_date: z.string(),
  main: z.string(),
});

let app: Awaited<ReturnType<typeof application>>;
let collector: Awaited<ReturnType<typeof receiver>>;
let distFiles: string[];
let workerStack: string;

function uploaded(path: string) {
  return collector.requests.find(
    (request) =>
      request.method === "PUT" &&
      new URL(request.url, "http://receiver").searchParams.get("path") === path,
  )?.body;
}

function finalized() {
  return collector.requests
    .filter((request) =>
      new URL(request.url, "http://receiver").pathname.endsWith(
        "/source-maps/finalize",
      ),
    )
    .flatMap((request) => finalizeSchema.parse(JSON.parse(request.body)).files);
}

beforeAll(async () => {
  collector = await receiver({
    respond: (request) => {
      const { pathname } = new URL(request.url, "http://receiver");

      if (request.method === "POST" && pathname === "/v3/releases") {
        return HttpResponse.json({
          build: { hash: "0123456789abcdef0123456789abcdef01234567" },
          destination: {
            environment: "production",
            id: "release_worker",
            slug: "rel_0123456789abcdef",
          },
          org: { id: "org_worker", slug: "worker" },
          source: { id: "source_worker" },
          surface: { id: "surface_worker", slug: "worker" },
        });
      }

      if (pathname.endsWith("/source-maps/sign")) {
        return HttpResponse.json({
          expiresAt: Math.floor(Date.now() / MILLIS_PER_SECOND) + 600,
          uploads: signSchema
            .parse(JSON.parse(request.body))
            .files.map(({ path }) => ({ path })),
        });
      }

      if (pathname.endsWith("/source-maps/finalize")) {
        return HttpResponse.json({
          ok: true,
          fileCount: finalizeSchema.parse(JSON.parse(request.body)).files
            .length,
        });
      }

      return;
    },
  });

  app = await application(`${import.meta.dirname}/..`, [
    "@interfere/vite",
    "@cloudflare/vite-plugin",
    "vite",
  ]);

  await app.copyFixture("cloudflare-worker");

  await app.build({
    INTERFERE_API_KEY: "interfere_ak_worker",
    INTERFERE_API_URL: collector.url,
    INTERFERE_ENVIRONMENT: "production",
    INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
  });

  distFiles = await readdir(join(app.root, "dist"), { recursive: true });

  const workerDir = join(app.root, "dist", WORKER_ENVIRONMENT);

  const config = workerConfigSchema.parse(
    JSON.parse(await readFile(join(workerDir, "wrangler.json"), "utf8")),
  );

  const worker = new Miniflare(
    convertV4MiniflareOptions({
      compatibilityDate: config.compatibility_date,
      modules: true,
      modulesRoot: workerDir,
      scriptPath: join(workerDir, config.main),
    }),
  );

  try {
    const response = await worker.dispatchFetch("http://worker.test/boom");

    workerStack = await response.text();
  } finally {
    await worker.dispose();
  }
}, 120_000);

afterAll(async () => {
  await app.close();

  await collector.close();
});

test("a Cloudflare Worker build uploads its Worker and client source maps under their deployed paths", () => {
  const paths = finalized().map((file) => [file.path, file.chunkUrl]);

  expect(paths).toContainEqual(["index.js.map", "index.js"]);

  expect(
    paths.some(
      ([path, chunkUrl]) =>
        path?.startsWith("assets/") && chunkUrl?.startsWith("assets/"),
    ),
  ).toBe(true);

  expect(
    paths.every(
      ([path]) =>
        !path?.startsWith("client/") &&
        !path?.startsWith(`${WORKER_ENVIRONMENT}/`),
    ),
  ).toBe(true);
});

test("a Cloudflare Worker build leaves no source map in the Worker or client output", async () => {
  expect.hasAssertions();

  expect(distFiles.filter((file) => file.endsWith(".map"))).toStrictEqual([]);

  const chunks = distFiles.filter((file) => file.endsWith(".js"));

  expect(chunks).toContain(join(WORKER_ENVIRONMENT, "index.js"));

  for (const chunk of chunks) {
    // oxlint-disable-next-line eslint/no-await-in-loop
    await expect(
      readFile(join(app.root, "dist", chunk), "utf8"),
    ).resolves.not.toContain("sourceMappingURL");
  }
});

test("the uploaded Worker map symbolicates a stack the built Worker throws in workerd", () => {
  expect(workerStack).toContain("worker failure at /boom");

  const frame = FRAME_RE.exec(workerStack)?.groups;

  expect(frame).toBeDefined();

  const file = finalized().find((entry) =>
    frame?.["file"]?.endsWith(`/${entry.chunkUrl}`),
  );

  expect(file).toBeDefined();

  const map = uploaded(file?.path ?? "");

  expect(map).toBeDefined();

  const original = originalPositionFor(new TraceMap(map ?? "{}"), {
    line: Number(frame?.["line"]),
    column: Number(frame?.["column"]) - 1,
  });

  expect(original.source).toMatch(/worker\.ts$/u);
  expect(original.line).toBe(2);
});
