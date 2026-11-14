/* oxlint-disable typescript/no-deprecated */
import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";

import type { NextMiddleware, NextRequest } from "next/server";

import { containAsync } from "@interfere/server-core/contain";

import { captureError, rememberProxyThrow } from "../capture/capture.js";
import { flushServerTelemetry } from "../exporters/server-flush.js";

/**
 * Wraps a Next.js proxy (formerly `middleware`) so thrown errors are captured
 * by Interfere (mechanism `auto.function.nextjs.proxy`, with request method +
 * path) and then re-thrown unchanged. Next also reports proxy errors to
 * `onRequestError`; that report is skipped for a throw the wrapper already
 * captured, so each throw records one exception. The wrapper is optional when
 * `onRequestError` is exported from `instrumentation.ts`.
 *
 * ```ts
 * // proxy.ts
 * import { withInterfereProxy } from "@interfere/next/server";
 *
 * export default withInterfereProxy((request) => {
 *   // ...your proxy
 * });
 * ```
 */
export function withInterfereProxy(proxy: NextMiddleware) {
  // oxlint-disable-next-line typescript/promise-function-async
  return ((request, event) => {
    try {
      const result = proxy(request, event);

      if (result instanceof Promise) {
        // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Preserve synchronous middleware returns and rethrow arbitrary JavaScript promise rejection values unchanged.
        return result.catch((error: unknown) => {
          reportProxyError(error, request);

          throw error;
        });
      }

      return result;
    } catch (error) {
      reportProxyError(error, request);

      throw error;
    }
  }) satisfies NextMiddleware;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Middleware can throw any JavaScript value; capture normalizes it while the wrapper rethrows the original identity.
function reportProxyError(error: unknown, request: NextRequest) {
  rememberProxyThrow(error);

  captureError(error, request, {
    mechanism: { handled: false, type: MECHANISM_TYPE.nextjs.proxy },
    nextjs: {
      requestMethod: request.method,
      requestPath: request.nextUrl.pathname,
      routeType: "proxy",
    },
  });

  void containAsync("withInterfereProxy()", flushServerTelemetry);
}
