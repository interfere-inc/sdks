import { HttpResponse, getResponse, http } from "msw";
import { buffer } from "node:stream/consumers";
import { once } from "node:events";
import { readFile } from "node:fs/promises";
import { createServer } from "node:http";
import type { IncomingMessage, ServerResponse } from "node:http";
import { extname, resolve as resolvePath, sep } from "node:path";
import { setTimeout } from "node:timers/promises";
import { gunzipSync } from "node:zlib";

export interface ReceivedRequest {
  method: string;
  url: string;
  headers: Headers;
  body: string;
  status: number;
  response?: string;
}

export async function receiver(
  options: {
    readonly sdkConfig?: Readonly<Record<string, string>>;
    readonly respond?: (
      request: ReceivedRequest,
      body: Uint8Array,
    ) => Response | undefined;
  } = {},
) {
  const requests: ReceivedRequest[] = [];

  const arrivals = new EventTarget();

  const failures = new Map<
    string,
    { status: number; remaining: number; body: string }
  >();

  const stalls = new Map<string, () => Promise<void>>();

  let assets: string | undefined;
  let upstream: ((request: Request) => Promise<Response>) | undefined;

  async function resolveResponse(
    request: Pick<Request, "url" | "method" | "headers">,
    incoming: IncomingMessage,
  ) {
    const url = new URL(request.url);
    const { headers } = request;

    const responseHeaders = new Headers({
      "access-control-allow-origin": "*",
      "access-control-allow-headers":
        headers.get("access-control-request-headers") ?? "*",
      "access-control-allow-methods": "GET, POST, OPTIONS",
    });

    try {
      if (request.method === "OPTIONS") {
        return new HttpResponse(null, {
          status: 204,
          headers: responseHeaders,
        });
      }

      if (
        assets &&
        request.method === "GET" &&
        !url.pathname.startsWith("/v3/")
      ) {
        const file = resolvePath(
          assets,
          `.${url.pathname === "/" ? "/index.html" : url.pathname}`,
        );

        if (!file.startsWith(`${resolvePath(assets)}${sep}`)) {
          return new HttpResponse(null, {
            status: 403,
            headers: responseHeaders,
          });
        }

        responseHeaders.set(
          "content-type",
          extname(file) === ".html" ? "text/html" : "text/javascript",
        );

        return new HttpResponse(await readFile(file), {
          headers: responseHeaders,
        });
      }

      const stall = [...stalls].find(([prefix]) =>
        url.pathname.startsWith(prefix),
      )?.[1];

      if (stall !== undefined) {
        incoming.pause();

        await stall();

        incoming.resume();
      }

      const raw = await buffer(incoming);

      const decoded =
        headers.get("content-encoding") === "gzip" ? gunzipSync(raw) : raw;

      const body = decoded.toString();

      const failure = [...failures].find(([prefix]) =>
        url.pathname.startsWith(prefix),
      )?.[1];

      const entry: ReceivedRequest = {
        method: request.method,
        url: url.pathname + url.search,
        headers,
        body,
        status: 200,
      };

      requests.push(entry);

      arrivals.dispatchEvent(new Event("request"));

      if (failure && failure.remaining > 0) {
        failure.remaining -= 1;

        entry.status = failure.status;

        responseHeaders.set("content-type", "application/json");

        return new HttpResponse(
          [204, 205, 304].includes(failure.status) ? null : failure.body,
          {
            status: failure.status,
            headers: responseHeaders,
          },
        );
      }

      if (upstream) {
        const init: RequestInit = { method: entry.method, headers };

        if (raw.length > 0) {
          init.body = raw;
        }

        entry.status = 500;

        const result = await upstream(new Request(url, init));

        entry.status = result.status;

        for (const [key, value] of result.headers) {
          responseHeaders.set(key, value);
        }

        const payload =
          result.body === null ? null : await result.arrayBuffer();

        if ((result.status < 200 || result.status >= 300) && payload) {
          entry.response = new TextDecoder().decode(payload);
        }

        return new HttpResponse(payload, {
          status: result.status,
          headers: responseHeaders,
        });
      }

      const custom = options.respond?.(entry, decoded);

      if (custom) {
        entry.status = custom.status;

        for (const [key, value] of responseHeaders) {
          custom.headers.set(key, value);
        }

        return custom;
      }

      if (url.pathname === "/v3/releases/config") {
        return HttpResponse.json(
          {
            org: { id: "org_acceptance", slug: "acceptance" },
            surface: {
              id: "surface_acceptance",
              slug: "web",
              sourceProvider: "github",
              destinationProvider: "cli",
              environmentFiltering: true,
              sdkConfig: null,
            },
          },
          { headers: responseHeaders },
        );
      }

      if (entry.method === "POST" && url.pathname === "/v3/releases") {
        return HttpResponse.json(
          {
            build: { hash: "0123456789abcdef0123456789abcdef01234567" },
            destination: {
              environment: "preview",
              id: "release_acceptance",
              slug: "rel_0123456789abcdef",
            },
            org: { id: "org_acceptance", slug: "acceptance" },
            source: { id: "source_acceptance" },
            surface: { id: "surface_acceptance", slug: "web" },
            token: "irt_receiver_acceptance",
          },
          { headers: responseHeaders },
        );
      }

      return HttpResponse.json(
        url.pathname.includes("config")
          ? {
              plugins: {
                errors: true,
                logs: true,
                device: true,
                pageEvents: true,
                rageClick: true,
                replay: true,
              },
              ...options.sdkConfig,
            }
          : {},
        { headers: responseHeaders },
      );
    } catch (error) {
      return new HttpResponse(new TextEncoder().encode(String(error)), {
        status: 500,
        headers: responseHeaders,
      });
    }
  }

  async function handle(request: IncomingMessage, response: ServerResponse) {
    try {
      const headers = new Headers();

      for (const [key, value] of Object.entries(request.headers)) {
        if (value !== undefined) {
          headers.set(key, Array.isArray(value) ? value.join(", ") : value);
        }
      }

      const method = request.method ?? "GET";
      const init: RequestInit = { method, headers };

      const resolved = await getResponse(
        [
          http.all("*", async ({ request: intercepted }) =>
            resolveResponse(intercepted, request),
          ),
        ],
        new Request(new URL(request.url ?? "/", "http://localhost"), init),
      );

      if (!resolved) {
        throw new Error("Missing telemetry receiver response");
      }

      for (const [key, value] of resolved.headers) {
        response.setHeader(key, value);
      }

      response
        .writeHead(resolved.status)
        .end(Buffer.from(await resolved.arrayBuffer()));
    } catch (error) {
      response.destroy(
        error instanceof Error ? error : new Error(String(error)),
      );
    }
  }

  const server = createServer((request, response) => {
    void handle(request, response);
  });

  server.listen(0, "127.0.0.1");

  await once(server, "listening");

  const address = server.address();

  if (!(address instanceof Object)) {
    throw new TypeError("Receiver did not bind a TCP port");
  }

  return {
    url: `http://127.0.0.1:${address.port}`,
    requests,
    serve(directory: string) {
      assets = directory;
    },
    forward(handler: (request: Request) => Promise<Response>) {
      upstream = handler;
    },
    reset() {
      requests.length = 0;

      failures.clear();

      stalls.clear();
    },
    fail(path: string, status: number, attempts = 1, body = "{}") {
      failures.set(path, { status, remaining: attempts, body });
    },
    stall(path: string, ms: number) {
      stalls.set(path, async () => setTimeout(ms));
    },
    async arrived(path: string, count = 1) {
      const matching = () =>
        requests.filter((request) => request.url.startsWith(path));

      while (matching().length < count) {
        // oxlint-disable-next-line eslint/no-await-in-loop -- Each wake-up re-reads the history, so a request recorded between two waits is never missed.
        await once(arrivals, "request");
      }

      return matching();
    },
    hold(path: string, until: Promise<unknown>) {
      stalls.set(path, async () => {
        await until;
      });
    },
    async close() {
      server.closeAllConnections();

      await new Promise<void>((resolve, reject) => {
        server.close((error) => {
          if (error) {
            reject(error);
          } else {
            resolve();
          }
        });
      });
    },
  };
}
