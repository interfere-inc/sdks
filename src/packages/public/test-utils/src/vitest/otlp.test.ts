import { expect, test } from "vitest";
import { attribute, exceptions, exportedSpans } from "./otlp";
import type { ReceivedRequest } from "./telemetry-receiver";

test("reads accepted OTLP spans and exceptions without counting rejected retries", () => {
  const values = [
    { key: "service.name", value: { stringValue: "checkout" } },
    { key: "sampled", value: { boolValue: false } },
    { key: "count", value: { intValue: "0" } },
  ];

  const body = JSON.stringify({
    resourceSpans: [
      {
        resource: { attributes: values },
        scopeSpans: [
          {
            spans: [
              { name: "request", traceId: "trace", spanId: "parent" },
              {
                name: "failure",
                traceId: "trace",
                spanId: "child",
                parentSpanId: "parent",
                status: { code: 2 },
                events: [
                  {
                    name: "exception",
                    attributes: [
                      {
                        key: "exception.message",
                        value: { stringValue: "payment failed" },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      },
    ],
  });

  const request: ReceivedRequest = {
    method: "POST",
    url: "/drain/v1/traces",
    headers: new Headers(),
    body,
    status: 200,
  };

  const requests = [
    request,
    { ...request, status: 503 },
    { ...request, url: "/drain/v1/logs" },
  ];

  const spans = exportedSpans(requests);

  expect(spans).toHaveLength(2);

  expect(spans[0]).toMatchObject({
    name: "request",
    attributes: [],
    events: [],
    resource: values,
  });

  expect(spans[1]).toMatchObject({
    parentSpanId: "parent",
    status: { code: 2 },
  });

  expect(attribute(values, "service.name")).toBe("checkout");
  expect(attribute(values, "sampled")).toBe(false);
  expect(attribute(values, "count")).toBe("0");
  expect(attribute(values, "absent")).toBeUndefined();

  expect(
    exceptions(requests, "payment failed").map((span) => span.name),
  ).toStrictEqual(["failure"]);

  expect(exceptions(requests, "other")).toStrictEqual([]);

  expect(() => exportedSpans([{ ...request, body: "{}" }])).toThrow(
    "resourceSpans",
  );
});
