import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { attribute, exceptions } from "@interfere/test-utils/vitest/otlp";
import { context, metrics, propagation, trace } from "@opentelemetry/api";
import { logs } from "@opentelemetry/api-logs";
import {
  afterAll,
  assert,
  beforeAll,
  beforeEach,
  expect,
  test,
  vi,
} from "vitest";

import { register } from "./register";
import { runtime } from "./runtime";

let collector: Awaited<ReturnType<typeof receiver>>;

const uncaught = process.listeners("uncaughtException");
const rejected = process.listeners("unhandledRejection");

function stubServerEnv() {
  vi.stubEnv("OTEL_SERVICE_NAME", undefined);

  vi.stubEnv("INTERFERE_API_KEY", "key+value");

  vi.stubEnv("INTERFERE_API_URL", collector.url);

  vi.stubEnv("INTERFERE_SOURCE_ID", "a".repeat(40));
}

beforeAll(async () => {
  collector = await receiver();

  stubServerEnv();

  await register({ consoleBridge: false });
});

beforeEach(stubServerEnv);

afterAll(async () => {
  for (const listener of process.listeners("uncaughtException")) {
    if (!uncaught.includes(listener)) {
      process.off("uncaughtException", listener);
    }
  }

  for (const listener of process.listeners("unhandledRejection")) {
    if (!rejected.includes(listener)) {
      process.off("unhandledRejection", listener);
    }
  }

  await runtime.shutdown();

  trace.disable();

  metrics.disable();

  logs.disable();

  context.disable();

  propagation.disable();

  vi.unstubAllEnvs();

  await collector.close();
});

test("captures the server process's uncaught exceptions and unhandled rejections", async () => {
  const onUncaught = process
    .listeners("uncaughtException")
    .find((listener) => !uncaught.includes(listener));

  const onRejected = process
    .listeners("unhandledRejection")
    .find((listener) => !rejected.includes(listener));

  assert(
    onUncaught && onRejected,
    "register() installs no process error listeners",
  );

  onUncaught(new Error("vite-server-uncaught"), "uncaughtException");

  onRejected(new Error("vite-server-rejection"), Promise.resolve());

  await expect
    .poll(() =>
      ["vite-server-uncaught", "vite-server-rejection"].map((message) =>
        exceptions(collector.requests, message).flatMap((span) =>
          span.events.map((event) =>
            attribute(event.attributes, "interfere.exception.mechanism"),
          ),
        ),
      ),
    )
    .toStrictEqual([
      ["auto.node.global_handlers.uncaught_exception"],
      ["auto.node.global_handlers.unhandled_rejection"],
    ]);
});
