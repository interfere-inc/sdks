/* oxlint-disable typescript/no-unsafe-assignment, typescript/no-unsafe-call, typescript/no-unsafe-member-access, typescript/require-await, typescript/strict-boolean-expressions */
import { SessionTracker } from "./api.js";
import { DeviceManager } from "./device.js";
import { InterfereClient } from "@interfere/javascript";
import { HttpResponse, http } from "msw";
import { worker } from "../../test/msw.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";
import type { MockInstance } from "vitest";

const TARGET = {
  publicKeyHeader: { apiKey: "test-key" },
  baseUrl: "https://in.test.com",
};

const WAIT_OPTS = { timeout: 5000 };

const trackers: SessionTracker[] = [];

const releases: (() => void)[] = [];

const client = new InterfereClient(TARGET);

const sessionsPrototype: InterfereClient["sessions"] = Object.getPrototypeOf(
  client.sessions,
);

let creates: MockInstance<InterfereClient["sessions"]["createSession"]>;

let previousCookie: Awaited<ReturnType<CookieStore["get"]>>;

async function fingerprintedDevice() {
  const device = new DeviceManager();

  device.init();

  await device.whenFingerprintReady();

  return device;
}

type FetchObservation = ReturnType<typeof observeFetch>;

function sessionCalls(fetcher: FetchObservation) {
  return fetcher.mock.calls.filter(
    ([input, init]) =>
      (input instanceof Request ? input.method : init?.method) === "POST",
  );
}

async function settleSessions() {
  await Promise.allSettled(
    creates.mock.results.flatMap((result) =>
      result.type === "return" ? [result.value] : [],
    ),
  );
}

const SOURCE = { name: "Test", type: "custom" } as const;

async function waitForPost(fetcher: FetchObservation) {
  await vi.waitFor(() => {
    expect(sessionCalls(fetcher).length).toBeGreaterThan(0);
  }, WAIT_OPTS);

  await settleSessions();
}

async function waitForOneCall(fetcher: FetchObservation) {
  await vi.waitFor(
    () => expect(sessionCalls(fetcher)).toHaveLength(1),
    WAIT_OPTS,
  );

  await settleSessions();
}

function identifyRequests(fetcher: FetchObservation) {
  return sessionCalls(fetcher).filter((call) =>
    new Request(call[0]).url.endsWith("/identify"),
  );
}

function pendingFingerprint() {
  const fingerprint = Promise.withResolvers<string | null>();

  releases.push(() => fingerprint.resolve(null));

  return {
    device: new DeviceManager(async () => fingerprint.promise),
    fingerprint,
  };
}

async function bodyOf(call: Parameters<typeof fetch> | []) {
  if (call.length === 0) {
    throw new Error("Expected a captured session request");
  }

  const decodedValue: Promise<unknown> = new Request(...call).json();

  return decodedValue;
}

async function identifiedBodies(fetcher: FetchObservation) {
  return (
    await Promise.all(
      sessionCalls(fetcher).map(async (call) => await bodyOf(call)),
    )
  ).filter((body) =>
    expect
      .objectContaining({ deviceId: expect.stringMatching(/./u) })
      .asymmetricMatch(body),
  );
}

async function nextMacrotask() {
  const { promise, resolve } = Promise.withResolvers<undefined>();

  setTimeout(() => {
    resolve(undefined);
  }, 0);

  return promise;
}

function observeFetch() {
  return vi.spyOn(globalThis, "fetch");
}

function makeTracker(
  opts: {
    target?: typeof TARGET;
    fetcher?: FetchObservation;
    device?: DeviceManager;
  } = {},
) {
  const fetcher = opts.fetcher ?? observeFetch();

  const tracker = new SessionTracker({
    device: opts.device ?? new DeviceManager(),
    target: opts.target ?? TARGET,
  });

  trackers.push(tracker);

  return { fetcher, tracker };
}

