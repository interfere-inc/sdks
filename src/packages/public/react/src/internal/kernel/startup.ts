import { getGlobal, getStringGlobal } from "../../util/global.js";
import { createLogger } from "../../util/log.js";
import { SessionTracker } from "../../tracking/api.js";
import {
  FORCE_ENABLE_GLOBAL,
  SDK_STACK_GLOBAL,
} from "@interfere/types/sdk/injected-globals";
import { SERVICE_NAME_GLOBAL } from "@interfere/types/sdk/service-name";
import type { Kernel } from "../kernel.js";
import { buildResourceAttributes } from "../capture/resource-attributes.js";
import { resolveTargets } from "../config/config.js";
import type { resolveRemoteConfig } from "../config/remote-config.js";
import type { PluginRuntime } from "../plugins/plugin-runtime.js";
import { buildSdkStack, readReleaseSlugFromGlobals } from "./globals.js";
import type { KernelInternalOptions } from "./options.js";

const log = createLogger("kernel");

type RemoteConfig = Awaited<ReturnType<typeof resolveRemoteConfig>>;

export function prepareKernel(opts: KernelInternalOptions) {
  const fetcher = opts.fetch ?? globalThis.fetch.bind(globalThis);

  const serviceName = opts.serviceName || getStringGlobal(SERVICE_NAME_GLOBAL);
  const service = { serviceName, serviceNamespace: opts.serviceNamespace };

  const forceEnabled = Boolean(getGlobal(FORCE_ENABLE_GLOBAL));

  if (forceEnabled) {
    log.warn(
      "FORCE_ENABLE is active. Events will be dropped by production ingest. This should only be set during local development.",
    );
  }

  const tracker = new SessionTracker({
    fetcher,
    target: {
      ...resolveTargets(opts.apiHost, undefined, service).ingest,
      baseUrl: () => resolveTargets(opts.apiHost).ingest.baseUrl,
    },
  });

  // oxlint-disable-next-line eslint/no-underscore-dangle -- Published option that other @interfere packages pass across versions.
  const wrapperVersions = opts._wrapperVersions;

  if (wrapperVersions?.length) {
    Object.assign(globalThis, { [SDK_STACK_GLOBAL]: wrapperVersions });
  }

  return { fetcher, service, serviceName, tracker };
}

interface RemoteStartupInput {
  anonymousReplayPolicy: RemoteConfig["anonymousReplayPolicy"];
  disposeStartupNavigation: () => void;
  kernel: Kernel;
  opts: KernelInternalOptions;
  protocolVersion: RemoteConfig["protocolVersion"];
  service: {
    serviceName: string | undefined;
    serviceNamespace: string | undefined;
  };
  serviceName: string | undefined;
  tracker: SessionTracker;
}

export function applyRemoteStartup(input: RemoteStartupInput) {
  const {
    anonymousReplayPolicy,
    disposeStartupNavigation,
    kernel,
    opts,
    protocolVersion,
    service,
    serviceName,
    tracker,
  } = input;

  kernel.remoteProtocolVersion = protocolVersion ?? null;

  kernel.anonymousReplayPolicy = anonymousReplayPolicy;

  const targets = resolveTargets(opts.apiHost, undefined, service);

  log.info("ingest: %s", targets.ingest.baseUrl);

  tracker.start();

  const tracingInput = {
    kernel,
    releaseSlug: readReleaseSlugFromGlobals(),
    // oxlint-disable-next-line eslint/no-underscore-dangle -- Published option that other @interfere packages pass across versions.
    sdkStack: buildSdkStack(opts._wrapperVersions),
    targets,
  };

  if (opts.tracing === false) {
    disposeStartupNavigation();

    kernel.attachCaptureOnlySink({
      clientOptions: targets.ingest,
      resource: buildResourceAttributes({
        ...tracingInput,
        serviceName,
        serviceNamespace: opts.serviceNamespace,
      }),
    });
  }

  return tracingInput;
}

export function startRemotePlugins(
  runtime: PluginRuntime,
  remotePlugins: RemoteConfig["plugins"],
) {
  runtime.startWithRemoteConfig(remotePlugins ?? {});

  if (remotePlugins) {
    runtime.applyRemoteConfig(remotePlugins);
  } else {
    runtime.applyRemoteConfigDefaults();
  }
}
