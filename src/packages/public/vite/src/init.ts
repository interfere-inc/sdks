import { createWrapperSingleton } from "@interfere/react/internal/wrapper-singleton";
import type { WrapperSingleton } from "@interfere/react/internal/wrapper-singleton";

import { applyBrowserGlobals } from "./internal/browser-globals";
import { PRODUCER_VERSION } from "./version";

export { capture, span } from "@interfere/react/api";

export type { Kernel, KernelOptions } from "@interfere/react/internal/kernel";

export type {
  ConsentCategory,
  ConsentDecision,
  ConsentState,
  GateableCategory,
} from "@interfere/types/sdk/plugins/manifest";

const wrapper = createWrapperSingleton({
  initEntryName: "Vite",
  producerVersion: PRODUCER_VERSION,
});

export const init: WrapperSingleton["init"] = async (opts) => {
  if (import.meta.env.VITE_INTERFERE_DISABLED) {
    return null;
  }

  applyBrowserGlobals();

  return wrapper.init(opts);
};

export const close: WrapperSingleton["close"] = wrapper.close;

export const consent: WrapperSingleton["consent"] = wrapper.consent;

export const getKernel: WrapperSingleton["getKernel"] = wrapper.getKernel;

export const getKernelOrNull: WrapperSingleton["getKernelOrNull"] =
  wrapper.getKernelOrNull;

export const identity: WrapperSingleton["identity"] = wrapper.identity;

export const subscribeToKernel: WrapperSingleton["subscribeToKernel"] =
  wrapper.subscribeToKernel;
