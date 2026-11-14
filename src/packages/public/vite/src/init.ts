import type { KernelOptions } from "@interfere/react/internal/kernel";
import { createWrapperSingleton } from "@interfere/react/internal/wrapper-singleton";

import { applyBrowserGlobals } from "./internal/browser-globals";
import { PRODUCER_VERSION } from "./version";

export { capture, span } from "@interfere/react/api";

export type { Kernel, KernelOptions } from "@interfere/react/internal/kernel";

export type {
  ConsentCategory,
  ConsentInput,
  ConsentState,
  GateableCategory,
} from "@interfere/types/sdk/plugins/manifest";

const wrapper = createWrapperSingleton({
  initEntryName: "Vite",
  producerVersion: PRODUCER_VERSION,
});

export async function init(opts?: KernelOptions) {
  if (import.meta.env.VITE_INTERFERE_DISABLED) {
    return null;
  }

  applyBrowserGlobals();

  return wrapper.init(opts);
}

export const {
  close,
  consent,
  getKernel,
  getKernelOrNull,
  identity,
  subscribeToKernel,
} = wrapper;
