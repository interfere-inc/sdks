import { API_URL } from "@interfere/constants/api";
import { fetchNetwork } from "@interfere/test-config/fetch-network";
import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { pathToFileURL } from "node:url";
import { HttpResponse, http } from "msw";
import { build } from "vite";
import { afterAll, beforeAll, expect, test } from "vitest";

const CLOUDFLARE_RESOLVE_CONDITIONS = [
  "workerd",
  "worker",
  "module",
  "browser",
];

interface ServerEntry {
  captureError: (error: Error) => void;
  flush: () => Promise<void>;
  register: (options: {
    serviceName: string;
    captureUncaughtException: boolean;
    captureUnhandledRejection: boolean;
    consoleBridge: boolean;
  }) => Promise<void>;
}

const traces: string[] = [];

const network = fetchNetwork(
  http.get(`${API_URL}/v3/config`, () =>
    HttpResponse.json(null, { status: 404 }),
  ),
  http.post(`${API_URL}/drain/v1/traces`, async ({ request }) => {
    traces.push(await request.text());

    return HttpResponse.json({});
  }),
  http.post(`${API_URL}/drain/v1/:signal`, () => HttpResponse.json({})),
);

let directory: string;

let server: ServerEntry;

beforeAll(async () => {
  directory = await mkdtemp(path.join(tmpdir(), "interfere-cloudflare-"));

  const entry = path.join(directory, "entry.ts");

  await writeFile(
    entry,
    [
      `export { flush, register } from ${JSON.stringify(path.join(import.meta.dirname, "register.ts"))};`,
      `export { captureError } from ${JSON.stringify(path.join(import.meta.dirname, "capture.ts"))};`,
    ].join("\n"),
  );

  await build({
    configFile: false,
    define: {
      "import.meta.env.VITE_INTERFERE_RELEASE_TOKEN":
        JSON.stringify("irt_workerd_test"),
    },
    logLevel: "silent",
    plugins: [
      {
        name: "workers-platform-module",
        resolveId: (id) => (id === "cloudflare:workers" ? `\0${id}` : null),
        load: (id) =>
          id === "\0cloudflare:workers"
            ? "export function waitUntil(promise) { promise.catch(() => {}); }"
            : null,
      },
    ],
    root: path.join(import.meta.dirname, "../../.."),
    ssr: {
      noExternal: true,
      resolve: { conditions: CLOUDFLARE_RESOLVE_CONDITIONS },
    },
    build: {
      minify: false,
      outDir: directory,
      rolldownOptions: { output: { entryFileNames: "server.mjs" } },
      ssr: entry,
    },
  });

  network.enable();

  server = (await import(
    pathToFileURL(path.join(directory, "server.mjs")).href
  )) as ServerEntry;
}, 60_000);

afterAll(async () => {
  network.disable();

  await rm(directory, { force: true, recursive: true });
});

test("a server bundled with Cloudflare's resolve conditions registers and exports a captured error (website-production 2026-11-22T02:09Z)", async () => {
  await server.register({
    serviceName: "cloudflare-bundle-test",
    captureUncaughtException: false,
    captureUnhandledRejection: false,
    consoleBridge: false,
  });

  server.captureError(new Error("cloudflare bundle probe"));

  await server.flush();

  expect(traces.join("\n")).toContain("cloudflare bundle probe");
});
