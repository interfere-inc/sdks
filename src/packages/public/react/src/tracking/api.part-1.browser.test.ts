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

function identifyRequests(fetcher: FetchObservation) {
  return sessionCalls(fetcher).filter((call) =>
    new Request(call[0]).url.endsWith("/identify"),
  );
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
});
