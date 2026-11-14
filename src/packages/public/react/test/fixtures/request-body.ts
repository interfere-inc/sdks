import { gunzipSync } from "fflate";

/** Reads a captured request body as text, undoing the gzip content encoding the SDK applies to larger exports. */
export async function decodedBodyText(request: Request) {
  const bytes = new Uint8Array(await request.clone().arrayBuffer());

  return new TextDecoder().decode(
    request.headers.get("content-encoding") === "gzip"
      ? gunzipSync(bytes)
      : bytes,
  );
}

/** Parses a captured JSON request body after undoing its gzip content encoding. */
export async function decodedBodyJson(request: Request) {
  const parsed: unknown = JSON.parse(await decodedBodyText(request));

  return parsed;
}
