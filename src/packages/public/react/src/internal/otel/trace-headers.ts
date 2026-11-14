import { bytesToHex } from "@interfere/helpers/hex";
import {
  ROOT_CONTEXT,
  TraceFlags,
  defaultTextMapSetter,
  isSpanContextValid,
  trace,
} from "@opentelemetry/api";
import type { Context, TextMapPropagator } from "@opentelemetry/api";
import { W3CTraceContextPropagator } from "@opentelemetry/core";

import type { HeaderSet } from "./instrumentations/header-policy.js";

const TRACEPARENT = "traceparent";

const TRACESTATE = "tracestate";

const BAGGAGE = "baggage";

const W3C_TRACE_CONTEXT = new W3CTraceContextPropagator();

/** Request headers as the writer sees them: readable by name and append-only, like `Headers`. */
export interface HeaderCarrier {
  append: (name: string, value: string) => void;
  get: (name: string) => string | null;
}

/** Writes the trace headers `allowed` permits into a carrier and returns the span context of the `traceparent` the request leaves with, or `undefined` when it carries none. */
export type TraceHeaderWriter = (
  ctx: Context,
  carrier: HeaderCarrier,
  allowed: HeaderSet,
) => ReturnType<typeof trace.getSpanContext>;

const getter = {
  get: (carrier: HeaderCarrier, key: string) => carrier.get(key) ?? undefined,
  keys: () => [],
};

export function readTraceContext(carrier: Pick<HeaderCarrier, "get">) {
  const value = carrier.get(TRACEPARENT);

  if (!value || !/^00-[0-9a-f]{32}-[0-9a-f]{16}-[0-9a-f]{2}$/u.test(value)) {
    return;
  }

  return trace.getSpanContext(
    W3C_TRACE_CONTEXT.extract(ROOT_CONTEXT, carrier, getter),
  );
}

function randomId(length: number) {
  const bytes = crypto.getRandomValues(new Uint8Array(length));

  if (bytes.every((byte) => byte === 0)) {
    bytes[length - 1] = 1;
  }

  return bytesToHex(bytes);
}

function propagationContext(ctx: Context) {
  const active = trace.getSpanContext(ctx);

  if (active && isSpanContextValid(active)) {
    return ctx;
  }

  return trace.setSpanContext(ctx, {
    spanId: randomId(8),
    traceFlags: TraceFlags.SAMPLED,
    traceId: randomId(16),
  });
}

/**
 * Writes trace context only when the request carries no `traceparent`, so a
 * header another tool wrote first is never replaced, and appends baggage as
 * further list members, so an existing `baggage` header keeps every entry.
 * The per-request `allowed` set from the propagation policy decides which
 * of `traceparent`, `tracestate` and `baggage` are written at all; an
 * empty set writes nothing and still reports the context the request
 * leaves with. The context written is the active span's; a request sent
 * with no valid active span leaves with a fresh sampled root, so the
 * server span continues a trace the request observation also records.
 * Suppressed tracing writes no trace context.
 */
export function createTraceHeaderWriter(input: {
  baggage: TextMapPropagator | null;
  traceContext: TextMapPropagator;
}) {
  const { baggage, traceContext } = input;

  const write: TraceHeaderWriter = (ctx, carrier, allowed) => {
    if (allowed.has(TRACEPARENT) && carrier.get(TRACEPARENT) === null) {
      traceContext.inject(propagationContext(ctx), carrier, {
        set: (target: HeaderCarrier, key: string, value: string) => {
          if (key !== TRACESTATE || allowed.has(TRACESTATE)) {
            target.append(key, value);
          }
        },
      });
    }

    if (allowed.has(BAGGAGE)) {
      const serialized: Record<string, string> = {};

      baggage?.inject(ctx, serialized, defaultTextMapSetter);

      const value = serialized[BAGGAGE];

      if (value !== undefined) {
        carrier.append(BAGGAGE, value);
      }
    }

    return trace.getSpanContext(
      W3C_TRACE_CONTEXT.extract(ROOT_CONTEXT, carrier, getter),
    );
  };

  return write;
}
