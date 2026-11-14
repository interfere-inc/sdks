import type { Interfere } from "@interfere/javascript";

export type BuildTelemetry = Omit<
  Interfere.TelemetryRequest,
  "result" | "preflight" | "releaseSlug"
>;

export const EMPTY_TELEMETRY: BuildTelemetry = {
  cleanup: 0,
  createRelease: 0,
  discover: 0,
  fileCount: 0,
  total: 0,
  totalBytes: 0,
  upload: 0,
};
