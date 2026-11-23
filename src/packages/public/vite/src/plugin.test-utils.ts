import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";

import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { HttpResponse, http } from "msw";
import type { setupServer } from "msw/node";
import { vi } from "vitest";

const MILLIS_PER_SECOND = 1000;

// oxlint-disable-next-line import/no-mutable-exports -- Importing tests read the directory each installVitePluginCase call creates.
export let projectDir = "";

export const configRequests: Request[] = [];

export const releaseRequests: Request[] = [];

export const uploadRequests: Request[] = [];

export const finalizeRequests: Request[] = [];

export function installVitePluginCase(input: {
  gitDir: string;
  gitSha: string;
  releaseApi: string;
  server: ReturnType<typeof setupServer>;
  slug: string;
}) {
  vi.clearAllMocks();

  vi.stubEnv("GIT_DIR", path.join(input.gitDir, ".git"));

  for (const key of releaseSourceIdEnvKeys) {
    vi.stubEnv(key, undefined);
  }

  vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "interfere_pk_test_fake");

  vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

  vi.stubEnv("NEXT_PUBLIC_INTERFERE_BUILD_ID", undefined);

  vi.stubEnv("OTEL_SERVICE_NAME", undefined);

  vi.stubEnv("INTERFERE_SOURCE_ID", undefined);

  vi.stubEnv("NEXT_BUILD_ID", undefined);

  vi.stubEnv("VERCEL_GIT_COMMIT_SHA", undefined);

  vi.stubEnv("GITHUB_SHA", undefined);

  vi.stubEnv("VERCEL_ENV", undefined);

  vi.stubEnv("VERCEL_TARGET_ENV", undefined);

  vi.stubEnv("CLOUDFLARE_ENV", undefined);

  vi.stubEnv("INTERFERE_ENVIRONMENT", "production");

  projectDir = mkdtempSync(path.join(tmpdir(), "interfere-vite-build-"));

  mkdirSync(path.join(projectDir, "dist"));

  writeFileSync(
    path.join(projectDir, "dist/app.js"),
    "console.log('app');\n//# sourceMappingURL=app.js.map",
  );

  writeFileSync(
    path.join(projectDir, "dist/app.js.map"),
    JSON.stringify({
      version: 3,
      sources: ["src/app.ts"],
      names: [],
      mappings: "AAAA",
      sourcesContent: ["console.log('app');"],
    }),
  );

  vi.stubEnv("INTERFERE_API_URL", input.releaseApi);

  configRequests.length = 0;

  input.server.use(
    http.get(`${input.releaseApi}/v3/releases/config`, ({ request }) => {
      configRequests.push(request);

      return HttpResponse.json(
        {
          org: { id: "org_1", slug: "acme" },
          surface: {
            destinationProvider: "cloudflare",
            environmentFiltering: false,
            sdkConfig: null,
            id: "surface_1",
            slug: "vite",
            sourceProvider: "github",
          },
        },
        { status: 200 },
      );
    }),
  );

  releaseRequests.length = 0;

  input.server.use(
    http.post(`${input.releaseApi}/v3/releases`, ({ request }) => {
      releaseRequests.push(request.clone());

      return HttpResponse.json({
        build: { hash: input.gitSha },
        destination: {
          environment: "production",
          id: "release_1",
          slug: input.slug,
        },
        org: { id: "org_1", slug: "acme" },
        source: { id: "source_1" },
        surface: { id: "surface_1", slug: "vite" },
      });
    }),
  );

  input.server.use(
    http.post(
      `${input.releaseApi}/v3/releases/${input.slug}/source-maps/sign`,
      () =>
        HttpResponse.json(
          {
            expiresAt: Math.floor(Date.now() / MILLIS_PER_SECOND) + 600,
            uploads: [
              {
                path: "app.js.map",
              },
            ],
          },
          { status: 200 },
        ),
    ),
  );

  uploadRequests.length = 0;

  input.server.use(
    http.put(
      `${input.releaseApi}/v3/releases/${input.slug}/source-maps`,
      ({ request }) => {
        uploadRequests.push(request.clone());

        return new HttpResponse(null);
      },
    ),
  );

  finalizeRequests.length = 0;

  input.server.use(
    http.post(
      `${input.releaseApi}/v3/releases/${input.slug}/source-maps/finalize`,
      ({ request }) => {
        finalizeRequests.push(request);

        return HttpResponse.json(
          {
            fileCount: 1,
            ok: true,
          },
          { status: 200 },
        );
      },
    ),
  );
}
