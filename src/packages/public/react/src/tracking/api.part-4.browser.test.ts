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
  });
});
