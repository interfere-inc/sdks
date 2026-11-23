import type { resolveTargets } from "../../../internal/config/config.js";
import { stampSentAt } from "../../../internal/ingest-backoff/ingest-backoff.js";

const MILLIS_PER_SECOND = 1000;

export interface UploadReplayChunkOptions {
  keepalive: boolean;
  signal?: AbortSignal | undefined;
}

export async function uploadReplayChunk(
  target: ReturnType<typeof resolveTargets>["ingest"],
  sessionId: string,
  body: Uint8Array<ArrayBuffer>,
  headers: Record<string, string>,
  options?: UploadReplayChunkOptions,
) {
  const requestHeaders = new Headers(target.headers);

  if (target.publicKeyHeader) {
    requestHeaders.set("x-interfere-public-key", target.publicKeyHeader.apiKey);
  }

  for (const [name, value] of Object.entries(headers)) {
    requestHeaders.set(name, value);
  }

  requestHeaders.set("content-type", "application/octet-stream");

  stampSentAt(requestHeaders);

  return fetch(
    new URL(
      `/v3/replay/upload/${encodeURIComponent(sessionId)}`,
      target.baseUrl,
    ),
    {
      method: "POST",
      credentials: "omit",
      headers: requestHeaders,
      body: new Blob([body]),
      keepalive: options?.keepalive ?? false,
      signal: AbortSignal.any([
        AbortSignal.timeout(target.timeoutInSeconds * MILLIS_PER_SECOND),
        ...(options?.signal ? [options.signal] : []),
      ]),
    },
  );
}
