import { z } from "zod";

export const cliIntegration = z.object({
  metadata: z.object({
    installationUrl: z.string().nullable().default(null),
  }),
  name: z.literal("cli"),
});
