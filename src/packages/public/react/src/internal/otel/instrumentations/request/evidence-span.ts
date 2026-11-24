import { ROOT_CONTEXT, trace } from "@opentelemetry/api";
import type { Span, Tracer } from "@opentelemetry/api";
import { stampHttpSpanRoute } from "../../http-span-route.js";

export function requestEvidenceSpan(
  parent: Span,
  tracer: Tracer,
  method: string,
  url: string,
  resolveRoute?: (pathname: string) => string | undefined,
) {
  const context = trace.setSpanContext(ROOT_CONTEXT, parent.spanContext());
  const { pathname } = new URL(url);

  return () => {
    const span = tracer.startSpan(
      "HTTP request evidence",
      {
        attributes: {
          "url.full": url,
          "url.path": pathname,
          "http.request.method": method,
        },
      },
      context,
    );

    stampHttpSpanRoute(span, method, pathname, resolveRoute);

    return span;
  };
}
