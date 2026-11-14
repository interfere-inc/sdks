/* oxlint-disable unicorn/no-useless-promise-resolve-reject */
"use client";

import type { IdentifyParams } from "@interfere/types/sdk/identify";
import type {
  ConsentInput,
  ConsentState,
} from "@interfere/types/sdk/plugins/manifest";

import { createContext } from "react";

import type { Kernel } from "./kernel.js";

export interface InterfereContextValue {
  consent: {
    get: () => ConsentState | null;
    set: (state?: ConsentInput) => void;
  };
  device: {
    getDeviceId: () => string | null;
    getFpHash: () => string | null;
  };
  identity: {
    clear: () => void;
    get: () => IdentifyParams | null;
    set: (params: IdentifyParams) => Promise<void>;
  };
  /**
   * The active kernel, or `null` when the SDK isn't initialized yet — for
   * example during server-side prerender, when `init()` is client-only.
   * Accessor objects (consent/device/identity/session) above stay safe to
   * call in either state; they no-op when the kernel is null.
   */
  kernel: Kernel | null;
  session: {
    getId: () => string | null;
    getWindowId: () => string | null;
  };
}

// oxlint-disable-next-line eslint/no-empty-function
const NULL_CONSENT = { get: () => null, set: () => {} };

const NULL_DEVICE = { getDeviceId: () => null, getFpHash: () => null };

const NULL_IDENTITY = {
  // oxlint-disable-next-line eslint/no-empty-function
  clear: () => {},
  get: () => null,
  set: async () => Promise.resolve(),
};

const NULL_SESSION = { getId: () => null, getWindowId: () => null };

/**
 * Context value used when the provider is mounted without a kernel — every
 * accessor returns `null` and mutations are no-ops. Lets `useInterfere()`
 * work seamlessly across SSR/CSR boundaries (server gets nulls, client
 * gets real values once the kernel resolves).
 */
export const NULL_CONTEXT_VALUE: InterfereContextValue = {
  consent: NULL_CONSENT,
  device: NULL_DEVICE,
  identity: NULL_IDENTITY,
  kernel: null,
  session: NULL_SESSION,
};

/**
 * Lives in its own module so `provider.tsx` and `internal/capture-boundary.tsx`
 * can both import it without forming an import cycle.
 */
export const InterfereContext = createContext<InterfereContextValue | null>(
  null,
);
