import { browserStorage } from "@interfere/test-config/browser-storage";
import { beforeEach, expect, test } from "vitest";

import { SessionSlot } from "./session-slot.js";

const SESSION_ID_KEY = "interfere:session_id";

let rotations: string[];

let slot: SessionSlot;

beforeEach(() => {
  browserStorage.local.clear();

  browserStorage.session.clear();

  rotations = [];

  slot = new SessionSlot((sessionId) => {
    rotations.push(sessionId);
  });
});

test("a prepared session is readable before start, and start adopts it and reports its rotation once", () => {
  slot.prepare();

  const pending = slot.pendingSessionId();

  expect(pending).not.toBeNull();

  expect(slot.manager).toBeNull();

  expect(rotations).toStrictEqual([]);

  slot.start();

  expect(slot.manager?.sessionId).toBe(pending);

  expect(slot.pendingSessionId()).toBeNull();

  expect(rotations).toStrictEqual([pending]);
});

test("a persistence switch before start replaces the prepared session and reports the upgrade from start", () => {
  const upgrades: string[][] = [];
  const switches: boolean[] = [];

  slot.onUpgrade((previousId, sessionId) => {
    upgrades.push([previousId, sessionId]);
  });

  slot.setPersistent(false, () => {
    switches.push(false);
  });

  slot.prepare();

  const page = slot.pendingSessionId();

  expect(
    slot.setPersistent(true, () => {
      switches.push(true);
    }),
  ).toBe(false);

  const persisted = slot.pendingSessionId();

  expect(persisted).not.toBe(page);

  slot.start();

  expect(slot.manager?.sessionId).toBe(persisted);

  expect(rotations).toStrictEqual([persisted]);

  expect(upgrades).toStrictEqual([[page, persisted]]);

  expect(switches).toStrictEqual([]);
});

test("a downgrade before start erases the persisted session the prepared one created", () => {
  slot.prepare();

  const persisted = slot.pendingSessionId();

  expect(localStorage.getItem(SESSION_ID_KEY)).toBe(persisted);

  slot.setPersistent(false, () => undefined);

  expect(localStorage.getItem(SESSION_ID_KEY)).toBeNull();

  slot.start();

  expect(slot.manager?.getSessionId()).not.toBe(persisted);

  expect(localStorage.getItem(SESSION_ID_KEY)).toBeNull();

  expect(rotations).toStrictEqual([]);
});
