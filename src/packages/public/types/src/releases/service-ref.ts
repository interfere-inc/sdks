import { z } from "zod";

export const releaseServiceRefSchema = z
  .object({
    id: z.string(),
    name: z.string(),
    namespace: z.string().nullable(),
  })
  .meta({
    description:
      "Service the release belongs to, identified by its OpenTelemetry service.name and service.namespace.",
    id: "ReleaseServiceRef",
    title: "ReleaseServiceRef",
  });
