import { trace } from "@opentelemetry/api";
import type { Span, TracerProvider } from "@opentelemetry/api";

import { settleFlush } from "@interfere/server-core/shutdown";

const SETTLE_CAP_MS = 2000;

const SPAN_END_POLL_MS = 10;

const NEXT_REQUEST_CONTEXT = Symbol.for("@next/request-context");

interface NextRequestContext {
  readonly get?: () =>
    | { readonly waitUntil?: (promise: Promise<unknown>) => void }
    | undefined;
}

let flushFn: (() => Promise<void>) | null = null;

export function setServerFlush(fn: () => Promise<void>) {
  flushFn = fn;
}

interface DelegatingProvider extends TracerProvider {
  getDelegate: () => TracerProvider;
}

interface FlushingProvider extends TracerProvider {
  forceFlush: () => Promise<void>;
}

function isDelegating(
  provider: TracerProvider,
): provider is DelegatingProvider {
  return "getDelegate" in provider;
}

function isFlushing(provider: TracerProvider): provider is FlushingProvider {
  return "forceFlush" in provider;
}

async function flushGlobalProvider() {
  const global = trace.getTracerProvider();

  const delegate = isDelegating(global) ? global.getDelegate() : global;

  if (isFlushing(delegate)) {
    await delegate.forceFlush();
  }
}

async function ended(span: Span | undefined) {
  const deadline = Date.now() + SETTLE_CAP_MS;

  do {
    // oxlint-disable-next-line eslint/no-await-in-loop -- The request span exposes no end signal; poll it until Next ends it or the cap passes.
    await new Promise((resolve) => {
      setTimeout(resolve, SPAN_END_POLL_MS);
    });
  } while (span?.isRecording() === true && Date.now() < deadline);
}

function platformWaitUntil() {
  const host: typeof globalThis & {
    readonly [NEXT_REQUEST_CONTEXT]?: NextRequestContext;
  } = globalThis;

  const context = host[NEXT_REQUEST_CONTEXT];

  return context?.get?.()?.waitUntil;
}

/** Flushes once the request span holding the capture ends, without delaying the response. */
// oxlint-disable-next-line typescript/require-await -- Callers await scheduling only; delivery runs after the response.
export async function flushServerTelemetry() {
  // WARNING: Next bundles the proxy separately, where register() never ran; without the global provider fallback its captures wait for the batch timer.
  const flush = flushFn ?? flushGlobalProvider;

  const request = trace.getActiveSpan();

  const delivery = async () => {
    await ended(request);

    await flush();
  };

  const settled = settleFlush(delivery);

  platformWaitUntil()?.(settled);
}
