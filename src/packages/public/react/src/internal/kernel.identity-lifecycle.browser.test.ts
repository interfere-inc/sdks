import { browserStorage } from "@interfere/test-config/browser-storage";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import type { ConsentDecision } from "@interfere/types/sdk/plugins/manifest";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

import { HttpResponse, http } from "msw";

import { DISABLE_ALL_PLUGINS } from "../../test/fixtures/disable-all-plugins.js";
import { EXPIRE_AFTER_MS } from "./activity/timing.js";
import { worker } from "../../test/msw.js";
import {
  identifies,
  identifiesArrived,
  recordIngest,
  sessions,
  sessionsArrived,
  settle,
  withinASecond,
} from "../tracking/lifecycle.fixture.js";
import { createKernel } from "./kernel.js";
import type { Kernel } from "./kernel.js";

const API_HOST = "https://kernel-lifecycle.invalid";

const SOURCE = { name: "Audit", type: "custom" } as const;

const WAIT = { timeout: 5000 };

const logBodies: string[] = [];

const kernels = new Set<Kernel>();

const PAGE_ID = /^page_[0-9a-f-]{36}$/u;

const SESSION_ID = /^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-/u;

async function kernelWith(decision?: ConsentDecision) {
  const kernel = await createKernel({
    onCreated: (created) => {
      if (decision !== undefined) {
        created.consent.set(decision);
      }
    },
    opts: { apiHost: API_HOST, consent: "required", tracing: false },
  });

  kernels.add(kernel);

  kernel.syncProviderConfig({});

  kernel.session.getId();

  if (kernel.consent.get().analytics) {
    await sessionsArrived(1);
  }

  return kernel;
}

beforeEach(() => {
  recordIngest(API_HOST);

  logBodies.length = 0;

  worker.use(
    http.post(`${API_HOST}/drain/v1/logs`, async ({ request }) => {
      logBodies.push(await request.text());

      return new HttpResponse(null, { status: 204 });
    }),
  );

  setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", "interfere_pk_test");

  setInjectedConfiguration("__INTERFERE_PLUGINS__", {
    ...DISABLE_ALL_PLUGINS,
    device: true,
  });
});

afterEach(async () => {
  vi.useRealTimers();

  await Promise.all([...kernels].map(async (kernel) => kernel.dispose()));

  kernels.clear();

  await settle();

  setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", undefined);

  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  await cookieStore.delete({ name: "interfere_did", path: "/" });
});

describe("A50 analytics consent granted after init", () => {
  test("replaces the page id with a persisted session that carries the device", async () => {
    expect.hasAssertions();

    const kernel = await kernelWith();

    const pageId = kernel.session.getId();

    expect(pageId).toMatch(PAGE_ID);

    kernel.consent.set({ replay: false });

    await vi.waitFor(
      () => expect(kernel.device.getDeviceId()).not.toBeNull(),
      WAIT,
    );

    const sessionId = kernel.session.getId();

    expect(sessionId).toMatch(SESSION_ID);
    expect(localStorage.getItem("interfere:session_id")).toBe(sessionId);

    await sessionsArrived(1);

    expect(sessions().at(-1)).toMatchObject({
      deviceId: kernel.device.getDeviceId(),
      sessionId,
    });

    await kernel.flush();

    expect(logBodies.join("")).toContain(`"stringValue":"${String(pageId)}"`);
    expect(logBodies.join("")).toContain("session.previous_id");
  });
});

describe("A51 consent required and not granted", () => {
  test("keeps a page id in memory and stores, cookies and posts nothing", async () => {
    expect.hasAssertions();

    const kernel = await kernelWith();

    expect(kernel.session.getId()).toMatch(PAGE_ID);
    expect(kernel.session.getId()).toBe(kernel.session.getId());

    await settle();

    expect(kernel.device.getDeviceId()).toBeNull();
    expect(sessions()).toStrictEqual([]);
    expect(browserStorage.local.keys()).toStrictEqual([]);
    expect(browserStorage.session.keys()).toStrictEqual([]);

    expect(browserStorage.cookies.get("interfere_did")).toBeNull();
  });
});

describe("A52 analytics consent revoked", () => {
  test("purges the device and the persisted session and returns to a new page id", async () => {
    expect.hasAssertions();

    const kernel = await kernelWith({ replay: false });

    expect(kernel.device.getDeviceId()).not.toBeNull();

    kernel.consent.set(false);

    expect(kernel.device.getDeviceId()).toBeNull();
    expect(localStorage.getItem("interfere:device_id")).toBeNull();
    expect(localStorage.getItem("interfere:session_id")).toBeNull();
    expect(kernel.session.getId()).toMatch(PAGE_ID);

    expect(browserStorage.cookies.get("interfere_did")).toBeNull();

    const before = sessions().length;

    kernel.identity.clear();

    kernel.session.getId();

    await settle();

    expect(sessions()).toHaveLength(before);
  });
});

