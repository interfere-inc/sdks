import { createRequestCore } from "./request-core.js";
import type { RequestInstrumentationsInput } from "./request-core.js";
import { createFetchInstrumentation } from "./request-fetch.js";
import { createXhrInstrumentation } from "./request-xhr.js";

export type { RequestInstrumentationsInput } from "./request-core.js";

/**
 * Builds the upstream fetch and XMLHttpRequest instrumentations with
 * Interfere's request behaviour attached through their hooks: the header
 * policy, failed-request evidence, propagation diagnostics and trace
 * correlation. The upstream wrappers are the only patches on `fetch` and
 * `XMLHttpRequest`; where upstream has no hook (the fetch response before
 * the application reads it, XHR header injection), Interfere composes into
 * the upstream wrapper instead of adding another.
 */
export function createRequestInstrumentations(
  input: RequestInstrumentationsInput,
) {
  const core = createRequestCore(input);

  return {
    dispose: () => {
      core.dispose();
    },
    endOpenRequests: () => {
      core.endOpenRequests();
    },
    instrumentations: [
      createFetchInstrumentation(core, input),
      createXhrInstrumentation(core, input),
    ],
  };
}
