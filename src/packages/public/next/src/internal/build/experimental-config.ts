import type { NextConfig } from "next";

const NEXT_INSTRUMENTATION_HOOK_FLAG_LAST_MAJOR = 14;

const CLIENT_TRACE_METADATA_KEYS = [
  "traceparent",
  "tracestate",
  "baggage",
] as const;

export type NextExperimentalConfig = NonNullable<NextConfig["experimental"]> & {
  clientTraceMetadata?: readonly string[];
  instrumentationHook?: boolean;
};

export function needsInstrumentationHookFlag(nextMajor: number | null) {
  return (
    nextMajor === null || nextMajor <= NEXT_INSTRUMENTATION_HOOK_FLAG_LAST_MAJOR
  );
}

export function interfereExperimentalConfig({
  existing,
  instrumentationHook,
  traceMetadata,
  sourceMaps,
}: {
  readonly existing: NextExperimentalConfig | undefined;
  readonly instrumentationHook: boolean;
  /**
   * Next's `clientTraceMetadata` is enabled whenever the browser SDK has a
   * credential. Managed `register()` writes the stamped SSR trace meta the
   * browser accepts; when the application owns tracing its propagator decides
   * what the meta carries and the browser rejects an unstamped `traceparent`,
   * so the keys are safe in either mode and the owner is only known at runtime.
   */
  readonly traceMetadata: boolean;
  readonly sourceMaps: boolean;
}) {
  const merged: NextExperimentalConfig = {
    ...existing,
  };

  if (instrumentationHook) {
    merged.instrumentationHook = true;
  }

  if (sourceMaps) {
    merged.serverSourceMaps = true;
  }

  if (traceMetadata) {
    merged.clientTraceMetadata = [
      ...new Set([
        ...(existing?.clientTraceMetadata ?? []),
        ...CLIENT_TRACE_METADATA_KEYS,
      ]),
    ];
  }

  return (Object.keys(merged).length > 0 ? merged : undefined) satisfies
    | NextExperimentalConfig
    | undefined;
}
