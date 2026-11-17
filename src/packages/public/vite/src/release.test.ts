import { mkdtemp, readFile, readdir, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { HttpResponse, http } from "msw";
import { setupServer } from "msw/node";
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

import { interfere } from "./plugin";
import { releaseBuildOutput } from "./release";

const RELEASE_API = "https://vite-release-output.test";

const requests: {
  environment: string | null;
  path: string;
  service?: { name: string } | undefined;
}[] = [];

const serviceHeaders: (string | null)[] = [];

const server = setupServer(
  http.get(`${RELEASE_API}/v3/releases/config`, () =>
    HttpResponse.json({
      org: { id: "org_1", slug: "acme" },
      surface: {
        destinationProvider: "cloudflare",
        environmentFiltering: false,
        sdkConfig: null,
        id: "surface_1",
        slug: "vite",
        sourceProvider: "github",
      },
    }),
  ),
  http.post(`${RELEASE_API}/v3/releases`, async ({ request }) => {
    serviceHeaders.push(request.headers.get("x-interfere-service"));

    const body = (await request.json()) as {
      destination: { environment?: string };
      service?: { name: string };
    };

    requests.push({
      environment: body.destination.environment ?? null,
      path: "releases",
      service: body.service,
    });

    return HttpResponse.json({
      build: { hash: "deadbeef12345678" },
      destination: {
        environment: body.destination.environment ?? "production",
        id: "release_1",
        slug: "rel_deadbeef12345678",
      },
      org: { id: "org_1", slug: "acme" },
      source: { id: "source_1" },
      surface: { id: "surface_1", slug: "vite" },
    });
  }),
  http.post(
    `${RELEASE_API}/v3/releases/:slug/source-maps/sign`,
    async ({ request }) => {
      const body = (await request.json()) as {
        environment?: string;
        files: { path: string }[];
      };

      requests.push({
        environment:
          new URL(request.url).searchParams.get("environment") ??
          body.environment ??
          null,
        path: "sign",
      });

      return HttpResponse.json({
        expiresAt: Math.floor(Date.now() / 1000) + 600,
        uploads: body.files.map((file) => ({ path: file.path })),
      });
    },
  ),
  http.put(`${RELEASE_API}/v3/releases/:slug/source-maps`, ({ request }) => {
    requests.push({
      environment: new URL(request.url).searchParams.get("environment"),
      path: "upload",
    });

    return new HttpResponse(null);
  }),
  http.post(
    `${RELEASE_API}/v3/releases/:slug/source-maps/finalize`,
    async ({ request }) => {
      const body = (await request.json()) as { files: unknown[] };

      requests.push({ environment: null, path: "finalize" });

      return HttpResponse.json({ ok: true, fileCount: body.files.length });
    },
  ),
);

beforeAll(() => {
  server.listen({ onUnhandledRequest: "error" });
});

afterEach(() => {
  requests.length = 0;

  server.resetHandlers();

  vi.unstubAllEnvs();
});

afterAll(() => {
  server.close();
});

async function buildRetained(root: string) {
  await writeFile(
    path.join(root, "entry.js"),
    "globalThis.ready = () => { throw new Error('boom'); };",
  );

  await writeFile(
    path.join(root, "index.html"),
    '<html><head><script type="module" src="/entry.js"></script></head></html>',
  );

  await build({
    configFile: false,
    root,
    envDir: root,
    logLevel: "silent",
    plugins: [
      interfere({
        serviceName: "retained-web",
        buildReceipt: ".cache/receipt.json",
        commitSha: "0123456789abcdef0123456789abcdef01234567",
        sourceMaps: { retain: true },
      }),
    ],
    build: { outDir: "dist", minify: false },
  });

  return readdir(path.join(root, "dist", "assets"));
}

test("retains debug-id-paired source maps without registering a release at build time", async () => {
  expect.hasAssertions();

  const root = await mkdtemp(path.join(tmpdir(), "vite-release-output-"));

  onTestFinished(async () => rm(root, { recursive: true, force: true }));

  vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_build");

  vi.stubEnv("INTERFERE_API_URL", RELEASE_API);

  const assets = await buildRetained(root);

  const chunk = assets.find((file) => file.endsWith(".js"));

  expect(chunk).toBeDefined();
  expect(assets).toContain(`${chunk}.map`);
  expect(requests).toStrictEqual([]);

  const code = await readFile(
    path.join(root, "dist/assets", chunk ?? ""),
    "utf8",
  );

  expect(code).toMatch(/\/\/# debugId=[0-9a-f-]{36}/u);
  expect(code).toMatch(/\/\/# sourceMappingURL=/u);

  const receipt = JSON.parse(
    await readFile(path.join(root, ".cache/receipt.json"), "utf8"),
  ) as { build: { releaseSlug: string | null; environment: string | null } };

  expect(receipt.build.releaseSlug).toMatch(/^rel_/u);
  expect(receipt.build.environment).toBeNull();
});

test("releases the retained output per environment and deletes the maps", async () => {
  expect.hasAssertions();

  const root = await mkdtemp(path.join(tmpdir(), "vite-release-output-"));

  onTestFinished(async () => rm(root, { recursive: true, force: true }));

  await buildRetained(root);

  const before = await readFile(
    path.join(
      root,
      "dist/assets",
      (await readdir(path.join(root, "dist/assets"))).find((f) =>
        f.endsWith(".js"),
      ) ?? "",
    ),
    "utf8",
  );

  const result = await releaseBuildOutput({
    apiKey: "interfere_ak_deploy",
    apiUrl: RELEASE_API,
    distDir: "dist",
    environment: "preview",
    logger: {
      error: vi.fn<(message: string) => void>(),
      info: vi.fn<(message: string) => void>(),
      warn: vi.fn<(message: string) => void>(),
    },
    projectDir: root,
    publicKey: "interfere_pk_test",
    receipt: ".cache/receipt.json",
  });

  expect(result).toStrictEqual({
    fileCount: 1,
    releaseSlug: "rel_deadbeef12345678",
    status: "released",
  });

  expect(requests.map((request) => request.path)).toStrictEqual([
    "releases",

    "sign",
    "upload",
    "finalize",
  ]);

  expect(serviceHeaders.at(-1)).toBe("retained-web");
  expect(requests[0]?.service).toStrictEqual({ name: "retained-web" });
  expect(requests[0]?.environment).toBe("preview");
  expect(requests[1]?.environment).toBe("preview");

  const assets = await readdir(path.join(root, "dist/assets"));

  expect(assets.some((file) => file.endsWith(".map"))).toBe(false);

  const chunk = assets.find((file) => file.endsWith(".js")) ?? "";

  // WARNING: the release step deletes the maps and strips their sourceMappingURL comment, exactly as the in-build upload does; every other chunk byte (including the debug id) stays as built, so per-environment releases of one build serve identical chunks.
  await expect(
    readFile(path.join(root, "dist/assets", chunk), "utf8"),
  ).resolves.toBe(before.replace(/\n?\/\/# sourceMappingURL=\S+\s*$/u, ""));
});

test("only deletes the retained maps when no API key is available", async () => {
  expect.hasAssertions();

  const root = await mkdtemp(path.join(tmpdir(), "vite-release-output-"));

  onTestFinished(async () => rm(root, { recursive: true, force: true }));

  await buildRetained(root);

  const result = await releaseBuildOutput({
    distDir: "dist",
    logger: {
      error: vi.fn<(message: string) => void>(),
      info: vi.fn<(message: string) => void>(),

      warn: vi.fn<(message: string) => void>(),
    },
    projectDir: root,
    receipt: ".cache/receipt.json",
  });

  expect(result).toMatchObject({ status: "skipped" });

  expect(requests).toStrictEqual([]);

  expect(
    (await readdir(path.join(root, "dist/assets"))).some((file) =>
      file.endsWith(".map"),
    ),
  ).toBe(false);
});

test("skips when the release already holds source maps from another build of the same bundle", async () => {
  expect.hasAssertions();

  const root = await mkdtemp(path.join(tmpdir(), "vite-release-output-"));

  onTestFinished(async () => rm(root, { recursive: true, force: true }));

  await buildRetained(root);

  server.use(
    http.post(`${RELEASE_API}/v3/releases/:slug/source-maps/sign`, () =>
      HttpResponse.json(
        {
          code: "RELEASE_UPLOAD_CONFLICT",
          message:
            "This release already has different source map content or metadata.",
          service: "frontier-buildtime",
        },
        { status: 409 },
      ),
    ),
  );

  const result = await releaseBuildOutput({
    apiKey: "interfere_ak_deploy",
    apiUrl: RELEASE_API,
    distDir: "dist",
    environment: "production",
    logger: {
      error: vi.fn<(message: string) => void>(),
      info: vi.fn<(message: string) => void>(),
      warn: vi.fn<(message: string) => void>(),
    },
    projectDir: root,
    publicKey: "interfere_pk_test",
    receipt: ".cache/receipt.json",
  });

  expect(result).toMatchObject({
    reason: expect.stringContaining("already holds source maps") as string,
    status: "skipped",
  });

  expect(
    (await readdir(path.join(root, "dist/assets"))).some((file) =>
      file.endsWith(".map"),
    ),
  ).toBe(false);
});

test("retries a bundle once when a release request fails transiently", async () => {
  expect.hasAssertions();

  const root = await mkdtemp(path.join(tmpdir(), "vite-release-output-"));

  onTestFinished(async () => rm(root, { recursive: true, force: true }));

  await buildRetained(root);

  server.use(
    http.post(`${RELEASE_API}/v3/releases`, () => HttpResponse.error(), {
      once: true,
    }),
  );

  const warn = vi.fn<(message: string) => void>();

  const result = await releaseBuildOutput({
    apiKey: "interfere_ak_deploy",
    apiUrl: RELEASE_API,
    distDir: "dist",
    environment: "production",
    logger: {
      error: vi.fn<(message: string) => void>(),
      info: vi.fn<(message: string) => void>(),
      warn,
    },
    projectDir: root,
    publicKey: "interfere_pk_test",
    receipt: ".cache/receipt.json",
  });

  expect(result).toStrictEqual({
    fileCount: 1,
    releaseSlug: "rel_deadbeef12345678",
    status: "released",
  });

  expect(warn).toHaveBeenCalledWith(
    expect.stringContaining("release attempt 1 for dist failed"),
  );

  expect(requests.map((request) => request.path)).toStrictEqual([
    "releases",
    "sign",
    "upload",
    "finalize",
  ]);
});