describe(SessionTracker, () => {
  beforeEach(async () => {
    creates = vi.spyOn(sessionsPrototype, "createSession");

    worker.use(
      ...[TARGET.baseUrl, `${location.origin}/api/interfere`].flatMap(
        (origin) =>
          ["/v3/session", "/v3/session/identify"].map((path) =>
            http.post(origin + path, () => HttpResponse.json({ ok: true })),
          ),
      ),
    );

    previousCookie = await cookieStore.get("interfere_did");
  });

  afterEach(async () => {
    const results = await Promise.allSettled(
      trackers.splice(0).map(async (tracker) => tracker.dispose()),
    );

    results.push(
      ...(await Promise.allSettled(
        releases.splice(0).map(async (release) => release()),
      )),
    );

    await settleSessions();

    await cookieStore.delete("interfere_did");

    if (previousCookie?.name && previousCookie.value !== undefined) {
      await cookieStore.set({
        ...previousCookie,
        name: previousCookie.name,
        value: previousCookie.value,
      });
    }

    vi.useRealTimers();

    const failures = results.filter((result) => result.status === "rejected");

    if (failures.length > 0) {
      throw new AggregateError(
        failures.map((result) => {
          const decodedValue: unknown = result.reason;

          return decodedValue;
        }),
        "Session cleanup failed",
      );
    }
  });

  test("returns null before start", () => {
    const { tracker } = makeTracker();

    expect(tracker.sessionId()).toBeNull();
  });

  test("returns a session id after start", () => {
    const { tracker } = makeTracker();

    tracker.start();

    expect(tracker.sessionId()).toMatch(/\S/u);
  });

  test("returns a window id after start", () => {
    const { tracker } = makeTracker();

    tracker.start();

    expect(tracker.windowId()).toMatch(/\S/u);
  });

  test("reports the session start time only after start", () => {
    const { tracker } = makeTracker();

    expect(tracker.getSessionStartedAt()).toBe(0);

    tracker.start();

    expect(tracker.getSessionStartedAt()).toBeGreaterThan(0);
  });

  test("rotates to a new session id on rotateSession", () => {
    const { tracker } = makeTracker();

    tracker.start();

    const first = tracker.sessionId();

    tracker.rotateSession();

    expect(tracker.sessionId()).not.toBe(first);
  });

  test("markActivity is a no-op before start and callable after", () => {
    const { tracker } = makeTracker();

    expect(() => {
      tracker.markActivity();
    }).not.toThrow();

    tracker.start();

    expect(() => {
      tracker.markActivity();
    }).not.toThrow();
  });

  test("emits a session POST on rotate", async () => {
    expect.hasAssertions();

    const { tracker, fetcher } = makeTracker();

    tracker.start();

    const original = tracker.sessionId();

    await waitForPost(fetcher);

    fetcher.mockClear();

    tracker.rotateSession();

    const rotated = tracker.sessionId();

    expect(rotated).not.toBe(original);

    await waitForOneCall(fetcher);

    const [url, init] = sessionCalls(fetcher)[0] ?? [];

    expect(url).toBe(`${TARGET.baseUrl}/v3/session`);
    expect(init?.method).toBe("POST");

    await expect(new Response(init?.body).json()).resolves.toMatchObject({
      sessionId: rotated,
    });
  });

  test("returns null identity before identify", () => {
    const { tracker } = makeTracker();

    expect(tracker.getIdentity()).toBeNull();
  });

  test("pOSTs to {target.url}/identify on identify", async () => {
    expect.hasAssertions();

    const { tracker, fetcher } = makeTracker();

    tracker.start();

    await waitForPost(fetcher);

    fetcher.mockClear();

    await tracker.identify({
      identifier: "user-1",
      name: "Alice",
      source: SOURCE,
      traits: { plan: "pro", flags: ["reports"], metadata: { count: 2 } },
    });

    await waitForOneCall(fetcher);

    const [url, init] = sessionCalls(fetcher)[0] ?? [];

    expect(url).toBe(`${TARGET.baseUrl}/v3/session/identify`);
    expect(init?.method).toBe("POST");

    await expect(new Response(init?.body).json()).resolves.toStrictEqual({
      deviceId: null,
      fpHash: null,
      identifier: "user-1",
      name: "Alice",
      sessionId: expect.stringMatching(/./u),
      source: SOURCE,
      traits: { plan: "pro", flags: ["reports"], metadata: { count: 2 } },
    });
  });

  test.each([
    [
      "constructs the identify URL when target.url is a relative path",
      "user-relative",
    ],
    ["uses generated header auth for identification", "user-query"],
  ])("%s", async (_name, identifier) => {
    expect.hasAssertions();

    const { tracker, fetcher } = makeTracker({
      target: {
        publicKeyHeader: { apiKey: "test-key" },
        baseUrl: "/api/interfere",
      },
    });

    tracker.start();

    await waitForPost(fetcher);

    fetcher.mockClear();

    await tracker.identify({ identifier, source: SOURCE });

    await waitForOneCall(fetcher);

    const [url] = sessionCalls(fetcher)[0] ?? [];

    expect(url).toBe("/api/interfere/v3/session/identify");
  });

  test("deduplicates identify calls for the same session", async () => {
    expect.hasAssertions();

    const { tracker, fetcher } = makeTracker();

    tracker.start();

    const initialSessionId = tracker.sessionId();

    await waitForPost(fetcher);

    fetcher.mockClear();

    await tracker.identify({
      identifier: "user-1",
      source: SOURCE,
    });

    await tracker.identify({
      identifier: "user-1",
      source: SOURCE,
    });

    await vi.waitFor(() => {
      const identifyCalls = sessionCalls(fetcher).filter(
        (call) =>
          call[1]?.method === "POST" &&
          new Request(call[0]).url.endsWith("/identify"),
      );

      expect(identifyCalls).toHaveLength(1);
    }, WAIT_OPTS);

    expect(tracker.sessionId()).toBe(initialSessionId);
  });

  test("re-identifies with the device once device identity starts after identify", async () => {
    expect.hasAssertions();

    const { tracker, fetcher } = makeTracker();

    tracker.start();

    tracker.sessionId();

    await waitForPost(fetcher);

    await tracker.identify({ identifier: "user-early", source: SOURCE });

    await vi.waitFor(
      () => expect(identifyRequests(fetcher)).toHaveLength(1),
      WAIT_OPTS,
    );

    tracker.startDeviceIdentity();

    await vi.waitFor(
      () => expect(identifyRequests(fetcher)).toHaveLength(2),
      WAIT_OPTS,
    );

    const bodies = await Promise.all(
      identifyRequests(fetcher).map(async (call) => bodyOf(call)),
    );

    expect(bodies).toStrictEqual([
      expect.objectContaining({ deviceId: null, identifier: "user-early" }),
      expect.objectContaining({
        deviceId: tracker.getDeviceId(),
        identifier: "user-early",
      }),
    ]);

    await tracker.identify({ identifier: "user-early", source: SOURCE });

    await nextMacrotask();

    expect(identifyRequests(fetcher)).toHaveLength(2);
  });

  test("reset clears identity, rotates session, and allows identifying the new session", async () => {
    expect.hasAssertions();

    const { tracker, fetcher } = makeTracker({
      device: await fingerprintedDevice(),
    });

    tracker.start();

    tracker.startDeviceIdentity();

    const initialSessionId = tracker.sessionId();
    const initialDeviceId = tracker.getDeviceId();

    await vi.waitFor(async () => {
      await expect(identifiedBodies(fetcher)).resolves.toHaveLength(1);
    }, WAIT_OPTS);

    fetcher.mockClear();

    await tracker.identify({
      identifier: "user-reset",
      source: SOURCE,
    });

    await vi.waitFor(
      () => expect(identifyRequests(fetcher)).toHaveLength(1),
      WAIT_OPTS,
    );

    fetcher.mockClear();

    tracker.reset();

    const resetSessionId = tracker.sessionId();

    expect(tracker.getIdentity()).toBeNull();
    expect(resetSessionId).toMatch(/\S/u);
    expect(resetSessionId).not.toBe(initialSessionId);
    expect(tracker.getDeviceId()).toMatch(/\S/u);
    expect(tracker.getDeviceId()).not.toBe(initialDeviceId);

    await vi.waitFor(
      () =>
        expect(
          sessionCalls(fetcher).filter(
            (call) => call[0] === `${TARGET.baseUrl}/v3/session`,
          ),
        ).toHaveLength(1),
      WAIT_OPTS,
    );

    fetcher.mockClear();

    await tracker.identify({
      identifier: "user-after-reset",
      source: SOURCE,
    });

    await vi.waitFor(
      () => expect(identifyRequests(fetcher)).toHaveLength(1),
      WAIT_OPTS,
    );

    const [, init] = sessionCalls(fetcher).find((call) =>
      new Request(call[0]).url.endsWith("/identify"),
    ) ?? [undefined, undefined];

    const body: unknown = await new Response(init?.body).json();

    expect(body).toMatchObject({
      identifier: "user-after-reset",
      sessionId: resetSessionId,
    });
  });

  test("post-reset session-create posts with the rotated device id", async () => {
    expect.hasAssertions();

    const { tracker, fetcher } = makeTracker({
      device: await fingerprintedDevice(),
    });

    tracker.start();

    tracker.startDeviceIdentity();

    await vi.waitFor(async () => {
      await expect(identifiedBodies(fetcher)).resolves.toHaveLength(1);
    }, WAIT_OPTS);

    const initialDeviceId = tracker.getDeviceId();

    fetcher.mockClear();

    tracker.reset();

    const rotatedDeviceId = tracker.getDeviceId();

    expect(rotatedDeviceId).not.toBe(initialDeviceId);

    await vi.waitFor(
      () =>
        expect(
          sessionCalls(fetcher).filter(
            (call) => call[0] === `${TARGET.baseUrl}/v3/session`,
          ),
        ).toHaveLength(1),
      WAIT_OPTS,
    );

    const [, init] = sessionCalls(fetcher).find(
      (call) => call[0] === `${TARGET.baseUrl}/v3/session`,
    ) ?? [undefined, undefined];

    const body: unknown = await new Response(init?.body).json();

    expect(body).toMatchObject({ deviceId: rotatedDeviceId });
  });

  test("stores the current identity", async () => {
    const { tracker } = makeTracker();

    tracker.start();

    await tracker.identify({
      identifier: "user-2",
      source: SOURCE,
    });

    expect(tracker.getIdentity()).toStrictEqual({
      identifier: "user-2",
      source: SOURCE,
    });
  });

  test("clears identity on dispose", async () => {
    const { tracker } = makeTracker();

    tracker.start();

    await tracker.identify({
      identifier: "user-3",
      source: SOURCE,
    });

    tracker.dispose();

    expect(tracker.getIdentity()).toBeNull();
  });

  test("identify is a no-op before start", async () => {
    const { tracker, fetcher } = makeTracker();

    await tracker.identify({
      identifier: "user-4",
      source: SOURCE,
    });

    expect(sessionCalls(fetcher)).toHaveLength(0);
    expect(tracker.getIdentity()).toBeNull();
  });

  test("retries session POST when initial sync fails (after cooldown)", async () => {
    expect.hasAssertions();

    vi.useFakeTimers({ toFake: ["Date"] });

    const start = 1_000_000;

    vi.setSystemTime(start);

    const fetcher = observeFetch();

    worker.use(
      http.post(`${TARGET.baseUrl}/v3/session`, () => HttpResponse.error(), {
        once: true,
      }),
    );

    const { tracker } = makeTracker({ fetcher });

    tracker.start();

    await waitForOneCall(fetcher);

    fetcher.mockClear();

    vi.setSystemTime(Date.now() + 5001);

    tracker.sessionId();

    await waitForOneCall(fetcher);

    const [url, init] = sessionCalls(fetcher)[0] ?? [];

    expect(url).toBe(`${TARGET.baseUrl}/v3/session`);
    expect(init?.method).toBe("POST");

    vi.useRealTimers();
  });

  test("does not retry within cooldown window", async () => {
    expect.hasAssertions();

    const fetcher = observeFetch();

    worker.use(
      http.post(`${TARGET.baseUrl}/v3/session`, () => HttpResponse.error(), {
        once: true,
      }),
    );

    const { tracker } = makeTracker({ fetcher });

    tracker.start();

    await waitForOneCall(fetcher);

    fetcher.mockClear();

    tracker.sessionId();

    expect(sessionCalls(fetcher)).toHaveLength(0);
  });

  test("stops retrying after a successful sync", async () => {
    expect.hasAssertions();

    const { tracker, fetcher } = makeTracker();

    tracker.start();

    await waitForOneCall(fetcher);

    fetcher.mockClear();

    const currentTime = Date.now();

    vi.useFakeTimers({ toFake: ["Date"] });

    vi.setSystemTime(currentTime + 5001);

    tracker.sessionId();

    expect(sessionCalls(fetcher)).toHaveLength(0);

    vi.useRealTimers();
  });

  test("nullifies session after dispose", () => {
    const { tracker } = makeTracker();

    tracker.start();

    expect(tracker.sessionId()).toMatch(/\S/u);

    tracker.dispose();

    expect(tracker.sessionId()).toBeNull();
  });

  describe("device identity consent gate", () => {
    test("start() alone writes no device storage and runs no fingerprint or network work", () => {
      const fingerprint = vi.fn<() => Promise<string>>();

      const network = observeFetch();

      const { tracker } = makeTracker({
        device: new DeviceManager(fingerprint),
      });

      tracker.start();

      expect(tracker.getDeviceId()).toBeNull();
      expect(localStorage.getItem("interfere:device_id")).toBeNull();
      expect(document.cookie).not.toContain("interfere_did=");
      expect(fingerprint).not.toHaveBeenCalled();
      expect(network).not.toHaveBeenCalled();
    });

    test("syncs the session anonymously before identity starts", async () => {
      expect.hasAssertions();

      const { tracker, fetcher } = makeTracker();

      tracker.start();

      await waitForPost(fetcher);

      await expect(
        bodyOf(sessionCalls(fetcher)[0] ?? []),
      ).resolves.toMatchObject({
        sessionId: expect.stringMatching(/./u),
        deviceId: null,
        fpHash: null,
      });
    });

    test("backfills the already-synced session with device identity once it starts", async () => {
      expect.hasAssertions();

      const device = await fingerprintedDevice();

      const hash = device.getFpHash();

      expect(hash).toMatch(/^[a-f0-9]{16,}$/iu);

      const { tracker, fetcher } = makeTracker({ device });

      tracker.start();

      await waitForPost(fetcher);

      const sessionId = tracker.sessionId();

      await expect(
        bodyOf(sessionCalls(fetcher)[0] ?? []),
      ).resolves.toMatchObject({
        sessionId,
      });

      fetcher.mockClear();

      tracker.startDeviceIdentity();

      await waitForPost(fetcher);

      await expect(
        bodyOf(sessionCalls(fetcher)[0] ?? []),
      ).resolves.toMatchObject({
        sessionId,
        deviceId: expect.stringMatching(/./u),
        fpHash: hash,
      });
    });

    test("registers device identity while fingerprinting remains pending", async () => {
      expect.hasAssertions();

      const { device, fingerprint } = pendingFingerprint();
      const { tracker, fetcher } = makeTracker({ device });

      tracker.start();

      await waitForPost(fetcher);

      fetcher.mockClear();

      tracker.startDeviceIdentity();

      await vi.waitFor(async () => {
        await expect(identifiedBodies(fetcher)).resolves.toStrictEqual([
          expect.objectContaining({
            deviceId: tracker.getDeviceId(),
            fpHash: null,
          }),
        ]);
      }, WAIT_OPTS);

      fingerprint.resolve("resolved-fingerprint");

      await vi.waitFor(async () => {
        await expect(identifiedBodies(fetcher)).resolves.toStrictEqual([
          expect.objectContaining({ fpHash: null }),
          expect.objectContaining({ fpHash: "resolved-fingerprint" }),
        ]);
      }, WAIT_OPTS);
    });

    test("starts a session while the existing device fingerprint is pending", async () => {
      expect.hasAssertions();

      const { device } = pendingFingerprint();
      const { tracker, fetcher } = makeTracker({ device });

      tracker.startDeviceIdentity();

      tracker.start();

      await vi.waitFor(async () => {
        await expect(identifiedBodies(fetcher)).resolves.toStrictEqual([
          expect.objectContaining({
            deviceId: tracker.getDeviceId(),
            fpHash: null,
          }),
        ]);
      }, WAIT_OPTS);
    });

    test("does not send a fingerprint after device consent is revoked", async () => {
      expect.hasAssertions();

      const { device, fingerprint } = pendingFingerprint();
      const { tracker, fetcher } = makeTracker({ device });

      tracker.startDeviceIdentity();

      tracker.start();

      await waitForOneCall(fetcher);

      tracker.discardDeviceIdentity();

      fingerprint.resolve("late-fingerprint");

      await nextMacrotask();

      expect(sessionCalls(fetcher)).toHaveLength(1);
      expect(tracker.getFpHash()).toBeNull();
    });

    test("startDeviceIdentity is idempotent", () => {
      const { tracker } = makeTracker();

      tracker.start();

      tracker.startDeviceIdentity();

      const id = tracker.getDeviceId();

      tracker.startDeviceIdentity();

      expect(tracker.getDeviceId()).toBe(id);
    });

    test("discardDeviceIdentity purges storage and a re-grant mints a fresh identity", () => {
      const { tracker } = makeTracker();

      tracker.start();

      tracker.startDeviceIdentity();

      const first = tracker.getDeviceId();

      expect(first).toMatch(/\S/u);
      expect(localStorage.getItem("interfere:device_id")).toBe(first);

      tracker.discardDeviceIdentity();

      expect(tracker.getDeviceId()).toBeNull();
      expect(localStorage.getItem("interfere:device_id")).toBeNull();
      expect(document.cookie).not.toContain("interfere_did=");

      tracker.startDeviceIdentity();

      const second = tracker.getDeviceId();

      expect(second).toMatch(/\S/u);
      expect(second).not.toBe(first);
    });

    test("a purge racing the fingerprint probe stays purged", async () => {
      const { device, fingerprint } = pendingFingerprint();

      const { tracker } = makeTracker({ device });

      tracker.start();

      tracker.startDeviceIdentity();

      tracker.discardDeviceIdentity();

      fingerprint.resolve("late-fp");

      await fingerprint.promise;

      expect(tracker.getFpHash()).toBeNull();
    });

    test("reset() before identity starts does not mint a device id", () => {
      const { tracker } = makeTracker();

      tracker.start();

      tracker.reset();

      expect(tracker.getDeviceId()).toBeNull();
      expect(document.cookie).not.toContain("interfere_did=");
    });

    test("backfill lands on the current session when rotation happens mid-fingerprint", async () => {
      expect.hasAssertions();

      const { device, fingerprint: fp } = pendingFingerprint();

      const { tracker, fetcher } = makeTracker({ device });

      tracker.start();

      const firstSession = tracker.sessionId();

      await waitForPost(fetcher);

      fetcher.mockClear();

      tracker.startDeviceIdentity();

      tracker.rotateSession();

      const rotatedSession = tracker.sessionId();

      expect(rotatedSession).not.toBe(firstSession);

      fp.resolve("fp-hash");

      await vi.waitFor(async () => {
        const identified = await identifiedBodies(fetcher);

        expect(identified).toHaveLength(2);
      }, WAIT_OPTS);

      await nextMacrotask();

      const identified = await identifiedBodies(fetcher);

      expect(identified).toStrictEqual([
        expect.objectContaining({ sessionId: rotatedSession, fpHash: null }),
        expect.objectContaining({
          sessionId: rotatedSession,
          fpHash: "fp-hash",
        }),
      ]);
    });

    test("dedupes concurrent syncs for the same identity state to one POST", async () => {
      expect.hasAssertions();

      const { device, fingerprint: fp } = pendingFingerprint();

      const { tracker, fetcher } = makeTracker({ device });

      tracker.start();

      tracker.startDeviceIdentity();

      fp.resolve(null);

      await waitForPost(fetcher);

      await nextMacrotask();

      expect(sessionCalls(fetcher)).toHaveLength(1);

      await expect(
        bodyOf(sessionCalls(fetcher)[0] ?? []),
      ).resolves.toMatchObject({
        deviceId: tracker.getDeviceId(),
      });
    });

    test("supports identity starting before the session starts", async () => {
      expect.hasAssertions();

      const { tracker, fetcher } = makeTracker();

      tracker.startDeviceIdentity();

      tracker.start();

      await vi.waitFor(async () => {
        const identified = await identifiedBodies(fetcher);

        expect(identified.length).toBeGreaterThanOrEqual(1);
      }, WAIT_OPTS);

      const bodies = await Promise.all(
        sessionCalls(fetcher).map(async (call) => await bodyOf(call)),
      );

      expect(bodies).toContainEqual(
        expect.objectContaining({
          sessionId: tracker.sessionId(),
          deviceId: tracker.getDeviceId(),
        }),
      );
    });

    test("retries a failed backfill with identity intact once the cooldown passes", async () => {
      expect.hasAssertions();

      vi.useFakeTimers({ toFake: ["Date"] });

      const start = 2_000_000;

      vi.setSystemTime(start);

      const fetcher = observeFetch();

      const { tracker } = makeTracker({
        fetcher,
        device: await fingerprintedDevice(),
      });

      tracker.start();

      await waitForOneCall(fetcher);

      fetcher.mockClear();

      worker.use(
        http.post(`${TARGET.baseUrl}/v3/session`, () => HttpResponse.error(), {
          once: true,
        }),
      );

      tracker.startDeviceIdentity();

      await waitForOneCall(fetcher);

      fetcher.mockClear();

      vi.setSystemTime(Date.now() + 5001);

      tracker.sessionId();

      await waitForOneCall(fetcher);

      await expect(
        bodyOf(sessionCalls(fetcher)[0] ?? []),
      ).resolves.toMatchObject({
        deviceId: expect.stringMatching(/./u),
      });

      vi.useRealTimers();
    });

    test("a backfill in flight when the tracker is disposed does not post", async () => {
      expect.hasAssertions();

      const { device, fingerprint: fp } = pendingFingerprint();

      const { tracker, fetcher } = makeTracker({ device });

      tracker.start();

      await waitForPost(fetcher);

      fetcher.mockClear();

      tracker.startDeviceIdentity();

      tracker.dispose();

      fp.resolve("fp-late");

      await fp.promise;

      expect(sessionCalls(fetcher)).toHaveLength(0);
    });
  });
});
