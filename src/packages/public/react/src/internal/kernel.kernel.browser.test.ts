import {
  SpanStatusCode,
  context,
  propagation,
  trace,
} from "@opentelemetry/api";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  StackContextManager,
  TracerProvider,
} from "@opentelemetry/sdk-trace";

import { buildKernel, ingestHandlers } from "../../test/fixtures/kernel.js";
import type { createKernel } from "./kernel.js";
import { HttpResponse, http } from "msw";
import {
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  onTestFinished,
  test,
  vi,
} from "vitest";

import { addBaselineHandlers } from "../../test/msw.js";

beforeAll(() => {
  addBaselineHandlers(
    http.get("https://api.interfere.com/v3/config", () =>
      HttpResponse.json({ plugins: {} }),
    ),
    ...ingestHandlers("https://api.interfere.com"),
    ...ingestHandlers("https://in.example.com"),
  );
});

const ownedKernels = new Set<Awaited<ReturnType<typeof createKernel>>>();

const providers = new Set<TracerProvider>();

let priorUrl: string;

let priorHistory: unknown;

let priorCookie: Awaited<ReturnType<CookieStore["get"]>>;

function withInMemoryTracer<T>(fn: (exporter: InMemorySpanExporter) => T) {
  const exporter = new InMemorySpanExporter();

  const provider = new TracerProvider({
    spanProcessors: [new SimpleSpanProcessor({ exporter })],
  });

  providers.add(provider);

  trace.disable();

  context.disable();

  trace.setGlobalTracerProvider(provider);

  context.setGlobalContextManager(new StackContextManager().enable());

  try {
    return fn(exporter);
  } finally {
    context.disable();

    trace.disable();
  }
}

beforeEach(async () => {
  priorUrl = globalThis.location.href;

  priorHistory = globalThis.history.state;

  priorCookie = await cookieStore.get("interfere_did");
});

afterEach(async () => {
  try {
    const results = await Promise.allSettled([
      ...Array.from(ownedKernels, async (kernel) => kernel.dispose()),
      ...Array.from(providers, async (provider) => provider.shutdown()),
    ]);

    const failures = results.filter((result) => result.status === "rejected");

    if (failures.length > 0) {
      throw new AggregateError(
        failures.map((result) => {
          const decodedValue: unknown = result.reason;

          return decodedValue;
        }),
        "Kernel cleanup failed",
      );
    }
  } finally {
    ownedKernels.clear();

    providers.clear();

    context.disable();

    trace.disable();

    propagation.disable();

    globalThis.history.replaceState(priorHistory, "", priorUrl);

    await cookieStore.delete("interfere_did");

    if (priorCookie?.name && priorCookie.value !== undefined) {
      await cookieStore.set({
        ...priorCookie,
        name: priorCookie.name,
        value: priorCookie.value,
      });
    }

    vi.unstubAllGlobals();
  }
});

function recordInSpan(
  exporter: InMemorySpanExporter,
  errors: Error[],
  handled = true,
) {
  const { kernel, tracker } = buildKernel();

  ownedKernels.add(kernel);

  tracker.start();

  const span = trace.getTracer("test").startSpan("op");

  context.with(trace.setSpan(context.active(), span), () => {
    for (const error of errors) {
      kernel.recordException(error, {
        mechanism: {
          handled,
          type: handled
            ? "manual.api.capture"
            : "auto.browser.global_handlers.onerror",
        },
      });
    }
  });

  span.end();

  const [finished] = exporter.getFinishedSpans();

  return {
    events: finished?.events.filter((e) => e.name === "exception"),
    status: finished?.status,
  };
}

function setDebugIdRegistry(
  value: Record<string, string> | undefined | null | boolean | number | string,
) {
  const original = Object.getOwnPropertyDescriptor(globalThis, "_debugIds");

  Object.defineProperty(globalThis, "_debugIds", {
    configurable: true,
    writable: true,
    value,
  });

  onTestFinished(() => {
    if (original) {
      Object.defineProperty(globalThis, "_debugIds", original);
    } else {
      Reflect.deleteProperty(globalThis, "_debugIds");
    }
  });
}

