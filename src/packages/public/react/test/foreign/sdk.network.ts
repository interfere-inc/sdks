import { HttpResponse, delay, http } from "msw";

export const TRACKED_HEADERS = [
  "traceparent",
  "tracestate",
  "baggage",
  "x-vendor-trace-id",
  "x-vendor-parent-id",
  "x-vendor-sampling-priority",
  "b3",
  "x-b3-traceid",
  "x-b3-spanid",
  "x-b3-sampled",
] as const;

export interface WireRequest {
  url: string;
  case: string | null;
  headers: Record<string, string | null>;
  method: string;
  origin: string;
  path: string;
}

const wire: WireRequest[] = [];

export function wireFor(label: string) {
  return wire.filter((entry) => entry.case === label);
}

function corsHeaders(request: Request) {
  return new Headers({
    "access-control-allow-origin": request.headers.get("origin") ?? "*",
    "access-control-allow-methods": "GET,POST,OPTIONS",
    "access-control-allow-headers":
      request.headers.get("access-control-request-headers") ?? "*",
  });
}

export const handlers = [
  http.all("*", async ({ request }) => {
    const url = new URL(request.url);
    const cors = corsHeaders(request);

    if (url.pathname.startsWith("/ingest")) {
      return HttpResponse.json({}, { headers: cors });
    }

    if (request.method === "OPTIONS") {
      return new HttpResponse(null, { headers: cors, status: 204 });
    }

    wire.push({
      url: url.href,
      case: url.searchParams.get("case"),
      headers: Object.fromEntries(
        TRACKED_HEADERS.map((name) => [name, request.headers.get(name)]),
      ),
      method: request.method,
      origin: url.origin,
      path: url.pathname,
    });

    if (url.pathname === "/slow") {
      await delay(1200);
    }

    return HttpResponse.json({ ok: true }, { headers: cors });
  }),
];
