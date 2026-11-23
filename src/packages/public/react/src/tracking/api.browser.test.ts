/* oxlint-disable typescript/no-unsafe-assignment, typescript/require-await */
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
});
