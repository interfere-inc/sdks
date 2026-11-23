/* oxlint-disable no-console */
import {
  CONSOLE_LEVELS,
  onConsoleCall,
  pinConsolePatch,
  resetConsolePatchForTests,
} from "./console-patch.js";
import type { ConsoleHandler } from "./console-patch.js";
import { afterEach, expect, test, vi } from "vitest";

afterEach(() => {
  resetConsolePatchForTests();

  vi.restoreAllMocks();
});

test.each(CONSOLE_LEVELS)(
  "forwards native %s calls and restores the original method after unsubscribe",
  (level) => {
    const original = vi.spyOn(console, level);
    const handler = vi.fn<ConsoleHandler>();
    const unsubscribe = onConsoleCall(handler);

    console[level]("message", 42);

    expect(original).toHaveBeenCalledExactlyOnceWith("message", 42);
    expect(handler).toHaveBeenCalledExactlyOnceWith(level, ["message", 42]);

    unsubscribe();

    expect(console[level]).toBe(original);

    console[level]("after unsubscribe");

    expect(original).toHaveBeenCalledTimes(2);
    expect(handler).toHaveBeenCalledTimes(1);
  },
);

test("keeps native forwarding installed until the last subscriber leaves", () => {
  const original = vi.spyOn(console, "info");
  const first = vi.fn<ConsoleHandler>();
  const second = vi.fn<ConsoleHandler>();
  const removeFirst = onConsoleCall(first);
  const installed = console.info;
  const removeSecond = onConsoleCall(second);

  expect(console.info).toBe(installed);

  removeFirst();

  console.info("remaining subscriber");

  expect(first).not.toHaveBeenCalled();

  expect(second).toHaveBeenCalledExactlyOnceWith("info", [
    "remaining subscriber",
  ]);

  expect(original).toHaveBeenCalledExactlyOnceWith("remaining subscriber");
  expect(console.info).toBe(installed);

  removeSecond();

  expect(console.info).toBe(original);
});

test("reset removes old subscribers and supports a fresh native installation", () => {
  const originals = CONSOLE_LEVELS.map((level) => console[level]);
  const stale = vi.fn<ConsoleHandler>();
  const fresh = vi.fn<ConsoleHandler>();

  onConsoleCall(stale);

  resetConsolePatchForTests();

  expect(CONSOLE_LEVELS.map((level) => console[level])).toStrictEqual(
    originals,
  );

  const unsubscribe = onConsoleCall(fresh);

  console.info("fresh installation");

  expect(stale).not.toHaveBeenCalled();
  expect(fresh).toHaveBeenCalledExactlyOnceWith("info", ["fresh installation"]);

  unsubscribe();

  expect(CONSOLE_LEVELS.map((level) => console[level])).toStrictEqual(
    originals,
  );
});

test("leaves a wrapper installed later in place and stops forwarding beneath it", () => {
  const original = console.warn;
  const handler = vi.fn<ConsoleHandler>();

  const unsubscribe = onConsoleCall(handler);

  const ours = console.warn;

  const seen: unknown[][] = [];

  const foreign = (...args: unknown[]) => {
    seen.push(args);

    ours.apply(console, args);
  };

  // oxlint-disable-next-line landmines/no-test-doubles -- A real passthrough wrapper models a later SDK; the native method is still called.
  console.warn = foreign;

  console.warn("shared");

  expect(handler).toHaveBeenCalledExactlyOnceWith("warn", ["shared"]);

  unsubscribe();

  expect(console.warn).toBe(foreign);

  console.warn("after");

  expect(seen).toStrictEqual([["shared"], ["after"]]);
  expect(handler).toHaveBeenCalledTimes(1);

  const fresh = vi.fn<ConsoleHandler>();

  const unsubscribeFresh = onConsoleCall(fresh);

  console.warn("fresh");

  expect(fresh).toHaveBeenCalledExactlyOnceWith("warn", ["fresh"]);
  expect(handler).toHaveBeenCalledTimes(1);

  unsubscribeFresh();

  expect(console.warn).toBe(foreign);

  console.warn = original;
});

test("a console.error reference saved after pinning reaches subscribers that arrive later", () => {
  pinConsolePatch();

  const saved = console.error.bind(console);
  const handler = vi.fn<ConsoleHandler>();
  const first = onConsoleCall(() => undefined);

  first();

  const unsubscribe = onConsoleCall(handler);

  saved("framework caught", 1);

  unsubscribe();

  expect(handler).toHaveBeenCalledExactlyOnceWith("error", [
    "framework caught",
    1,
  ]);
});
