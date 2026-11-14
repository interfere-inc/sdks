/* oxlint-disable eslint/no-inline-comments, typescript/no-unsafe-assignment */
import { createLogger, getLogLevel, setLogLevel } from "./log.js";
import type { LogLevel } from "./log.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

describe("logger", () => {
  let originalLevel: LogLevel;

  beforeEach(() => {
    originalLevel = getLogLevel();

    vi.spyOn(globalThis.console, "debug");

    vi.spyOn(globalThis.console, "info");

    vi.spyOn(globalThis.console, "warn");

    vi.spyOn(globalThis.console, "error");

    setLogLevel("debug");
  });

  afterEach(() => {
    setLogLevel(originalLevel);
  });

  test("includes scoped prefix in output", () => {
    const log = createLogger("test");

    log.info("hello");

    expect(globalThis.console.info).toHaveBeenCalledWith(
      "[Interfere:test]",
      "hello",
    );
  });

  test("routes to the correct console method", () => {
    const log = createLogger("routing");

    log.debug("d");

    log.info("i");

    log.warn("w");

    log.error("e");

    expect(globalThis.console.debug).toHaveBeenCalledTimes(1);
    expect(globalThis.console.info).toHaveBeenCalledTimes(1);
    expect(globalThis.console.warn).toHaveBeenCalledTimes(1);
    expect(globalThis.console.error).toHaveBeenCalledTimes(1);
  });

  test("suppresses messages below threshold", () => {
    setLogLevel("error");

    const log = createLogger("filtered");

    log.debug("no");

    log.info("no");

    log.warn("no");

    log.error("yes");

    expect(globalThis.console.debug).not.toHaveBeenCalled();
    expect(globalThis.console.info).not.toHaveBeenCalled();
    expect(globalThis.console.warn).not.toHaveBeenCalled();
    expect(globalThis.console.error).toHaveBeenCalledTimes(1);
  });

  test("suppresses all messages at none level", () => {
    setLogLevel("none");

    const log = createLogger("silent");

    log.debug("no");

    log.info("no");

    log.warn("no");

    log.error("no");

    expect(globalThis.console.debug).not.toHaveBeenCalled();
    expect(globalThis.console.info).not.toHaveBeenCalled();
    expect(globalThis.console.warn).not.toHaveBeenCalled();
    expect(globalThis.console.error).not.toHaveBeenCalled();
  });

  test("passes through multiple arguments", () => {
    const log = createLogger("args");

    log.warn("a", "b", 3);

    expect(globalThis.console.warn).toHaveBeenCalledWith(
      "[Interfere:args]",
      "a",
      "b",
      3,
    );
  });
});

describe("getLogLevel / setLogLevel", () => {
  let originalLevel: LogLevel;

  beforeEach(() => {
    originalLevel = getLogLevel();
  });

  afterEach(() => {
    setLogLevel(originalLevel);
  });

  test("defaults to warn in a fresh module without explicit configuration", async () => {
    const modulePath = "./log.ts?default-level";

    const logger: typeof import("./log.js") = await import(
      /* @vite-ignore */ modulePath
    );

    const debug = vi.spyOn(console, "debug");
    const info = vi.spyOn(console, "info");
    const warn = vi.spyOn(console, "warn");
    const error = vi.spyOn(console, "error");
    const log = logger.createLogger("default");

    expect(logger.getLogLevel()).toBe("warn");

    log.debug("hidden");

    log.info("hidden");

    log.warn("visible warning");

    log.error("visible error");

    expect(debug).not.toHaveBeenCalled();
    expect(info).not.toHaveBeenCalled();
    expect(warn).toHaveBeenCalledWith("[Interfere:default]", "visible warning");
    expect(error).toHaveBeenCalledWith("[Interfere:default]", "visible error");
  });

  test("reflects changes from setLogLevel", () => {
    setLogLevel("debug");

    expect(getLogLevel()).toBe("debug");

    setLogLevel("error");

    expect(getLogLevel()).toBe("error");
  });
});
