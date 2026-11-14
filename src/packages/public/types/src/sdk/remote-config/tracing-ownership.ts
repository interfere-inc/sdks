import { z } from "zod";

export const tracingOwnershipSchema = z
  .enum(["managed", "application"])
  .describe(
    "Who owns OpenTelemetry for the process: managed (Interfere constructs the providers) or application (the host does, Interfere correlates).",
  );

/** Who owns OpenTelemetry for the process; a code option is the only way to name one. */
export type TracingOwnership = z.infer<typeof tracingOwnershipSchema>;

/** Owner assumed when no option names one. */
export const DEFAULT_TRACING_OWNERSHIP = "managed" satisfies TracingOwnership;
