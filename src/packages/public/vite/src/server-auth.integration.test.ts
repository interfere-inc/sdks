import { HttpResponse } from "msw";
import { afterAll, beforeAll, expect, test } from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { exceptions } from "@interfere/test-utils/vitest/otlp";

let collector: Awaited<ReturnType<typeof receiver>>;
let app: Awaited<ReturnType<typeof application>>;
let tokenApp: Awaited<ReturnType<typeof application>>;
const cleanups: (() => Promise<void>)[] = [];

const WORKSPACE_KEY = "interfere_secret_us_server_auth";
const TOKEN = "irt_vite_server_auth";

const RUNTIME_ENV = {
  INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
  INTERFERE_ENVIRONMENT: "preview",
};

function drainRequests() {
  return collector.requests.filter((request) =>
    request.url.startsWith("/drain/"),
  );
}

beforeAll(async () => {
  collector = await receiver({
    respond: (request) =>
      request.method === "POST" &&
      new URL(request.url, "http://receiver").pathname === "/v3/releases"
        ? HttpResponse.json({
            build: { hash: RUNTIME_ENV.INTERFERE_SOURCE_ID },
            destination: {
              environment: "preview",
              id: "release_server_auth",
              slug: "rel_0123456789abcdef",
            },
            org: { id: "org_server_auth", slug: "server-auth" },
            source: { id: "source_server_auth" },
            surface: { id: "surface_server_auth", slug: "web" },
            token: TOKEN,
          })
        : undefined,
  });

  cleanups.push(async () => collector.close());

  app = await application(`${import.meta.dirname}/..`, [
    "@interfere/vite",
    "react",
    "vite",
  ]);

  cleanups.push(async () => app.close());

  await app.copyFixture("server-auth");

  await app.build({});

  tokenApp = await application(`${import.meta.dirname}/..`, [
    "@interfere/vite",
    "react",
    "vite",
  ]);

  cleanups.push(async () => tokenApp.close());

  await tokenApp.copyFixture("server-auth");

  await tokenApp.copyFixture("server-release-token");

  await tokenApp.build({
    ...RUNTIME_ENV,
    INTERFERE_API_KEY: WORKSPACE_KEY,
    INTERFERE_API_URL: collector.url,
  });
}, 180_000);

afterAll(async () => {
  for (const cleanup of cleanups.toReversed()) {
    // oxlint-disable-next-line eslint/no-await-in-loop -- Resources close in reverse order of creation.
    await cleanup();
  }
});

test("the Node server runtime sends telemetry with the workspace API key and no public key", async () => {
  collector.reset();

  await app.run("server-dist/server.js", {
    ...RUNTIME_ENV,
    INTERFERE_API_KEY: WORKSPACE_KEY,
    INTERFERE_API_URL: collector.url,
  });

  const drains = drainRequests();

  expect(exceptions(drains, "vite-server-error")).toHaveLength(1);

  expect(
    new Set(drains.map((request) => request.headers.get("authorization"))),
  ).toStrictEqual(new Set([`Bearer ${WORKSPACE_KEY}`]));

  expect(
    drains.filter(
      (request) =>
        request.url.includes("public_key") ||
        request.headers.has("x-interfere-public-key"),
    ),
  ).toStrictEqual([]);
});

test("without INTERFERE_API_KEY the Node server runtime sends telemetry with the release token baked into the server bundle", async () => {
  collector.reset();

  await tokenApp.run("server-dist/server.js", {
    ...RUNTIME_ENV,
    INTERFERE_API_URL: collector.url,
  });

  const drains = drainRequests();

  expect(exceptions(drains, "vite-server-error")).toHaveLength(1);

  expect(
    drains.map((request) => [
      request.headers.get("x-interfere-release-token"),
      request.headers.get("authorization"),
      request.headers.get("x-interfere-public-key"),
    ]),
  ).toStrictEqual(drains.map(() => [TOKEN, null, null]));
});
