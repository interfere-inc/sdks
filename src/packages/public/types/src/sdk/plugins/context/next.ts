import { z } from "zod";

export const renderSourceSchema = z
  .enum([
    "react-server-components",
    "react-server-components-payload",
    "server-rendering",
  ])
  .meta({ id: "RenderSource", title: "RenderSource" });

export const renderTypeSchema = z
  .enum(["dynamic", "dynamic-resume"])
  .meta({ id: "RenderType", title: "RenderType" });

export const revalidateReasonSchema = z
  .enum(["on-demand", "stale"])
  .meta({ id: "RevalidateReason", title: "RevalidateReason" });

export const routeTypeSchema = z
  .enum(["render", "route", "action", "middleware", "proxy"])
  .meta({ id: "RouteType", title: "RouteType" });

export const routerKindSchema = z
  .enum(["Pages Router", "App Router"])
  .meta({ id: "RouterKind", title: "RouterKind" });

export const nextjsContextSchema = z
  .object({
    errorDigest: z.string().optional(),
    renderSource: renderSourceSchema.optional(),
    renderType: renderTypeSchema.optional(),
    requestMethod: z.string().optional(),
    requestPath: z.string().optional(),
    revalidateReason: revalidateReasonSchema.optional(),
    routePath: z.string().optional(),
    routeType: routeTypeSchema.optional(),
    routerKind: routerKindSchema.optional(),
    runtime: z.literal("nextjs"),
  })
  .meta({
    description: "Request context for the Next.js runtime.",
    id: "NextjsContext",
    title: "NextjsContext",
  });

export type NextjsContext = z.infer<typeof nextjsContextSchema>;