describe("Kernel", () => {
  describe("recordException", () => {
    test("emits an exception event on the active span with mechanism + parsed exceptions", () => {
      expect.assertions(1);

      withInMemoryTracer((exporter) => {
        const exceptionEvent = recordInSpan(exporter, [
          new TypeError("undefined is not a function"),
        ]).events?.[0];

        expect(exceptionEvent?.attributes).toMatchObject({
          "exception.message": "undefined is not a function",
          "exception.type": "TypeError",
          "interfere.exception.handled": true,
          "interfere.exception.mechanism": "manual.api.capture",
        });
      });
    });

    test("forwards the raw _debugIds registry on exception event attributes", () => {
      expect.assertions(1);

      withInMemoryTracer((exporter) => {
        setDebugIdRegistry({
          "Error\n    at fn (https://app.com/chunks/app.js:1:1)":
            "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        });

        const exceptionEvent = recordInSpan(exporter, [new Error("boom")])
          .events?.[0];

        expect(
          JSON.parse(
            String(
              exceptionEvent?.attributes?.["interfere.exception.debug_ids"],
            ),
          ),
        ).toStrictEqual({
          "Error\n    at fn (https://app.com/chunks/app.js:1:1)":
            "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        });
      });
    });

    test.each([{}, undefined, null, false, 42, "not-a-registry"])(
      "does not emit debug ID attributes for empty or non-object registry %j",
      (registry) => {
        expect.assertions(1);

        withInMemoryTracer((exporter) => {
          setDebugIdRegistry(registry);

          const exceptionEvent = recordInSpan(exporter, [new Error("boom")])
            .events?.[0];

          expect(
            exceptionEvent?.attributes?.["interfere.exception.debug_ids"],
          ).toBeUndefined();
        });
      },
    );

    test("walks Error.cause into the chain attribute", () => {
      expect.assertions(2);

      withInMemoryTracer((exporter) => {
        const root = new Error("root");

        root.name = "RootError";

        const mid = new Error("mid", { cause: root });

        mid.name = "MidError";

        const tip = new Error("tip", { cause: mid });

        tip.name = "TipError";

        const exceptionEvent = recordInSpan(exporter, [tip]).events?.[0];

        const chainAttr =
          exceptionEvent?.attributes?.["interfere.exception.chain"];

        expect(chainAttr).toBeTypeOf("string");

        const chain: unknown = JSON.parse(String(chainAttr));

        expect(chain).toStrictEqual([
          expect.objectContaining({ type: "TipError" }),
          expect.objectContaining({ type: "MidError" }),
          expect.objectContaining({ type: "RootError" }),
        ]);
      });
    });

    test("drops browser-extension noise without emitting", () => {
      expect.assertions(1);

      withInMemoryTracer((exporter) => {
        const err = new Error("Extension blew up");

        err.stack = [
          "Error: Extension blew up",
          "    at handler (chrome-extension://abc/content.js:10:5)",
        ].join("\n");

        expect(recordInSpan(exporter, [err]).events).toHaveLength(0);
      });
    });

    test("marks the active span ERROR only for unhandled captures", () => {
      expect.assertions(2);

      withInMemoryTracer((exporter) => {
        expect(
          recordInSpan(exporter, [new Error("caught")], true).status,
        ).toMatchObject({ code: SpanStatusCode.UNSET });
      });

      withInMemoryTracer((exporter) => {
        expect(
          recordInSpan(exporter, [new Error("uncaught")], false).status,
        ).toMatchObject({ code: SpanStatusCode.ERROR, message: "uncaught" });
      });
    });

    test("dedupes repeated calls for the same Error instance", () => {
      expect.assertions(1);

      withInMemoryTracer((exporter) => {
        const err = new Error("boom");

        expect(recordInSpan(exporter, [err, err]).events).toHaveLength(1);
      });
    });
  });
});
