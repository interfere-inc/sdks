import type { Interfere } from "@interfere/javascript";

import {
  mkdirSync,
  mkdtempSync,
  realpathSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { afterAll, afterEach, beforeAll, vi } from "vitest";
import { http } from "msw";
import { setupServer } from "msw/node";

import { tmpdir } from "node:os";
import path from "node:path";

export const releaseNetwork = setupServer();

export const releaseRequests: Request[] = [];

export const sdkConfigRequests: Request[] = [];

releaseNetwork.events.on("request:start", ({ request }) => {
  const isSdkConfig = new URL(request.url).pathname.endsWith("/v3/config");

  (isSdkConfig ? sdkConfigRequests : releaseRequests).push(request.clone());
});

export function capturedReleaseRequest(index: number) {
  const request = releaseRequests[index];

  if (!request) {
    throw new Error(`Missing release request ${index}`);
  }

  return request;
}

beforeAll(() => releaseNetwork.listen({ onUnhandledRequest: "error" }));

afterEach(() => {
  releaseNetwork.resetHandlers();

  releaseRequests.length = 0;

  sdkConfigRequests.length = 0;
});

afterAll(() => releaseNetwork.close());

export const RELEASE_SLUG = "rel_abc123def4567890";

export const RELEASE_RESPONSE = {
  build: { hash: "abc123def456" },
  destination: {
    environment: "production",
    id: "dst_123",
    slug: RELEASE_SLUG,
  },
  org: { id: "org_123", slug: "acme" },
  source: { id: "github" },
  surface: { id: "srf_123", slug: "frontend" },
} satisfies Interfere.CreateReleaseResponse;

export function parseFetchCall(call: Parameters<typeof fetch> | []) {
  const [input, init] = call;

  if (input === undefined) {
    throw new Error("Expected a recorded fetch call");
  }

  if (input instanceof Request) {
    return input;
  }

  return new Request(input, init) satisfies Request;
}

export async function withTempProject(
  prefix: string,
  run: (projectDir: string) => void | Promise<void>,
) {
  const originalDirectory = process.cwd();
  const projectDir = realpathSync(mkdtempSync(path.join(tmpdir(), prefix)));

  try {
    await run(projectDir);
  } finally {
    process.chdir(originalDirectory);

    rmSync(projectDir, { force: true, recursive: true });
  }
}

export function createReleaseFixture(
  projectDir: string,
  options: {
    failure?:
      | "config"
      | "register"
      | "sign"
      | "upload"
      | "finalize"
      | "timeout"
      | "incomplete";
    destinationProvider?: "cli" | "cloudflare" | "vercel" | null;
    sourceProvider?: "github" | null;
    cleanupFailure?: boolean;
    telemetryFailure?: boolean;
    maps?: boolean;
  } = {},
) {
  const chunk = path.join(projectDir, ".next/static/chunks/app.js");
  const map = `${chunk}.map`;

  mkdirSync(path.dirname(chunk), { recursive: true });

  if (options.maps !== false) {
    writeFileSync(
      chunk,
      "console.log('app');\n//# sourceMappingURL=app.js.map",
    );

    writeFileSync(
      map,
      JSON.stringify({ mappings: "", sources: ["app.tsx"], version: 3 }),
    );
  }

  releaseNetwork.use(
    http.all("https://in.example.com/*", async ({ request }) => {
      const url = new URL(request.url);

      const fail = () =>
        Response.json(
          { message: "original pipeline failure" },
          { status: 422 },
        );

      if (url.pathname.endsWith("/config")) {
        return options.failure === "config"
          ? fail()
          : Response.json({
              org: { id: "org_123", slug: "acme" },
              surface: {
                destinationProvider:
                  options.destinationProvider === undefined
                    ? "cli"
                    : options.destinationProvider,
                environmentFiltering: true,
                id: "srf_123",
                sdkConfig: null,
                slug: "frontend",
                sourceProvider:
                  options.sourceProvider === undefined
                    ? "github"
                    : options.sourceProvider,
              },
            });
      }

      if (url.pathname === "/v3/releases") {
        return options.failure === "register"
          ? fail()
          : Response.json(RELEASE_RESPONSE);
      }

      if (url.pathname.endsWith("/sign")) {
        return options.failure === "sign"
          ? fail()
          : Response.json({
              expiresAt:
                Math.floor(Date.now() / 1000) +
                (options.failure === "timeout" ? 2 : 600),
              uploads: [{ path: "_next/static/chunks/app.js.map" }],
            });
      }

      if (request.method === "PUT") {
        return options.failure === "upload"
          ? fail()
          : new Response(null, { status: 200 });
      }

      if (url.pathname.endsWith("/finalize")) {
        if (options.cleanupFailure) {
          rmSync(chunk);

          mkdirSync(chunk);
        }

        if (options.failure === "timeout") {
          return await new Promise<Response>((_resolve, reject) => {
            request.signal.addEventListener(
              "abort",
              () =>
                reject(
                  new DOMException("Finalization timed out", "TimeoutError"),
                ),
              { once: true },
            );
          });
        }

        return options.failure === "finalize"
          ? fail()
          : Response.json({
              fileCount: options.failure === "incomplete" ? 0 : 1,
              ok: true,
            });
      }

      if (url.pathname.endsWith("/telemetry")) {
        return options.telemetryFailure
          ? Response.json(
              { message: "telemetry reporting failed" },
              { status: 400 },
            )
          : Response.json({ ok: true });
      }

      throw new Error(`Unexpected request: ${request.method} ${request.url}`);
    }),
  );

  const fetcher = vi.spyOn(globalThis, "fetch");

  return { chunk, map, fetcher };
}
