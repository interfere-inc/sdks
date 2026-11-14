import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { HttpResponse, http } from "msw";
import { expect, test, vi } from "vitest";

import { worker } from "../../../../test/msw.js";
import { requestSpans } from "../../../../test/request-spans.test-utils.js";
import { SessionManager } from "../../../tracking/session.js";
import { ActivityMonitor, EXPIRE_AFTER_MS } from "../../activity/activity.js";
import {
  ALLOWED,
  FOREIGN,
  propagationHarness,
} from "./propagation.test-utils.js";

const harness = propagationHarness();

function siblingOrigin() {
  const sibling = new URL(globalThis.location.origin);

  sibling.port = sibling.port === "1" ? "2" : "1";

  return sibling.origin;
}

test.each([
  ["page origin", () => globalThis.location.origin, true],
  ["allowlisted origin", () => ALLOWED, true],
  ["unlisted origin", () => "https://unlisted.cors.test", false],
  ["same-site sibling origin", siblingOrigin, false],
  ["different scheme", () => "http://allowed.cors.test", false],
  ["different port", () => "https://allowed.cors.test:8443", false],
])(
  "injects only approved exact origins: %s",
  async (_name, origin, injected) => {
    harness.install();

    const seen: (string | null)[] = [];
    const url = `${origin()}/orders`;

    worker.use(
      http.get(url, ({ request }) => {
        seen.push(request.headers.get("traceparent"));

        return new HttpResponse(null, { status: 204 });
      }),
    );

    await fetch(url);

    expect(seen).toHaveLength(1);
    expect(Boolean(seen[0])).toBe(injected);
  },
);

test.each([
  ["GET", undefined, ""],
  ["HEAD", undefined, ""],
  ["POST", "customer-order", "customer-order"],
] as const)(
  "cors-style TypeError produces one %s attempt, one stamped diagnostic, and marks the origin header-less for the session",
  async (method, body, expectedBody) => {
    expect.hasAssertions();

    harness.install();

    const seen: { traceparent: string | null; body: string }[] = [];

    worker.use(
      http.all(`${ALLOWED}/*`, async ({ request }) => {
        seen.push({
          traceparent: request.headers.get("traceparent"),
          body: await request.text(),
        });

        return HttpResponse.error();
      }),
    );

    const request = new Request(
      `${ALLOWED}/orders`,
      omitUndefined({ method, body }),
    );

    await expect(fetch(request)).rejects.toThrow(TypeError);

    expect(seen).toHaveLength(1);

    await expect(fetch(`${ALLOWED}/again`, { method })).rejects.toThrow(
      TypeError,
    );

    expect(seen.map(({ traceparent }) => traceparent !== null)).toStrictEqual([
      true,
      false,
    ]);

    expect(seen[0]?.body).toBe(expectedBody);

    expect(console.warn).toHaveBeenCalledExactlyOnceWith(
      `[interfere] Requests to ${ALLOWED} fail at the network layer while carrying trace headers. If ${ALLOWED} does not allow "traceparent, tracestate, baggage" in Access-Control-Allow-Headers, add them there or give ${ALLOWED} a narrower headers list in tracing.propagateTo. Interfere did not retry the request and sends no trace headers to ${ALLOWED} for the rest of this browser session.`,
    );

    expect(
      (await harness.failedRequests()).map(
        (span) => span.attributes["interfere.propagation.suspect_cors"],
      ),
    ).toStrictEqual([true]);
  },
);

test.each([true, false])(
  "a completed response suppresses the diagnostic with failed-request capture %s",
  async (captureFailedRequests) => {
    harness.install({ captureFailedRequests });

    worker.use(
      http.get(
        `${ALLOWED}/success`,
        () => new HttpResponse(null, { status: 204 }),
      ),
    );

    await fetch(`${ALLOWED}/success`);

    worker.use(http.get(`${ALLOWED}/failure`, () => HttpResponse.error()));

    await expect(fetch(`${ALLOWED}/failure`)).rejects.toThrow(TypeError);

    expect(console.warn).not.toHaveBeenCalled();
  },
);

test("an origin marked header-less stays so across session rotation and the warning does not repeat", async () => {
  expect.hasAssertions();

  vi.useFakeTimers({ toFake: ["Date"] });

  const session = new SessionManager();
  const monitor = new ActivityMonitor();

  const fail = async () =>
    expect(fetch(`${ALLOWED}/failure`)).rejects.toThrow(TypeError);

  harness.watch(monitor);

  monitor.start({
    markActivity: () => session.markActivity(),
    reset: () => session.reset(),
    startedAt: () => session.getSessionStartedAt(),
    lastActivityAt: () => session.getLastActivityAt(),
  });

  harness.install({
    onSessionResume: (handler) => monitor.onResume(handler),
  });

  const seen: (string | null)[] = [];

  worker.use(
    http.all(`${ALLOWED}/*`, ({ request }) => {
      seen.push(request.headers.get("traceparent"));

      return request.url.endsWith("/failure")
        ? HttpResponse.error()
        : new HttpResponse(null, { status: 204 });
    }),
  );

  await fail();

  expect(console.warn).toHaveBeenCalledTimes(1);

  await fetch(`${ALLOWED}/success`);

  const previousSession = session.getSessionId();

  vi.setSystemTime(Date.now() + EXPIRE_AFTER_MS);

  globalThis.dispatchEvent(new PointerEvent("pointerdown"));

  expect(session.getSessionId()).not.toBe(previousSession);

  await fail();

  await fail();

  expect(console.warn).toHaveBeenCalledTimes(1);

  expect(seen.map((traceparent) => traceparent !== null)).toStrictEqual([
    true,
    false,
    false,
    false,
  ]);
});

