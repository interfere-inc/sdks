import { nativeBrowserFixture } from "@interfere/test-config/native-browser";
import { HttpResponse, http } from "msw";
import type { Page } from "playwright";

const MILLIS_PER_SECOND = 1000;

export interface SeenRequest {
  path: string;
  method: string;
  headers: Record<string, string | null>;
}

const TRACKED_HEADERS = [
  "baggage",
  "traceparent",
  "tracestate",
  "x-foreign-trace",
];

export const TRACEPARENT = /^00-[0-9a-f]{32}-[0-9a-f]{16}-0[01]$/u;

export const TRACE_ID = "4bf92f3577b34da6a3ce929d0e0e4736";

export const SPAN_ID = "00f067aa0ba902b7";

export function freshnessStamp() {
  return `interfere=${Math.floor(Date.now() / MILLIS_PER_SECOND)}`;
}

/** Serves the foreign-SDK fixture page and records every `/api/*` request's trace headers into `seen`. */
export function foreignSdkPages(seen: SeenRequest[]) {
  return nativeBrowserFixture(
    new URL("sdk-context.fixture.ts", import.meta.url),
    [
      http.all("*", ({ request }) => {
        const path = new URL(request.url).pathname;

        if (path.startsWith("/api/")) {
          seen.push({
            path,
            method: request.method,
            headers: Object.fromEntries(
              TRACKED_HEADERS.map((name) => [name, request.headers.get(name)]),
            ),
          });

          return new HttpResponse("ok");
        }

        if (path.startsWith("/drain/")) {
          return new HttpResponse(null, { status: 204 });
        }

        if (path.startsWith("/v3/session")) {
          return new HttpResponse("ok");
        }

        return new HttpResponse(null, { status: 404 });
      }),
    ],
  );
}

export function collectWarnings(page: Page) {
  const warnings: string[] = [];

  page.on("console", (message) => {
    if (message.type() === "warning") {
      warnings.push(message.text());
    }
  });

  return warnings;
}

type Fixture = typeof globalThis.foreignSdkContextFixture;

/** Node-side handle on the fixture page: every call runs inside the real browser. */
export function driver(page: Page) {
  return {
    install: async (opts: Parameters<Fixture["install"]>[0] = {}) =>
      page.evaluate(
        async (options) => globalThis.foreignSdkContextFixture.install(options),
        opts,
      ),
    foreign: async (
      kind: Parameters<Fixture["installForeign"]>[0],
      options?: Parameters<Fixture["installForeign"]>[1],
    ) =>
      page.evaluate(
        ([which, extra]) =>
          globalThis.foreignSdkContextFixture.installForeign(which, extra),
        [kind, options] as const,
      ),
    close: async () =>
      page.evaluate(async () => globalThis.foreignSdkContextFixture.close()),
    request: async (path: string) =>
      page.evaluate(
        async (target) => globalThis.foreignSdkContextFixture.request(target),
        path,
      ),
    xhr: async (path: string) =>
      page.evaluate(
        async (target) => globalThis.foreignSdkContextFixture.xhr(target),
        path,
      ),
    navigate: async (path: string) =>
      page.evaluate(
        (target) => globalThis.foreignSdkContextFixture.navigate(target),
        path,
      ),
    throwLater: async (message: string) =>
      page.evaluate(
        (text) => globalThis.foreignSdkContextFixture.throwLater(text),
        message,
      ),
    spans: async () =>
      page.evaluate(() => globalThis.foreignSdkContextFixture.spans()),
    pageViews: async () =>
      page.evaluate(() => globalThis.foreignSdkContextFixture.pageViews()),
    state: async () =>
      page.evaluate(() => globalThis.foreignSdkContextFixture.state()),
  };
}
