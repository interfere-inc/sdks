import { gzipSync } from "fflate";

import { createLogger } from "../../../util/log.js";

const log = createLogger("replay/compressor");

// WARNING: Runs on the main thread while the page unloads; a body above this ceiling is sent uncompressed because the gzip would outlive the page.
const UNLOAD_GZIP_CEILING_BYTES = 2 * 1024 * 1024;

/** Gzips an unload body synchronously so it can fit the keepalive budget. Returns null when the body is too large to compress during unload or compression fails. */
export function gzipForUnload(bytes: Uint8Array<ArrayBuffer>) {
  if (bytes.byteLength > UNLOAD_GZIP_CEILING_BYTES) {
    return null;
  }

  try {
    return new Uint8Array(gzipSync(bytes)) satisfies Uint8Array<ArrayBuffer>;
  } catch (error) {
    log.warn("synchronous gzip failed for the unload chunk: %o", error);

    return null;
  }
}
