import { EXPIRE_AFTER_MS } from "../internal/activity/activity.js";
import {
  FP_HASH,
  SOURCE,
  failIdentify,
  identifies,
  identifiesArrived,
  identifySessionBody,
  pendingDevice,
  sessions,
  sessionsArrived,
  settle,
  setupLifecycle,
  started,
  teardownLifecycle,
  withinASecond,
} from "./lifecycle.fixture.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

beforeEach(setupLifecycle);

afterEach(teardownLifecycle);

describe("A01 identify request contract", () => {
  test("posts sessionId, deviceId, fpHash and the identify params to /v3/session/identify", async () => {
    expect.hasAssertions();

    const tracker = await started();

    tracker.startDeviceIdentity();

    await settle();

    await tracker.identify({
      avatar: "https://example.com/a.png",
      company: { tier: "Enterprise" },
      email: "jane@example.com",
      identifier: "user-a01",
      name: "Jane",
      source: SOURCE,
      traits: { plan: "pro" },
    });

    await identifiesArrived(1);

    expect(identifies()[0]).toStrictEqual({
      avatar: "https://example.com/a.png",
      company: { tier: "Enterprise" },
      deviceId: tracker.getDeviceId(),
      email: "jane@example.com",
      fpHash: FP_HASH,
      identifier: "user-a01",
      name: "Jane",
      sessionId: tracker.sessionId(),
      source: SOURCE,
      traits: { plan: "pro" },
    });
  });
});

describe("A02 identify attaches the user to the current session", () => {
  test("the identify session id is the session already in use and the session does not rotate", async () => {
    expect.hasAssertions();

    const tracker = await started();

    const before = tracker.sessionId();

    await tracker.identify({ identifier: "user-a02", source: SOURCE });

    await identifiesArrived(1);

    expect(identifies()[0]?.sessionId).toBe(before);
    expect(sessions()[0]?.sessionId).toBe(before);
    expect(tracker.sessionId()).toBe(before);
  });
});

describe("A03 identify before device identity starts", () => {
  test("re-sends identify with the device id once device identity starts", async () => {
    expect.hasAssertions();

    const tracker = await started();

    await tracker.identify({ identifier: "user-a03", source: SOURCE });

    await identifiesArrived(1);

    tracker.startDeviceIdentity();

    await withinASecond(identifiesArrived(2));

    expect(identifies()).toHaveLength(2);

    expect(identifies()[1]).toMatchObject({
      deviceId: tracker.getDeviceId(),
      identifier: "user-a03",
    });
  });

  test("the first identify is sent without a device rather than waiting for device identity", async () => {
    expect.hasAssertions();

    const tracker = await started();

    await tracker.identify({ identifier: "user-a03", source: SOURCE });

    await identifiesArrived(1);

    expect(identifies()[0]).toMatchObject({ deviceId: null, fpHash: null });
  });
});

describe("A04 identify after device identity started", () => {
  test("carries the device id and the fingerprint once it has resolved", async () => {
    expect.hasAssertions();

    const tracker = await started();

    tracker.startDeviceIdentity();

    await settle();

    await tracker.identify({ identifier: "user-a04", source: SOURCE });

    await identifiesArrived(1);

    expect(identifies()[0]).toMatchObject({
      deviceId: tracker.getDeviceId(),
      fpHash: FP_HASH,
    });

    expect(tracker.getDeviceId()).toMatch(/^[\da-f-]{36}$/u);
  });
});

describe("A05 identify while fingerprinting is pending", () => {
  test("sends at once with the local ids and links the late hash through the session", async () => {
    expect.hasAssertions();

    const { device, release } = pendingDevice();

    const tracker = await started(device);

    tracker.startDeviceIdentity();

    await withinASecond(
      tracker.identify({ identifier: "user-a05", source: SOURCE }),
    );

    expect(identifies()).toStrictEqual([
      expect.objectContaining({
        deviceId: tracker.getDeviceId(),
        fpHash: null,
        identifier: "user-a05",
      }),
    ]);

    release("late-hash");

    await expect
      .poll(() => sessions().filter((b) => b.fpHash === "late-hash"))
      .toStrictEqual([
        expect.objectContaining({ deviceId: tracker.getDeviceId() }),
      ]);

    await settle();

    expect(identifies()).toHaveLength(1);
  });

  test("a rejected fingerprint never delays or blocks identify", async () => {
    expect.hasAssertions();

    const { device, release } = pendingDevice();

    const tracker = await started(device);

    tracker.startDeviceIdentity();

    release(null);

    await withinASecond(
      tracker.identify({ identifier: "user-a05b", source: SOURCE }),
    );

    expect(identifies()).toStrictEqual([
      expect.objectContaining({ fpHash: null, identifier: "user-a05b" }),
    ]);
  });
});