test("an open streaming response is a diagnostic success once its headers arrive, with failed-request capture disabled", async () => {
  harness.install({ captureFailedRequests: false });

  const finish = Promise.withResolvers<undefined>();

  worker.use(
    http.get(
      `${ALLOWED}/stream`,
      () =>
        new HttpResponse(
          new ReadableStream({
            async start(controller) {
              controller.enqueue(new TextEncoder().encode("streamed response"));

              await finish.promise;

              controller.close();
            },
          }),
        ),
    ),
    http.get(`${ALLOWED}/failure`, () => HttpResponse.error()),
  );

  const response = await fetch(`${ALLOWED}/stream`);

  try {
    await expect(fetch(`${ALLOWED}/failure`)).rejects.toThrow(TypeError);

    expect(console.warn).not.toHaveBeenCalled();
  } finally {
    finish.resolve(undefined);
  }

  await expect(response.text()).resolves.toBe("streamed response");
});

test("preserves foreign headers on unlisted origins without diagnosing them", async () => {
  harness.install();

  const seen: (string | null)[] = [];

  worker.use(
    http.get("https://foreign.test/orders", ({ request }) => {
      seen.push(request.headers.get("traceparent"));

      return HttpResponse.error();
    }),
  );

  await expect(
    fetch("https://foreign.test/orders", { headers: { traceparent: FOREIGN } }),
  ).rejects.toThrow(TypeError);

  expect(seen).toStrictEqual([FOREIGN]);
  expect(console.warn).not.toHaveBeenCalled();
});

/* oxlint-disable interfere-testing/no-test-doubles -- Real passthrough wrappers model a later SDK; native request behavior is never replaced. */
/* oxlint-disable typescript/unbound-method -- Identity assertions compare references without invoking them unbound. */
test("later foreign fetch and XHR wrappers survive disposal while Interfere becomes inert", async () => {
  const originalFetch = globalThis.fetch;
  const originalOpen = XMLHttpRequest.prototype.open;
  const originalSend = XMLHttpRequest.prototype.send;

  harness.install();

  const interfereFetch = globalThis.fetch;
  const interfereOpen = XMLHttpRequest.prototype.open;
  const interfereSend = XMLHttpRequest.prototype.send;

  const foreignFetch = vi.fn<typeof fetch>(async (...args) =>
    interfereFetch(...args),
  );

  const foreignOpen = vi.fn<
    (this: XMLHttpRequest, method: string, url: string | URL) => void
  >(function open(this: XMLHttpRequest, method: string, url: string | URL) {
    interfereOpen.call(this, method, url, true);
  });

  const foreignSend = vi.fn<
    (this: XMLHttpRequest, body?: XMLHttpRequestBodyInit | null) => void
  >(function send(this: XMLHttpRequest, body?: XMLHttpRequestBodyInit | null) {
    interfereSend.call(this, body);
  });

  globalThis.fetch = foreignFetch;

  XMLHttpRequest.prototype.open = foreignOpen;
  XMLHttpRequest.prototype.send = foreignSend;

  const seen: (string | null)[] = [];

  worker.use(
    http.get(`${ALLOWED}/after`, ({ request }) => {
      seen.push(request.headers.get("traceparent"));

      return new HttpResponse(null, { status: 204 });
    }),
  );

  try {
    harness.instrumentations()?.dispose();

    expect(globalThis.fetch).toBe(foreignFetch);
    expect(XMLHttpRequest.prototype.open).toBe(foreignOpen);
    expect(XMLHttpRequest.prototype.send).toBe(foreignSend);

    await fetch(`${ALLOWED}/after`);

    await new Promise<void>((resolve, reject) => {
      const xhr = new XMLHttpRequest();

      xhr.addEventListener("load", () => resolve());

      xhr.addEventListener("error", () => reject(new Error("XHR failed")));

      xhr.open("GET", `${ALLOWED}/after`);

      xhr.send();
    });

    expect(foreignFetch).toHaveBeenCalledTimes(1);
    expect(foreignOpen).toHaveBeenCalledTimes(1);
    expect(foreignSend).toHaveBeenCalledTimes(1);
    expect(seen).toStrictEqual([null, null]);
    expect(requestSpans(harness.exporter())).toHaveLength(0);
  } finally {
    globalThis.fetch = originalFetch;

    XMLHttpRequest.prototype.open = originalOpen;
    XMLHttpRequest.prototype.send = originalSend;
  }
});
