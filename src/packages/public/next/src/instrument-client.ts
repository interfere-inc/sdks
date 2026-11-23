import type { KernelOptions } from "@interfere/react/internal/kernel";
import { createWrapperSingleton } from "@interfere/react/internal/wrapper-singleton";
import type { WrapperSingleton } from "@interfere/react/internal/wrapper-singleton";

import {
  buildIntegrationRan,
  hasEnvCredentials,
  interfereDisabled,
  warnBuildIntegrationDegraded,
  warnBuildIntegrationMissing,
} from "./internal/build-marker-guard.js";
import { PRODUCER_VERSION } from "./internal/version.js";

export { capture, span } from "@interfere/react/api";

export type { Kernel, KernelOptions } from "@interfere/react/internal/kernel";

export type {
  ConsentCategory,
  ConsentDecision,
  ConsentState,
  GateableCategory,
} from "@interfere/types/sdk/plugins/manifest";

const wrapper = createWrapperSingleton({
  initEntryName: "instrumentation-client",
  pinConsole: true,
  producerVersion: PRODUCER_VERSION,
});

export const close: WrapperSingleton["close"] = wrapper.close;

export const consent: WrapperSingleton["consent"] = wrapper.consent;

export const getKernel: WrapperSingleton["getKernel"] = wrapper.getKernel;

export const getKernelOrNull: WrapperSingleton["getKernelOrNull"] =
  wrapper.getKernelOrNull;

export const identity: WrapperSingleton["identity"] = wrapper.identity;

export const subscribeToKernel: WrapperSingleton["subscribeToKernel"] =
  wrapper.subscribeToKernel;

export async function init(opts?: KernelOptions) {
  if (buildIntegrationRan()) {
    return initKernel(opts);
  }

  return initAfterHoistedImports(opts);
}

async function initKernel(opts?: KernelOptions) {
  return interfereDisabled() ? null : wrapper.init(opts);
}

async function initAfterHoistedImports(opts?: KernelOptions) {
  await Promise.resolve();

  if (!buildIntegrationRan()) {
    const { promise, resolve } = Promise.withResolvers<undefined>();

    setTimeout(resolve, 0);

    await promise;
  }

  if (buildIntegrationRan()) {
    return initKernel(opts);
  }

  if (hasEnvCredentials()) {
    warnBuildIntegrationDegraded();

    return wrapper.init(opts);
  }

  warnBuildIntegrationMissing();

  return null;
}
