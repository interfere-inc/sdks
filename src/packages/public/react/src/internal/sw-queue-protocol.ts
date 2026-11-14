import { z } from "zod/mini";

export const INGEST_QUEUE_QUERY = "interfere.sw.accepts";

const INGEST_QUEUE_ANSWER = "interfere.sw.accepted";

/** Message a page posts to its controlling service worker to ask whether the ingest queue would durably queue a POST to `url`. */
export const ingestQueueQuerySchema = z.object({
  type: z.literal(INGEST_QUEUE_QUERY),
  url: z.string(),
});

/** Reply the ingest queue posts on the query's port. */
export const ingestQueueAnswerSchema = z.object({
  accepted: z.boolean(),
  type: z.literal(INGEST_QUEUE_ANSWER),
});

export type IngestQueueQuery = z.infer<typeof ingestQueueQuerySchema>;

type IngestQueueAnswer = z.infer<typeof ingestQueueAnswerSchema>;

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
