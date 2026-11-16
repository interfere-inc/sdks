import { createLogger } from "../../../util/log.js";

const log = createLogger("replay/compressor");

/** `gzip` is `null` when compression is unavailable — callers must POST the chunk uncompressed. */
export interface Compressor {
  readonly dispose: () => void;
  readonly gzip:
    | ((
        bytes: Uint8Array,
        signal?: AbortSignal,
      ) => Promise<Uint8Array<ArrayBuffer>>)
    | null;
}

const FAIL_OPEN: Compressor = { dispose: () => undefined, gzip: null };

const gzip = async (bytes: Uint8Array, signal?: AbortSignal) => {
  signal?.throwIfAborted();

  const cs = new CompressionStream("gzip");
  const writer = cs.writable.getWriter();

  const abort = () => {
    void writer.abort(signal?.reason);
  };

  signal?.addEventListener("abort", abort, { once: true });

  const write = async () => {
    await writer.write(new Uint8Array(bytes));

    await writer.close();
  };

  try {
    const [compressed] = await Promise.all([
      new Response(cs.readable).arrayBuffer(),
      write(),
    ]);

    return new Uint8Array(compressed) satisfies Uint8Array<ArrayBuffer>;
  } finally {
    signal?.removeEventListener("abort", abort);
  }
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
