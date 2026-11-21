import { FetchInterceptor } from "@mswjs/interceptors/fetch/web";
import { InterceptorSource, defineNetwork } from "msw/experimental";
import type { HttpHandler } from "msw";

export function fetchNetwork(...handlers: HttpHandler[]) {
  return defineNetwork({
    handlers,
    context: { quiet: true },
    sources: [
      new InterceptorSource({ interceptors: [new FetchInterceptor()] }),
    ],
    onUnhandledFrame: "error",
  });
}
