import { readFile, readdir } from "node:fs/promises";
import { join } from "node:path";
import { HttpResponse } from "msw";
import { afterAll, beforeAll, expect, test } from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";

const TOKEN = "irt_keyless_acceptance";

let app: Awaited<ReturnType<typeof application>>;
let collector: Awaited<ReturnType<typeof receiver>>;

beforeAll(async () => {
  collector = await receiver({
    respond: (request) => {
      const { pathname } = new URL(request.url, "http://receiver");

      if (pathname === "/v3/releases/config") {
        return HttpResponse.json(
          {
            code: "AUTH_AK_SURFACE_REQUIRED",
            message:
              "Workspace secret keys need the `x-interfere-public-key` header to choose a surface.",
          },
          { status: 401 },
        );
      }

      return request.method === "POST" && pathname === "/v3/releases"
        ? HttpResponse.json({
            build: { hash: "0123456789abcdef0123456789abcdef01234567" },
            destination: {
              environment: "preview",
              id: "release_keyless",
              slug: "rel_0123456789abcdef",
            },
            org: { id: "org_acceptance", slug: "acceptance" },
            source: { id: "github" },
            surface: { id: "surface_acceptance", slug: "web" },
            token: TOKEN,
          })
        : undefined;
    },
  });

  app = await application(`${import.meta.dirname}/..`, [
    "@interfere/vite",
    "@interfere/react",
    "react",
    "react-dom",
    "vite",
  ]);

  await app.copyFixture("acceptance");

  await app.build({
    INTERFERE_API_KEY: "interfere_ak_acceptance",
    INTERFERE_API_URL: collector.url,
    ACCEPTANCE_INGEST_URL: collector.url,
    INTERFERE_ENVIRONMENT: "preview",
    INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
  });
}, 120_000);

afterAll(async () => {
  await app.close();

  await collector.close();
});

test("a build whose workspace secret has no App config still registers its release and bakes the token", async () => {
  const registrations = collector.requests.filter(
    (request) =>
      request.method === "POST" &&
      new URL(request.url, "http://receiver").pathname === "/v3/releases",
  );

  expect(registrations).toHaveLength(1);

  expect(registrations[0]?.headers.get("x-interfere-public-key")).toBeNull();

  expect(JSON.parse(registrations[0]?.body ?? "{}")).toMatchObject({
    buildId: "0123456789abcdef0123456789abcdef01234567",
    destination: { environment: "preview", provider: "cli" },
  });

  const assets = join(app.root, "dist", "assets");

  const bundles = await Promise.all(
    (await readdir(assets))
      .filter((name) => name.endsWith(".js"))
      .map(async (name) => readFile(join(assets, name), "utf8")),
  );

  expect(bundles.some((code) => code.includes(TOKEN))).toBe(true);
});
