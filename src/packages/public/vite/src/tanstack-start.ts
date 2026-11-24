import { createMiddleware } from "@tanstack/react-start";

import {
  captureServerFunctionError,
  withRequestSpan,
} from "./internal/server/request/trace";

export {
  type TraceMetaDescriptor,
  traceMeta,
} from "./internal/server/request/trace";

/** Opens a SERVER span per request; register it in `createStart({ requestMiddleware })`. */
export const interfereRequestMiddleware = createMiddleware().server(
  async ({ next, request }) => withRequestSpan(request, async () => next()),
);

/** Captures `createServerFn` handler throws, which Start serializes to the client; register it in `createStart({ functionMiddleware })`. */
export const interfereFunctionMiddleware = createMiddleware({
  type: "function",
}).server(async ({ next }) => {
  try {
    return await next();
  } catch (error) {
    captureServerFunctionError(error);

    throw error;
  }
});