describe("A53 consent re-granted after revoke", () => {
  test("mints a fresh device id instead of restoring the purged one", async () => {
    expect.hasAssertions();

    const kernel = await kernelWith({ replay: false });

    const purged = kernel.device.getDeviceId();

    kernel.consent.set(false);

    kernel.consent.set({ replay: false });

    await vi.waitFor(
      () => expect(kernel.device.getDeviceId()).not.toBeNull(),
      WAIT,
    );

    expect(kernel.device.getDeviceId()).not.toBe(purged);
  });
});

describe("A54 identify before analytics consent is granted", () => {
  test("holds identify until consent and then sends it once with the device", async () => {
    expect.hasAssertions();

    const kernel = await kernelWith();

    await kernel.identity.set({ identifier: "user-a54", source: SOURCE });

    await settle();

    expect(identifies()).toStrictEqual([]);

    kernel.consent.set({ replay: false });

    await withinASecond(identifiesArrived(1));

    expect(identifies()[0]).toMatchObject({
      deviceId: kernel.device.getDeviceId(),
      identifier: "user-a54",
      sessionId: kernel.session.getId(),
    });
  });
});

describe("A07 account switch before analytics consent is granted", () => {
  test("sends only the latest user once consent is granted", async () => {
    expect.hasAssertions();

    const kernel = await kernelWith();

    await kernel.identity.set({ identifier: "user-first", source: SOURCE });

    await kernel.identity.set({ identifier: "user-second", source: SOURCE });

    kernel.consent.set({ replay: false });

    await withinASecond(identifiesArrived(1));

    await settle();

    expect(identifies()).toHaveLength(1);

    expect(identifies()[0]).toMatchObject({
      identifier: "user-second",
      sessionId: kernel.session.getId(),
    });
  });
});

describe("A07 account switch after analytics consent is granted", () => {
  test("attaches the new user to the same persisted session", async () => {
    expect.hasAssertions();

    const kernel = await kernelWith({ replay: false });

    await kernel.identity.set({ identifier: "user-first", source: SOURCE });

    await identifiesArrived(1);

    await kernel.identity.set({ identifier: "user-second", source: SOURCE });

    await withinASecond(identifiesArrived(2));

    expect(identifies().map((body) => body.identifier)).toStrictEqual([
      "user-first",
      "user-second",
    ]);

    expect(identifies()[1]?.sessionId).toBe(identifies()[0]?.sessionId);
    expect(kernel.identity.get()).toMatchObject({ identifier: "user-second" });
  });
});

describe("A12 identity after idle rotation with consent granted", () => {
  test("the identified user is attached to the persisted session that replaces an expired one", async () => {
    expect.hasAssertions();

    vi.useFakeTimers({ toFake: ["Date", "performance"] });

    const kernel = await kernelWith({ replay: false });

    await kernel.identity.set({ identifier: "user-a12", source: SOURCE });

    await identifiesArrived(1);

    const original = kernel.session.getId();

    vi.advanceTimersByTime(EXPIRE_AFTER_MS + 1000);

    globalThis.dispatchEvent(new PointerEvent("pointerdown"));

    const rotated = kernel.session.getId();

    expect(rotated).toMatch(SESSION_ID);
    expect(rotated).not.toBe(original);

    await withinASecond(identifiesArrived(2));

    expect(identifies()[1]).toMatchObject({
      deviceId: kernel.device.getDeviceId(),
      identifier: "user-a12",
      sessionId: rotated,
    });
  });
});

describe("A12 identity after consent is revoked and re-granted", () => {
  test("the identified user is attached to the new persisted session", async () => {
    expect.hasAssertions();

    const kernel = await kernelWith({ replay: false });

    await kernel.identity.set({ identifier: "user-regrant", source: SOURCE });

    await identifiesArrived(1);

    kernel.consent.set(false);

    kernel.consent.set({ replay: false });

    await withinASecond(identifiesArrived(2));

    await vi.waitFor(
      () =>
        expect(identifies().at(-1)).toMatchObject({
          deviceId: kernel.device.getDeviceId(),
          identifier: "user-regrant",
          sessionId: kernel.session.getId(),
        }),
      WAIT,
    );

    expect(identifies().at(-1)?.sessionId).not.toBe(identifies()[0]?.sessionId);
  });
});

describe("A55 consent changes move between page and persisted sessions", () => {
  test("grant replaces the page id and revoke starts a new one", async () => {
    expect.hasAssertions();

    const kernel = await kernelWith();

    const pageId = kernel.session.getId();

    kernel.consent.set({ replay: false });

    const sessionId = kernel.session.getId();

    expect(sessionId).not.toBe(pageId);

    kernel.consent.set(false);

    const nextPageId = kernel.session.getId();

    expect(nextPageId).toMatch(PAGE_ID);
    expect(nextPageId).not.toBe(pageId);
    expect(nextPageId).not.toBe(sessionId);
  });
});

describe("A42 device id stable across reloads through the kernel", () => {
  test("a second kernel on the same storage reports the same device id", async () => {
    expect.hasAssertions();

    const first = await kernelWith({ replay: false });

    const id = first.device.getDeviceId();

    await first.dispose();

    browserStorage.session.remove("interfere:window_id");

    const second = await kernelWith({ replay: false });

    expect(second.device.getDeviceId()).toBe(id);
  });
});
