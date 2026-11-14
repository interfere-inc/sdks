import { setupWorker } from "msw/browser";
import { afterAll, beforeAll, beforeEach } from "vitest";

export const network = setupWorker();

const unexpectedRequests: string[] = [];

const inflight = new Set<Promise<Response>>();

const originalFetch = globalThis.fetch;

async function forget(response: Promise<Response>) {
  await Promise.allSettled([response]);

  inflight.delete(response);
}

// WARNING: the service worker relays a request to this page a few milliseconds after fetch() is called, so a request the previous test fired and forgot reaches the handler lookup after resetHandlers() and is reported as the next test's unexpected request; every fetch the page started is awaited before the reset (held responses are released in onTestFinished, which runs before the next beforeEach).
// oxlint-disable-next-line interfere-testing/no-test-doubles -- Observation only: every call is delegated to the real fetch and only its returned promise is recorded so the reset can wait for it.
globalThis.fetch = Object.assign(
  async (input: RequestInfo | URL, init?: RequestInit) => {
    const response = originalFetch(input, init);

    inflight.add(response);

    void forget(response);

    return response;
  },
  originalFetch,
);

async function settleInflightFetches() {
  if (inflight.size === 0) {
    return;
  }

  await Promise.allSettled(inflight);

  await settleInflightFetches();
}

export const requests: ReturnType<Request["clone"]>[] = [];

const arrivals = new Set<{
  count: number;
  resolve: (value: undefined) => void;
}>();

export async function requested(count: number) {
  if (requests.length >= count) {
    return;
  }

  const arrival = Promise.withResolvers<undefined>();

  arrivals.add({ count, resolve: arrival.resolve });

  await arrival.promise;
}

beforeAll(async () => {
  const pendingRequests = new Map<string, ReturnType<Request["clone"]>>();

  network.events.on("request:start", ({ request, requestId }) => {
    pendingRequests.set(requestId, request.clone());
  });

  network.events.on("request:match", ({ requestId }) => {
    const request = pendingRequests.get(requestId);

    if (!request) {
      throw new Error("Missing captured network request");
    }

    requests.push(request);

    for (const arrival of arrivals) {
      if (requests.length >= arrival.count) {
        arrivals.delete(arrival);

        arrival.resolve(undefined);
      }
    }
  });

  network.events.on("request:end", ({ requestId }) => {
    pendingRequests.delete(requestId);
  });

  await network.start({
    quiet: true,
    onUnhandledRequest(request, print) {
      const url = new URL(request.url);

      if (
        (url.origin === globalThis.location.origin &&
          (/^(?:\/@(?:fs|id|vite)\/|\/__vitest|\/node_modules\/|\/src\/|\/test\/|\/mockServiceWorker\.js$|\/favicon\.ico$)/u.test(
            url.pathname,
          ) ||
            (url.searchParams.has("browserv") &&
              /\.browser\.test\.[cm]?[jt]sx?$/u.test(url.pathname)))) ||
        /^https:\/\/clerk-[\da-f-]+\.clerk\.accounts\.dev\//u.test(request.url)
      ) {
        return;
      }

      unexpectedRequests.push(`${request.method} ${request.url}`);

      print.error();
    },
    serviceWorker: { url: "/mockServiceWorker.js" },
  });
});

beforeEach(async ({ onTestFinished }) => {
  await settleInflightFetches();

  requests.length = 0;

  arrivals.clear();

  unexpectedRequests.length = 0;

  network.resetHandlers();

  onTestFinished(() => {
    if (unexpectedRequests.length > 0) {
      throw new Error(
        `Unexpected network requests: ${unexpectedRequests.join(", ")}`,
      );
    }
  });
});

afterAll(async () => {
  await settleInflightFetches();

  network.stop();
});
