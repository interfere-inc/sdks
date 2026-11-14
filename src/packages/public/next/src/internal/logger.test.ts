import { writeConsoleLines } from "./console-output";
import { FatalError, log } from "./logger.js";
import { expect, test, vi } from "vitest";

test.each([
  ["info", "log"],
  ["warn", "warn"],
  ["error", "error"],
] as const)("uses the native console receiver for %s", (level, method) => {
  vi.stubEnv("VITEST", "");

  vi.stubEnv("VITEST_WORKER_ID", "");

  const output = vi.spyOn(console, method);

  log[level]("Request failed", ["First detail", "Second detail"]);

  expect(output).toHaveBeenCalledTimes(3);
  expect(output.mock.contexts).toStrictEqual([console, console, console]);

  expect(output).toHaveBeenNthCalledWith(
    1,
    expect.stringContaining("Request failed"),
  );

  expect(output).toHaveBeenNthCalledWith(
    2,
    expect.stringContaining("First detail"),
  );

  expect(output).toHaveBeenNthCalledWith(
    3,
    expect.stringContaining("Second detail"),
  );
});

test.each([undefined, null, false, "unavailable"])(
  "skips unavailable console methods: %j",
  (method) => {
    const output = vi.spyOn(console, "log");

    expect(() =>
      writeConsoleLines(method, ["Request failed", "Detail"]),
    ).not.toThrow();

    expect(output).not.toHaveBeenCalled();
  },
);

test("suppresses logs and throws fatal errors in tests", () => {
  vi.stubEnv("VITEST", "true");

  const output = vi.spyOn(console, "log");

  log.info("Hidden", ["Detail"]);

  expect(output).not.toHaveBeenCalled();
  expect(() => log.fatal("Failure", [])).toThrow(FatalError);
});
