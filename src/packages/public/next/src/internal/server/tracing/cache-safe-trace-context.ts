import { z } from "zod";

import type { Context, TextMapSetter } from "@opentelemetry/api";
import { W3CTraceContextPropagator } from "@interfere/server-core/otel/core/propagation";

import { isStaticGenerationRender } from "../static-generation.js";

const metadataEntrySchema = z.object({ key: z.string() });

export interface CacheSafeTraceContextPropagatorOptions {
  isStaticRender?: () => boolean;
}

/**
 * W3C trace-context propagator that refuses to feed `clientTraceMetadata`
 * during static-generation renders (build prerender, runtime ISR
 * revalidation). Trace context injected into such a render is baked into
 * cached HTML and served verbatim to every visitor in the cache window —
 * there is no per-visitor server trace for a cache hit, so the idiomatic
 * output is no trace meta at all, exactly what Next produces for
 * build-time prerenders. Detected by the carrier shape: Next's
 * `clientTraceMetadata` collector is the only inject path that passes an
 * array carrier. Every other carrier (outgoing fetch headers during a
 * revalidation render included) injects normally.
 */
export class CacheSafeTraceContextPropagator extends W3CTraceContextPropagator {
  private readonly isStaticRender: () => boolean;

  constructor(options: CacheSafeTraceContextPropagatorOptions = {}) {
    super();

    this.isStaticRender = options.isStaticRender ?? isStaticGenerationRender;
  }

  override inject<Carrier>(
    context: Context,
    carrier: Carrier,
    setter: TextMapSetter<Carrier>,
  ) {
    if (Array.isArray(carrier)) {
      for (let index = carrier.length - 1; index >= 0; index--) {
        const entry = metadataEntrySchema.safeParse(carrier[index]);

        if (
          entry.success &&
          (entry.data.key === "traceparent" || entry.data.key === "tracestate")
        ) {
          carrier.splice(index, 1);
        }
      }

      if (this.isStaticRender()) {
        return;
      }
    }

    super.inject(context, carrier, setter);
  }
}
