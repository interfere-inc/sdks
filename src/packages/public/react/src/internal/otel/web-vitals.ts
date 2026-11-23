/* oxlint-disable typescript/no-unnecessary-condition */
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { createLogger } from "../../util/log.js";

import type { Histogram, Meter } from "@opentelemetry/api";
import { onCLS, onFCP, onINP, onLCP, onTTFB } from "web-vitals";
import type { Metric } from "web-vitals";

import { identityAttributes } from "../../tracking/view.js";

const log = createLogger("web-vitals");

const ATTR_URL_PATH = "url.path" as const;

const MS_TO_S = 1 / 1000;

const LOAD_VITALS = new Set(["TTFB", "FCP", "LCP"]);

const subscribers = new Set<(metric: Metric) => void>();
let observing = false;

function publish(metric: Metric) {
  for (const subscriber of subscribers) {
    subscriber(metric);
  }
}

interface VitalEntry {
  histogram: Histogram;
  toSeconds: boolean;
}

export interface WebVitalsInput {
  /**
   * Called after the page-load vitals have all fired. The kernel passes
   * its own `flush()` so short-visit exports don't depend on the
   * `visibilitychange` listener firing.
   */
  flush: () => Promise<void>;
  /** Consented device id at record time. */
  getDeviceId?: () => string | null;
  /** Session and view ids at record time; each data point carries them. */
  getIdentity?: () => { sessionId: string | null; viewId: string | null };
  /** OTel meter from the kernel's private provider. */
  meter: Meter;
  /**
   * Pathname → low-cardinality route template (e.g. `/blog/[slug]`).
   * Without this, dynamic routes produce one unique `url.path` label
   * per visited slug — a metric cardinality hazard.
   */
  resolveRoute?: (pathname: string) => string | undefined;
}

/**
 * Subscribes to Core Web Vitals (LCP/FCP/TTFB/INP/CLS) and records OTel
 * histograms. Time-based vitals are converted to seconds per
 * observability convention; CLS is dimensionless and recorded as-is.
 *
 * Each record carries `web_vital.rating`, `web_vital.navigation_type`,
 * `url.path` (templated when `resolveRoute` is provided), and the
 * session, view and consented device ids at record time.
 *
 * Bucket boundaries match the rum.ts SDK — same dashboards, same
 * histograms, same percentile shape.
 */
export function captureWebVitals(input: WebVitalsInput) {
  if (!("document" in globalThis)) {
    return () => undefined;
  }

  const { meter, resolveRoute, flush } = input;

  const lcp = meter.createHistogram("web_vitals.lcp", {
    advice: {
      explicitBucketBoundaries: [0.5, 1, 1.5, 2, 2.5, 3, 4, 5, 7.5, 10],
    },
    description: "Largest Contentful Paint",
    unit: "s",
  });

  const fcp = meter.createHistogram("web_vitals.fcp", {
    advice: {
      explicitBucketBoundaries: [0.5, 1, 1.5, 1.8, 2, 2.5, 3, 4, 5, 7.5],
    },
    description: "First Contentful Paint",
    unit: "s",
  });

  const ttfb = meter.createHistogram("web_vitals.ttfb", {
    advice: {
      explicitBucketBoundaries: [0.1, 0.2, 0.3, 0.5, 0.8, 1, 1.5, 2, 3, 5],
    },
    description: "Time to First Byte",
    unit: "s",
  });

  const inp = meter.createHistogram("web_vitals.inp", {
    advice: {
      explicitBucketBoundaries: [
        0.05, 0.1, 0.15, 0.2, 0.3, 0.4, 0.5, 0.75, 1, 2,
      ],
    },
    description: "Interaction to Next Paint",
    unit: "s",
  });

  const cls = meter.createHistogram("web_vitals.cls", {
    advice: {
      explicitBucketBoundaries: [
        0.01, 0.025, 0.05, 0.075, 0.1, 0.15, 0.2, 0.25, 0.5, 1,
      ],
    },
    description: "Cumulative Layout Shift",
    unit: "1",
  });

  const vitals = {
    CLS: { histogram: cls, toSeconds: false },
    FCP: { histogram: fcp, toSeconds: true },
    INP: { histogram: inp, toSeconds: true },
    LCP: { histogram: lcp, toSeconds: true },
    TTFB: { histogram: ttfb, toSeconds: true },
  } satisfies Record<Metric["name"], VitalEntry>;

  const received = new Set<string>();

  let flushed = false;

  function report(metric: Metric) {
    const entry = vitals[metric.name];

    if (entry === undefined) {
      return;
    }

    const value = entry.toSeconds ? metric.value * MS_TO_S : metric.value;
    const { pathname } = globalThis.location;
    const route = resolveRoute?.(pathname) ?? pathname;
    const identity = input.getIdentity?.();

    entry.histogram.record(
      value,
      omitUndefined({
        [ATTR_URL_PATH]: route,
        "web_vital.rating": metric.rating || undefined,
        "web_vital.navigation_type": metric.navigationType || undefined,
        ...identityAttributes({
          getDeviceId: input.getDeviceId,
          getSessionId: () => identity?.sessionId ?? null,
          getViewId: () => identity?.viewId ?? null,
        }),
      }),
    );

    if (!flushed && LOAD_VITALS.has(metric.name)) {
      received.add(metric.name);

      if (received.size === LOAD_VITALS.size) {
        flushed = true;

        // oxlint-disable-next-line promise/prefer-await-to-then
        flush().catch(log.debug);
      }
    }
  }

  subscribers.add(report);

  if (!observing) {
    observing = true;

    onLCP(publish);

    onCLS(publish);

    onINP(publish);

    onTTFB(publish);

    onFCP(publish);
  }

  return () => {
    subscribers.delete(report);
  };
}
