"use client";

import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { InterfereProvider as CoreInterfereProvider } from "@interfere/react/provider";
import type { PluginOverrides } from "@interfere/react/provider";

import { useSyncExternalStore } from "react";
import type { PropsWithChildren } from "react";

import { getKernelOrNull, subscribeToKernel } from "./instrument-client.js";

const nullSnapshot = () => null;

interface InterfereProviderProps extends PropsWithChildren {
  /** Reports render errors and rethrows to the host boundary. Defaults to true. */
  captureRenderErrors?: boolean;
  plugins?: PluginOverrides | undefined;
}

/**
 * Next.js wrapper around `@interfere/react`'s provider that resolves the
 * kernel from module scope (set by the matching `init()` call). Customer
 * code never passes the kernel explicitly. Uses `getKernelOrNull()` so the
 * provider is safe to render during SSR/SSG when `init()` hasn't run
 * (`init()` is client-only); the core provider mounts with null-safe
 * accessors and switches to the real kernel on client hydration.
 */
export function InterfereProvider({
  children,
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
  ConsentDecision,
  ConsentState,
  GateableCategory,
} from "@interfere/types/sdk/plugins/manifest";
