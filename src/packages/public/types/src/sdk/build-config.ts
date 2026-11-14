import { catch as catchSchema, z } from "zod";

import { remoteConfigSchema } from "./remote-config.ts";

const remoteConfigFields = remoteConfigSchema.shape;

export const buildConfigSchema = catchSchema(
  z.object({
    anonymousReplayPolicy: catchSchema(
      remoteConfigFields.anonymousReplayPolicy,
      undefined,
    ),
    backendHosts: catchSchema(remoteConfigFields.backendHosts, undefined),
    ingestUrl: catchSchema(remoteConfigFields.ingestUrl, undefined),
    logSourcing: catchSchema(remoteConfigFields.logSourcing, undefined),
    platform: catchSchema(remoteConfigFields.platform, undefined),
    plugins: catchSchema(
      remoteConfigFields.plugins.partial().optional(),
      undefined,
    ),
    serverTracing: catchSchema(remoteConfigFields.serverTracing, undefined),
  }),
  {},
);
