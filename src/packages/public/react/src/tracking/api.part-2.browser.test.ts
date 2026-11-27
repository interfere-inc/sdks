/* oxlint-disable typescript/no-unsafe-assignment, typescript/no-unsafe-call, typescript/no-unsafe-member-access, typescript/require-await, typescript/strict-boolean-expressions */
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

const SOURCE = { name: "Test", type: "custom" } as const;

async function waitForOneCall(fetcher: FetchObservation) {
  await vi.waitFor(
    () => expect(sessionCalls(fetcher)).toHaveLength(1),
    WAIT_OPTS,
  );

  await settleSessions();
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
});
