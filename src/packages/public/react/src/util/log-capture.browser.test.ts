import { setupErrorsPlugin } from "../../test/fixtures/errors.test-utils.js";
import {
  CONSOLE_LEVELS,
  onConsoleCall,
  resetConsolePatchForTests,
} from "../internal/console-patch/console-patch.js";
import type { ConsoleHandler } from "../internal/console-patch/console-patch.js";
import { createLogger, getLogLevel, setLogLevel } from "./log.js";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

let originalLevel: ReturnType<typeof getLogLevel>;

beforeEach(() => {
  originalLevel = getLogLevel();

  setLogLevel("debug");
});

afterEach(() => {
  setLogLevel(originalLevel);

  resetConsolePatchForTests();

  vi.restoreAllMocks();
});

test("sdk log output reaches the native console but no console subscriber", () => {
  const natives = new Map(
    CONSOLE_LEVELS.map((level) => [level, vi.spyOn(console, level)]),
  );

  const subscriber = vi.fn<ConsoleHandler>();

  onConsoleCall(subscriber);

  const log = createLogger("isolation");

  log.debug("debug line");

  log.info("info line");

  log.warn("warn line");

  log.error("error line", new Error("sdk failure"));

  expect(subscriber).not.toHaveBeenCalled();

  expect(natives.get("debug")).toHaveBeenCalledExactlyOnceWith(
    "[Interfere:isolation]",
    "debug line",
  );

  expect(natives.get("error")).toHaveBeenCalledWith(
    "[Interfere:isolation]",
    "error line",
    expect.any(Error),
  );

  console.warn("host warning");

  expect(subscriber).toHaveBeenCalledExactlyOnceWith("warn", ["host warning"]);
});

test("an SDK error logged with an Error is not captured as a customer exception", () => {
  const { exporter } = setupErrorsPlugin();

  createLogger("device").error(
    "fingerprint init failed",
    new Error("fingerprint unavailable"),
  );

  expect(exporter.getFinishedSpans()).toStrictEqual([]);
});
