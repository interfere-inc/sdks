import { z } from "zod";

import { tracingOwnershipSchema } from "./tracing-ownership.ts";

export const tracingModeSchema = z
  .enum(["auto", ...tracingOwnershipSchema.options])
  .describe(
    "The browser tracing mode: auto decides the owner from the page (managed unless a tracing vendor is present); managed and application are explicit overrides.",
  );

/** The browser tracing mode an application requests; `auto` resolves to an owner at init. */
export type TracingMode = z.infer<typeof tracingModeSchema>;

/** Mode used when the init option names none. */
export const DEFAULT_TRACING_MODE = "auto" satisfies TracingMode;
