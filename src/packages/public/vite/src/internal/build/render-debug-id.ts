import { injectDebugIdIntoJs } from "@interfere/types/data/source-maps";

import { createHash } from "node:crypto";

export function renderDebugId(code: string) {
  const hash = createHash("sha256").update(code).digest("hex");

  const debugId = [
    hash.slice(0, 8),
    hash.slice(8, 12),
    `8${hash.slice(13, 16)}`,
    `8${hash.slice(17, 20)}`,
    hash.slice(20, 32),
  ].join("-");

  return injectDebugIdIntoJs(code, debugId);
}
