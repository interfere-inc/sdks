import {
  FP_HASH,
  readyDevice,
  sessions,
  sessionsArrived,
  settle,
  setupLifecycle,
  started,
  teardownLifecycle,
} from "./lifecycle.fixture.js";
import { browserStorage } from "@interfere/test-config/browser-storage";
import { afterEach, beforeEach, describe, expect, test } from "vitest";

beforeEach(setupLifecycle);

afterEach(teardownLifecycle);

describe("A43 device id restored from the cookie", () => {
  test("localStorage cleared but the cookie present restores the same device id", () => {
    expect.hasAssertions();

    const first = readyDevice();

    first.init();

    const id = first.getDeviceId();

    browserStorage.local.remove("interfere:device_id");

    const reloaded = readyDevice();

    reloaded.init();

    expect(reloaded.getDeviceId()).toBe(id);
    expect(localStorage.getItem("interfere:device_id")).toBe(id);
  });
});

describe("A44 device id restored from localStorage", () => {
  test("cookie cleared but localStorage present restores the id and rewrites the cookie", async () => {
    expect.hasAssertions();

    const first = readyDevice();

    first.init();

    const id = first.getDeviceId();

    await cookieStore.delete("interfere_did");

    const reloaded = readyDevice();

    reloaded.init();

    expect(reloaded.getDeviceId()).toBe(id);

    await expect(cookieStore.get("interfere_did")).resolves.toMatchObject({
      value: id,
    });
  });
});

describe("A45 starting device identity twice", () => {
  test("sends one backfill request", async () => {
    expect.hasAssertions();

    const tracker = await started();

    tracker.startDeviceIdentity();

    tracker.startDeviceIdentity();

    await sessionsArrived(2);

    await settle();

    expect(sessions()).toHaveLength(2);

    expect(sessions()[1]).toMatchObject({
      deviceId: tracker.getDeviceId(),
      fpHash: FP_HASH,
    });
  });
});

describe("A47 rotated session after device start", () => {
  test("the new session is posted with the device id", async () => {
    expect.hasAssertions();

    const tracker = await started();

    tracker.startDeviceIdentity();

    await sessionsArrived(2);

    tracker.rotateSession();

    const rotated = tracker.sessionId();

    await sessionsArrived(3);

    expect(sessions().find((b) => b.sessionId === rotated)).toMatchObject({
      deviceId: tracker.getDeviceId(),
      fpHash: FP_HASH,
    });
  });
});

describe("A61 identity.clear before device identity", () => {
  test("rotates the session without writing device storage or a cookie", async () => {
    expect.hasAssertions();

    const tracker = await started();

    const first = tracker.sessionId();

    tracker.reset();

    expect(tracker.sessionId()).not.toBe(first);
    expect(tracker.getDeviceId()).toBeNull();
    expect(localStorage.getItem("interfere:device_id")).toBeNull();

    await expect(cookieStore.get("interfere_did")).resolves.toBeNull();
  });
});
