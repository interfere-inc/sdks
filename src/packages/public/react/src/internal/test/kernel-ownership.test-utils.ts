import { browserStorage } from "@interfere/test-config/browser-storage";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { context, propagation, trace } from "@opentelemetry/api";
import type {
  Context,
  TextMapPropagator,
  TextMapSetter,
} from "@opentelemetry/api";
import {
  InMemoryLogRecordExporter,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, vi } from "vitest";

import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { worker } from "../../../test/msw.js";
import { createKernel } from "../kernel.js";
import type { Kernel, KernelInternalOptions } from "../kernel.js";
import { REQUEST_OBSERVATION_EVENT_NAME } from "../otel/request-observer/request-observer-events.js";

export const API = "https://kernel.invalid";

export const PAGE_URL = `${location.origin}/ownership-test`;

export const FOREIGN_TRACE_ID = "12345678901234567890123456789012";

export const FOREIGN_TRACEPARENT = `00-${FOREIGN_TRACE_ID}-1234567890123456-01`;

export const OWN_TRACEPARENT = /^00-[0-9a-f]{32}-[0-9a-f]{16}-01$/u;

const VENDOR_GLOBALS = ["DD_RUM", "__SENTRY__"];

export interface SeenRequest {
  baggage: string | null;
  foreign: string | null;
  traceparent: string | null;
}

export class ForeignPropagator implements TextMapPropagator<
  Record<string, string>
> {
  inject(
    _context: Context,
    carrier: Record<string, string>,
    setter: TextMapSetter<Record<string, string>>,
  ) {
    setter.set(carrier, "x-foreign-trace", "foreign-1");
  }

  extract(active: Context) {
    return active;
  }

  fields() {
    return ["x-foreign-trace"];
  }
}

export async function sendXhr(
  url: string,
  headers: Record<string, string> = {},
  method = "GET",
) {
  return new Promise<void>((resolve) => {
    const xhr = new XMLHttpRequest();

    xhr.addEventListener("loadend", () => resolve(), { once: true });

    xhr.open(method, url);

    for (const [name, value] of Object.entries(headers)) {
      xhr.setRequestHeader(name, value);
    }

    xhr.send();
  });
}

export async function settle() {
  await new Promise<void>((resolve) => {
    setTimeout(resolve, 0);
  });
}

export function exporters() {
  const logs = new InMemoryLogRecordExporter();
  const spans = new InMemorySpanExporter();

  return {
    observations: () =>
      logs
        .getFinishedLogRecords()
        .filter(
          (record) => record.eventName === REQUEST_OBSERVATION_EVENT_NAME,
        ),
    options: {
      _internalAdditionalLogRecordProcessors: [
        new SimpleLogRecordProcessor({ exporter: logs }),
      ],
      _internalAdditionalSpanProcessors: [new SimpleSpanProcessor(spans)],
    },
    requestSpans: () =>
      spans
        .getFinishedSpans()
        .filter((span) => span.attributes["url.full"] === PAGE_URL),
  };
}

export function sessionBaggage(kernel: Kernel) {
  return `interfere.session.id=${kernel.session.getId()}`;
}

/** Replaces an own `traceparent` with `"own"` so expectations stay literal. */
export function tagged(entry: SeenRequest) {
  return {
    ...entry,
    traceparent:
      entry.traceparent !== null &&
      entry.traceparent !== FOREIGN_TRACEPARENT &&
      OWN_TRACEPARENT.test(entry.traceparent)
        ? "own"
        : entry.traceparent,
  };
}

/** The trace id of the `traceparent` each request left with, in wire order. */
export function traceIdsLeft(seen: SeenRequest[]) {
  return seen.map((entry) => entry.traceparent?.split("-")[1] ?? null);
}

/** Warnings carrying the auto-mode vendor diagnostic. */
export function vendorDiagnostics(warn: { mock: { calls: unknown[][] } }) {
  return warn.mock.calls.filter(
    ([text]) =>
      String(text).includes("owns tracing on this page") ||
      String(text).includes("another propagator owns trace headers"),
  );
}

/**
 * Serves the page URL through msw, records the trace headers each request
 * carried, resets OpenTelemetry globals around every test and disposes the
 * kernel a test booted. Vendor globals a test installs are removed.
 */
export function ownershipHarness() {
  let ownedKernel: Kernel | undefined;
  let seen: SeenRequest[] = [];

  beforeEach(() => {
    ownedKernel = undefined;

    seen = [];

    browserStorage.session.remove("interfere:propagation_proven");

    browserStorage.session.remove("interfere:propagation_denied");

    setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

    setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

    trace.disable();

    context.disable();

    propagation.disable();

    worker.use(
      http.get(`${API}/v3/config`, () =>
        HttpResponse.json({ plugins: DISABLE_ALL_PLUGINS }),
      ),
      http.all(`${API}/*`, () => new HttpResponse(null, { status: 204 })),
      http.get(PAGE_URL, ({ request }) => {
        seen.push({
          baggage: request.headers.get("baggage"),
          foreign: request.headers.get("x-foreign-trace"),
          traceparent: request.headers.get("traceparent"),
        });

        return new HttpResponse(null, { status: 204 });
      }),
    );
  });

  afterEach(async () => {
    try {
      await ownedKernel?.dispose();
    } finally {
      trace.disable();

      context.disable();

      propagation.disable();

      vi.restoreAllMocks();

      for (const name of VENDOR_GLOBALS) {
        Reflect.deleteProperty(globalThis, name);
      }
    }
  });

  return {
    boot: async (opts: Omit<KernelInternalOptions, "apiHost"> = {}) => {
      const kernel = await createKernel({ opts: { ...opts, apiHost: API } });

      ownedKernel = kernel;

      return kernel;
    },
    dispose: async () => {
      await ownedKernel?.dispose();

      ownedKernel = undefined;
    },
    seen: () => seen,
  };
}
