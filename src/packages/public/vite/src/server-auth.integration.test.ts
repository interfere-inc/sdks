import { afterAll, beforeAll, expect, test } from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { exceptions } from "@interfere/test-utils/vitest/otlp";

let collector: Awaited<ReturnType<typeof receiver>>;
let app: Awaited<ReturnType<typeof application>>;
const cleanups: (() => Promise<void>)[] = [];

const WORKSPACE_KEY = "interfere_secret_us_server_auth";

beforeAll(async () => {
  collector = await receiver();

  cleanups.push(async () => collector.close());

  app = await application(`${import.meta.dirname}/..`, [
    "@interfere/vite",
    "react",
    "vite",
  ]);

  cleanups.push(async () => app.close());

  await app.copyFixture("server-auth");

  await app.build({});
}, 120_000);

afterAll(async () => {
  for (const cleanup of cleanups.toReversed()) {
    // oxlint-disable-next-line eslint/no-await-in-loop -- Resources close in reverse order of creation.
    await cleanup();
  }
});

test("the Node server runtime sends telemetry with the workspace API key and no public key", async () => {
  await app.run("server-dist/server.js", {
    INTERFERE_API_KEY: WORKSPACE_KEY,
    INTERFERE_API_URL: collector.url,
    INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
    INTERFERE_ENVIRONMENT: "preview",
  });

  const drains = collector.requests.filter((request) =>
    request.url.startsWith("/drain/"),
  );

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
