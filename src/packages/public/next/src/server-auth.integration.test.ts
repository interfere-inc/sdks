import { afterAll, beforeAll, expect, test } from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import type { ReceivedRequest } from "@interfere/test-utils/vitest/telemetry-receiver";
import { exceptions } from "@interfere/test-utils/vitest/otlp";

let collector: Awaited<ReturnType<typeof receiver>>;
let app: Awaited<ReturnType<typeof application>>;
let host: Awaited<ReturnType<typeof app.start>>;
const cleanups: (() => Promise<void>)[] = [];

const WORKSPACE_KEY = "interfere_secret_us_server_auth";

const BUILD_ENV = {
  INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
  INTERFERE_ENVIRONMENT: "preview",
};

function drainRequests() {
  return collector.requests.filter((request) =>
    request.url.startsWith("/drain/"),
  );
}

function fromEdge(request: ReceivedRequest) {
  return request.headers.get("x-fern-runtime") === "edge-runtime";
}

function exportsByRuntime() {
  return {
    node: drainRequests().filter((request) => !fromEdge(request)),
    edge: drainRequests().filter((request) => fromEdge(request)),
  };
}

async function failOnNodeAndEdge(url: string) {
  for (const path of ["/api/failure", "/api/edge-failure"]) {
    // oxlint-disable-next-line eslint/no-await-in-loop -- Each route fails on its own runtime before export is checked.
    const response = await fetch(url + path);

    expect(response.status).toBe(500);

    // oxlint-disable-next-line eslint/no-await-in-loop -- Consume each response before continuing.
    await response.text();
  }

  await expect
    .poll(
      () => {
        const { node, edge } = exportsByRuntime();

        return [
          exceptions(node, "next-route-error").length,
          exceptions(edge, "next-edge-route-error").length,
        ].every((count) => count > 0);
      },
      { timeout: 15_000 },
    )
    .toBe(true);

  return exportsByRuntime();
}

beforeAll(async () => {
  collector = await receiver();

  cleanups.push(async () => collector.close());

  app = await application(`${import.meta.dirname}/..`, [
    "@interfere/next",
    "next",
    "react",
    "react-dom",
    "typescript",
    "@types/node",
    "@types/react",
    "@types/react-dom",
  ]);

  cleanups.push(async () => app.close());

  await app.copyFixture("acceptance");

  await app.copyFixture("server-auth");

  await app.buildNext(
    { ...BUILD_ENV, INTERFERE_API_URL: collector.url },
    "turbopack",
  );

  host = await app.start("serve.ts", {
    ...BUILD_ENV,
    INTERFERE_API_URL: collector.url,
    INTERFERE_API_KEY: WORKSPACE_KEY,
    OTEL_BSP_SCHEDULE_DELAY: "250",
  });

  cleanups.push(async () => host.close());
}, 240_000);

afterAll(async () => {
  for (const cleanup of cleanups.toReversed()) {
    // oxlint-disable-next-line eslint/no-await-in-loop -- Resources close in reverse order of creation.
    await cleanup();
  }
});

test("server runtimes on Node and Edge send telemetry with the workspace API key and no public key", async () => {
  expect.hasAssertions();

  const { node, edge } = await failOnNodeAndEdge(host.url);

  expect(
    node.map((request) => request.headers.get("authorization")),
  ).toStrictEqual(node.map(() => `Bearer ${WORKSPACE_KEY}`));

  expect(edge.map((request) => request.headers.get("x-api-key"))).toStrictEqual(
    edge.map(() => WORKSPACE_KEY),
  );

  expect(
    drainRequests().filter(
      (request) =>
        request.url.includes("public_key") ||
        request.headers.has("x-interfere-public-key"),
    ),
  ).toStrictEqual([]);
});
