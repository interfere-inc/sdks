import type { Context, Exception, TimeInput } from "@opentelemetry/api";
import type { Span, SpanProcessor } from "@opentelemetry/sdk-trace";

import { contain } from "@interfere/server-core/contain";
import { controlFlowOutcome } from "./control-flow.js";
import { captureState } from "./shared-state.js";

const NEXT_TRACER = "next.js";

const SITE = "Next.js span exception";

const { claimed, recorded } = captureState();

function isObject(value: unknown): value is object {
  return value !== null && typeof value === "object";
}

/** Marks an error as captured by Interfere and withdraws the duplicate exception events Next's tracer recorded for it. */
export function claimException(
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Capture receives arbitrary thrown values; only objects have an identity to claim.
  error: unknown,
) {
  if (!isObject(error)) {
    return;
  }

  claimed.add(error);

  for (const { event, events } of recorded.get(error) ?? []) {
    const index = events.indexOf(event);

    if (index !== -1) {
      events.splice(index, 1);
    }
  }

  recorded.delete(error);
}

function wrapRecordException(span: Span) {
  const record = span.recordException.bind(span);

  // WARNING: Next's span event and Interfere's capture describe the same throw; recording both exports two exceptions per failure.
  span.recordException = (exception: Exception, time?: TimeInput) => {
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- OpenTelemetry's Exception union admits plain strings, which carry no identity to match.
    if (typeof exception === "string") {
      record(exception, time);

      return;
    }

    const skip = contain(
      SITE,
      () =>
        claimed.has(exception) || controlFlowOutcome(exception) !== undefined,
    );

    if (skip === true) {
      return;
    }

    record(exception, time);

    contain(SITE, () => {
      const event = span.events.at(-1);

      if (event?.name === "exception") {
        recorded.set(exception, [
          ...(recorded.get(exception) ?? []),
          { event, events: span.events },
        ]);
      }
    });
  };
}

/** Keeps one exception per throw by deferring Next's own span exception events to Interfere's capture. */
export const nextSpanExceptionProcessor = {
  async forceFlush() {
    await Promise.resolve();
  },
  // oxlint-disable-next-line eslint/no-empty-function
  onEnd() {},
  onStart(span: Span, _parentContext: Context) {
    contain(SITE, () => {
      if (span.instrumentationScope.name === NEXT_TRACER) {
        wrapRecordException(span);
      }
    });
  },
  async shutdown() {
    await Promise.resolve();
  },
} satisfies SpanProcessor;
