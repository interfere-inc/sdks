import type { NextjsContext } from "@interfere/types/sdk/plugins/context/next";
import type { ErrorMechanism } from "@interfere/types/sdk/plugins/payload/errors";

import type { ServerActionMarker } from "./actions/server-action-marker.js";

export type OnRequestErrorContext = Omit<
  NextjsContext,
  "errorDigest" | "requestMethod" | "requestPath" | "runtime"
>;

export interface CaptureErrorContext {
  readonly mechanism?: ErrorMechanism;
  readonly nextjs?: Omit<NextjsContext, "runtime">;
  readonly requestHeaders?: Readonly<Record<string, string>>;
  readonly serverAction?: ServerActionMarker;
}
