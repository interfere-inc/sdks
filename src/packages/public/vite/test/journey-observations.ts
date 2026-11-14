import { z } from "zod";
import type { ReceivedRequest } from "@interfere/test-utils/vitest/telemetry-receiver";

const identity = z.object({
  sessionId: z.string(),
  identifier: z.string(),
  source: z.object({ type: z.string(), name: z.string() }),
});

export function identities(requests: ReceivedRequest[]) {
  return requests.flatMap((request) =>
    request.url === "/v3/session/identify"
      ? [identity.parse(JSON.parse(request.body))]
      : [],
  );
}

export function replay(requests: ReceivedRequest[]) {
  return requests
    .flatMap((request) =>
      request.url.startsWith("/v3/replay/") ? [request.body] : [],
    )
    .join("\n");
}

export function replayWindows(requests: ReceivedRequest[]) {
  return new Set(
    requests.flatMap((request) =>
      request.url.startsWith("/v3/replay/")
        ? [
            z
              .object({ windowId: z.string() })
              .parse(
                JSON.parse(
                  Buffer.from(
                    request.headers.get("x-replay-meta") ?? "",
                    "base64",
                  ).toString(),
                ),
              ).windowId,
          ]
        : [],
    ),
  );
}
