import { backendHostsSchema } from "./remote-config/backend-hosts.ts";
import { catch as catchSchema, z } from "zod";

import { anonymousReplayPolicySchema } from "./remote-config/anonymous-replay.ts";
import { protocolVersionSchema } from "./remote-config/protocol.ts";
import { logSourcingModeSchema } from "./log-sourcing.ts";
import { PLUGIN_MANIFEST } from "./plugins/manifest.ts";
import type { PluginKey } from "./plugins/manifest.ts";
import { hostingPlatformSchema } from "../integrations/platforms/hosting.ts";
import { serverTracingModeSchema } from "./server-tracing.ts";

export type RemotePluginConfig = z.infer<
  typeof partialRemotePluginConfigSchema
>;

export const remotePluginConfigSchema = z
  .object(
    // oxlint-disable-next-line typescript/no-unsafe-type-assertion
    Object.fromEntries(
      PLUGIN_MANIFEST.map((p) => [p.name, z.boolean()]),
    ) as Record<PluginKey, z.ZodBoolean>,
  )
  .meta({ id: "RemotePluginConfig", title: "RemotePluginConfig" });

export const remoteConfigSchema = z
  .object({
    anonymousReplayPolicy: anonymousReplayPolicySchema.optional(),
    backendHosts: backendHostsSchema,
    ingestUrl: z.url().optional(),
    logSourcing: logSourcingModeSchema.optional(),
    platform: hostingPlatformSchema.optional(),
    plugins: remotePluginConfigSchema,
    protocolVersion: protocolVersionSchema.optional(),
    serverTracing: serverTracingModeSchema.optional(),
  })
  .meta({ id: "RemoteConfig", title: "RemoteConfig" });

export type RemoteConfig = z.infer<typeof remoteConfigSchema>;

const partialRemotePluginConfigSchema = remotePluginConfigSchema.partial();

export const remoteConfigInputSchema = remoteConfigSchema.extend({
  anonymousReplayPolicy: catchSchema(
    anonymousReplayPolicySchema.optional(),
    undefined,
  ),
  backendHosts: catchSchema(backendHostsSchema, undefined),
  plugins: partialRemotePluginConfigSchema.optional(),
  protocolVersion: catchSchema(protocolVersionSchema.optional(), undefined),
});

/** Plugin settings resolved for the application at build time. */
export const PLUGINS_GLOBAL = "__INTERFERE_PLUGINS__";

export function readBakedPlugins() {
  const scope = globalThis;
  const injected = PLUGINS_GLOBAL in scope ? scope[PLUGINS_GLOBAL] : undefined;

  if (injected === undefined || injected === null) {
    return null;
  }

  const parsed = partialRemotePluginConfigSchema.safeParse(injected);

  return (
    parsed.success ? parsed.data : null
  ) satisfies RemotePluginConfig | null;
}
