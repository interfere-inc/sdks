/* oxlint-disable react-perf/jsx-no-new-object-as-prop */
"use client";

import { useContext, useLayoutEffect } from "react";
import type { PropsWithChildren } from "react";

import { CaptureBoundary } from "./internal/capture/capture-boundary.js";
import type { Kernel } from "./internal/kernel.js";
import type { InterfereContextValue } from "./internal/react-context.js";
import {
  InterfereContext,
  NULL_CONTEXT_VALUE,
} from "./internal/react-context.js";
import type { PluginOverrides } from "./plugins/lib/loader.js";

interface InterfereProviderProps extends PropsWithChildren {
  /** Reports render errors and rethrows to the host boundary. Defaults to true. */
  captureRenderErrors?: boolean;
  /** Active kernel returned by init(); pass null before initialization or during server rendering. */
  kernel: Kernel | null;
  /** Per-plugin overrides merged over defaults; replay accepts masking options, while privacy classes always apply. */
  plugins?: PluginOverrides | undefined;
}

export function InterfereProvider({
  kernel,
  children,
  plugins,
  captureRenderErrors,
}: InterfereProviderProps) {
  useLayoutEffect(() => {
    kernel?.syncProviderConfig({ plugins });
  }, [kernel, plugins]);

  const value: InterfereContextValue = kernel
    ? {
        consent: kernel.consent,
        device: kernel.device,
        identity: kernel.identity,
        kernel,
        session: kernel.session,
      }
    : NULL_CONTEXT_VALUE;

  const body =
    (captureRenderErrors ?? true) ? (
      <CaptureBoundary>{children}</CaptureBoundary>
    ) : (
      children
    );

  return <InterfereContext value={value}>{body}</InterfereContext>;
}

export function useInterfere() {
  const ctx = useContext(InterfereContext);

  if (!ctx) {
    throw new Error("useInterfere must be used within <InterfereProvider>");
  }

  return ctx;
}

export function useSession() {
  "use no memo";

  return useInterfere().session.getId();
}

export type { InterfereContextValue } from "./internal/react-context.js";

export { InterfereContext } from "./internal/react-context.js";

export type { PluginOverrides } from "./plugins/lib/loader.js";

export type {
  PageEventsConfig,
  PageEventsOption,
} from "./plugins/page-events-config.js";

export type { ReplayConfig, ReplayOption } from "./plugins/replay/config.js";
