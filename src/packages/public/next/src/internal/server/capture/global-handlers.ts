import { installGlobalHandlers as installProcessHandlers } from "@interfere/server-core/global-handlers";
import type { GlobalHandlerOptions as ProcessHandlerOptions } from "@interfere/server-core/global-handlers";

import { captureError } from "./capture.js";

export { uninstallGlobalHandlers } from "@interfere/server-core/global-handlers";

export type GlobalHandlerOptions = Omit<ProcessHandlerOptions, "capture">;

export function installGlobalHandlers(options: GlobalHandlerOptions) {
  installProcessHandlers({
    ...options,
    capture: (error, mechanism) => {
      captureError(error, undefined, { mechanism });
    },
  });
}
