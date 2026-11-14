import { z } from "zod";

import {
  cloudflareDestinationIdEnvKeys,
  cloudflareSourceIdEnvKeys,
} from "./platforms/cloudflare.ts";
import { githubSourceIdEnvKeys } from "./platforms/github.ts";
import { vercelDestinationIdEnvKeys } from "./platforms/vercel.ts";

export const githubReleaseSourceSchema = z
  .object({
    authorLogin: z.string().nullable().optional(),
    authorAvatarUrl: z.url().nullable().optional(),
    branch: z.string(),
    commitMessage: z.string(),
    commitSha: z.string().nullable(),
    provider: z.literal("github"),
    runId: z.string().nullable().optional(),
    runUrl: z.string().nullable().optional(),
    runnerName: z.string().nullable().optional(),
  })
  .meta({
    description: "Release source metadata coming from GitHub.",
    id: "GithubReleaseSource",
    title: "GithubReleaseSource",
  });

export const cliReleaseSourceSchema = z
  .object({
    commitMessage: z.string().nullable(),
    commitRef: z.string().nullable(),
    commitSha: z.string().nullable(),
    commitUrl: z.string().nullable(),
    deploymentUrl: z.string().nullable().optional(),
    environment: z.string().nullable().optional(),
    provider: z.literal("cli"),
    runId: z.string().nullable(),
    runUrl: z.string().nullable(),
    runnerName: z.string().nullable(),
  })
  .meta({
    description: "Release source metadata from the Interfere CLI.",
    id: "CliReleaseSource",
    title: "CliReleaseSource",
  });

export const releaseSourceMetadataSchema = z
  .discriminatedUnion("provider", [
    githubReleaseSourceSchema,
    cliReleaseSourceSchema,
  ])
  .meta({ id: "ReleaseSource", title: "ReleaseSource" });

export type ReleaseSourceMetadata = z.infer<typeof releaseSourceMetadataSchema>;

export const releaseSourceProviderSchema = z
  .enum(["github", "cli"])
  .meta({ id: "ReleaseSourceProvider", title: "ReleaseSourceProvider" });

export type ReleaseSourceProvider = z.infer<typeof releaseSourceProviderSchema>;

export const vercelReleaseDestinationSchema = z
  .object({
    deploymentId: z.string().nullable(),
    deploymentUrl: z.string().nullable(),
    destinationReleaseId: z.string().nullable(),
    environment: z.string().nullable(),
    environmentName: z.string().nullable(),
    environmentTarget: z.string().nullable(),
    provider: z.literal("vercel"),
  })
  .meta({
    description: "Release destination metadata for a Vercel deployment.",
    id: "VercelReleaseDestination",
    title: "VercelReleaseDestination",
  });

export const cloudflareReleaseDestinationSchema = z
  .object({
    accountId: z.string().nullable(),
    deploymentUrl: z.string().nullable(),
    destinationReleaseId: z.string().nullable(),
    environment: z.string().nullable(),
    environmentName: z.string().nullable(),
    environmentTarget: z.string().nullable(),
    provider: z.literal("cloudflare"),
    deploymentId: z.string().nullable().optional(),
    trafficPercentage: z.number().min(0).max(100).nullable().optional(),
    scriptName: z.string().nullable(),
    versionId: z.string().nullable(),
  })
  .meta({
    description:
      "Release destination metadata for a Cloudflare Workers deployment.",
    id: "CloudflareReleaseDestination",
    title: "CloudflareReleaseDestination",
  });

export const cliReleaseDestinationSchema = z
  .object({
    deploymentUrl: z.string().nullable(),
    destinationReleaseId: z.string().nullable(),
    environment: z.string().nullable(),
    provider: z.literal("cli"),
  })
  .meta({
    description: "Release destination metadata from the Interfere CLI.",
    id: "CliReleaseDestination",
    title: "CliReleaseDestination",
  });

export const releaseDestinationMetadataSchema = z
  .discriminatedUnion("provider", [
    vercelReleaseDestinationSchema,
    cloudflareReleaseDestinationSchema,
    cliReleaseDestinationSchema,
  ])
  .meta({ id: "ReleaseDestination", title: "ReleaseDestination" });

export type ReleaseDestinationMetadata = z.infer<
  typeof releaseDestinationMetadataSchema
>;

export const releaseDestinationProviderSchema = z
  .enum(["vercel", "cloudflare", "cli"])
  .meta({
    id: "ReleaseDestinationProvider",
    title: "ReleaseDestinationProvider",
  });

export type ReleaseDestinationProvider = z.infer<
  typeof releaseDestinationProviderSchema
>;

export const configuredSourceProviderSchema = z
  .enum(["github", "ghe_cloud", "ghe_server"])
  .meta({ id: "ConfiguredSourceProvider", title: "ConfiguredSourceProvider" });

export type ConfiguredSourceProvider = z.infer<
  typeof configuredSourceProviderSchema
>;

export const configuredDestinationProviderSchema = z
  .enum(["vercel", "cloudflare", "cli"])
  .meta({
    id: "ConfiguredDestinationProvider",
    title: "ConfiguredDestinationProvider",
  });

export type ConfiguredDestinationProvider = z.infer<
  typeof configuredDestinationProviderSchema
>;

export const sourceProviderSchema = configuredSourceProviderSchema.meta({
  id: "SourceProvider",
  title: "SourceProvider",
});

export type SourceProvider = z.infer<typeof sourceProviderSchema>;

export const destinationProviderSchema = z
  .enum(["vercel", "cloudflare"])
  .meta({ id: "DestinationProvider", title: "DestinationProvider" });

export type DestinationProvider = z.infer<typeof destinationProviderSchema>;

const releaseSourceIdEnvKeySchema = z.enum([
  "NEXT_PUBLIC_INTERFERE_BUILD_ID",
  "INTERFERE_SOURCE_ID",
  "NEXT_BUILD_ID",
  ...githubSourceIdEnvKeys,
  ...cloudflareSourceIdEnvKeys,
  "CI_COMMIT_SHA",
  "COMMIT_REF",
  "RENDER_GIT_COMMIT",
  "CIRCLE_SHA1",
  "BITBUCKET_COMMIT",
  "BUILD_SOURCEVERSION",
  "BUILDKITE_COMMIT",
  "RAILWAY_GIT_COMMIT_SHA",
  "AWS_COMMIT_ID",
  "SOURCE_VERSION",
]);

export const releaseSourceIdEnvKeys = releaseSourceIdEnvKeySchema.options;

const releaseDestinationIdEnvKeySchema = z.enum([
  "NEXT_PUBLIC_INTERFERE_RELEASE_ID",
  ...vercelDestinationIdEnvKeys,
  ...cloudflareDestinationIdEnvKeys,
  ...githubSourceIdEnvKeys,
]);

export const releaseDestinationIdEnvKeys =
  releaseDestinationIdEnvKeySchema.options;
