import type { Attributes } from "@opentelemetry/api";
import { z } from "zod";

const evidenceSchema = z.object({
  cancellationInitiator: z.enum(["user", "navigation", "unload", "timeout"]),
});

type CancellationInitiator = z.infer<
  typeof evidenceSchema
>["cancellationInitiator"];

const recoveryReports = new WeakMap<
  DOMException,
  { report: (() => void) | null }
>();

/** Pass to AbortController.abort() to identify the cancellation initiator. */
export function requestCancellation(
  cancellationInitiator: CancellationInitiator,
) {
  const reason = Object.assign(
    new DOMException(
      "Request cancelled",
      cancellationInitiator === "timeout" ? "TimeoutError" : "AbortError",
    ),
    { cancellationInitiator },
  );

  recoveryReports.set(reason, { report: null });

  return reason;
}

/** Call after the cancelled operation succeeds; returns false without an observed cancellation. */
export function requestRecovered(reason: DOMException) {
  const pending = recoveryReports.get(reason);

  if (!pending?.report) {
    return false;
  }

  const { report } = pending;

  recoveryReports.delete(reason);

  report();

  return true;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- AbortSignal.reason is an untrusted value; only a reason created by this SDK can accept a later recovery report.
export function bindRequestRecovery(reason: unknown, report: () => void) {
  if (!(reason instanceof DOMException)) {
    return;
  }

  const pending = recoveryReports.get(reason);

  if (pending) {
    pending.report = report;
  }
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- AbortSignal.reason is application-controlled and must be validated before recording.
export function cancellationEvidence(reason: unknown) {
  const parsed = evidenceSchema.safeParse(reason);

  if (!parsed.success) {
    return null;
  }

  const attributes: Attributes = {
    "interfere.http.cancellation_initiator": parsed.data.cancellationInitiator,
  };

  return attributes;
}
