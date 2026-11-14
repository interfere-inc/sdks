import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { QueueStore } from "workbox-background-sync/lib/QueueStore.js";
import { StorableRequest } from "workbox-background-sync/lib/StorableRequest.js";
import {
  IngestBackoffError,
  fetchIngest,
} from "../src/internal/ingest-backoff/ingest-backoff.js";
import type { IngestQueueMessage } from "../src/internal/sw-queue.js";

const queueName = new URL(location.href).searchParams.get("queue");

if (!queueName) {
  throw new Error("Missing queue name");
}

const store = new QueueStore(queueName);

export const messages: IngestQueueMessage[] = [];

navigator.serviceWorker.addEventListener(
  "message",
  (event: MessageEvent<IngestQueueMessage>) => {
    messages.push(event.data);
  },
);

export async function seed(
  initialEntries: {
    path: string;
    body?: string;
    metadata?: { bytes?: unknown };
  }[],
) {
  for (const entry of initialEntries) {
    const request = new Request(new URL(entry.path, location.href), {
      method: entry.body === undefined ? "GET" : "POST",
      body: entry.body ?? null,
    });

    // oxlint-disable-next-line no-await-in-loop
    const stored = await StorableRequest.fromRequest(request);

    // oxlint-disable-next-line no-await-in-loop
    await store.pushEntry(
      omitUndefined({
        requestData: stored.toObject(),
        metadata: entry.metadata,
        timestamp: Date.now(),
      }),
    );
  }
}

export async function register() {
  const controlled = new Promise<void>((resolve) => {
    navigator.serviceWorker.addEventListener(
      "controllerchange",
      () => resolve(),
      { once: true },
    );
  });

  await navigator.serviceWorker.register(`/worker.js?queue=${queueName}`);

  const registration = await navigator.serviceWorker.ready;

  const worker = registration.active;

  if (worker && worker.state !== "activated") {
    await new Promise<void>((resolve) => {
      worker.addEventListener("statechange", () => {
        if (worker.state === "activated") {
          resolve();
        }
      });
    });
  }

  if (!navigator.serviceWorker.controller) {
    await controlled;
  }
}

export async function backOff() {
  try {
    await fetchIngest("/unqueued", async () => fetch("http://0.0.0.0:0/"));
  } catch {
    return;
  }
}

export async function ingest(path: string) {
  try {
    const response = await fetchIngest(path, async () =>
      fetch(path, { method: "POST", body: "export" }),
    );

    return response.status;
  } catch (error) {
    return error instanceof IngestBackoffError ? "backoff" : String(error);
  }
}

export async function entries() {
  return store.getAll();
}

declare global {
  var queueFixture: {
    seed: typeof seed;
    register: typeof register;
    entries: typeof entries;
    backOff: typeof backOff;
    ingest: typeof ingest;
    messages: typeof messages;
  };
}

globalThis.queueFixture = {
  seed,
  register,
  entries,
  messages,
  backOff,
  ingest,
};
