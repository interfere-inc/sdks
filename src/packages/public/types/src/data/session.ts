import { z } from "zod";

export const storedSessionIdSchema = z.string().min(1).brand<"SessionId">();

const wireSessionIdSchema = z.union([
  z.uuidv7(),
  z.templateLiteral(["server_", z.uuidv7()]),
]);

export const sessionIdSchema = storedSessionIdSchema
  .and(wireSessionIdSchema)
  .meta({
    description:
      "UUIDv7 (from the client SDK) or server_<uuidv7> (server-side fallback).",
    id: "SessionId",
    title: "SessionId",
  });

export type SessionId = z.infer<typeof sessionIdSchema>;

/** Page-scoped session id the browser SDK uses while consent is required and not granted: held in memory only and new on every page load. */
export const pageSessionIdSchema = z.templateLiteral(["page_", z.uuidv7()]);

export type PageSessionId = z.infer<typeof pageSessionIdSchema>;
