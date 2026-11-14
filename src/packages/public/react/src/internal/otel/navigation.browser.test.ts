/* oxlint-disable unicorn/no-useless-promise-resolve-reject -- LogRecordProcessor requires promise-returning flush and shutdown. */
/* oxlint-disable interfere-testing/no-test-doubles -- A real passthrough wrapper models a later SDK; browser history behavior is never replaced. */
/* oxlint-disable typescript/unbound-method -- Identity assertions compare wrapper references without invoking them unbound. */
import { INTERFERE_BROWSER_EVENTS_LOGGER_NAME } from "@interfere/types/data/browser-event-loggers";
import { PAGE_VIEW_EVENT_NAME } from "@interfere/types/data/correlation-events";

import { ROOT_CONTEXT, TraceFlags, trace } from "@opentelemetry/api";
import {
  InMemoryLogRecordExporter,
  LoggerProvider,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";

import { ViewIds, identityAttributes } from "../../tracking/view.js";
import { installViewNavigation, installViewTracker } from "./navigation.js";
import { afterEach, beforeEach, describe, expect, test } from "vitest";

const TRACE_ID = "0af7651916cd43dd8448eb211c80319c";
const SPAN_ID = "b7ad6b7169203331";

let exporter: InMemoryLogRecordExporter;
let views: ViewIds;
let clock: number;
let enabled: boolean;
let tracker: ReturnType<typeof installViewTracker> | null = null;
let previousUrl: string;
let previousState: unknown;

function install(
  documentContext = null as Parameters<
    typeof installViewTracker
  >[0]["documentContext"],
) {
  const loggerProvider = new LoggerProvider({
    processors: [
      {
        forceFlush: async () => Promise.resolve(),
        onEmit: (record) => {
          record.setAttributes(
            identityAttributes({ getViewId: () => views.viewId() }),
          );
        },
        shutdown: async () => Promise.resolve(),
      },
      new SimpleLogRecordProcessor({ exporter }),
    ],
  });

  tracker = installViewTracker({
    documentContext,
    isEnabled: () => enabled,
    loggerProvider,
    redactUrl: (url) => url.replace(/token=[^&]+/u, "token=[redacted]"),
    resolveRoute: (pathname) =>
      pathname.startsWith("/items/") ? "/items/[id]" : undefined,
    views,
  });

  return tracker;
}

function pageViews() {
  return exporter.getFinishedLogRecords().map((record) => ({
    eventName: record.eventName,
    scope: record.instrumentationScope.name,
    trigger: record.attributes["interfere.view.trigger"],
    type: record.attributes["browser.navigation.type"],
    viewId: record.attributes["interfere.view.id"],
    url: record.attributes["url.full"],
    route: record.attributes["http.route"],
    traceId: record.spanContext?.traceId,
    traceFlags: record.spanContext?.traceFlags,
  }));
}

beforeEach(() => {
  previousUrl = globalThis.location.href;

  previousState = globalThis.history.state;

  globalThis.history.replaceState(null, "", "/");

  exporter = new InMemoryLogRecordExporter();

  clock = 1_000_000;

  enabled = true;

  views = new ViewIds({ now: () => clock });
});

afterEach(() => {
  tracker?.dispose();

  tracker = null;

  globalThis.history.replaceState(previousState, "", previousUrl);
});

describe(installViewTracker, () => {
  test("retains a startup navigation until page-view capture is ready", () => {
    const initial = views.viewId();
    const disposeStartup = installViewNavigation(views);

    globalThis.history.pushState(null, "", "/during-config");

    disposeStartup();

    const rotated = views.viewId();

    expect(rotated).not.toBe(initial);

    install().recordCurrentView();

    tracker?.recordCurrentView();

    expect(pageViews()).toStrictEqual([
      expect.objectContaining({ trigger: "push", viewId: rotated }),
    ]);
  });

  test("deduplicates each current view across consent and max-age rotations", () => {
    const current = install();

    for (let index = 0; index < 100; index += 1) {
      clock += 30 * 60 * 1000 + 1;

      current.recordCurrentView();

      enabled = false;

      current.recordCurrentView();

      enabled = true;

      current.recordCurrentView();
    }

    expect(pageViews()).toHaveLength(100);
    expect(new Set(pageViews().map((row) => row.viewId)).size).toBe(100);

    expect(Object.keys(current).toSorted()).toStrictEqual([
      "dispose",
      "recordCurrentView",
    ]);
  });

  test("emits the initial page view once, with the view minted at load", () => {
    const initial = views.viewId();

    install().recordCurrentView();

    tracker?.recordCurrentView();

    expect(pageViews()).toStrictEqual([
      expect.objectContaining({
        eventName: PAGE_VIEW_EVENT_NAME,
        scope: INTERFERE_BROWSER_EVENTS_LOGGER_NAME,
        trigger: "pageload",
        type: "pageload",
        viewId: initial,
        traceId: undefined,
      }),
    ]);
  });

  test.each([
    ["01", TraceFlags.SAMPLED],
    ["00", TraceFlags.NONE],
  ] as const)(
    "the initial page view carries the fresh SSR context with flags %s",
    (_, flags) => {
      install(
        trace.setSpanContext(ROOT_CONTEXT, {
          traceId: TRACE_ID,
          spanId: SPAN_ID,
          traceFlags: flags,
          isRemote: true,
        }),
      ).recordCurrentView();

      globalThis.history.pushState(null, "", "/next");

      const [initial, next] = pageViews();

      expect(initial).toMatchObject({ traceId: TRACE_ID, traceFlags: flags });

      expect(next?.traceId).toBeUndefined();
    },
  );

  test("starts a new view per real URL change and skips hash-only and same-URL changes", () => {
    install().recordCurrentView();

    globalThis.history.pushState(null, "", "/items/42?token=secret");

    globalThis.history.pushState(null, "", "/items/42?token=secret#tab");

    globalThis.history.replaceState(null, "", "/items/42/?token=secret");

    globalThis.history.replaceState(null, "", "/other");

    const rows = pageViews();

    expect(rows.map((row) => [row.trigger, row.type])).toStrictEqual([
      ["pageload", "pageload"],
      ["push", "push"],
      ["replace", "replace"],
    ]);

    expect(rows[1]).toMatchObject({
      route: "/items/[id]",
      url: `${globalThis.location.origin}/items/42?token=[redacted]`,
    });

    expect(new Set(rows.map((row) => row.viewId)).size).toBe(3);

    expect(rows.at(-1)?.viewId).toBe(views.viewId());
  });

  test("popstate starts a traverse view", () => {
    const unwrappedReplace = globalThis.history.replaceState;

    install();

    unwrappedReplace.call(globalThis.history, null, "", "/back");

    globalThis.dispatchEvent(new PopStateEvent("popstate"));

    expect(pageViews().map((row) => [row.trigger, row.type])).toStrictEqual([
      ["popstate", "traverse"],
    ]);
  });

  test("a parked tab rotates to a max_age view after 30 minutes on one URL", () => {
    install().recordCurrentView();

    const first = views.viewId();

    clock += 30 * 60 * 1000 + 1;

    const second = views.viewId();

    expect(second).not.toBe(first);

    expect(pageViews().at(-1)).toMatchObject({
      trigger: "max_age",
      type: undefined,
      viewId: second,
    });
  });

  test("view ids still change while page events are disabled, and no event is emitted", () => {
    enabled = false;

    install().recordCurrentView();

    const before = views.viewId();

    globalThis.history.pushState(null, "", "/quiet");

    expect(views.viewId()).not.toBe(before);

    expect(pageViews()).toStrictEqual([]);

    enabled = true;

    tracker?.recordCurrentView();

    expect(pageViews().map((row) => row.trigger)).toStrictEqual(["push"]);
  });

  test("dispose restores history methods only while they are still ours", () => {
    const originalPush = globalThis.history.pushState;
    const originalReplace = globalThis.history.replaceState;

    const first = install();

    const ours = globalThis.history.pushState;

    const foreign: History["pushState"] = function foreign(
      this: History,
      ...args: Parameters<History["pushState"]>
    ) {
      ours.apply(this, args);
    };

    globalThis.history.pushState = foreign;

    first.dispose();

    tracker = null;

    expect(globalThis.history.pushState).toBe(foreign);
    expect(globalThis.history.replaceState).toBe(originalReplace);

    const before = views.viewId();

    globalThis.history.pushState(null, "", "/after-dispose");

    expect(views.viewId()).toBe(before);

    globalThis.history.pushState = originalPush;
  });
});
