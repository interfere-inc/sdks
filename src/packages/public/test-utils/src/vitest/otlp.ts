import { z } from "zod";
import type { ReceivedRequest } from "./telemetry-receiver";

const attributes = z.array(
  z.object({
    key: z.string(),
    value: z.object({
      stringValue: z.string().optional(),
      boolValue: z.boolean().optional(),
      intValue: z.union([z.string(), z.number()]).optional(),
    }),
  }),
);

const span = z.object({
  name: z.string(),
  traceId: z.string(),
  spanId: z.string(),
  parentSpanId: z.string().optional(),
  attributes: attributes.default([]),
  events: z
    .array(z.object({ name: z.string(), attributes: attributes.default([]) }))
    .default([]),
  links: z
    .array(z.object({ traceId: z.string(), spanId: z.string() }))
    .default([]),
  status: z.object({ code: z.number().optional() }).optional(),
});

const traces = z.object({
  resourceSpans: z.array(
    z.object({
      resource: z.object({ attributes }),
      scopeSpans: z.array(z.object({ spans: z.array(span) })),
    }),
  ),
});

export function exportedSpans(requests: ReceivedRequest[]) {
  return requests
    .filter(
      (request) =>
        request.url.startsWith("/drain/v1/traces") && request.status < 300,
    )
    .flatMap((request) =>
      traces.parse(JSON.parse(request.body)).resourceSpans.flatMap((resource) =>
        resource.scopeSpans.flatMap((scope) =>
          scope.spans.map((item) => ({
            ...item,
            resource: resource.resource.attributes,
          })),
        ),
      ),
    );
}

export function attribute(values: z.infer<typeof attributes>, key: string) {
  const value = values.find((item) => item.key === key)?.value;

  return value?.stringValue ?? value?.boolValue ?? value?.intValue;
}

export function exceptions(requests: ReceivedRequest[], message: string) {
  return exportedSpans(requests).filter((item) =>
    item.events.some(
      (event) =>
        event.name === "exception" &&
        attribute(event.attributes, "exception.message") === message,
    ),
  );
}

const logs = z.object({
  resourceLogs: z.array(
    z.object({
      resource: z.object({ attributes }),
      scopeLogs: z.array(
        z.object({
          logRecords: z.array(
            z.object({
              eventName: z.string().optional(),
              attributes: attributes.default([]),
              traceId: z.string().optional(),
              spanId: z.string().optional(),
            }),
          ),
        }),
      ),
    }),
  ),
});

export function exportedLogRecords(requests: ReceivedRequest[]) {
  return requests
    .filter(
      (request) =>
        request.url.startsWith("/drain/v1/logs") && request.status < 300,
    )
    .flatMap((request) =>
      logs.parse(JSON.parse(request.body)).resourceLogs.flatMap((resource) =>
        resource.scopeLogs.flatMap((scope) =>
          scope.logRecords.map((item) => ({
            ...item,
            resource: resource.resource.attributes,
          })),
        ),
      ),
    );
}
