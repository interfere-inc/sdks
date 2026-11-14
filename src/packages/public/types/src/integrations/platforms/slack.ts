import { z } from "zod";

export const routableCategorySchema = z.enum([
  "problems",
  "team",
  "collaboration",
]);

export type RoutableCategory = z.infer<typeof routableCategorySchema>;

export const slackChatRoutingEntrySchema = z.object({
  channelId: z.string(),
  channelName: z.string(),
});

export type SlackChatRoutingEntry = z.infer<typeof slackChatRoutingEntrySchema>;

export const slackIntegration = z.object({
  metadata: z.object({
    teamId: z.string(),
    teamName: z.string(),
    installationUrl: z.string().nullable(),
    chatRouting: z
      .partialRecord(routableCategorySchema, slackChatRoutingEntrySchema)
      .default({}),
  }),
  name: z.literal("slack"),
});
