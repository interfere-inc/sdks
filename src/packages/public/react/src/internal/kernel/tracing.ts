import { omitUndefined } from "@interfere/helpers/omit-undefined";

import type { SessionTracker } from "../../tracking/api.js";
import { resolveTargets } from "../config/config.js";
import { sdkRequestUrls } from "../otel/ingest-ignore.js";
import { installEarlyBaggage } from "../otel/instrumentations/early-baggage.js";
import { propagationTargets } from "../otel/instrumentations/propagation-targets.js";
import { THIRD_PARTY_IGNORE } from "../third-party-hosts.js";
import type { ResolvedTracingOptions } from "../otel/wire.js";
import type { Kernel } from "./kernel.js";
import type { KernelInternalOptions } from "./options.js";

/** Prepares the session and stamps its baggage on requests sent before tracing is wired; `undefined` when tracing or baggage is off for every origin. */
export function installStartupBaggage(
  kernel: Kernel,
  tracker: SessionTracker,
  opts: KernelInternalOptions,
) {
  const { tracing } = opts;

  if (tracing === false) {
    return;
  }

  const policy = propagationTargets(
    omitUndefined({
      pageOrigin: globalThis.location.origin,
      headers: tracing?.headers,
      propagateTraceBaggage: tracing?.propagateTraceBaggage,
      propagateTo: tracing?.propagateTo,
    }),
  );

  if (![...policy.values()].some((entry) => entry.headers.has("baggage"))) {
    return;
  }

  tracker.slot.prepare();

  return installEarlyBaggage({
    getDeviceId: () => kernel.device.getDeviceId(),
    getSessionId: () =>
      kernel.session.getId() ?? tracker.slot.pendingSessionId(),
    ignoreUrls: [
      ...sdkRequestUrls(resolveTargets(opts.apiHost)),
      ...THIRD_PARTY_IGNORE,
      ...(tracing?.ignoreUrls ?? []),
    ],
    policy,
  });
}

export function resolveTracingOptions(
  opts: KernelInternalOptions,
  serviceName: string | undefined,
  backendHosts: ResolvedTracingOptions["backendHosts"],
) {
  const tracingOpts =
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- The public tracing option accepts an instrumentation-options object or false; anything else means defaults.
    typeof opts.tracing === "object" ? opts.tracing : undefined;

  const resolvedTracing: ResolvedTracingOptions = { ...tracingOpts };

  if (serviceName) {
    resolvedTracing.serviceName = serviceName;
  }

  if (opts.serviceNamespace !== undefined) {
    resolvedTracing.serviceNamespace = opts.serviceNamespace;
  }

  /* oxlint-disable eslint/no-underscore-dangle -- Published option that other @interfere packages pass across versions. */
  if (opts._internalAdditionalSpanProcessors) {
    resolvedTracing.additionalSpanProcessors =
      opts._internalAdditionalSpanProcessors;
  }

  if (opts._internalAdditionalLogRecordProcessors) {
    resolvedTracing.additionalLogRecordProcessors =
      opts._internalAdditionalLogRecordProcessors;
  }

  if (opts._internalAdditionalMetricReaders) {
    resolvedTracing.additionalMetricReaders =
      opts._internalAdditionalMetricReaders;
  }

  /* oxlint-enable eslint/no-underscore-dangle */
  if (backendHosts) {
    resolvedTracing.backendHosts = backendHosts;
  }

  return resolvedTracing;
}
