import { describe, expect, test } from "vitest";
import {
  attribute,
  exceptions,
  exportedSpans,
} from "@interfere/test-utils/vitest/otlp";
import { journey } from "../test/journeys.fixture";

function interfereCaptures(message: string) {
  return exportedSpans(journey.collector.requests).flatMap((span) =>
    span.events.filter(
      (event) =>
        event.name === "exception" &&
        attribute(event.attributes, "exception.message") === message &&
        attribute(event.attributes, "interfere.exception.mechanism") !==
          undefined,
    ),
  );
}

function exceptionEvents(message: string) {
  return exceptions(journey.collector.requests, message).flatMap((span) =>
    span.events
      .filter(
        (event) =>
          event.name === "exception" &&
          attribute(event.attributes, "exception.message") === message,
      )
      .map((event) => ({
        mechanism: attribute(event.attributes, "interfere.exception.mechanism"),
        span: span.name,
      })),
  );
}

async function exportedOnce(marker: string, path: string) {
  await (await fetch(`${journey.host.url}${path}`)).text();

  await expect
    .poll(() => interfereCaptures(marker).length, { timeout: 4000 })
    .toBe(1);

  const barrier = `${marker}-barrier`;

  await (
    await fetch(`${journey.host.url}/api/journey?marker=${barrier}`)
  ).text();

  await expect
    .poll(() => interfereCaptures(barrier).length, { timeout: 4000 })
    .toBe(1);

  return exceptionEvents(marker);
}

describe("one server throw exports one exception", () => {
  test("a route handler throw reaches the receiver as a single exception event", async () => {
    const events = await exportedOnce(
      "spans-route-error",
      "/api/journey?marker=spans-route-error",
    );

    expect(
      events.map(({ mechanism }) => mechanism),
      JSON.stringify(events),
    ).toStrictEqual(["auto.function.nextjs.route_handler"]);
  });

  test("a server component render throw reaches the receiver as a single exception event", async () => {
    const events = await exportedOnce(
      "spans-render-error",
      "/failures/rsc?marker=spans-render-error",
    );

    expect(
      events.map(({ mechanism }) => mechanism),
      JSON.stringify(events),
    ).toStrictEqual(["auto.function.nextjs.on_request_error"]);
  });
});
