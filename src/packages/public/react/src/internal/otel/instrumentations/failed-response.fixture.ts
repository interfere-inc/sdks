import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace-base";
import { HttpResponse } from "msw";
import { expect } from "vitest";

export function response(
  body: string,
  headers: Record<string, string> = {},
  status = 422,
) {
  return new HttpResponse(body, {
    status,
    headers: {
      "content-type": "application/json",
      "content-length": String(new TextEncoder().encode(body).byteLength),
      ...headers,
    },
  });
}

export async function failedSpan(exporter: InMemorySpanExporter) {
  await expect
    .poll(() =>
      exporter
        .getFinishedSpans()
        .some((span) =>
          span.events.some((event) => event.name === "exception"),
        ),
    )
    .toBe(true);

  const span = exporter
    .getFinishedSpans()
    .find((item) => item.events.some((event) => event.name === "exception"));

  if (!span) {
    throw new Error("Missing failed span");
  }

  return span;
}