describe("A06 repeated identify for the same user", () => {
  test("re-sends a changed company tier while deduplicating the same tier", async () => {
    expect.hasAssertions();

    const tracker = await started();

    const identity = { identifier: "user-a06-tier", source: SOURCE };

    await tracker.identify(identity);

    await tracker.identify(identity);

    await tracker.identify({ ...identity, company: { tier: "Pro" } });

    await tracker.identify({ ...identity, company: { tier: "Pro" } });

    await tracker.identify({ ...identity, company: { tier: "Enterprise" } });

    expect(identifies()).toStrictEqual([
      expect.objectContaining({ identifier: identity.identifier }),
      expect.objectContaining({ company: { tier: "Pro" } }),
      expect.objectContaining({ company: { tier: "Enterprise" } }),
    ]);

    expect(identifies()[0]).not.toHaveProperty("company");

    expect(tracker.getIdentity()).toStrictEqual({
      ...identity,
      company: { tier: "Enterprise" },
    });
  });
});

describe("A07 identify a different user in the same session", () => {
  test("an account switch without identity.clear() sends the new user", async () => {
    expect.hasAssertions();

    const tracker = await started();

    tracker.startDeviceIdentity();

    await tracker.identify({ identifier: "user-first", source: SOURCE });

    await identifiesArrived(1);

    await tracker.identify({ identifier: "user-second", source: SOURCE });

    await withinASecond(identifiesArrived(2));

    expect(identifies()).toHaveLength(2);

    expect(tracker.getIdentity()).toMatchObject({ identifier: "user-second" });
  });
});

describe("A08 a failed identify is re-sent", () => {
  test("the next identify call after a 500 sends again", async () => {
    expect.hasAssertions();

    const tracker = await started();

    tracker.startDeviceIdentity();

    await settle();

    failIdentify(500);

    await tracker.identify({ identifier: "user-a08", source: SOURCE });

    await identifiesArrived(1);

    await settle();

    failIdentify(200);

    await tracker.identify({ identifier: "user-a08", source: SOURCE });

    await identifiesArrived(2);

    expect(identifies()).toHaveLength(2);
  });
});

describe("A12 identity after idle rotation", () => {
  test("the identified user is attached to the session that replaces an expired one", async () => {
    expect.hasAssertions();

    vi.useFakeTimers({ toFake: ["Date", "performance"] });

    const tracker = await started();

    await tracker.identify({ identifier: "user-a12", source: SOURCE });

    await identifiesArrived(1);

    tracker.startDeviceIdentity();

    await identifiesArrived(2);

    const original = tracker.sessionId();

    vi.advanceTimersByTime(EXPIRE_AFTER_MS + 1000);

    globalThis.dispatchEvent(new PointerEvent("pointerdown"));

    const rotated = tracker.sessionId();

    expect(rotated).not.toBe(original);

    await withinASecond(identifiesArrived(3));

    expect(identifies()).toHaveLength(3);

    expect(identifies()[2]).toMatchObject({
      identifier: "user-a12",
      sessionId: rotated,
    });
  });
});

describe("A13 concurrent identify calls", () => {
  test("two in-flight identify calls for the same user send one request", async () => {
    expect.hasAssertions();

    const { device, release } = pendingDevice();

    const tracker = await started(device);

    tracker.startDeviceIdentity();

    const calls = [
      tracker.identify({ identifier: "user-a13", source: SOURCE }),
      tracker.identify({ identifier: "user-a13", source: SOURCE }),
    ];

    release(FP_HASH);

    await Promise.all(calls);

    await identifiesArrived(1);

    await settle();

    expect(identifies()).toHaveLength(1);
  });
});

describe("A14 consent revoked while fingerprinting is pending", () => {
  test("the late fingerprint is never sent and nothing after the revoke carries the purged device id", async () => {
    expect.hasAssertions();

    const { device, release } = pendingDevice();

    const tracker = await started(device);

    tracker.startDeviceIdentity();

    const purged = tracker.getDeviceId();

    await sessionsArrived(2);

    await tracker.identify({ identifier: "user-a14", source: SOURCE });

    const before = {
      identifies: identifies().length,
      sessions: sessions().length,
    };

    tracker.discardDeviceIdentity();

    release(FP_HASH);

    await settle();

    const after = [
      ...identifies().slice(before.identifies),
      ...sessions().slice(before.sessions),
    ];

    expect(after.filter((b) => b.deviceId === purged)).toStrictEqual([]);

    expect(
      [...identifies(), ...sessions()].filter((b) => b.fpHash === FP_HASH),
    ).toStrictEqual([]);
  });
});

describe("A71 identify request contract", () => {
  test("the identify body parses with the server's identify shape and omits an unknown country", async () => {
    expect.hasAssertions();

    const tracker = await started();

    tracker.startDeviceIdentity();

    await tracker.identify({
      email: "jane@example.com",
      identifier: "user_a71",
      company: { tier: "Enterprise" },
      source: { name: "Clerk", type: "clerk" },
    });

    await identifiesArrived(1);

    expect(identifySessionBody.safeParse(identifies()[0]).success).toBe(true);
    expect(identifies()[0]).not.toHaveProperty("country");
  });
});
