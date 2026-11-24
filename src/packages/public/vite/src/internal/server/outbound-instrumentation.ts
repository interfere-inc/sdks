import { UndiciInstrumentation } from "@opentelemetry/instrumentation-undici";

import type { RequestPropagation } from "./request/propagation";

/** Node's global fetch runs on undici, so Node traces outbound requests through its diagnostics channels. */
export function outboundInstrumentations(
  ignoreUrls: readonly (string | RegExp)[],
  requestPropagation: RequestPropagation,
) {
  return [
    new UndiciInstrumentation({
      ignoreRequestHook: ({ origin, path }) => {
        const url = `${origin}${path}`;

        return ignoreUrls.some((pattern) =>
          // WARNING: a global or sticky RegExp keeps lastIndex between test() calls, so match against a copy.
          pattern instanceof RegExp
            ? new RegExp(pattern.source, pattern.flags).test(url)
            : url.includes(pattern),
        );
      },
      requestHook: (span, { origin, path }) =>
        requestPropagation.observe(span, `${origin}${path}`),
    }),
  ];
}
