import { z } from "zod";

export const errorSourceSchema = z
  .enum(["client", "server", "edge"])
  .meta({ id: "ErrorSource", title: "ErrorSource" });

export type ErrorSource = z.infer<typeof errorSourceSchema>;

export const errorMechanismSchema = z
  .object({
    handled: z.boolean(),
    synthetic: z.boolean().optional(),
    type: z.string(),
  })
  .meta({
    description: "How the exception was captured by the SDK.",
    id: "ErrorMechanism",
    title: "ErrorMechanism",
  });

export type ErrorMechanism = z.infer<typeof errorMechanismSchema>;

const errorExceptionSchema = z.object({
  kind: z.literal("error"),
  mechanism: errorMechanismSchema.optional(),
  stack: z.string(),
  type: z.string(),
  value: z.string(),
});

const nonErrorExceptionSchema = z.object({
  kind: z.literal("non-error"),
  mechanism: errorMechanismSchema.optional(),
  serialized: z.unknown(),
  type: z.string(),
  value: z.string(),
});

export const exceptionValueSchema = z
  .union([errorExceptionSchema, nonErrorExceptionSchema])
  .meta({
    description:
      "A single exception. Error-shaped exceptions carry the raw `error.stack` string; non-Error rejections (e.g. `Promise.reject({ code: 401 })`) carry the structured payload as `serialized` instead.",
    id: "Exception",
    title: "Exception",
  });

export type ExceptionValue = z.infer<typeof exceptionValueSchema>;
