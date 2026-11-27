import { SpanKind, SpanStatusCode } from "@opentelemetry/api";
import { resourceFromAttributes } from "@opentelemetry/resources";

import { createTraceExporter } from "../src/internal/otel/exporter.js";
import { onPageUnloading } from "../src/internal/page-lifecycle/page-lifecycle.js";

addEventListener("pagehide", () => {
  const until = performance.now() + 20;

  while (performance.now() < until) {
    Math.random();
  }
});

const exporter = createTraceExporter({
  clientOptions: {
    baseUrl: location.origin,
    releaseTokenHeader: { apiKey: "unload-test-key" },
  },
});

let buffered = true;

onPageUnloading(() => {
  if (!buffered) {
    return;
  }

  buffered = false;

  exporter.export(
    [
      {
        attributes: {},
        droppedAttributesCount: 0,
        droppedEventsCount: 0,
        droppedLinksCount: 0,
        duration: [0, 0],
        endTime: [0, 0],
        ended: true,
        events: [],
        instrumentationScope: { name: "unload" },
        kind: SpanKind.INTERNAL,
        links: [],
        name: "unload-span",
        resource: resourceFromAttributes({}),
        spanContext: () => ({
          traceId: "a".repeat(32),
          spanId: "b".repeat(16),
          traceFlags: 1,
        }),
        startTime: [0, 0],
        status: { code: SpanStatusCode.UNSET },
      },
    ],
    () => undefined,
  );
});
