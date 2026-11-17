import { catch as catchSchema, z } from "zod";

import { ANONYMOUS_REPLAY_POLICIES } from "./remote-config/anonymous-replay.ts";
import { protocolVersionSchema } from "./remote-config/protocol.ts";
import { logSourcingModeSchema } from "./log-sourcing.ts";
import { PLUGIN_MANIFEST } from "./plugins/manifest.ts";
import type { PluginKey } from "./plugins/manifest.ts";
import { hostingPlatformSchema } from "../integrations/platforms/hosting.ts";
import { serverTracingModeSchema } from "./server-tracing.ts";

export const anonymousReplayPolicySchema = z
  .enum(ANONYMOUS_REPLAY_POLICIES)
  .describe(
    "When sessions without an identified user record replay: always, only once an error occurs, or never.",
  );

export const backendHostsSchema = z.array(z.string()).optional();

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
