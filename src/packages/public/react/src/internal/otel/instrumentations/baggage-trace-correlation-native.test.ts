import { expect, test } from "vitest";
import {
  baggageKeys,
  installForeign,
  installInterfere,
  openRuntime,
  owner,
  probe,
} from "../../../../test/foreign-sdk.test-utils.js";

const KINDS = ["fetch", "fetch-request", "xhr"] as const;
const MODES = ["default"] as const;

test.each(MODES)(
  "%s baggage opt-out preserves foreign baggage and trace ownership on fetch and XHR",
  async () => {
    expect.hasAssertions();

    const runtime = await openRuntime();

    await installInterfere(runtime, { propagateTraceBaggage: false });

    await installForeign(runtime, { styles: ["w3c"] });

    const step = await probe(runtime, "baggage-disabled", [...KINDS]);

    expect(step.observations).toHaveLength(KINDS.length);

    for (const observation of step.observations) {
      expect(observation.sessionId).toBe(
        "11111111-1111-4111-8111-111111111111",
      );

      expect(observation.deviceId).toBe("device-fixture");

      const request = step.wire.find((entry) => entry.url === observation.url);

      expect(request).toBeDefined();

      expect(observation.requestTraceId).toBe(
        request?.headers["traceparent"]?.split("-")[1],
      );
    }

    expect(step.results.map((result) => result.status)).toStrictEqual([
      200, 200, 200,
    ]);

    expect(step.wire).toHaveLength(KINDS.length);

    for (const entry of step.wire) {
      expect(baggageKeys(entry.headers["baggage"] ?? null)).toStrictEqual([
        "session.id",
      ]);

      expect(owner(entry.headers["traceparent"] ?? null)).toBe("foreign");
    }
  },
);

test.each(MODES)(
  "%s baggage-free requests export identity in the actual outgoing trace",
  async () => {
    expect.hasAssertions();

    const runtime = await openRuntime();

    await installInterfere(runtime, { propagateTraceBaggage: false });

    const step = await probe(runtime, "baggage-off-no-vendor", [...KINDS]);

    expect(step.wire).toHaveLength(KINDS.length);
    expect(step.observations).toHaveLength(KINDS.length);

    for (const observation of step.observations) {
      const entry = step.wire.find(
        (request) => request.url === observation.url,
      );

      expect(entry).toBeDefined();

      expect(entry?.headers["traceparent"]).toMatch(
        /^00-[a-f0-9]{32}-[a-f0-9]{16}-01$/u,
      );

      expect(entry?.headers["baggage"]).toBeNull();

      expect(observation.sessionId).toBe(
        "11111111-1111-4111-8111-111111111111",
      );

      expect(observation.deviceId).toBe("device-fixture");

      expect(observation.traceId).toBe(
        entry?.headers["traceparent"]?.split("-")[1],
      );

      expect(observation.requestTraceId).toBe(observation.traceId);
    }
  },
);

test("captures wire identity from the Datadog-shaped passthrough fixture installed later", async () => {
  expect.hasAssertions();

  const runtime = await openRuntime();

  await runtime.page.evaluate(() =>
    globalThis.foreignSdkFixture.foreign.importUninitialized(),
  );

  await installInterfere(runtime, {
    propagation: "external",
    propagateTraceBaggage: false,
  });

  await installForeign(runtime, {
    global: "DD_RUM",
    styles: ["w3c"],
    baggage: false,
  });

  const step = await probe(runtime, "baggage-off-dd-imported", [...KINDS]);

  expect(step.wire).toHaveLength(KINDS.length);
  expect(step.observations).toHaveLength(KINDS.length);

  for (const observation of step.observations) {
    const entry = step.wire.find((request) => request.url === observation.url);

    expect(entry).toBeDefined();

    expect(entry?.headers["baggage"]).toBeNull();

    expect(observation.sessionId).toBe("11111111-1111-4111-8111-111111111111");

    expect(observation.deviceId).toBe("device-fixture");

    expect(observation.kind).toBe("span");

    expect(observation.requestTraceId).toBe(
      entry?.headers["traceparent"]?.split("-")[1],
    );
  }
});
