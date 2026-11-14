import { createLogger } from "../../../util/log.js";

const log = createLogger("replay/compressor");

/** `gzip` is `null` when compression is unavailable — callers must POST the chunk uncompressed. */
export interface Compressor {
  readonly dispose: () => void;
  readonly gzip:
    | ((bytes: Uint8Array) => Promise<Uint8Array<ArrayBuffer>>)
    | null;
}

const FAIL_OPEN: Compressor = { dispose: () => undefined, gzip: null };

const gzip = async (bytes: Uint8Array) => {
  const cs = new CompressionStream("gzip");
  const compressed = new Response(cs.readable).arrayBuffer();
  const writer = cs.writable.getWriter();

  await writer.write(new Uint8Array(bytes));

  await writer.close();

  return new Uint8Array(await compressed) satisfies Uint8Array<ArrayBuffer>;
};

export function createCompressor() {
  if (typeof CompressionStream === "undefined") {
    log.warn(
      "CompressionStream unavailable; replay chunks will POST uncompressed",
    );

    return FAIL_OPEN;
  }

  return { dispose: () => undefined, gzip } satisfies Compressor;
}
