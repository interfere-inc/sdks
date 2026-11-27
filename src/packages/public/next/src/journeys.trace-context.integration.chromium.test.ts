import { randomBytes } from "node:crypto";
import { afterAll, beforeAll, expect, test } from "vitest";
import {
  attribute,
  exceptions,
  exportedSpans,
} from "@interfere/test-utils/vitest/otlp";
import { journey } from "../test/journeys.fixture";

const { delivered } = journey;

let unsampled: Awaited<ReturnType<typeof journey.app.start>>;

beforeAll(async () => {
  unsampled = await journey.app.start("serve.ts", {
    INTERFERE_API_KEY: journey.secret,
    INTERFERE_API_URL: journey.collector.url,
    INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
    INTERFERE_ENVIRONMENT: "preview",
  });
});

afterAll(async () => unsampled.close());

test("the managed server continues a foreign traceparent and records the Interfere session merged into its baggage", async () => {
  const traceId = randomBytes(16).toString("hex");

  const session = "0193a1b2-c3d4-7e5f-8a6b-7c8d9e0f1a2b";

  await (
    await fetch(
      `${journey.host.url}/api/journey?marker=foreign-trace-merged-baggage`,
      {
        headers: {
          baggage: `vendor=1,interfere.session.id=${session}`,
          traceparent: `00-${traceId}-${randomBytes(8).toString("hex")}-01`,
        },
      },
    )
  ).text();

  const span = await delivered("foreign-trace-merged-baggage");

  expect(span.traceId).toBe(traceId);

  expect(attribute(span.attributes, "interfere.session.id")).toBe(session);

  expect(attribute(span.attributes, "vendor")).toBeUndefined();
});

test("a route that throws under -00 still delivers its exception", async () => {
  const healthyTrace = randomBytes(16).toString("hex");

  const failingTrace = randomBytes(16).toString("hex");

  const parent = (traceId: string, flags: string) =>
    `00-${traceId}-${randomBytes(8).toString("hex")}-${flags}`;

  const request = async (query: string, traceparent: string) =>
    (
      await fetch(`${unsampled.url}/api/journey?${query}`, {
        headers: { traceparent },
      })
    ).text();

  await request("mode=healthy", parent(healthyTrace, "00"));

  await request("marker=unsampled-route-throw", parent(failingTrace, "00"));

  await request(
    "marker=sampled-route-barrier",
    parent(randomBytes(16).toString("hex"), "01"),
  );

  await delivered("sampled-route-barrier");

  const spans = exportedSpans(journey.collector.requests);

  expect(
    exceptions(journey.collector.requests, "unsampled-route-throw").map(
      (span) => span.traceId,
    ),
  ).toStrictEqual([failingTrace]);

  expect(spans.filter((span) => span.traceId === healthyTrace)).toStrictEqual(
    [],
  );
});
