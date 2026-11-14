import { setupWorker } from "msw/browser";
import type { RequestHandler } from "msw";
import { afterEach } from "vitest";

const baseline: RequestHandler[] = [];

export const worker = setupWorker(...baseline);

const unhandled: string[] = [];

const inflight = new Set<Promise<Response>>();

const originalFetch = globalThis.fetch;

async function readyFetch(...args: Parameters<typeof fetch>) {
  await ready;

  return originalFetch(...args);
}

async function forget(response: Promise<Response>) {
  await Promise.allSettled([response]);

  inflight.delete(response);
}

// WARNING: the service worker relays a request to this page a few milliseconds after fetch() is called, so a request a test fires and forgets (session sync, identify) reaches the handler lookup after resetHandlers() and is reported unhandled; every fetch the page started is awaited before the reset.
// oxlint-disable-next-line interfere-testing/no-test-doubles -- Observation only: every call is delegated to the real fetch and only its returned promise is recorded so the reset can wait for it.
globalThis.fetch = async (input, init) => {
  const response = readyFetch(input, init);

  inflight.add(response);

  void forget(response);

  return response;
};

async function settleInflightFetches() {
  if (inflight.size === 0) {
    return;
  }

  await Promise.allSettled(inflight);

  await settleInflightFetches();
}

export function addBaselineHandlers(...handlers: RequestHandler[]) {
  baseline.push(...handlers);

  worker.use(...handlers);
}

async function startWorker() {
  await worker.start({
    onUnhandledRequest: (request) => {
      if (new URL(request.url).origin !== location.origin) {
        unhandled.push(`${request.method} ${request.url}`);
      }
    },
    quiet: true,
  });
}

let ready = startWorker();

await ready;

// WARNING: msw closes its service-worker client on beforeunload, which would send every later fetch to the real network; this listener runs after msw's, stops and restarts the worker (a bare start is a no-op while msw still counts itself enabled), and fetches wait for it, so a synthetic beforeunload in a browser test keeps mocking.
globalThis.addEventListener("beforeunload", () => {
  worker.stop();

  ready = startWorker();
});

afterEach(async () => {
  await settleInflightFetches();

  worker.resetHandlers(...baseline);

  const requests = unhandled.splice(0);

  if (requests.length > 0) {
    throw new Error(`Unhandled requests: ${requests.join(", ")}`);
  }
});
