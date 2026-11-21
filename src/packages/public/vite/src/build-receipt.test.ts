import { createHash } from "node:crypto";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { build } from "vite";
import { HttpResponse, http } from "msw";
import { setupServer } from "msw/node";
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  expect,
  test,
  vi,
} from "vitest";

import { interfere } from "./plugin";

const server = setupServer();
let root: string;
const receiptPath = ".cache/build-receipt.json";

beforeAll(() => server.listen({ onUnhandledFrame: "error" }));

beforeEach(async () => {
  root = await mkdtemp(path.join(tmpdir(), "vite-private-receipt-"));

  vi.stubEnv("VITEST", "");

  vi.stubEnv("VITEST_WORKER_ID", "");

  vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "interfere_pk_test");

  vi.stubEnv("INTERFERE_ENVIRONMENT", "preview");

  vi.stubEnv("PRIVATE_DEPLOYMENT_SECRET", "never-in-receipt");

  vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

  vi.stubEnv("INTERFERE_API_URL", "https://release.test");

  server.use(
    http.get("https://release.test/v3/releases/config", () =>
      HttpResponse.json({ message: "offline" }, { status: 400 }),
    ),
  );

  await writeFile(
    path.join(root, "entry.js"),
    "export const config = import.meta.env.VITE_INTERFERE_PLUGINS;",
  );
});

afterEach(async () => {
  server.resetHandlers();

  vi.unstubAllEnvs();

  await rm(root, { recursive: true, force: true });
});

afterAll(() => server.close());

async function compile(destination = receiptPath) {
  await build({
    root,
    configFile: false,
    logLevel: "silent",
    plugins: [
      interfere({
        apiHost: "https://config.test",
        commitSha: "a".repeat(40),
        sourceMaps: { upload: false },
        buildReceipt: destination,
      }),
    ],
    build: {
      ssr: path.join(root, "entry.js"),
      outDir: "dist",
      minify: false,
      rolldownOptions: { output: { entryFileNames: "entry.js" } },
    },
  });
}

test("retains consumed configuration and final native output identities privately", async () => {
  server.use(
    http.get("https://config.test/v3/config", () =>
      HttpResponse.json({
        plugins: { errors: false },
        privateToken: "never-in-receipt",
      }),
    ),
  );

  await compile();

  const content = await readFile(path.join(root, receiptPath), "utf8");

  const bytes = await readFile(path.join(root, "dist/entry.js"));

  expect(JSON.parse(content)).toMatchObject({
    build: {
      commitSha: "a".repeat(40),
      buildId: "a".repeat(40),
      environment: "preview",
    },
    config: {
      status: "fetched",
      consumed: { plugins: { errors: false } },
    },
    source: null,
    assets: [
      {
        path: "dist/entry.js",
        sha256: createHash("sha256").update(bytes).digest("hex"),
      },
    ],
  });

  expect(content).toMatch(/"digest": "[a-f0-9]{64}"/u);
  expect(content).not.toContain("never-in-receipt");
  expect(bytes.toString()).not.toContain("schemaVersion");
});

test("changes the digest when consumed configuration changes", async () => {
  let errors = false;

  server.use(
    http.get("https://config.test/v3/config", () =>
      HttpResponse.json({ plugins: { errors } }),
    ),
  );

  await compile();

  const first = await readFile(path.join(root, receiptPath), "utf8");

  errors = true;

  await compile();

  const second = await readFile(path.join(root, receiptPath), "utf8");

  expect(
    /"digest": "(?<digest>[a-f0-9]+)"/u.exec(second)?.groups?.["digest"],
  ).not.toBe(
    /"digest": "(?<digest>[a-f0-9]+)"/u.exec(first)?.groups?.["digest"],
  );

  expect(JSON.parse(second)).toMatchObject({
    config: { consumed: { plugins: { errors: true } } },
  });
});
