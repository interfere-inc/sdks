import { z } from "zod";

export const identifySourceSchema = z
  .discriminatedUnion("type", [
    z
      .object({ name: z.literal("Clerk"), type: z.literal("clerk") })
      .meta({ id: "ClerkIdentifySource", title: "ClerkIdentifySource" }),
    z
      .object({ name: z.literal("Auth0"), type: z.literal("auth0") })
      .meta({ id: "Auth0IdentifySource", title: "Auth0IdentifySource" }),
    z
      .object({ name: z.literal("WorkOS"), type: z.literal("workos") })
      .meta({ id: "WorkOSIdentifySource", title: "WorkOSIdentifySource" }),
    z
      .object({ name: z.string(), type: z.literal("custom") })
      .meta({ id: "CustomIdentifySource", title: "CustomIdentifySource" }),
  ])
  .meta({
    description: "Auth provider that surfaced this identification.",
    id: "IdentifySource",
    title: "IdentifySource",
  });

export type IdentifySource = z.infer<typeof identifySourceSchema>;

const companySchema = z
  .strictObject({
    tier: z.string().trim().min(1).max(100),
  })
  .meta({
    description:
      "Company attributes; tier is a 1-100 character label, created on first use.",
    id: "IdentifyCompany",
    title: "IdentifyCompany",
  });

export const identitySignatureSchema = z
  .string()
  .regex(/^[0-9a-fA-F]{64}$/u)
  .meta({
    description:
      "Hex HMAC-SHA256 of identifier keyed with your identify secret.",
    id: "IdentitySignature",
    title: "IdentitySignature",
  });

const traitsSchema = z.record(z.string(), z.unknown()).meta({
  description: "Free-form user attributes.",
  id: "IdentifyTraits",
  title: "IdentifyTraits",
});

export const identifyParamsSchema = z
  .object({
    avatar: z.string().optional(),
    company: companySchema.optional(),
    email: z.email().optional(),
    identifier: z.string(),
    name: z.string().optional(),
    signature: identitySignatureSchema.optional(),
    source: identifySourceSchema,
    traits: traitsSchema.optional(),
  })
  .superRefine((params, ctx) => {
    if (
      params.source.type === "clerk" &&
      !params.identifier.startsWith("user_")
    ) {
      ctx.addIssue({
        code: "custom",
        message:
          "Clerk identifiers must be raw Clerk user ids starting with user_.",
        path: ["identifier"],
      });
    }
  })
  .meta({
    description: "User attributes attached when identifying a session.",
    id: "IdentifyParams",
    title: "IdentifyParams",
  });

export type IdentifyParams = z.infer<typeof identifyParamsSchema>;
