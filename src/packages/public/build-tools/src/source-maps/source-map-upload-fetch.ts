import { promisify } from "node:util";
import { gzip } from "node:zlib";

const compress = promisify(gzip);

/** Compresses requests marked with Content-Encoding: gzip by the source-map uploader. */
export const sourceMapUploadFetch: typeof fetch = Object.assign(
  async (input: RequestInfo | URL, init?: RequestInit) => {
    const request = new Request(input, init);

    if (
      request.method === "POST" &&
      new URL(request.url).pathname.endsWith("/source-maps/sign")
    ) {
      request.headers.set("x-interfere-source-map-compression", "gzip");
    }

    if (request.headers.get("content-encoding") !== "gzip") {
      return fetch(request);
    }

    const bytes = await request.arrayBuffer();

    const body = await compress(bytes);

    const headers = new Headers(request.headers);

    headers.delete("content-length");

    if (body.byteLength >= bytes.byteLength) {
      headers.delete("content-encoding");

      return fetch(
        new Request(request, { body: bytes, headers, method: request.method }),
      );
    }

    return fetch(
      new Request(request, { body, headers, method: request.method }),
    );
  },
  fetch,
);
