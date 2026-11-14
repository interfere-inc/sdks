import { trace } from "@opentelemetry/api";

export const dynamic = "force-dynamic";

const ECHOED = [
  "traceparent",
  "tracestate",
  "baggage",
  "x-vendor-trace-id",
  "x-vendor-parent-id",
  "x-vendor-sampling-priority",
] as const;

export function GET(request: Request) {
  const active = trace.getActiveSpan()?.spanContext();

  return Response.json({
    case: new URL(request.url).searchParams.get("case"),
    headers: Object.fromEntries(
      ECHOED.map((name) => [name, request.headers.get(name)]),
    ),
    server: active
      ? { traceFlags: active.traceFlags, traceId: active.traceId }
      : null,
  });
}
