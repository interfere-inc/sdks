import { createServer } from "node:http";
import type { IncomingMessage, ServerResponse } from "node:http";
import { buffer } from "node:stream/consumers";
import { Readable } from "node:stream";
import { ReadableStream } from "node:stream/web";
import { pipeline } from "node:stream/promises";
import { getResponse } from "msw";
import type { HttpHandler } from "msw";

async function respond(
  handlers: HttpHandler[],
  incoming: IncomingMessage,
  outgoing: ServerResponse,
) {
  const abort = new AbortController();

  const disconnect = () => {
    if (!outgoing.writableFinished) {
      abort.abort();
    }
  };

  incoming.once("aborted", disconnect);

  outgoing.once("close", disconnect);

  try {
    const headers = new Headers();

    for (const [name, values] of Object.entries(incoming.headers)) {
      if (values !== undefined) {
        for (const value of Array.isArray(values) ? values : [values]) {
          headers.append(name, value);
        }
      }
    }

    const method = incoming.method ?? "GET";

    const init: RequestInit = { method, headers, signal: abort.signal };

    if (method !== "GET" && method !== "HEAD") {
      init.body = new Uint8Array(await buffer(incoming));
    }

    const request = new Request(
      `http://${incoming.headers.host ?? "127.0.0.1"}${incoming.url ?? "/"}`,
      init,
    );

    const response = await getResponse(handlers, request);

    if (!response || response.type === "error") {
      outgoing.destroy();

      return;
    }

    if (abort.signal.aborted) {
      await response.body?.cancel();

      return;
    }

    for (const [name, value] of response.headers) {
      if (name !== "set-cookie") {
        outgoing.setHeader(name, value);
      }
    }

    const cookies = response.headers.getSetCookie();

    if (cookies.length > 0) {
      outgoing.setHeader("set-cookie", cookies);
    }

    outgoing.writeHead(response.status);

    if (method === "HEAD" || !response.body) {
      await response.body?.cancel();

      outgoing.end();

      return;
    }

    outgoing.flushHeaders();

    if (!(response.body instanceof ReadableStream)) {
      throw new TypeError("Expected a native Node response stream");
    }

    await pipeline(Readable.fromWeb(response.body), outgoing);
  } catch {
    outgoing.destroy();
  } finally {
    incoming.off("aborted", disconnect);

    outgoing.off("close", disconnect);
  }
}

export function mswServer(...handlers: HttpHandler[]) {
  return createServer((request, response) => {
    void respond(handlers, request, response);
  });
}
