import {
  INGEST_QUEUE_QUERY,
  isIngestQueueAnswer,
} from "../sw/queue-protocol.js";
import type {
  IngestQueueController,
  IngestQueueQuery,
} from "../sw/queue-protocol.js";

const FIRST_BACKOFF_MS = 5000;

const MAX_BACKOFF_MS = 10 * 60 * 1000;

const QUEUE_QUERY_TIMEOUT_MS = 1000;

const MAX_QUEUE_QUERIES = 32;

interface IngestQueueState {
  answers: Map<string, Promise<boolean>>;
  controller: IngestQueueController;
  responsive: boolean;
}

let consecutiveFailures = 0;

let blockedUntil = 0;

let queueState: IngestQueueState | null = null;

/** Test-only reset of the ingest network backoff. */
export function resetIngestBackoff() {
  consecutiveFailures = 0;

  blockedUntil = 0;

  queueState = null;
}

/** Rejection for an ingest request skipped because an earlier network failure is still backing off. */
export class IngestBackoffError extends Error {
  constructor() {
    super(
      "Interfere ingest is backing off after a network failure; request not sent",
    );

    this.name = "IngestBackoffError";
  }
}

function noteNetworkFailure() {
  if (Date.now() < blockedUntil) {
    return;
  }

  consecutiveFailures += 1;

  blockedUntil =
    Date.now() +
    Math.min(FIRST_BACKOFF_MS * 2 ** (consecutiveFailures - 1), MAX_BACKOFF_MS);
}

function absoluteUrl(input: RequestInfo | URL) {
  try {
    return new URL(
      input instanceof Request ? input.url : String(input),
      globalThis.location.href,
    ).href;
  } catch {
    return null;
  }
}

function currentController() {
  try {
    return "serviceWorker" in navigator
      ? navigator.serviceWorker.controller
      : null;
  } catch {
    return null;
  }
}

async function askIngestQueue(state: IngestQueueState, url: string) {
  return new Promise<boolean>((resolve) => {
    const channel = new MessageChannel();

    channel.port1.addEventListener("message", (event) => {
      const answer: unknown = event.data;

      state.responsive = true;

      resolve(isIngestQueueAnswer(answer) && answer.accepted);

      channel.port1.close();
    });

    channel.port1.start();

    const query: IngestQueueQuery = { type: INGEST_QUEUE_QUERY, url };

    try {
      state.controller.postMessage(query, [channel.port2]);
    } catch {
      channel.port1.close();

      resolve(false);
    }
  });
}

function queryIngestQueue(
  input: RequestInfo | URL,
  controller: IngestQueueController | null,
) {
  const url = absoluteUrl(input);

  if (!controller || url === null) {
    return null;
  }

  if (queueState?.controller !== controller) {
    queueState = { answers: new Map(), controller, responsive: false };
  }

  const state = queueState;

  let answer = state.answers.get(url);

  if (!answer && state.answers.size < MAX_QUEUE_QUERIES) {
    answer = askIngestQueue(state, url);

    state.answers.set(url, answer);
  }

  return answer ? { answer, state } : null;
}

async function ingestQueueAccepts(query: ReturnType<typeof queryIngestQueue>) {
  if (!query?.state.responsive) {
    return false;
  }

  let timer: ReturnType<typeof setTimeout> | undefined;

  const timeout = new Promise<boolean>((resolve) => {
    timer = setTimeout(() => {
      resolve(false);
    }, QUEUE_QUERY_TIMEOUT_MS);
  });

  try {
    return await Promise.race([query.answer, timeout]);
  } finally {
    clearTimeout(timer);
  }
}

// WARNING: Ad blockers and corporate proxies reject every request to the ingest host; without this gate a blocked tab fails a trace export every two seconds, and each failure shows up in the host page's own error monitoring.
/**
 * Sends one ingest request to `url` unless an earlier network failure is still backing off. A fetch rejection with a `TypeError` (no HTTP response: blocked, offline or unreachable) pauses every ingest request on the page for 5 s, doubling per consecutive failure up to 10 minutes. Any HTTP response clears the backoff. During a pause, a request still goes out when the page's controlling service worker runs `installIngestQueue` and confirms it queues `url`, because that worker answers a network failure with a queued 202 instead of a page-visible error. `controller` defaults to the page's controlling service worker.
 */
export async function fetchIngest(
  url: RequestInfo | URL,
  send: () => Promise<Response>,
  controller: IngestQueueController | null = currentController(),
) {
  const query = queryIngestQueue(url, controller);

  let viaQueue = false;

  if (Date.now() < blockedUntil) {
    viaQueue = await ingestQueueAccepts(query);

    if (!viaQueue) {
      throw new IngestBackoffError();
    }
  }

  let response: Response;

  try {
    response = await send();
  } catch (error) {
    if (error instanceof TypeError) {
      noteNetworkFailure();
    }

    throw error;
  }

  if (!viaQueue) {
    consecutiveFailures = 0;

    blockedUntil = 0;
  }

  return response;
}

export const SENT_AT_HEADER = "x-interfere-sent-at";

/** Stamps the device's current epoch milliseconds on an ingest request so the gateway can measure device clock skew; call once per attempt. */
export function stampSentAt(headers: Headers) {
  headers.set(SENT_AT_HEADER, String(Date.now()));

  return headers;
}
