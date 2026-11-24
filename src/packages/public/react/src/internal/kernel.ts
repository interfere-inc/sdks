import { Kernel } from "./kernel/kernel.js";
import { readBakedPlugins } from "@interfere/types/sdk/remote-config/plugins";

import { resolveRemoteConfig } from "./config/remote-config.js";
import { installViewNavigation } from "./otel/navigation.js";
import { PluginRuntime } from "./plugins/plugin-runtime.js";
import type { KernelInternalOptions } from "./kernel/options.js";
import {
  applyRemoteStartup,
  prepareKernel,
  startRemotePlugins,
} from "./kernel/startup.js";
import { resolveTracingOptions } from "./kernel/tracing.js";

export type {
  CaptureFailedRequestsOption,
  CaptureFailedRequestsOptions,
  HttpStatusCodeRange,
} from "./otel/instrumentations/instrumentations.js";

export type { PrivacyOptions } from "./dom/privacy.js";

export type { RecordExceptionOpts } from "./capture/exception-span.js";

export type {
  KernelConsent,
  KernelDevice,
  KernelIdentity,
  KernelSession,
} from "./kernel/handles.js";

export type {
  KernelInternalOptions,
  KernelOptions,
  TracingOptions,
} from "./kernel/options.js";

export { buildSdkStack, readReleaseSlugFromGlobals } from "./kernel/globals.js";

export { Kernel } from "./kernel/kernel.js";

interface CreateKernelInput {
  onCreated?: (kernel: Kernel) => void;
  opts?: KernelInternalOptions;
}

export async function createKernel(input: CreateKernelInput = {}) {
  const { opts = {} } = input;
  const { fetcher, service, serviceName, tracker } = prepareKernel(opts);

  const kernel = new Kernel(tracker, opts.privacy, service, {
    beforeSend: opts.beforeSend,
    denyUrls: opts.denyUrls,
    errors: opts.errors,
    ignoreErrors: opts.ignoreErrors,
  });

  const baked = readBakedPlugins();
  const runtime = new PluginRuntime(kernel, opts.consent, baked ?? {}, true);

  kernel.attachRuntime(runtime);

  kernel.holdStartupExceptions();

  const disposeStartupNavigation = installViewNavigation(kernel.views);

  try {
    input.onCreated?.(kernel);

    runtime.start();

    const {
      plugins: remotePlugins,
      anonymousReplayPolicy,
      backendHosts,
      protocolVersion,
    } = await resolveRemoteConfig(opts.apiHost, fetcher, baked, service);

    const tracingInput = applyRemoteStartup({
      anonymousReplayPolicy,
      disposeStartupNavigation,
      kernel,
      opts,
      protocolVersion,
      service,
      serviceName,
      tracker,
    });

    if (opts.tracing !== false) {
      const { wireOtel } = await import("./otel/index.js");

      const resolvedTracing = resolveTracingOptions(
        opts,
        serviceName,
        backendHosts,
      );

      disposeStartupNavigation();

      wireOtel({ ...tracingInput, tracing: resolvedTracing });
    }

    startRemotePlugins(runtime, remotePlugins);

    kernel.releaseStartupExceptions();

    return kernel;
  } catch (error) {
    disposeStartupNavigation();

    await kernel.dispose();

    throw error;
  }
}

export { isEnabledByEnvironment } from "./environment/environment.js";
