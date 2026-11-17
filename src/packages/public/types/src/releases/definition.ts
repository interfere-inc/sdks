import { z } from "zod";

import {
  releaseDestinationMetadataSchema,
  releaseSourceMetadataSchema,
} from "../integrations/index.ts";
import { releaseSlugSchema } from "./slug.ts";

export const releaseStatusSchema = z
  .enum(["pending", "building", "ready", "errored", "canceled", "unknown"])
  .meta({ id: "ReleaseStatus", title: "ReleaseStatus" });

export type ReleaseStatus = z.infer<typeof releaseStatusSchema>;

const orgRefSchema = z
  .object({
    id: z.string(),
    slug: z.string(),
  })
  .meta({ id: "OrgRef", title: "OrgRef" });

const surfaceRefSchema = z
  .object({
    id: z.string(),
    slug: z.string(),
  })
  .meta({ id: "SurfaceRef", title: "SurfaceRef" });

const releaseServiceSchema = z
  .object({
    name: z.string().max(255).regex(/\S/u),
    namespace: z.string().max(255).optional(),
  })
  .meta({ id: "ReleaseService", title: "ReleaseService" });

export const createReleaseRequest = z
  .object({
    buildId: z.string(),
    destination: releaseDestinationMetadataSchema.nullable(),
    producerVersion: z.string().optional(),
    service: releaseServiceSchema.optional(),
    /** Release slug derived from the commit SHA; when omitted, the server derives it from buildId. */
    slug: z.string().optional(),
    source: releaseSourceMetadataSchema,
  })
  .meta({
    description: "Request body for creating a release.",
    id: "CreateReleaseRequest",
    title: "CreateReleaseRequest",
  });

export type CreateReleaseRequest = z.infer<typeof createReleaseRequest>;

export const createReleaseResponse = z
  .object({
    build: z
      .object({
        hash: z.string(),
      })
      .meta({ id: "ReleaseBuildRef", title: "ReleaseBuildRef" }),
    destination: z
      .object({
        id: z.string(),
        slug: releaseSlugSchema,
        environment: z.string().nullable(),
      })
      .meta({ id: "ReleaseDestinationRef", title: "ReleaseDestinationRef" }),
    org: orgRefSchema,
    source: z
      .object({
        id: z.string(),
      })
      .meta({ id: "ReleaseSourceRef", title: "ReleaseSourceRef" }),
    surface: surfaceRefSchema.nullable(),
    token: z.string().nullable().optional().meta({
      description:
        "Signed release token for browser telemetry, baked into the bundle by the build plugin. It names the workspace, service and release, carries no environment and does not expire. Null when the release has no service.",
    }),
  })
  .meta({
    description: "Response body returned after creating a release.",
    id: "CreateReleaseResponse",
    title: "CreateReleaseResponse",
  });

export type CreateReleaseResponse = z.infer<typeof createReleaseResponse>;
