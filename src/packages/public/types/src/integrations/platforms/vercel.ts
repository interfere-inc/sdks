import { z } from "zod";

const vercelDestinationIdEnvKeySchema = z.enum(["VERCEL_DEPLOYMENT_ID"]);

export const vercelDestinationIdEnvKeys =
  vercelDestinationIdEnvKeySchema.options;

const vercelMetadataBase = z.object({
  avatar: z.string().nullable().default(null),
  installationUrl: z.string().nullable(),
  name: z.string().nullable().default(null),
  projects: z.record(z.string(), z.string().nullable()).default({}),
});

export const vercelTeamMetadata = vercelMetadataBase.extend({
  accountType: z.literal("team"),
  id: z.string(),
  slug: z.string(),
});

export const vercelUserMetadata = vercelMetadataBase.extend({
  accountType: z.literal("user"),
  email: z.string(),
  id: z.string(),
  username: z.string(),
});

const vercelMetadataSchema = z.discriminatedUnion("accountType", [
  vercelTeamMetadata,
  vercelUserMetadata,
]);

export const vercelIntegration = z.object({
  metadata: vercelMetadataSchema,
  name: z.literal("vercel"),
});
