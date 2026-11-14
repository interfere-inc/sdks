import { z } from "zod";

const cloudflareDestinationIdEnvKeySchema = z.enum([
  "CF_VERSION_ID",
  "WORKERS_CI_BUILD_UUID",
]);

export const cloudflareDestinationIdEnvKeys =
  cloudflareDestinationIdEnvKeySchema.options;

const cloudflareSourceIdEnvKeySchema = z.enum(["WORKERS_CI_COMMIT_SHA"]);

export const cloudflareSourceIdEnvKeys = cloudflareSourceIdEnvKeySchema.options;

export const cloudflareAccountMetadata = z.object({
  accountType: z.literal("account"),
  avatar: z.string().nullable().default(null),
  id: z.string(),
  installationUrl: z.string().nullable(),
  name: z.string().nullable().default(null),
  deploymentCutoffs: z.record(z.string(), z.string()).optional(),
  versionCutoffs: z.record(z.string(), z.string()).default({}),
  workers: z.record(z.string(), z.string()).default({}),
  assetsOnlyWorkers: z.array(z.string()).optional(),
  workersDiscoveredAt: z.number().int().nonnegative().optional(),
});

const cloudflareMetadataSchema = z.discriminatedUnion("accountType", [
  cloudflareAccountMetadata,
]);

export const cloudflareIntegration = z.object({
  metadata: cloudflareMetadataSchema,
  name: z.literal("cloudflare"),
});
