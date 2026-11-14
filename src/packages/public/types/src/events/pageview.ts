import { z } from "zod";

export const pageviewFactsPayloadSchema = z.object({
  referrer: z.string().optional(),
  releaseId: z.string().nullable().optional(),
  runtime: z.string().optional(),
  title: z.string().optional(),
  url: z.url(),
});

export type PageviewFactsPayload = z.infer<typeof pageviewFactsPayloadSchema>;
