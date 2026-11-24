import type { resolveTargets } from "../config/config.js";

export const INGEST_IGNORE_RE = /\/drain\/v1\/(?:traces|logs|metrics)(?:\?|$)/u;

const REPLAY_UPLOAD_RE = /\/v3\/replay\/upload\//u;

/** The SDK's own requests, which no request instrumentation stamps or observes. */
export function sdkRequestUrls(targets: ReturnType<typeof resolveTargets>) {
  return [
    INGEST_IGNORE_RE,
    `${targets.config.baseUrl}/v3/config`,
    `${targets.ingest.baseUrl}/v3/session`,
    REPLAY_UPLOAD_RE,
  ];
}
