import { z } from "zod";

export const pageleaveFactsPayloadSchema = z.object({
  durationMs: z.number().optional(),
  releaseId: z.string().nullable().optional(),
  runtime: z.string().optional(),
  url: z.url(),
});

export type PageleaveFactsPayload = z.infer<typeof pageleaveFactsPayloadSchema>;
