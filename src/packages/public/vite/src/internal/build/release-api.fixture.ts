import { HttpResponse, http } from "msw";
import { setupServer } from "msw/node";
import { vi } from "vitest";

export const RELEASE_API = "https://vite-debug-id.test";

export const uploads = new Map<string, string>();

export const finalized: { path: string; debugId: string }[] = [];

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
  http.post(`${RELEASE_API}/v3/releases`, () =>
    HttpResponse.json({
      build: { hash: "deadbeef12345678" },
      destination: {
        environment: "production",
        id: "release_1",
        slug: "rel_deadbeef12345678",
      },
      org: { id: "org_1", slug: "acme" },
      source: { id: "source_1" },
      surface: { id: "surface_1", slug: "vite" },
    }),
  ),
  http.post(
    `${RELEASE_API}/v3/releases/:slug/source-maps/sign`,
    async ({ request }) => {
      const body = (await request.json()) as { files: { path: string }[] };

      return HttpResponse.json({
        expiresAt: Math.floor(Date.now() / 1000) + 600,
        uploads: body.files.map((file) => ({ path: file.path })),
      });
    },
  ),
  http.put(
    `${RELEASE_API}/v3/releases/:slug/source-maps`,
    async ({ request }) => {
      uploads.set(
        new URL(request.url).searchParams.get("path") ?? "",
        await request.text(),
      );

      return new HttpResponse(null);
    },
  ),
  http.post(
    `${RELEASE_API}/v3/releases/:slug/source-maps/finalize`,
    async ({ request }) => {
      const body = (await request.json()) as {
        files: { path: string; debugId: string }[];
      };

      finalized.push(...body.files);

      return HttpResponse.json({ ok: true, fileCount: body.files.length });
    },
  ),
);

export function startReleaseApi() {
  server.listen({ onUnhandledRequest: "error" });
}

export function stopReleaseApi() {
  server.close();
}

export function serveSurfaceProviders(providers: {
  destinationProvider: string | null;
  sourceProvider: string | null;
}) {
  server.use(
    http.get(`${RELEASE_API}/v3/releases/config`, () =>
      HttpResponse.json({
        org: { id: "org_1", slug: "acme" },
        surface: {
          environmentFiltering: false,
          sdkConfig: null,
          id: "surface_1",
          slug: "vite",
          ...providers,
        },
      }),
    ),
  );
}

export function resetRecordedRequests() {
  server.resetHandlers();

  uploads.clear();

  finalized.length = 0;
}

export function stubReleaseEnv() {
  vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "interfere_pk_test");

  vi.stubEnv("INTERFERE_API_KEY", "interfere_ak_test");

  vi.stubEnv("INTERFERE_API_URL", RELEASE_API);

  vi.stubEnv("INTERFERE_ENVIRONMENT", "production");

  resetRecordedRequests();
}
