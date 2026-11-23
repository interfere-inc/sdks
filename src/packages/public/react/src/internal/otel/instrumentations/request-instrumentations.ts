import type { TracerProvider } from "@opentelemetry/api";
import { createRequestCore } from "./request-core.js";
import type { RequestInstrumentationsInput } from "./request-core.js";
import { createFetchInstrumentation } from "./request-fetch.js";
import { createXhrInstrumentation } from "./request-xhr.js";

export type { RequestInstrumentationsInput } from "./request-core.js";

/** Installs request instrumentation on the private provider and restores owned wrappers on disposal. */
export function createRequestInstrumentations(
  input: RequestInstrumentationsInput,
  tracerProvider: TracerProvider,
) {
  const core = createRequestCore(input);

  const disposeFetch = createFetchInstrumentation(core, input, tracerProvider);
  const disposeXhr = createXhrInstrumentation(core, input, tracerProvider);

  return {
    dispose: () => {
      core.dispose();

      disposeFetch();

      disposeXhr();
    },
    endOpenRequests: () => {
      core.endOpenRequests();
    },
  };
}
