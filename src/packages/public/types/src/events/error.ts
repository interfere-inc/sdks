import { z } from "zod";

import { ingestedFrame } from "#data/frame";

const baseEnrichedFrameSchema = z.object({
  column: z.number().nullable(),
  file: z.string().nullable(),
  fn: z.string().nullable(),
  inApp: z.boolean(),
  ingestedId: z.string(),
  line: z.number().nullable(),
});

const resolvedFrameSchema = baseEnrichedFrameSchema.extend({
  filePath: z.string().nullable(),
  fileReference: z.string().nullable(),
  originalColumn: z.number().nullable(),
  originalLine: z.number().nullable(),
  originalName: z.string().nullable(),
  resolved: z.literal(true),
  sourceContext: z
    .object({
      startLine: z.number().int().positive(),
      code: z.string(),
    })
    .optional(),
});

const unresolvedFrameSchema = baseEnrichedFrameSchema.extend({
  resolved: z.literal(false),
});

export const enrichedFrameSchema = z.discriminatedUnion("resolved", [
  resolvedFrameSchema,
  unresolvedFrameSchema,
]);

export type EnrichedFrame = z.infer<typeof enrichedFrameSchema>;

export type ResolvedFrame = z.infer<typeof resolvedFrameSchema>;

export type UnresolvedFrame = z.infer<typeof unresolvedFrameSchema>;

export const errorMechanismSchema = z.object({
  handled: z.boolean(),
  synthetic: z.boolean().optional(),
  type: z.string(),
});

export type ErrorMechanism = z.infer<typeof errorMechanismSchema>;

export const errorContextSchema = z.object({
  errorDigest: z.string().optional(),
  framework: z.string().optional(),
  renderSource: z
    .enum([
      "react-server-components",
      "react-server-components-payload",
      "server-rendering",
    ])
    .optional(),
  renderType: z.enum(["dynamic", "dynamic-resume"]).optional(),
  requestMethod: z.string().optional(),
  requestPath: z.string().optional(),
  revalidateReason: z.enum(["on-demand", "stale"]).optional(),
  routePath: z.string().optional(),
  routeType: z
    .enum(["render", "route", "action", "middleware", "proxy"])
    .optional(),
  routerKind: z.enum(["Pages Router", "App Router"]).optional(),
});

const errorPayloadCommonSchema = z.object({
  captureSource: z.string().optional(),
  context: errorContextSchema.optional(),
  frames: z.array(ingestedFrame),
  mechanism: errorMechanismSchema.optional(),
  message: z.string().nullable(),
  name: z.string().nullable(),
  runtime: z.string().optional(),
  stack: z.string().nullable(),
});

const errorPayloadFromErrorSchema = errorPayloadCommonSchema.extend({
  capture: z.literal("error"),
});

const errorPayloadFromNonErrorSchema = errorPayloadCommonSchema.extend({
  capture: z.literal("non-error"),
  serialized: z.json(),
});

export const errorPayloadSchema = z.discriminatedUnion("capture", [
  errorPayloadFromErrorSchema,
  errorPayloadFromNonErrorSchema,
]);

export type ErrorPayload = z.infer<typeof errorPayloadSchema>;
