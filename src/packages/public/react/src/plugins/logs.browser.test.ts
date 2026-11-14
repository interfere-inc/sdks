import { context, propagation } from "@opentelemetry/api";
import { SeverityNumber } from "@opentelemetry/api-logs";
import {
  InMemoryLogRecordExporter,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";

import { Kernel } from "../internal/kernel.js";
import { PluginRuntime } from "../internal/plugins/plugin-runtime.js";
import {
  registerKernel,
  unregisterKernel,
} from "../internal/kernel-registry.js";
import { CONSOLE_LEVELS } from "../internal/console-patch/console-patch.js";
import { buildOtelProvider } from "../internal/otel/provider.js";
import { SessionTracker } from "../tracking/api.js";
import logsPlugin from "./logs.js";
import { HttpResponse, http } from "msw";
import { afterEach, assert, beforeEach, expect, test, vi } from "vitest";

import { worker } from "../../test/msw.js";
import { DISABLE_ALL_PLUGINS } from "../../test/fixtures/disable-all-plugins.js";

let kernel: Kernel;

let exporter: InMemoryLogRecordExporter;

let cleanup: ReturnType<typeof logsPlugin.setup>;

let originals: Map<(typeof CONSOLE_LEVELS)[number], Console["log"]>;

beforeEach(() => {
  for (const level of CONSOLE_LEVELS) {
    vi.spyOn(console, level);
  }

  originals = new Map(CONSOLE_LEVELS.map((level) => [level, console[level]]));

  worker.use(
    http.post(
      "https://sdk.invalid/drain/v1/logs",
      () => new HttpResponse(null, { status: 204 }),
    ),
  );

  exporter = new InMemoryLogRecordExporter();

  kernel = new Kernel(
    new SessionTracker({
      target: {
        baseUrl: "https://sdk.invalid",
        publicKeyHeader: { apiKey: "test-key" },
      },
    }),
  );

  const handle = buildOtelProvider({
    additionalLogRecordProcessors: [new SimpleLogRecordProcessor({ exporter })],

    getDeviceId: () => kernel.device.getDeviceId(),
    getSessionId: () => kernel.session.getId(),
    clientOptions: {
      baseUrl: "https://sdk.invalid",
      publicKeyHeader: { apiKey: "test-key" },
    },
    sdkStack: [],
  });

  kernel.attachOtel(handle, () => handle.contextManager.disable());

  registerKernel(kernel);

  cleanup = logsPlugin.setup(kernel);
});

afterEach(async () => {
  vi.useRealTimers();

  cleanup?.();

  cleanup = undefined;

  unregisterKernel(kernel);

  try {
    await kernel.dispose();
  } finally {
    context.disable();

    propagation.disable();

    vi.restoreAllMocks();
  }
});

test.each([
  {
    level: "warn",
    args: ["deprecated API"],
    body: "deprecated API",
    severityText: "WARN",
    severityNumber: SeverityNumber.WARN,
  },
  {
    level: "log",
    args: ["hello-test"],
    body: "hello-test",
    severityText: "INFO",
    severityNumber: SeverityNumber.INFO,
  },
  {
    level: "log",
    args: [{ data: 42 }],
    body: '{"data":42}',
    severityText: "INFO",
    severityNumber: SeverityNumber.INFO,
  },
  {
    level: "warn",
    args: ["status:", 500, { ok: false }],
    body: 'status: 500 {"ok":false}',
    severityText: "WARN",
    severityNumber: SeverityNumber.WARN,
  },
] as const)(
  "records $body through the real kernel logger",
  async ({ level, args, body, severityNumber, severityText }) => {
    console[level](...args);

    await kernel.flush();

    expect(exporter.getFinishedLogRecords()).toHaveLength(1);

    expect(exporter.getFinishedLogRecords()[0]).toMatchObject({
      body,
      severityNumber,
      severityText,
      attributes: { "console.level": level },
      instrumentationScope: { name: "@interfere/react" },
    });
  },
);

test.each([
  [new Error("boom-singular")],
  ["react detected", new Error("boom-multi")],
])("leaves Error-bearing calls to the error plugin: %s", async (...args) => {
  console.error(...args);

  await kernel.flush();

  expect(exporter.getFinishedLogRecords()).toStrictEqual([]);

  console.warn("still recording ordinary logs");

  await kernel.flush();

  expect(exporter.getFinishedLogRecords()).toHaveLength(1);

  expect(exporter.getFinishedLogRecords()[0]?.body).toBe(
    "still recording ordinary logs",
  );
});

test("patches every supported console method", () => {
  expect(
    CONSOLE_LEVELS.map((level) => console[level] === originals.get(level)),
  ).toStrictEqual([false, false, false, false, false]);
});

test("redacts credentials and sensitive URL values in console arguments", async () => {
  console.warn(
    "request",
    {
      url: "https://person:secret@example.com/path?token=private&page=2",
    },
    "https://example.com/other?PASSWORD=hidden",
  );

  await kernel.flush();

  expect(exporter.getFinishedLogRecords()[0]?.body).toBe(
    'request {"url":"https://REDACTED:REDACTED@example.com/path?token=REDACTED&page=2"} https://example.com/other?PASSWORD=REDACTED',
  );
});

test.each(["a", "😀"])(
  "caps UTF-8 console bodies without splitting %s",
  async (character) => {
    console.warn(character.repeat(16_385));

    await kernel.flush();

    const body = exporter.getFinishedLogRecords()[0]?.body;

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- OTel log bodies can be any value; this plugin must emit a string.
    assert(typeof body === "string");

    expect(new TextEncoder().encode(body).length).toBeLessThanOrEqual(16_384);
    expect(body).toMatch(/\[Truncated\]$/u);
    expect(body).not.toContain("�");
  },
);

test("limits all console levels together and resumes after the minute window", async () => {
  vi.useFakeTimers({ toFake: ["Date"] });

  vi.setSystemTime(1000);

  for (let index = 0; index < 21; index += 1) {
    for (const level of CONSOLE_LEVELS) {
      console[level]("burst", index);
    }
  }

  await kernel.flush();

  expect(exporter.getFinishedLogRecords()).toHaveLength(100);

  vi.setSystemTime(60_999);

  console.warn("still limited");

  vi.setSystemTime(61_000);

  console.warn("next window");

  await kernel.flush();

  expect(exporter.getFinishedLogRecords()).toHaveLength(101);
  expect(exporter.getFinishedLogRecords().at(-1)?.body).toBe("next window");
});

test("restores every console method and stops recording after teardown", async () => {
  cleanup?.();

  cleanup = undefined;

  expect(
    new Map(CONSOLE_LEVELS.map((level) => [level, console[level]])),
  ).toStrictEqual(originals);

  console.warn("after teardown");

  await kernel.flush();

  expect(exporter.getFinishedLogRecords()).toStrictEqual([]);
});

test("captures logs only while analytics consent is granted", async () => {
  cleanup?.();

  cleanup = undefined;

  const runtime = new PluginRuntime(kernel, false);

  kernel.attachRuntime(runtime);

  runtime.applyProviderConfig({ ...DISABLE_ALL_PLUGINS, logs: true }, false);

  await vi.dynamicImportSettled();

  console.warn("denied");

  await kernel.flush();

  expect(exporter.getFinishedLogRecords()).toStrictEqual([]);

  kernel.consent.set({ analytics: true, replay: false });

  await vi.dynamicImportSettled();

  console.warn("granted");

  await kernel.flush();

  expect(
    exporter.getFinishedLogRecords().map((record) => record.body),
  ).toStrictEqual(["granted"]);

  kernel.consent.set(false);

  console.warn("revoked");

  await kernel.flush();

  expect(
    exporter.getFinishedLogRecords().map((record) => record.body),
  ).toStrictEqual(["granted"]);
});
