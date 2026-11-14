import { z } from "zod";

/**
 * Global stamped into the server bundle at build time with the surface's
 * hosting platform from the remote config: a {@link HostingPlatform}, or
 * `null` when the surface is not linked to one. Absent when the build could
 * not load the remote config.
 */
export const PLATFORM_GLOBAL = "__INTERFERE_PLATFORM__";

/** Hosting platform the surface is linked to through a destination connection. */
export const hostingPlatformSchema = z.enum(["vercel", "cloudflare"]);

export type HostingPlatform = z.infer<typeof hostingPlatformSchema>;
