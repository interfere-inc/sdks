/// <reference lib="webworker" />

import { isRetryableStatus } from "@interfere/helpers/retry-request";
import { Queue } from "workbox-background-sync";

import {
  answerIngestQueueQuery,
  isIngestQueueQuery,
} from "./queue-protocol.js";

declare const self: ServiceWorkerGlobalScope;

const DEFAULT_QUEUE_NAME = "interfere-ingest";

const DEFAULT_PATH_PREFIX = "/api/interfere/";

const MAX_RETENTION_MIN = 24 * 60;

const MAX_QUEUE_BYTES = 10 * 1024 * 1024;

const SYNTHETIC_QUEUED_RESPONSE: ResponseInit = {
  status: 202,
  statusText: "Queued",
};

export interface IngestQueueMessage {
  reason?: string;
  type:
    | "interfere.sw.queued"
    | "interfere.sw.replayed"
    | "interfere.sw.dropped";
  url: string;
}

export interface InstallIngestQueueOptions {
  /**
   * Predicate selecting which POSTs to durably queue on failure. Proxy-mode
   * customers leave this unset (same-origin `/api/interfere/*`); a host that
   * posts ingest elsewhere (e.g. a direct-mode SPA hitting the collector
   * cross-origin) passes its own matcher.
   */
  match?: (url: URL, request: Request) => boolean;
  queueName?: string;
}

async function postToClients(message: IngestQueueMessage) {
  const clients = await self.clients.matchAll({ includeUncontrolled: false });

  for (const client of clients) {
    // oxlint-disable-next-line unicorn/require-post-message-target-origin
    client.postMessage(message);
  }
}

async function totalQueueBytes(queue: Queue) {
  const entries = await queue.getAll();

  let total = 0;

  for (const entry of entries) {
    const meta = entry.metadata;

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Workbox restores arbitrary persisted metadata; only numeric byte counts contribute to queue size.
    if (meta && "bytes" in meta && typeof meta.bytes === "number") {
      total += meta.bytes;
    }
  }

  return total;
}

async function pruneOldest(queue: Queue, neededBytes: number) {
  let total = await totalQueueBytes(queue);

  while (total + neededBytes > MAX_QUEUE_BYTES) {
    // oxlint-disable-next-line no-await-in-loop -- The dequeued entry determines the remaining byte count and whether another oldest entry must be removed.
    const dropped = await queue.shiftRequest();

    if (!dropped) {
      break;
    }

    const meta = dropped.metadata;

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Evicted legacy or malformed entries may lack numeric byte counts and must not corrupt the running queue total.
    if (meta && "bytes" in meta && typeof meta.bytes === "number") {
      total -= meta.bytes;
    }

    // oxlint-disable-next-line no-await-in-loop -- Drop notifications must preserve the same FIFO order as the queue removals.
    await postToClients({
      reason: "queue_size_cap",
      type: "interfere.sw.dropped",
      url: dropped.request.url,
    });
  }
}

async function enqueue(queue: Queue, request: Request) {
  const cloned = request.clone();

  const body = await cloned.clone().arrayBuffer();

  const bytes = body.byteLength;

  if (bytes > MAX_QUEUE_BYTES) {
    await postToClients({
      type: "interfere.sw.dropped",
      url: request.url,
      reason: "payload_too_large",
    });

    return false;
  }

  await pruneOldest(queue, bytes);

  await queue.pushRequest({ metadata: { bytes }, request: cloned });

  await postToClients({ type: "interfere.sw.queued", url: request.url });

  return true;
}

/**
 * Wire the ingest durability queue into the current service worker. Adds
 * `fetch` (intercept + queue matching POSTs) and `activate` (replay)
 * listeners only — it does NOT touch `skipWaiting` / `clients.claim`, so
 * a host SW (e.g. the app's Workbox precache worker) keeps full ownership
 * of its update lifecycle. Additive listeners coexist with the host's.
 */
export function installIngestQueue(options: InstallIngestQueueOptions = {}) {
  const matches =
    options.match ?? ((url) => url.pathname.startsWith(DEFAULT_PATH_PREFIX));

  let replayInFlight: Promise<void> | undefined;

  const replay = async (q: Queue) => {
    replayInFlight ??= (async () => {
      try {
        let entry = await q.shiftRequest();

        while (entry) {
          try {
            // oxlint-disable-next-line no-await-in-loop -- FIFO replay must stop and restore the failed request before sending any later request.
            const res = await self.fetch(entry.request.clone());

            if (!res.ok && isRetryableStatus(res.status)) {
              throw new Error(`retry status ${res.status}`);
            }

            // oxlint-disable-next-line no-await-in-loop -- Replay notifications must retain delivery order before the next queued request is sent.
            await postToClients({
              type: "interfere.sw.replayed",
              url: entry.request.url,
            });
          } catch (error) {
            // oxlint-disable-next-line no-await-in-loop -- Restore the failed head entry before ending this replay so later retries retain FIFO order.
            await q.unshiftRequest(entry);

            throw error;
          }

          // oxlint-disable-next-line no-await-in-loop -- Dequeue the next request only after the current request has succeeded and its notification completed.
          entry = await q.shiftRequest();
        }
      } finally {
        replayInFlight = undefined;
      }
    })();

    await replayInFlight;
  };

  const queue = new Queue(options.queueName ?? DEFAULT_QUEUE_NAME, {
    maxRetentionTime: MAX_RETENTION_MIN,
    onSync: async ({ queue: q }) => {
      await replay(q);
    },
  });

  self.addEventListener("activate", (event) => {
    // oxlint-disable-next-line promise/prefer-await-to-then, eslint/no-empty-function
    event.waitUntil(replay(queue).catch(() => {}));
  });

  self.addEventListener("message", (event) => {
    const [port] = event.ports;

    const query: unknown = event.data;

    if (!port || !isIngestQueueQuery(query)) {
      return;
    }

    answerIngestQueueQuery(query, port, matches);
  });

  self.addEventListener("fetch", (event) => {
    const { request } = event;

    if (request.method !== "POST") {
      return;
    }

    let url: URL;

    try {
      url = new URL(request.url);
    } catch {
      return;
    }

    if (!matches(url, request)) {
      return;
    }

    event.respondWith(
      (async () => {
        try {
          const res = await self.fetch(request.clone());

          if (res.ok) {
            return res;
          }

          if (isRetryableStatus(res.status)) {
            return (await enqueue(queue, request))
              ? new Response(null, SYNTHETIC_QUEUED_RESPONSE)
              : res;
          }

          return res;
        } catch {
          return (await enqueue(queue, request))
            ? new Response(null, SYNTHETIC_QUEUED_RESPONSE)
            : new Response(null, {
                status: 503,
                statusText: "Queue capacity exceeded",
              });
        }
      })(),
    );
  });
}
