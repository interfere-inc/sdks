/* oxlint-disable typescript/no-unsafe-assignment */
import type {
  InMemorySpanExporter,
  ReadableSpan,
} from "@opentelemetry/sdk-trace-base";

import { setupErrorsPlugin } from "../../test/fixtures/errors.test-utils.js";
import { expect, onTestFinished, test } from "vitest";

const ONERROR = "auto.browser.global_handlers.onerror";

async function nextTask() {
  await new Promise<void>((resolve) => {
    setTimeout(resolve, 0);
  });
}

function hasTypeError(
  win: Window | null,
): win is Window & { TypeError: TypeErrorConstructor } {
  return win !== null && "TypeError" in win;
}

function spansFor(exporter: InMemorySpanExporter, mechanism: string) {
  return exporter
    .getFinishedSpans()
    .filter(
      (span) =>
        span.events[0]?.attributes?.["interfere.exception.mechanism"] ===
        mechanism,
    );
}

function chainOf(span: ReadableSpan | undefined) {
  return String(span?.events[0]?.attributes?.["interfere.exception.chain"]);
}

async function throwInTask(raise: () => never) {
  setTimeout(raise, 0);

  await nextTask();

  await nextTask();
}

test("records a thrown string from a timer as a synthetic Error", async () => {
  const { exporter } = setupErrorsPlugin();

  await throwInTask(() => {
    // oxlint-disable-next-line typescript/only-throw-error, no-throw-literal -- reproduces a host throwing a non-Error value
    throw "timer string";
  });

  const spans = spansFor(exporter, ONERROR);

  expect(spans).toHaveLength(1);

  expect(JSON.parse(chainOf(spans[0]))).toStrictEqual([
    {
      kind: "error",
      type: "Error",
      value: "timer string",
      stack: expect.any(String),
      mechanism: { handled: false, synthetic: true, type: ONERROR },
    },
  ]);
});

test("records a thrown plain object from a timer as a non-error exception", async () => {
  const { exporter } = setupErrorsPlugin();

  await throwInTask(() => {
    // oxlint-disable-next-line typescript/only-throw-error, no-throw-literal -- reproduces a host throwing a non-Error value
    throw { code: "E_TIMER" };
  });

  const spans = spansFor(exporter, ONERROR);

  expect(spans).toHaveLength(1);

  expect(spans[0]?.events[0]?.attributes).toMatchObject({
    "exception.type": "PlainObject",
    "interfere.exception.kind": "non-error",
    "interfere.exception.serialized": '{"code":"E_TIMER"}',
    "interfere.exception.handled": false,
  });
});

test("records an Error constructed in a same-origin iframe realm", async () => {
  const { exporter } = setupErrorsPlugin();

  const frame = document.createElement("iframe");

  document.body.append(frame);

  onTestFinished(() => {
    frame.remove();
  });

  const frameWindow = frame.contentWindow;

  if (!hasTypeError(frameWindow)) {
    throw new Error("iframe realm is unavailable");
  }

  await throwInTask(() => {
    throw new frameWindow.TypeError("from another realm");
  });

  const spans = spansFor(exporter, ONERROR);

  expect(spans).toHaveLength(1);

  expect(JSON.parse(chainOf(spans[0]))).toStrictEqual([
    {
      kind: "error",
      type: "TypeError",
      value: "from another realm",
      stack: expect.not.stringContaining("recoverError"),
      mechanism: { handled: false, type: ONERROR },
    },
  ]);
});

test("keeps capturing uncaught errors after the host replaces window.onerror", async () => {
  const { exporter } = setupErrorsPlugin();
  const original = globalThis.onerror;
  const hostCalls: unknown[] = [];

  // oxlint-disable-next-line unicorn/prefer-add-event-listener -- reproduces a host that overwrites the handler without chaining
  globalThis.onerror = (_msg, _source, _line, _col, error) => {
    hostCalls.push(error);

    return false;
  };

  onTestFinished(() => {
    // oxlint-disable-next-line unicorn/prefer-add-event-listener -- restores the handler replaced above
    globalThis.onerror = original;
  });

  await throwInTask(() => {
    throw new RangeError("after overwrite");
  });

  await throwInTask(() => {
    // oxlint-disable-next-line typescript/only-throw-error, no-throw-literal -- reproduces a host throwing a non-Error value
    throw "string after overwrite";
  });

  const spans = spansFor(exporter, ONERROR);

  expect(
    spans.map((span) => span.events[0]?.attributes?.["exception.message"]),
  ).toStrictEqual(["after overwrite", "string after overwrite"]);

  expect(hostCalls).toHaveLength(2);
});

test("records each real uncaught throw once while both capture paths see it", async () => {
  const { exporter } = setupErrorsPlugin();

  await throwInTask(() => {
    // oxlint-disable-next-line typescript/only-throw-error, no-throw-literal -- reproduces a host throwing a non-Error value
    throw "seen twice";
  });

  await throwInTask(() => {
    // oxlint-disable-next-line typescript/only-throw-error, no-throw-literal -- reproduces a host throwing a non-Error value
    throw "seen twice";
  });

  expect(spansFor(exporter, ONERROR)).toHaveLength(2);
});

test("still runs the host onerror when capture throws", () => {
  const original = globalThis.onerror;
  const hostCalls: unknown[] = [];

  // oxlint-disable-next-line unicorn/prefer-add-event-listener -- installs a preexisting host handler before the SDK chains it
  globalThis.onerror = (_msg, _source, _line, _col, error) => {
    hostCalls.push(error);

    return true;
  };

  onTestFinished(() => {
    // oxlint-disable-next-line unicorn/prefer-add-event-listener -- restores the handler replaced above
    globalThis.onerror = original;
  });

  const { kernel } = setupErrorsPlugin();

  kernel.recordException = () => {
    throw new Error("capture failed");
  };

  const error = new Error("host must see this");

  const result = globalThis.onerror("msg", "source", 1, 1, error);

  expect(hostCalls).toStrictEqual([error]);
  expect(result).toBe(true);
});

test("drops the opaque cross-origin Script error message", () => {
  const { exporter } = setupErrorsPlugin();

  globalThis.onerror?.("Script error.", "", 0, 0);

  expect(spansFor(exporter, ONERROR)).toStrictEqual([]);
});
