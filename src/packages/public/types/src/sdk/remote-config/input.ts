import type { z } from "zod";

import type { remoteConfigInputSchema } from "../remote-config.ts";
import { ANONYMOUS_REPLAY_POLICIES } from "./anonymous-replay.ts";
import { isBackendHosts } from "./backend-hosts.ts";
import { isPlainRecord, isString } from "../guards/json.ts";
import { isRemotePluginConfig, pickRemotePluginConfig } from "./plugins.ts";

/** Remote config as the SDK reads it, after {@link parseRemoteConfigInput}. */
export type RemoteConfigInput = z.output<typeof remoteConfigInputSchema>;

const LOG_SOURCING_MODES = ["sdk", "drain"] as const;

const HOSTING_PLATFORMS = ["vercel", "cloudflare"] as const;

const SERVER_TRACING_MODES = ["full", "drain"] as const;

function isOneOf<const T extends string>(
  values: readonly T[],
  value: unknown,
): value is T {
  return values.some((candidate) => candidate === value);
}

function isOptionalOneOf<const T extends string>(
  values: readonly T[],
  value: unknown,
): value is T | undefined {
  return value === undefined || isOneOf(values, value);
}

function isProtocolVersion(value: unknown): value is number {
  return Number.isSafeInteger(value) && Number(value) > 0;
}

function isUrl(value: unknown): value is string {
  if (!isString(value)) {
    return false;
  }

  try {
    return new URL(value.trim()).href.length > 0;
  } catch {
    return false;
  }
}

/**
 * Parses an SDK config response the way `remoteConfigInputSchema` does, without loading zod into browser bundles: the replay policy, backend hosts and protocol version fall back to undefined when malformed, and any other malformed field rejects the whole response.
 */
// oxlint-disable-next-line anti-slop/no-unknown-parameters -- The SDK config response body is untrusted until this parse.
export function parseRemoteConfigInput(body: unknown) {
  if (!isPlainRecord(body)) {
    return null;
  }

  const {
    anonymousReplayPolicy,
    backendHosts,
    ingestUrl,
    logSourcing,
    platform,
    plugins,
    protocolVersion,
    serverTracing,
  } = body;

  if (
    (ingestUrl !== undefined && !isUrl(ingestUrl)) ||
    !isOptionalOneOf(LOG_SOURCING_MODES, logSourcing) ||
    !isOptionalOneOf(HOSTING_PLATFORMS, platform) ||
    !isOptionalOneOf(SERVER_TRACING_MODES, serverTracing) ||
    (plugins !== undefined && !isRemotePluginConfig(plugins))
  ) {
    return null;
  }

  const config: RemoteConfigInput = {};

  if ("anonymousReplayPolicy" in body) {
    config.anonymousReplayPolicy = isOneOf(
      ANONYMOUS_REPLAY_POLICIES,
      anonymousReplayPolicy,
    )
      ? anonymousReplayPolicy
      : undefined;
  }

  if ("backendHosts" in body) {
    config.backendHosts = isBackendHosts(backendHosts)
      ? backendHosts
      : undefined;
  }

  if ("ingestUrl" in body) {
    config.ingestUrl = ingestUrl?.trim().replaceAll(/[\t\n\r]/gu, "");
  }

  if ("logSourcing" in body) {
    config.logSourcing = logSourcing;
  }

  if ("platform" in body) {
    config.platform = platform;
  }

  if ("plugins" in body) {
    config.plugins =
      plugins === undefined ? undefined : pickRemotePluginConfig(plugins);
  }

  if ("protocolVersion" in body) {
    config.protocolVersion = isProtocolVersion(protocolVersion)
      ? protocolVersion
      : undefined;
  }

  if ("serverTracing" in body) {
    config.serverTracing = serverTracing;
  }

  return config;
}
