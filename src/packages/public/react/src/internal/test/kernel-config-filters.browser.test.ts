import { HttpResponse, http } from "msw";
import { worker } from "../../../test/msw.js";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { INGEST_URL_GLOBAL } from "@interfere/types/sdk/injected-globals";
import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";
import type { NonErrorException } from "@interfere/types/sdk/errors";
import { context, propagation, trace } from "@opentelemetry/api";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace";
import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { createKernel } from "../kernel.js";
import type { Kernel, KernelOptions } from "../kernel.js";
import { afterEach, expect, test } from "vitest";

let ownedKernel: Kernel | undefined;

afterEach(async () => {
  try {
    await ownedKernel?.dispose();
  } finally {
    ownedKernel = undefined;

    trace.disable();

    context.disable();

    propagation.disable();
  }
});

const MANUAL = { handled: true, type: MECHANISM_TYPE.manual.capture };

const THIRD_PARTY_STACK = [
  "Error: widget failed",
  "    at render (https://cdn.thirdparty.invalid/widget.js:10:5)",
  "    at https://app.invalid/main.js:1:1",
].join("\n");

async function startKernel(filters: KernelOptions) {
  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  setInjectedConfiguration(INGEST_URL_GLOBAL, undefined);

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "test-key");

  worker.use(
    http.get("https://kernel.invalid/v3/config", () =>
      HttpResponse.json({
        ingestUrl: "https://ingest.kernel.invalid",
        plugins: { ...DISABLE_ALL_PLUGINS, errors: true },
      }),
    ),
    http.all(
      /^https:\/\/(?<ingest>ingest\.)?kernel\.invalid\//u,
      () => new HttpResponse(null, { status: 204 }),
    ),
  );

  const exporter = new InMemorySpanExporter();

  const kernel = await createKernel({
    opts: {
      ...filters,
      _internalAdditionalSpanProcessors: [
        new SimpleSpanProcessor({ exporter }),
      ],
      apiHost: "https://kernel.invalid",
    },
  });

  ownedKernel = kernel;

  return {
    exportedMessages: async () => {
      await kernel.flush();

      return exporter
        .getFinishedSpans()
        .flatMap((span) => span.events)
        .filter((event) => event.name === "exception")
        .map((event) => event.attributes?.["exception.message"]);
    },
    kernel,
  };
}

test("ignoreErrors drops exceptions whose message or type-prefixed message matches a substring or pattern, from every capture source", async () => {
  const { exportedMessages, kernel } = await startKernel({
    ignoreErrors: ["ResizeObserver loop", /^boom \d+$/u, /^TypeError: /u],
  });

  kernel.recordException(new Error("ResizeObserver loop limit exceeded"), {
    mechanism: MANUAL,
  });

  kernel.recordException(new Error("boom 42"), { mechanism: MANUAL });

  kernel.recordException(new TypeError("x is undefined"), {
    mechanism: MANUAL,
  });

  globalThis.onerror?.(
    "ResizeObserver loop completed with undelivered notifications",
    "https://app.invalid/main.js",
    1,
    1,
    new Error("ResizeObserver loop completed with undelivered notifications"),
  );

  const nonError: NonErrorException = {
    kind: "non-error",
    serialized: 42,
    type: "number",
    value: "boom 7",
  };

  kernel.recordException(nonError, { mechanism: MANUAL });

  kernel.recordException(new Error("keep me"), { mechanism: MANUAL });

  await expect(exportedMessages()).resolves.toStrictEqual(["keep me"]);
});

test("denyUrls drops exceptions whose top stack frame comes from a denied script, falling back to the window.onerror location", async () => {
  const { exportedMessages, kernel } = await startKernel({
    denyUrls: ["cdn.thirdparty.invalid", /kernel-config-filters/u],
  });

  const ownFrame = new Error("thrown in this test file");

  kernel.recordException(ownFrame, { mechanism: MANUAL });

  const thirdParty = new Error("widget failed");

  thirdParty.stack = THIRD_PARTY_STACK;

  kernel.recordException(thirdParty, { mechanism: MANUAL });

  const stackless = new Error("stackless third-party error");

  stackless.stack = "";

  globalThis.onerror?.(
    "stackless third-party error",
    "https://cdn.thirdparty.invalid/widget.js",
    3,
    7,
    stackless,
  );

  const appError = new Error("app error");

  appError.stack = [
    "Error: app error",
    "    at main (https://app.invalid/main.js:1:1)",
    "    at https://cdn.thirdparty.invalid/widget.js:10:5",
  ].join("\n");

  kernel.recordException(appError, { mechanism: MANUAL });

  await expect(exportedMessages()).resolves.toStrictEqual(["app error"]);
});

test.each([
  { rate: 0, min: 0, max: 0 },
  { rate: 1, min: 400, max: 400 },
  { rate: 0.5, min: 140, max: 260 },
])(
  "errors.sampleRate $rate exports between $min and $max of 400 exceptions",
  async ({ rate, min, max }) => {
    const { exportedMessages, kernel } = await startKernel({
      errors: { sampleRate: rate },
    });

    for (let index = 0; index < 400; index += 1) {
      kernel.recordException(new Error(`sampled ${index}`), {
        mechanism: MANUAL,
      });
    }

    const exported = await exportedMessages();

    expect(exported.length).toBeGreaterThanOrEqual(min);
    expect(exported.length).toBeLessThanOrEqual(max);
  },
);

test("beforeSend can rewrite, replace or drop an exception, and a throwing hook still delivers the original", async () => {
  const hints: unknown[] = [];

  const { exportedMessages, kernel } = await startKernel({
    beforeSend: (exception, hint) => {
      hints.push(hint);

      if (isNonError(exception)) {
        return exception;
      }

      if (exception.message === "drop me") {
        return null;
      }

      if (exception.message === "hook throws") {
        throw new Error("customer hook bug");
      }

      if (exception.message === "replace me") {
        return new Error("replaced");
      }

      exception.message = exception.message.replace(
        "secret-token",
        "[redacted]",
      );

      return exception;
    },
  });

  const original = new Error("auth failed for secret-token");

  kernel.recordException(original, { mechanism: MANUAL });

  kernel.recordException(new Error("drop me"), { mechanism: MANUAL });

  kernel.recordException(new Error("hook throws"), { mechanism: MANUAL });

  kernel.recordException(new Error("replace me"), { mechanism: MANUAL });

  await expect(exportedMessages()).resolves.toStrictEqual([
    "auth failed for [redacted]",
    "hook throws",
    "replaced",
  ]);

  expect(hints[0]).toStrictEqual({
    mechanism: MANUAL,
    originalException: original,
  });
});

test.each([
  { rate: 1, hooked: ["kept"], exported: ["kept"] },
  { rate: 0, hooked: [], exported: [] },
])(
  "beforeSend only sees exceptions that pass ignoreErrors and sampleRate $rate",
  async ({ rate, hooked, exported }) => {
    const seenByHook: string[] = [];

    const { exportedMessages, kernel } = await startKernel({
      beforeSend: (exception) => {
        seenByHook.push(
          isNonError(exception) ? exception.value : exception.message,
        );

        return exception;
      },
      errors: { sampleRate: rate },
      ignoreErrors: ["ignored"],
    });

    kernel.recordException(new Error("ignored"), { mechanism: MANUAL });

    kernel.recordException(new Error("kept"), { mechanism: MANUAL });

    await expect(exportedMessages()).resolves.toStrictEqual(exported);

    expect(seenByHook).toStrictEqual(hooked);
  },
);

function isNonError(
  value: Error | NonErrorException,
): value is NonErrorException {
  return !(value instanceof Error);
}
