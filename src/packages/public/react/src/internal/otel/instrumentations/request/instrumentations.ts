import type { TracerProvider } from "@opentelemetry/api";
import { createRequestCore } from "./core.js";
import type { RequestInstrumentationsInput } from "./core.js";
import { createFetchInstrumentation } from "./fetch.js";
import { createXhrInstrumentation } from "./xhr.js";

export type { RequestInstrumentationsInput } from "./core.js";

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
