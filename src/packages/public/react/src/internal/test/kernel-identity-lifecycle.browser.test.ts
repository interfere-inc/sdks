import { browserStorage } from "@interfere/test-config/browser-storage";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import {
  afterEach,
  beforeEach,
  describe,
  expect,
  onTestFinished,
  test,
  vi,
} from "vitest";

import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import {
  identifies,
  identifiesArrived,
  recordIngest,
  sessions,
  sessionsArrived,
  settle,
  withinASecond,
} from "../../tracking/lifecycle.fixture.js";
import { createKernel } from "../kernel.js";

const API_HOST = "https://kernel-lifecycle.invalid";

const SOURCE = { name: "Audit", type: "custom" } as const;

const WAIT = { timeout: 5000 };

async function kernelWith(consent: { analytics: boolean; replay: boolean }) {
  const kernel = await createKernel({
    opts: { apiHost: API_HOST, consent, tracing: false },
  });

  onTestFinished(async () => {
    await kernel.dispose();
  });

  kernel.syncProviderConfig({ consent });

  kernel.session.getId();

  await sessionsArrived(1);

  return kernel;
}

beforeEach(() => {
  recordIngest(API_HOST);

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

  setInjectedConfiguration("__INTERFERE_PLUGINS__", {
    ...DISABLE_ALL_PLUGINS,
    device: true,
  });
});

afterEach(async () => {
  await settle();

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", undefined);

  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  await cookieStore.delete({ name: "interfere_did", path: "/" });
});

describe("A50 analytics consent granted after init", () => {
  test("starts device identity and backfills the current session with the device", async () => {
    expect.hasAssertions();

    const kernel = await kernelWith({ analytics: false, replay: false });

    const sessionId = kernel.session.getId();

    expect(kernel.device.getDeviceId()).toBeNull();

    kernel.consent.set({ analytics: true, replay: false });

    await vi.waitFor(
      () => expect(kernel.device.getDeviceId()).not.toBeNull(),
      WAIT,
    );

    await sessionsArrived(2);

    expect(sessions()[1]).toMatchObject({
      deviceId: kernel.device.getDeviceId(),
      sessionId,
    });
  });
});

describe("A51 analytics consent denied", () => {
  test("creates the session anonymously with no device id, storage or cookie", async () => {
    expect.hasAssertions();

    const kernel = await kernelWith({ analytics: false, replay: false });

    await settle();

    expect(kernel.device.getDeviceId()).toBeNull();
    expect(localStorage.getItem("interfere:device_id")).toBeNull();

    expect(sessions().map((b) => b.deviceId ?? null)).toStrictEqual(
      sessions().map(() => null),
    );

    await expect(cookieStore.get("interfere_did")).resolves.toBeNull();
  });
});

describe("A52 analytics consent revoked", () => {
  test("purges the device id, storage and cookie, and later sessions carry no device", async () => {
    expect.hasAssertions();

    const kernel = await kernelWith({ analytics: true, replay: false });

    expect(kernel.device.getDeviceId()).not.toBeNull();

    kernel.consent.set({ analytics: false, replay: false });

    expect(kernel.device.getDeviceId()).toBeNull();
    expect(localStorage.getItem("interfere:device_id")).toBeNull();

    await expect(cookieStore.get("interfere_did")).resolves.toBeNull();

    const before = sessions().length;

    kernel.identity.clear();

    const rotated = kernel.session.getId();

    await sessionsArrived(before + 1);

    expect(sessions().slice(before)).toStrictEqual([
      { deviceId: null, fpHash: null, sessionId: rotated },
    ]);
  });
});

describe("A53 consent re-granted after revoke", () => {
  test("mints a fresh device id instead of restoring the purged one", async () => {
    expect.hasAssertions();

    const kernel = await kernelWith({ analytics: true, replay: false });

    const purged = kernel.device.getDeviceId();

    kernel.consent.set({ analytics: false, replay: false });

    kernel.consent.set({ analytics: true, replay: false });

    await vi.waitFor(
      () => expect(kernel.device.getDeviceId()).not.toBeNull(),
      WAIT,
    );

    expect(kernel.device.getDeviceId()).not.toBe(purged);
  });
});

describe("A54 identify before analytics consent is granted", () => {
  test("identify is re-sent with the device id once consent starts device identity", async () => {
    expect.hasAssertions();

    const kernel = await kernelWith({ analytics: false, replay: false });

    await kernel.identity.set({ identifier: "user-a54", source: SOURCE });

    await identifiesArrived(1);

    kernel.consent.set({ analytics: true, replay: false });

    await withinASecond(identifiesArrived(2));

    expect(identifies()).toHaveLength(2);

    expect(identifies()[1]).toMatchObject({
      deviceId: kernel.device.getDeviceId(),
      identifier: "user-a54",
    });
  });

  test("the identify sent before consent carries no device", async () => {
    expect.hasAssertions();

    const kernel = await kernelWith({ analytics: false, replay: false });

    await kernel.identity.set({ identifier: "user-a54", source: SOURCE });

    await identifiesArrived(1);

    expect(identifies()[0]).toMatchObject({ deviceId: null, fpHash: null });
  });
});

describe("A55 consent changes keep the session", () => {
  test("grant and revoke do not rotate the session id", async () => {
    expect.hasAssertions();

    const kernel = await kernelWith({ analytics: false, replay: false });

    const sessionId = kernel.session.getId();

    kernel.consent.set({ analytics: true, replay: false });

    await vi.waitFor(
      () => expect(kernel.device.getDeviceId()).not.toBeNull(),
      WAIT,
    );

    kernel.consent.set({ analytics: false, replay: false });

    expect(kernel.session.getId()).toBe(sessionId);
    expect(localStorage.getItem("interfere:session_id")).toBe(sessionId);
  });
});

describe("A42 device id stable across reloads through the kernel", () => {
  test("a second kernel on the same storage reports the same device id", async () => {
    expect.hasAssertions();

    const first = await kernelWith({ analytics: true, replay: false });

    const id = first.device.getDeviceId();

    await first.dispose();

    browserStorage.session.remove("interfere:window_id");

    const second = await kernelWith({ analytics: true, replay: false });

    expect(second.device.getDeviceId()).toBe(id);
  });
});
