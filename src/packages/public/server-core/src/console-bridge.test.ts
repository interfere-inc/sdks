/* oxlint-disable eslint/no-console -- Verifies console method identity across bridge installation and disposal. */
import { setImmediate } from "node:timers/promises";
import { SeverityNumber, logs } from "@opentelemetry/api";
import {
  InMemoryLogRecordExporter,
  LoggerProvider,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";

import { bridgeConsoleToOtel } from "./console-bridge.js";
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  test,
  vi,
} from "vitest";

const exporter = new InMemoryLogRecordExporter();

const provider = new LoggerProvider({
  processors: [new SimpleLogRecordProcessor({ exporter })],
});

let restoreConsole: ReturnType<typeof bridgeConsoleToOtel>;

beforeAll(() => {
  logs.setGlobalLoggerProvider(provider);
});

beforeEach(() => {
  vi.spyOn(console, "error");

  restoreConsole = bridgeConsoleToOtel();
});

afterEach(() => {
  restoreConsole();

  exporter.reset();

  vi.restoreAllMocks();
});

afterAll(async () => {
  await provider.shutdown();

  logs.disable();
});

describe("console exception attributes", () => {
  test("forwards unchanged arguments immediately and restores the original method", async () => {
    restoreConsole();

    const original = console.error;
    const value = { code: 500 };

    restoreConsole = bridgeConsoleToOtel();

    console.error("failure", value);

    expect(original).toHaveBeenCalledExactlyOnceWith("failure", value);

    await provider.forceFlush();

    await expect.poll(() => exporter.getFinishedLogRecords()).toHaveLength(1);

    restoreConsole();

    expect(console.error).toBe(original);

    console.error("terminal only");

    await setImmediate();

    await provider.forceFlush();

    expect(exporter.getFinishedLogRecords()).toHaveLength(1);
    expect(original).toHaveBeenLastCalledWith("terminal only");
  });

  test.each([
    { value: 'a "quoted" message', expected: 'a "quoted" message' },
    { value: null, expected: "null" },
    { value: undefined, expected: "" },
    { value: false, expected: "false" },
    { value: 42, expected: "42" },
    { value: 42n, expected: "42" },
    { value: Symbol("context"), expected: "" },
    { value: { code: 500 }, expected: '{"code":500}' },
  ])(
    "preserves console serialization for $expected",
    async ({ value, expected }) => {
      console.error(value);

      await provider.forceFlush();

      await expect
        .poll(() => exporter.getFinishedLogRecords()[0]?.body)
        .toBe(expected);
    },
  );

  test("uses string conversion when JSON serialization encounters a cycle", async () => {
    const value = {};

    Object.assign(value, { self: value });

    console.error(value);

    await provider.forceFlush();

    await expect
      .poll(() => exporter.getFinishedLogRecords()[0]?.body)
      .toBe("[object Object]");
  });

  test.each([
    [undefined, {}],
    ["", {}],
    [
      "Error: failed\n at checkout",
      { "exception.stacktrace": "Error: failed\n at checkout" },
    ],
  ] as const)(
    "preserves stack omission for %s",
    async (stack, expectedStack) => {
      const error = new Error("failed");

      Object.defineProperty(error, "stack", { value: stack });

      console.error(error);

      await provider.forceFlush();

      await expect.poll(() => exporter.getFinishedLogRecords()).toHaveLength(1);

      const records = exporter.getFinishedLogRecords();

      expect(records).toHaveLength(1);

      expect(records[0]?.attributes).toStrictEqual({
        "exception.type": "Error",
        "exception.message": "failed",
        ...expectedStack,
      });
    },
  );

  test("does not attach exception fields to ordinary console arguments", async () => {
    console.error("message", { code: 500 });

    await provider.forceFlush();

    await expect
      .poll(() => exporter.getFinishedLogRecords()[0]?.attributes)
      .toStrictEqual({});
  });

  test("uses the first error argument", async () => {
    console.error("failed", new TypeError("first"), new Error("second"));

    await provider.forceFlush();

    await expect
      .poll(() => exporter.getFinishedLogRecords()[0]?.attributes)
      .toMatchObject({
        "exception.type": "TypeError",
        "exception.message": "first",
      });
  });
});

test("disposal leaves methods below the threshold untouched", () => {
  restoreConsole();

  const originalDebug = console.debug;
  const originalInfo = console.info;
  const originalLog = console.log;
  const originalWarn = console.warn;
  const originalError = console.error;

  restoreConsole = bridgeConsoleToOtel(provider, SeverityNumber.WARN);

  expect(console.debug).toBe(originalDebug);
  expect(console.info).toBe(originalInfo);
  expect(console.log).toBe(originalLog);
  expect(console.warn).not.toBe(originalWarn);
  expect(console.error).not.toBe(originalError);

  const laterInfo = vi.spyOn(console, "info");

  restoreConsole();

  expect(console.info).toBe(laterInfo);
  expect(console.warn).toBe(originalWarn);
  expect(console.error).toBe(originalError);
});
