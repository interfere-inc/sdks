import { mkdir, rm, writeFile } from "node:fs/promises";
import path from "node:path";
import { HttpResponse, http } from "msw";
import { PHASE_DEVELOPMENT_SERVER } from "next/constants";

import {
  RELEASE_RESPONSE,
  RELEASE_SLUG,
  releaseNetwork as server,
  withTempProject,
} from "../test/fixtures/build.js";
import { withInterfere } from "./config.js";
import { log } from "./internal/logger.js";
import { afterEach, expect, test, vi } from "vitest";

afterEach(() => {
  vi.restoreAllMocks();

  vi.unstubAllEnvs();
});

test("reports no_maps through the production compile hook", async () => {
  expect.hasAssertions();

  await withTempProject("next-outcome-", async (projectDir) => {
    vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

    vi.stubEnv("INTERFERE_API_URL", "https://build.test");

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_BUILD_ID", "a".repeat(40));

    vi.stubEnv("INTERFERE_ENVIRONMENT", "production");

    process.chdir(projectDir);

    const warning = vi.spyOn(log, "warn");
    const info = vi.spyOn(log, "info");

    await writeFile(path.join(projectDir, "instrumentation-client.ts"), "");

    server.use(
      http.get("https://build.test/v3/releases/config", () =>
        HttpResponse.json(
          {
            org: { id: "org", slug: "acme" },
            surface: {
              destinationProvider: "cli",
              environmentFiltering: true,
              id: "surface",
              sdkConfig: null,
              slug: "web",
              sourceProvider: "github",
            },
          },
          { status: 200 },
        ),
      ),
    );

    const created: Request[] = [];

    server.use(
      http.post("https://build.test/v3/releases", ({ request }) => {
        created.push(request);

        return HttpResponse.json(RELEASE_RESPONSE, { status: 200 });
      }),
    );

    server.use(
      http.post("https://build.test/v3/releases/:slug/telemetry", () =>
        HttpResponse.json(
          {
            ok: true,
          },
          { status: 200 },
        ),
      ),
    );

    const config = await withInterfere()(PHASE_DEVELOPMENT_SERVER);

    const compile = config.compiler?.runAfterProductionCompile;

    expect(compile).toBeTypeOf("function");

    await compile?.({ distDir: ".next", projectDir });

    expect(created).toHaveLength(0);

    expect(warning).toHaveBeenCalledWith("Release remains unready", [
      expect.stringContaining("Enable source-map generation"),
    ]);

    expect(info.mock.calls.some(([message]) => message === "Completed")).toBe(
      false,
    );
  });
});

test("reports cleanup_failed through the production compile hook", async () => {
  expect.hasAssertions();

  await withTempProject("next-outcome-", async (projectDir) => {
    vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

    vi.stubEnv("INTERFERE_API_URL", "https://build.test");

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_BUILD_ID", "a".repeat(40));

    vi.stubEnv("INTERFERE_ENVIRONMENT", "production");

    process.chdir(projectDir);

    const warning = vi.spyOn(log, "warn");
    const info = vi.spyOn(log, "info");

    await writeFile(path.join(projectDir, "instrumentation-client.ts"), "");

    server.use(
      http.get("https://build.test/v3/releases/config", () =>
        HttpResponse.json(
          {
            org: { id: "org", slug: "acme" },
            surface: {
              destinationProvider: "cli",
              environmentFiltering: true,
              id: "surface",
              sdkConfig: null,
              slug: "web",
              sourceProvider: "github",
            },
          },
          { status: 200 },
        ),
      ),
    );

    const created: Request[] = [];

    server.use(
      http.post("https://build.test/v3/releases", ({ request }) => {
        created.push(request);

        return HttpResponse.json(RELEASE_RESPONSE, { status: 200 });
      }),
    );

    server.use(
      http.post("https://build.test/v3/releases/:slug/telemetry", () =>
        HttpResponse.json(
          {
            ok: true,
          },
          { status: 200 },
        ),
      ),
    );

    const mapPath = path.join(projectDir, ".next/static/chunks/app.js.map");

    {
      await mkdir(path.dirname(mapPath), { recursive: true });

      await writeFile(mapPath.slice(0, -4), "//# sourceMappingURL=app.js.map");

      await writeFile(
        mapPath,
        JSON.stringify({ mappings: "", sources: ["app.ts"], version: 3 }),
      );

      server.use(
        http.post(
          `https://build.test/v3/releases/${RELEASE_SLUG}/source-maps/sign`,
          () =>
            HttpResponse.json(
              {
                expiresAt: Math.floor(Date.now() / 1000) + 600,
                uploads: [
                  {
                    path: "_next/static/chunks/app.js.map",
                  },
                ],
              },
              { status: 200 },
            ),
        ),
      );

      server.use(
        http.put(
          "https://build.test/v3/releases/:slug/source-maps",
          () => new HttpResponse(null),
        ),
      );

      server.use(
        http.post(
          "https://build.test/v3/releases/:slug/source-maps/finalize",
          async () => {
            await rm(mapPath);

            await mkdir(mapPath);

            return HttpResponse.json({ fileCount: 1, ok: true });
          },
        ),
      );
    }

    const config = await withInterfere()(PHASE_DEVELOPMENT_SERVER);

    const compile = config.compiler?.runAfterProductionCompile;

    expect(compile).toBeTypeOf("function");

    await compile?.({ distDir: ".next", projectDir });

    expect(created).toHaveLength(1);

    expect(warning).toHaveBeenCalledWith(
      "Source-map cleanup left files on disk",
      [expect.stringContaining(mapPath)],
    );

    expect(info.mock.calls.some(([message]) => message === "Completed")).toBe(
      true,
    );
  });
});
