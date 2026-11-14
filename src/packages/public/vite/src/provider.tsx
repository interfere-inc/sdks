import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { InterfereProvider as CoreInterfereProvider } from "@interfere/react/provider";
import type { PluginOverrides } from "@interfere/react/provider";
import type { ConsentInput } from "@interfere/types/sdk/plugins/manifest";

import { useSyncExternalStore } from "react";
import type { PropsWithChildren } from "react";

import { getKernelOrNull, subscribeToKernel } from "./init";

const nullSnapshot = () => null;

interface InterfereProviderProps extends PropsWithChildren {
  consent?: ConsentInput | undefined;
  /** Reports render errors and rethrows to the host boundary. Defaults to true. */
  captureRenderErrors?: boolean;
  plugins?: PluginOverrides | undefined;
}

/**
 * Vite wrapper around `@interfere/react`'s provider that reads the
 * kernel created by `@interfere/vite/init`.
 */
export function InterfereProvider({
  children,
  consent,
  plugins,
  captureRenderErrors,
}: InterfereProviderProps) {
  const kernel = useSyncExternalStore(
    subscribeToKernel,
    getKernelOrNull,
    nullSnapshot,
  );

  return (
    <CoreInterfereProvider
      kernel={kernel}
      {...omitUndefined({
        captureRenderErrors,
        consent,
        plugins,
      })}
    >
      {children}
    </CoreInterfereProvider>
  );
}

export { useInterfere, useSession } from "@interfere/react/provider";

export type {
  ConsentCategory,
  ConsentInput,
  ConsentState,
  GateableCategory,
} from "@interfere/types/sdk/plugins/manifest";
