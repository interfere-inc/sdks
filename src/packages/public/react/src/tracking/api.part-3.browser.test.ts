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
  });
});
