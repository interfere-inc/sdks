import { ActivityMonitor } from "../internal/activity/activity.js";
import { EXPIRE_AFTER_MS } from "../internal/activity/timing.js";
import {
  SOURCE,
  createSessionBody,
  identifiesArrived,
  inits,
  makeTracker,
  sessions,
  sessionsArrived,
  settle,
  setupLifecycle,
  started,
  teardownLifecycle,
} from "./lifecycle.fixture.js";
import { SessionManager } from "./session.js";
import { browserStorage } from "@interfere/test-config/browser-storage";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

beforeEach(setupLifecycle);

afterEach(teardownLifecycle);

function otherTabScenario() {
  vi.useFakeTimers({ toFake: ["Date", "performance", "setInterval"] });

  const tab = new SessionManager();

  const monitor = new ActivityMonitor();

  monitor.start({
    markActivity: () => {
      tab.markActivity();
    },
    reset: () => {
      tab.reset();
    },
    startedAt: () => tab.getSessionStartedAt(),
    lastActivityAt: () => tab.getLastActivityAt(),
  });

  const shared = tab.getSessionId();

  vi.advanceTimersByTime(EXPIRE_AFTER_MS + 60_000);

  browserStorage.local.set(
    "interfere:last_activity",
    String(Date.now() - 1000),
  );

  globalThis.dispatchEvent(new PointerEvent("pointerdown"));

  monitor.stop();

  return { after: tab.getSessionId(), shared };
}

describe("A20 anonymous session creation", () => {
  test("persists the session id and posts it with no device before device identity starts", async () => {
    expect.hasAssertions();

    const tracker = await started();

    expect(localStorage.getItem("interfere:session_id")).toBe(
      tracker.sessionId(),
    );

    expect(sessions()).toStrictEqual([
      { deviceId: null, fpHash: null, sessionId: tracker.sessionId() },
    ]);
  });
});

describe("A21 ingest request options", () => {
  test("session and identify requests omit credentials and use keepalive", async () => {
    expect.hasAssertions();

    const tracker = await started();

    await tracker.identify({ identifier: "user-a21", source: SOURCE });

    await identifiesArrived(1);

    expect(inits().length).toBeGreaterThanOrEqual(2);

    expect(
      inits().map((init) => [init.credentials, init.keepalive]),
    ).toStrictEqual(inits().map(() => ["omit", true]));
  });
});

describe("A25 input after the expiry gap rotates the session", () => {
  test("a new session id is minted, persisted and posted", async () => {
    expect.hasAssertions();

    vi.useFakeTimers({ toFake: ["Date", "performance"] });

    const tracker = await started();

    const first = tracker.sessionId();

    vi.advanceTimersByTime(EXPIRE_AFTER_MS + 1000);

    globalThis.dispatchEvent(new KeyboardEvent("keydown"));

    const second = tracker.sessionId();

    expect(second).not.toBe(first);
    expect(localStorage.getItem("interfere:session_id")).toBe(second);

    await sessionsArrived(2);

    expect(sessions().at(-1)?.sessionId).toBe(second);
  });
});

describe("A26 visibility and rotation", () => {
  test.each([
    { gap: EXPIRE_AFTER_MS + 1000, rotates: true },
    { gap: EXPIRE_AFTER_MS - 60_000, rotates: false },
  ])(
    "shown after $gap ms hidden rotates: $rotates",
    async ({ gap, rotates }) => {
      expect.hasAssertions();

      vi.useFakeTimers({ toFake: ["Date", "performance"] });

      const tracker = await started();

      const first = tracker.sessionId();

      globalThis.dispatchEvent(
        new PageTransitionEvent("pagehide", { persisted: true }),
      );

      vi.advanceTimersByTime(gap);

      globalThis.dispatchEvent(
        new PageTransitionEvent("pageshow", { persisted: true }),
      );

      expect(tracker.sessionId() !== first).toBe(rotates);
    },
  );
});

describe("session expiry across system sleep", () => {
  test.each(["pointerdown", "pageshow"])(
    "rotates on %s after overnight sleep without advancing the monotonic clock",
    async (event) => {
      expect.hasAssertions();

      vi.useFakeTimers({ toFake: ["Date"] });

      const tracker = await started();

      const first = tracker.sessionId();

      if (event === "pageshow") {
        globalThis.dispatchEvent(
          new PageTransitionEvent("pagehide", { persisted: true }),
        );
      }

      vi.setSystemTime(Date.now() + 12 * 60 * 60 * 1000);

      globalThis.dispatchEvent(
        event === "pageshow"
          ? new PageTransitionEvent("pageshow", { persisted: true })
          : new PointerEvent("pointerdown"),
      );

      const second = tracker.sessionId();

      expect(second).not.toBe(first);

      await sessionsArrived(2);

      expect(sessions().at(-1)?.sessionId).toBe(second);
    },
  );
});

describe("A28 SPA navigation keeps the session", () => {
  test("pushState, replaceState, popstate and hashchange keep the session id and send no new session", async () => {
    expect.hasAssertions();

    const tracker = await started();

    const first = tracker.sessionId();

    const original = location.href;

    history.pushState({}, "", "/lifecycle/next");

    history.replaceState({}, "", "/lifecycle/replaced");

    globalThis.dispatchEvent(new PopStateEvent("popstate"));

    globalThis.dispatchEvent(new HashChangeEvent("hashchange"));

    history.replaceState({}, "", original);

    expect(tracker.sessionId()).toBe(first);

    await settle();

    expect(sessions()).toHaveLength(1);
  });
});

describe("A29 storage cleared mid-session", () => {
  test("the in-memory session id stays stable and no new session is posted", async () => {
    expect.hasAssertions();

    const tracker = await started();

    const first = tracker.sessionId();

    browserStorage.local.clear();

    expect(tracker.sessionId()).toBe(first);
    expect(tracker.sessionId()).toBe(first);

    await settle();

    expect(sessions()).toHaveLength(1);
  });
});

describe("A31 two tabs share a session", () => {
  test("a second tab joins the live session but keeps its own window id", async () => {
    expect.hasAssertions();

    const tabA = await started();

    const windowA = tabA.windowId();

    browserStorage.session.remove("interfere:window_id");

    const tabB = makeTracker();

    tabB.start();

    expect(tabB.sessionId()).toBe(tabA.sessionId());
    expect(tabB.windowId()).toMatch(/^win_/u);
    expect(tabB.windowId()).not.toBe(windowA);
  });
});

describe("A32 activity in another tab", () => {
  test("input in this tab after the user was active in another tab keeps the shared session", () => {
    expect.hasAssertions();

    const { after, shared } = otherTabScenario();

    expect(after).toBe(shared);
  });
});

describe("A35 repeated session reads", () => {
  test("many sessionId() reads send one session request", async () => {
    expect.hasAssertions();

    const tracker = await started();

    for (let i = 0; i < 20; i += 1) {
      tracker.sessionId();
    }

    await settle();

    expect(sessions()).toHaveLength(1);
  });
});

describe("A70 session request contract", () => {
  test("every session body parses with the server's create-session shape", async () => {
    expect.hasAssertions();

    const tracker = await started();

    tracker.startDeviceIdentity();

    await sessionsArrived(2);

    expect(
      sessions().map((body) => createSessionBody.safeParse(body).success),
    ).toStrictEqual([true, true]);
  });
});

describe("A72 tracking-disabled surfaces", () => {
  test("an ok response is treated as synced and no retry follows", async () => {
    expect.hasAssertions();

    const tracker = await started();

    await settle();

    tracker.sessionId();

    await settle();

    expect(sessions()).toHaveLength(1);
  });
});
