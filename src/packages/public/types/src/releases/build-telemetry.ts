import { z } from "zod";

/**
 * Outcome of one release-pipeline run, as reported by an SDK build
 * plugin. Mirrors `ReleaseDiagnosticCode` from `../sdk/build-diagnostics`
 * (lowercased), plus the two outcomes that are not diagnostics: `ok`, and
 * the early returns for a build that produced no maps.
 */
export const releaseBuildResultSchema = z
  .enum([
    "ok",
    "api_key_wrong_prefix",
    "api_key_unreadable",
    "api_key_rejected",
    "api_key_scope_mismatch",
    "api_key_surface_required",
    "pipeline_failed",
    "no_source_maps",
    "client_maps_unmapped",
  ])
  .meta({ id: "ReleaseBuildResult", title: "ReleaseBuildResult" });

export type ReleaseBuildResult = z.infer<typeof releaseBuildResultSchema>;

/**
 * Who has to fix it. Derived server-side from the reported result rather
 * than sent by the SDK, so an SDK bug can never relabel our own error
 * budget. `pipeline_failed` is the catch-all and counts as ours until
 * proven otherwise.
 */
export const releaseBuildFaultSchema = z
  .enum(["interfere", "customer"])
  .meta({ id: "ReleaseBuildFault", title: "ReleaseBuildFault" });

export type ReleaseBuildFault = z.infer<typeof releaseBuildFaultSchema>;

const CUSTOMER_FAULT_RESULTS = new Set<ReleaseBuildResult>([
  "api_key_wrong_prefix",
  "api_key_unreadable",
  "api_key_rejected",
  "api_key_scope_mismatch",
  "api_key_surface_required",
  "no_source_maps",
  "client_maps_unmapped",
]);

export function faultForBuildResult(result: ReleaseBuildResult) {
  return (
    CUSTOMER_FAULT_RESULTS.has(result) ? "customer" : "interfere"
  ) satisfies ReleaseBuildFault;
}

/**
 * Steps of the build pipeline, in order. `upload` covers `/sign`, the
 * direct-to-R2 PUTs and `/complete` as one span — the collector never
 * sees those PUTs, so it cannot break them down further. `total` is the
 * whole plugin, reported alongside the parts.
 */
export const releaseBuildStepSchema = z
  .enum(["discover", "create", "upload", "preflight", "cleanup", "total"])
  .meta({ id: "ReleaseBuildStep", title: "ReleaseBuildStep" });

export type ReleaseBuildStep = z.infer<typeof releaseBuildStepSchema>;
