import { setupServer } from "msw/node";

import { access, mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { HttpResponse, http } from "msw";

import { InterfereClient } from "@interfere/javascript";
import { releaseSlugSchema } from "@interfere/types/releases/slug";
import { runReleasePipeline } from "./pipeline";
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  expect,
  test,
} from "vitest";

const server = setupServer();

const origin = "https://release.test";

const client = new InterfereClient({
  apiKey: { token: "test-key" },
  baseUrl: origin,
});

const slug = releaseSlugSchema.parse("rel_aaaaaaaaaaaaaaaa");

let directory: string;

let file: Parameters<
  typeof runReleasePipeline
>[0]["discovery"]["files"][number];

let createCalls: Request[];

beforeAll(() => {
  server.listen({ onUnhandledRequest: "error" });
});

beforeEach(async () => {
  directory = await mkdtemp(path.join(tmpdir(), "release-preparation-"));

  file = {
    absolute: path.join(directory, "app.js.map"),
    chunkAbsolutes: [path.join(directory, "app.js")],
    chunkUrl: "app.js",
    content: "{}",
    debugId: "debug",
    hash: "hash",
    path: "app.js.map",
  };

  await writeFile(file.absolute, file.content);

  createCalls = [];

  server.use(
    http.post(`${origin}/v3/releases`, ({ request }) => {
      createCalls.push(request);

      return HttpResponse.json(
        {
          build: { hash: "build" },
          destination: { environment: "production", id: "release_1", slug },
          org: { id: "org_1", slug: "acme" },
          source: { id: "source_1" },
          surface: { id: "surface_1", slug: "web" },
        },
        { status: 200 },
      );
    }),
  );

  server.use(
    http.post(`${origin}/v3/releases/${slug}/source-maps/sign`, () =>
      HttpResponse.json(
        {
          expiresAt: Math.floor(Date.now() / 1000) + 600,
          uploads: [
            {
              path: file.path,
            },
          ],
        },
        { status: 200 },
      ),
    ),
  );

  server.use(
    http.put(`${origin}/v3/releases/${slug}/source-maps`, () =>
      HttpResponse.json({ objectKey: "maps/app" }),
    ),
  );

  server.use(
    http.post(
      `${origin}/v3/releases/${slug}/source-maps/finalize`,
      async () => {
        await access(file.absolute);

        return HttpResponse.json({ fileCount: 1, ok: true });
      },
    ),
  );
});

function options() {
  return {
    client,
    buildId: "build",
    environment: "production",
    producerVersion: "build-test",
    uploadSourceMaps: true,
    releaseSlug: slug,
    bundler: "rollup" as const,
    discovery: { files: [file], sourceFileCount: 1 },
    releasesConfig: {
      org: { id: "org_1", slug: "acme" },
      surface: {
        id: "surface_1",
        slug: "web",
        sourceProvider: "github" as const,
        destinationProvider: "cli" as const,
        environmentFiltering: true,
        sdkConfig: null,
      },
    },
  };
}

afterEach(async () => {
  await rm(directory, { force: true, recursive: true });
});

// oxlint-disable-next-line vitest/no-duplicate-hooks -- Independent teardown must still run after another cleanup fails, in native reverse registration order.
afterEach(() => {
  server.resetHandlers();
});

afterAll(() => {
  server.close();
});

test("keeps maps until complete upload acceptance, then cleans them", async () => {
  const result = await runReleasePipeline(options());

  expect(result).toMatchObject({
    cleanupFailures: [],
    fileCount: 1,
    releaseSlug: slug,
  });

  await expect(access(file.absolute)).rejects.toMatchObject({ code: "ENOENT" });
});

test.each([
  { step: "upload", expectedError: "Status code: 400" },
  { step: "finalize", expectedError: "did not accept the complete batch" },
])(
  "preserves artifacts after a rejected $step",
  async ({ step, expectedError }) => {
    server.use(
      step === "upload"
        ? http.put(
            `${origin}/v3/releases/${slug}/source-maps`,
            () => new HttpResponse(null, { status: 400 }),
          )
        : http.post(`${origin}/v3/releases/${slug}/source-maps/finalize`, () =>
            HttpResponse.json({ fileCount: 0, ok: false }),
          ),
    );

    await expect(runReleasePipeline(options())).rejects.toThrow(expectedError);

    await access(file.absolute);
  },
);

test("stops uploading once the server upload deadline has passed", async () => {
  const puts: Request[] = [];

  server.use(
    http.post(`${origin}/v3/releases/${slug}/source-maps/sign`, () =>
      HttpResponse.json({
        expiresAt: Date.UTC(2000, 0, 1) / 1000,
        uploads: [
          {
            path: file.path,
          },
        ],
      }),
    ),
    http.put(`${origin}/v3/releases/${slug}/source-maps`, ({ request }) => {
      puts.push(request);

      return new HttpResponse(null);
    }),
  );

  await expect(runReleasePipeline(options())).rejects.toThrow(
    "Source-map upload session expired",
  );

  expect(puts).toHaveLength(0);

  await access(file.absolute);
});

test("rejects an empty inventory before registering a release", async () => {
  const calls: Request[] = [];

  server.use(
    http.post(`${origin}/v3/releases`, ({ request }) => {
      calls.push(request);

      return HttpResponse.json({}, { status: 500 });
    }),
  );

  await expect(
    runReleasePipeline({
      ...options(),
      discovery: { files: [], sourceFileCount: 0 },
    }),
  ).rejects.toThrow("Source maps are required");

  expect(calls).toHaveLength(0);
});

test.each(
  [
    null,
    true,
    [],
    {},
    { ok: true, fileCount: "1" },
    { ok: 1, fileCount: 1 },
  ].map((response) => ({ response })),
)(
  "preserves artifacts after malformed finalization response %j",
  async ({ response }) => {
    server.use(
      http.post(`${origin}/v3/releases/${slug}/source-maps/finalize`, () =>
        HttpResponse.json(response),
      ),
    );

    await expect(runReleasePipeline(options())).rejects.toThrow(
      "did not accept the complete batch",
    );

    await access(file.absolute);
  },
);

test("registers a release at the collection root the gateway serves", async () => {
  await runReleasePipeline(options());

  expect(createCalls.map((call) => new URL(call.url).pathname)).toStrictEqual([
    "/v3/releases",
  ]);
});

test("reports cleanup failures without undoing acceptance", async () => {
  await rm(file.absolute);

  await mkdir(file.absolute);

  await expect(runReleasePipeline(options())).resolves.toMatchObject({
    cleanupFailures: [{ absolute: file.absolute }],
    fileCount: 1,
  });
});
