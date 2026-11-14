export {
  captureError,
  onRequestError,
} from "./internal/server/capture/capture.js";

export type {
  CaptureErrorContext,
  OnRequestErrorContext,
} from "./internal/server/types.js";

export { withInterfereProxy } from "./internal/server/actions/wrap-proxy.js";

export { withInterfereServerAction } from "./internal/server/actions/wrap-server-action.js";

export { withInterfereAfter } from "./internal/server/actions/wrap-after.js";
