/* oxlint-disable typescript/no-unsafe-assignment, typescript/require-await */
import { SessionTracker } from "./api.js";
import { DeviceManager } from "./device.js";
import { InterfereClient } from "@interfere/javascript";
import { HttpResponse, http } from "msw";
import { worker } from "../../test/msw.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";
import type { MockInstance } from "vitest";

const TARGET = {
  releaseTokenHeader: { apiKey: "test-key" },
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
