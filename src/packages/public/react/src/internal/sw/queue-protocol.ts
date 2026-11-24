import { isPlainRecord } from "@interfere/types/sdk/guards/json";

export const INGEST_QUEUE_QUERY = "interfere.sw.accepts";

const INGEST_QUEUE_ANSWER = "interfere.sw.accepted";

/** Message a page posts to its controlling service worker to ask whether the ingest queue would durably queue a POST to `url`. */
export interface IngestQueueQuery {
  type: typeof INGEST_QUEUE_QUERY;
  url: string;
}

/** Reply the ingest queue posts on the query's port. */
interface IngestQueueAnswer {
  accepted: boolean;
  type: typeof INGEST_QUEUE_ANSWER;
}

/** Whether a message posted to the service worker is an ingest queue query. */
export function isIngestQueueQuery(value: unknown): value is IngestQueueQuery {
  return (
    isPlainRecord(value) &&
    value["type"] === INGEST_QUEUE_QUERY &&
    typeof value["url"] === "string"
  );
}

/** Whether a message on the query's port is the ingest queue's answer. */
export function isIngestQueueAnswer(
  value: unknown,
): value is IngestQueueAnswer {
  return (
    isPlainRecord(value) &&
    value["type"] === INGEST_QUEUE_ANSWER &&
    typeof value["accepted"] === "boolean"
  );
}

/** Page-side handle to the service worker that may run the ingest queue. */
export interface IngestQueueController {
  postMessage: (message: IngestQueueQuery, transfer: Transferable[]) => void;
}

/** Answers an ingest queue query on its port with whether `matches` accepts its URL. */
export function answerIngestQueueQuery(
  query: IngestQueueQuery,
  port: MessagePort,
  matches: (url: URL, request: Request) => boolean,
) {
  let accepted: boolean;

  try {
    const url = new URL(query.url);

    accepted = matches(url, new Request(url, { method: "POST" }));
  } catch {
    accepted = false;
  }

  const answer: IngestQueueAnswer = { accepted, type: INGEST_QUEUE_ANSWER };

  port.postMessage(answer);
}
