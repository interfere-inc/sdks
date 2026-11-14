import type { ErrorMechanism } from "@interfere/types/sdk/plugins/payload/errors";

export interface CaptureErrorContext {
  readonly mechanism?: ErrorMechanism;
  /**
   * Optional request method / path for the captured error. Stamped
   * onto the OTel exception event as semconv `http.request.method`
   * / `url.path` so dashboards can slice server errors by route.
   */
  readonly request?: {
    method?: string;
    path?: string;
    /** Only x-forwarded-for is captured; privacy.clientIp can disable it. */
    headers?: Headers;
  };
}
