import { omitUndefined } from "@interfere/helpers/omit-undefined";
import type { Attributes, Span } from "@opentelemetry/api";

export const ATTR_HTTP_ROUTE = "http.route" as const;

export const ATTR_URL_FULL = "url.full" as const;

export const ATTR_URL_PATH = "url.path" as const;

export function currentUrlAttrs(
  resolveRoute: ((pathname: string) => string | undefined) | undefined,
  redactUrl: (url: string) => string = (url) => url,
) {
  if (typeof window === "undefined") {
    return {} satisfies Attributes;
  }

  const { href, pathname } = globalThis.location;
  const route = resolveRoute?.(pathname);

  return omitUndefined({
    [ATTR_URL_PATH]: route ?? pathname,
    [ATTR_URL_FULL]: redactUrl(href),
    [ATTR_HTTP_ROUTE]: route,
  }) satisfies Attributes;
}

export function stampUrlAttrs(
  span: Span,
  resolveRoute: ((pathname: string) => string | undefined) | undefined,
) {
  span.setAttributes(currentUrlAttrs(resolveRoute));
}
