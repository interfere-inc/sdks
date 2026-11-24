import { z } from "zod";

const phase = z.enum([
  "files",
  "manifest",
  "release",
  "continuation",
  "completion",
  "complete",
]);

const timestamp = z.number().int().nonnegative();

const maximum = z.object({
  attempt: z.number().int().positive(),
  durationMs: timestamp,
});

export const sourceMapPublicationDiagnostics = z.object({
  attempt: z.number().int().positive(),
  scheduledAtMs: timestamp.nullable(),
  startedAtMs: timestamp,
  completedAtMs: timestamp.nullable(),
  phase,
  publishedAtStart: z.number().int().nonnegative(),
  progressObservedAtMs: timestamp.nullable(),
  nativeRetryCount: z.number().int().nonnegative(),
  interruptedAttempts: z.number().int().nonnegative(),
  longestAttempt: maximum.nullable(),
  longestAlarmDelay: maximum.nullable(),
  nextAlarmAtMs: timestamp.nullable(),
  lastFailure: z
    .object({
      attempt: z.number().int().positive(),
      atMs: timestamp,
      phase,
      kind: z.enum(["timeout", "aborted", "error", "non-error"]),
    })
    .nullable(),
});

export type SourceMapPublicationDiagnostics = z.infer<
  typeof sourceMapPublicationDiagnostics
>;
