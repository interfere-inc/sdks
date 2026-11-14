import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { InterfereError } from "@interfere/javascript";
import { ExportResultCode } from "@opentelemetry/core";
import { toError } from "@interfere/types/sdk/errors";
import { z } from "zod";

import type { resolveTargets } from "../../../internal/config/config.js";
import { withExportCarryover } from "../../../internal/otel/export-carryover.js";
import { uploadReplayChunk } from "./upload.js";
import { reserveKeepaliveBody } from "../../../internal/page-lifecycle/page-lifecycle.js";

const envelopeSchema = z.object({
  id: z.string(),
  body: z
    .string()
    .regex(/^[A-Za-z0-9+/]*={0,2}$/u)
    .refine((value) => value.length % 4 === 0),
  sessionId: z.string(),
  meta: z.string(),
  encoding: z.enum(["gzip"]).nullable(),
});

type Envelope = z.infer<typeof envelopeSchema>;

interface PendingUpload {
  response?: Response;
}

/** Retains prepared replay segments for consented delivery after navigation. */
export function createReplayCarryover(
  target: ReturnType<typeof resolveTargets>["ingest"],
  allowed: () => boolean,
) {
  const pendingResponses = new Map<string, PendingUpload>();

  const carryover = withExportCarryover(
    `replay:${JSON.stringify([target.baseUrl, target.publicKeyHeader?.apiKey])}`,
    (payload, callback, signal, retained) => {
      async function upload() {
        if (!allowed()) {
          throw new Error("Replay is disabled");
        }

        let value: unknown;

        try {
          value = JSON.parse(new TextDecoder().decode(payload));
        } catch {
          throw new InterfereError({
            statusCode: 400,
            message: "Invalid replay carryover",
          });
        }

        const parsed = envelopeSchema.safeParse(value);

        if (!parsed.success) {
          throw new InterfereError({
            statusCode: 400,
            message: "Invalid replay carryover",
          });
        }

        const envelope = parsed.data;

        const body = Uint8Array.from(
          atob(envelope.body),
          (character) => character.codePointAt(0) ?? 0,
        );

        const release = reserveKeepaliveBody(body.byteLength);

        if (release !== null) {
          retained?.onKeepalive();
        }

        try {
          const response = await uploadReplayChunk(
            target,
            envelope.sessionId,
            body,
            omitUndefined({
              "x-replay-meta": envelope.meta,
              "content-encoding": envelope.encoding ?? undefined,
            }),
            { keepalive: release !== null, signal },
          );

          const pending = pendingResponses.get(envelope.id);

          if (response.ok && pending) {
            pending.response = response;
          } else {
            await response.body?.cancel();
          }

          if (!response.ok) {
            throw new InterfereError({ statusCode: response.status });
          }
        } finally {
          release?.();
        }
      }

      async function report() {
        try {
          await upload();

          callback({ code: ExportResultCode.SUCCESS });

          return;
        } catch (error) {
          callback({
            code: ExportResultCode.FAILED,
            error: toError(error),
          });

          return;
        }
      }

      void report();
    },
    allowed,
    () => true,
  );

  return {
    dispose: carryover.dispose,
    async send(body: Uint8Array, metadata: Omit<Envelope, "body" | "id">) {
      if (body.byteLength > 768 * 1024) {
        return uploadReplayChunk(
          target,
          metadata.sessionId,
          new Uint8Array(body),
          omitUndefined({
            "x-replay-meta": metadata.meta,
            "content-encoding": metadata.encoding ?? undefined,
          }),
        );
      }

      let binary = "";

      for (const byte of body) {
        binary += String.fromCodePoint(byte);
      }

      const id = crypto.randomUUID();
      const pending: PendingUpload = {};

      const payload = new TextEncoder().encode(
        JSON.stringify({ ...metadata, id, body: btoa(binary) }),
      );

      pendingResponses.set(id, pending);

      const keepalive = reserveKeepaliveBody(body.byteLength);

      keepalive?.();

      const retainAndSend = keepalive ? carryover.deliver : carryover.send;

      try {
        return await new Promise<Response>((resolve, reject) => {
          retainAndSend(payload, (result) => {
            if (result.code === ExportResultCode.SUCCESS) {
              resolve(
                pending.response ??
                  Response.json({ queued: true }, { status: 202 }),
              );
            } else if (
              result.error instanceof InterfereError &&
              result.error.statusCode
            ) {
              resolve(new Response(null, { status: result.error.statusCode }));
            } else {
              reject(result.error ?? new Error("Replay carryover failed"));
            }
          });
        });
      } finally {
        pendingResponses.delete(id);
      }
    },
  };
}
