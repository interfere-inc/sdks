import {
  isPageHidden,
  onPageUnloading,
} from "../../page-lifecycle/page-lifecycle.js";
import { isPageUnloading } from "../../page-lifecycle/unloading.js";

const MIN_GZIP_BYTES = 1024;

// WARNING: beforeunload flushes while the page still looks visible; an async gzip there can outlive the page, so exports stay raw for this long after any unloading signal, and a cancelled navigation only costs a few uncompressed exports.
const LEAVING_GRACE_MS = 5000;

const textEncoder = new TextEncoder();

let leavingAt: number | null = null;

let subscribed = false;

const abandonOnLeave = new Set<() => void>();

function subscribe() {
  if (subscribed) {
    return;
  }

  subscribed = true;

  onPageUnloading(() => {
    leavingAt = performance.now();

    for (const abandon of abandonOnLeave) {
      abandon();
    }

    abandonOnLeave.clear();
  });
}

subscribe();

function isLeaving() {
  return leavingAt !== null && performance.now() - leavingAt < LEAVING_GRACE_MS;
}

async function gzip(bytes: Uint8Array<ArrayBuffer>) {
  const stream = new Blob([bytes])
    .stream()
    .pipeThrough(new CompressionStream("gzip"));

  return new Uint8Array(await new Response(stream).arrayBuffer());
}

/** Gzips an export body while the page is visible; a hidden or unloading page sends it as is so the request can leave before teardown. */
export async function compressExportBody(body: string) {
  const raw = textEncoder.encode(body);

  if (
    raw.byteLength < MIN_GZIP_BYTES ||
    typeof CompressionStream === "undefined" ||
    isLeaving() ||
    isPageHidden() ||
    isPageUnloading()
  ) {
    return { body, bytes: raw.byteLength, encoding: null };
  }

  let abandon: (() => void) | undefined;

  const leaving = new Promise<null>((resolve) => {
    abandon = () => {
      resolve(null);
    };
  });

  if (abandon) {
    abandonOnLeave.add(abandon);
  }

  try {
    // WARNING: an unloading signal must win this race synchronously in its own event task, so the raw body reaches a keepalive fetch before the page is torn down.
    const compressed = await Promise.race([gzip(raw), leaving]);

    if (compressed === null) {
      return { body, bytes: raw.byteLength, encoding: null };
    }

    return {
      body: compressed,
      bytes: compressed.byteLength,
      encoding: "gzip" as const,
    };
  } catch {
    return { body, bytes: raw.byteLength, encoding: null };
  } finally {
    if (abandon) {
      abandonOnLeave.delete(abandon);
    }
  }
}
