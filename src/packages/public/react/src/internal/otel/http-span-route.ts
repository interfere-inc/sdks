import type { Span } from "@opentelemetry/api";

import { normalizePath } from "./normalize-path.js";
import { ATTR_HTTP_ROUTE } from "./url-attrs.js";

export function stampHttpSpanRoute(
  span: Span,
  method: string,
  pathname: string,
  resolveRoute: ((pathname: string) => string | undefined) | undefined,
) {
  const template = resolveRoute?.(pathname);
  const route = template ?? normalizePath(pathname);

  span.setAttribute(ATTR_HTTP_ROUTE, route);

  span.setAttribute(
    "interfere.route.source",
    template ? "template" : "normalized",
  );

  span.updateName(`${method} ${route}`);
}
