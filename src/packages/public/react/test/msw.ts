import { HttpNetworkFrame, WebSocketNetworkFrame } from "msw/experimental";
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

/** Resolves once every fetch the page has started, including ones started while waiting, has settled. */
export async function fetchesSettled() {
  if (inflight.size === 0) {
    return;
  }

  await Promise.allSettled(inflight);

  await fetchesSettled();
}

export function addBaselineHandlers(...handlers: RequestHandler[]) {
  baseline.push(...handlers);

  worker.use(...handlers);
}

async function startWorker() {
  await worker.start({
    onUnhandledFrame({ frame }) {
      if (frame instanceof WebSocketNetworkFrame) {
        const { url } = frame.data.connection.client;

        if (url.host !== location.host) {
          unhandled.push(`WS ${url.href}`);

          throw new Error(`Unexpected WebSocket: ${url.href}`);
        }

        return;
      }

      if (!(frame instanceof HttpNetworkFrame)) {
        throw new TypeError(`Unexpected network protocol: ${frame.protocol}`);
      }

      const { request } = frame.data;

      if (new URL(request.url).origin !== location.origin) {
        unhandled.push(`${request.method} ${request.url}`);
      }
    },
    quiet: true,
  });
}

async function restartWorker() {
  await worker.stop();

  await startWorker();
}

let ready = startWorker();

await ready;

// WARNING: MSW closes its client on a non-persisted pagehide. Tests dispatch the event without navigating, so restore the worker before the next fetch.
window.addEventListener("pagehide", (event) => {
  if (event.persisted || event.isTrusted) {
    return;
  }

  ready = restartWorker();
});

afterEach(async () => {
  await fetchesSettled();

  worker.resetHandlers(...baseline);

  const requests = unhandled.splice(0);

  if (requests.length > 0) {
    throw new Error(`Unhandled requests: ${requests.join(", ")}`);
  }
});
