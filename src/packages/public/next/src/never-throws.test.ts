/* oxlint-disable typescript/require-await */
import { context, trace } from "@opentelemetry/api";
import type { Context, ContextManager } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { NextRequest } from "next/server";
import { NextFetchEvent } from "next/dist/server/web/spec-extension/fetch-event";

import { nextSpanExceptionProcessor } from "./internal/server/capture/next-span-exceptions.js";
import {
  installGlobalHandlers,
  uninstallGlobalHandlers,
} from "./internal/server/capture/global-handlers.js";
import {
  captureError,
  onRequestError,
  withInterfereAfter,
  withInterfereProxy,
  withInterfereServerAction,
} from "./server.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

const NEXT_REQUEST_CONTEXT = Symbol.for("@next/request-context");

function revoked() {
  const { proxy, revoke } = Proxy.revocable(new Error("revoked"), {});

  revoke();

  return proxy;
}

function trapped() {
  const error = new Error("trapped");

  for (const key of ["name", "message", "stack", "cause", "digest", "code"]) {
    Object.defineProperty(error, key, {
      get() {
        throw new Error(`injected ${key} getter failure`);
      },
    });
  }

  return error;
}

const HOSTILE = [
  ["a revoked proxy", revoked],
  ["an error whose getters throw", trapped],
] as const;

const BROKEN_TRACER_PROVIDER = {
  getTracer() {
    throw new Error("injected tracer failure");
  },
};

function hostileRequest() {
  const request = new NextRequest("https://app.example.com/broken");

  for (const key of ["nextUrl", "method"]) {
    Object.defineProperty(request, key, {
      get() {
        throw new Error(`injected request.${key} failure`);
      },
    });
  }

  return request;
}

const BROKEN_CONTEXT_MANAGER = {
  active() {
    throw new Error("injected context failure");
  },
  bind<T>(_: Context, target: T) {
    return target;
  },
  disable() {
    return this;
  },
  enable() {
    return this;
  },
  with<A extends unknown[], F extends (...args: A) => ReturnType<F>>(
    _: Context,
    fn: F,
    thisArg?: ThisParameterType<F>,
    ...args: A
  ) {
    return fn.call(thisArg, ...args);
  },
} satisfies ContextManager;

function failureOf(run: () => void) {
  try {
    run();
  } catch (error) {
    return error instanceof Error ? error.message : "non-error";
  }

  return null;
}

const event = new NextFetchEvent({
  request: new NextRequest("https://app.example.com/"),
  page: "/",
  context: undefined,
});

let provider: BasicTracerProvider;

beforeEach(() => {
  vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

  vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_test_123");

  provider = new BasicTracerProvider({
    spanProcessors: [
      nextSpanExceptionProcessor,
      new SimpleSpanProcessor(new InMemorySpanExporter()),
    ],
  });

  trace.disable();

  context.disable();

  trace.setGlobalTracerProvider(provider);

  context.setGlobalContextManager(
    new AsyncLocalStorageContextManager().enable(),
  );

  vi.spyOn(console, "warn");
});

afterEach(async () => {
  uninstallGlobalHandlers();

  context.disable();

  trace.disable();

  await provider.shutdown();

  Reflect.deleteProperty(globalThis, NEXT_REQUEST_CONTEXT);

  vi.unstubAllEnvs();

  vi.restoreAllMocks();
});

describe.each(HOSTILE)("given %s", (_, hostile) => {
  test("captureError returns", () => {
    expect(() => {
      captureError(hostile());
    }).not.toThrow();
  });

  test("onRequestError resolves", async () => {
    await expect(
      onRequestError(hostile(), undefined, {
        routerKind: "App Router",
        routePath: "/broken",
        routeType: "route",
        renderSource: undefined,
        revalidateReason: undefined,
        renderType: undefined,
      }),
    ).resolves.toBeUndefined();
  });

  test("next's span adds no failure of its own when recording it", () => {
    const bare = new BasicTracerProvider();

    const expected = failureOf(() => {
      bare.getTracer("next.js").startSpan("bare").recordException(hostile());
    });

    const actual = failureOf(() => {
      trace
        .getTracer("next.js")
        .startSpan("executing api route")
        .recordException(hostile());
    });

    expect(actual).toBe(expected);
  });

  test("withInterfereProxy rethrows it unchanged", async () => {
    const thrown = hostile();

    const sync = withInterfereProxy(() => {
      throw thrown;
    });

    let caught: unknown;

    try {
      void sync(new NextRequest("https://app.example.com/"), event);
    } catch (error) {
      caught = error;
    }

    expect(Object.is(caught, thrown)).toBe(true);

    const rejecting = withInterfereProxy(async () => {
      throw thrown;
    });

    await expect(
      rejecting(new NextRequest("https://app.example.com/"), event),
    ).rejects.toBe(thrown);
  });

  test("withInterfereServerAction rethrows it unchanged", async () => {
    const thrown = hostile();

    await expect(
      withInterfereServerAction(
        "broken",
        async () => {
          throw thrown;
        },
        { args: [hostile()] },
      ),
    ).rejects.toBe(thrown);
  });

  test("withInterfereAfter rethrows it unchanged", async () => {
    const thrown = hostile();

    await expect(
      withInterfereAfter(async () => {
        throw thrown;
      })(),
    ).rejects.toBe(thrown);
  });

  test("the process handlers contain it", async () => {
    const flushed = Promise.withResolvers<undefined>();

    installGlobalHandlers({
      captureUncaughtException: false,
      captureUnhandledRejection: true,
      flush: async () => {
        flushed.resolve(undefined);

        throw new Error("injected flush failure");
      },
    });

    const listener = process.listeners("unhandledRejection").at(-1);

    expect(() => {
      listener?.(hostile(), Promise.resolve());
    }).not.toThrow();

    await flushed.promise;
  });
});

describe("a broken environment", () => {
  test("withInterfereProxy rethrows the customer's error when the request cannot be read", async () => {
    const failure = new Error("customer proxy failure");

    const sync = withInterfereProxy(() => {
      throw failure;
    });

    expect(() => {
      void sync(hostileRequest(), event);
    }).toThrow(failure);

    const rejecting = withInterfereProxy(async () => {
      throw failure;
    });

    await expect(rejecting(hostileRequest(), event)).rejects.toBe(failure);
  });

  test("withInterfereServerAction runs the action once when the active context cannot be read", async () => {
    let calls = 0;

    context.disable();

    context.setGlobalContextManager(BROKEN_CONTEXT_MANAGER);

    await expect(
      withInterfereServerAction("ok", () => {
        calls += 1;

        return "ok";
      }),
    ).resolves.toBe("ok");

    expect(calls).toBe(1);
  });

  test("onRequestError resolves when the platform waitUntil throws", async () => {
    Object.defineProperty(globalThis, NEXT_REQUEST_CONTEXT, {
      configurable: true,
      value: {
        get: () => ({
          waitUntil() {
            throw new Error("injected waitUntil failure");
          },
        }),
      },
    });

    await expect(
      onRequestError(new Error("customer failure"), undefined, {
        routerKind: "App Router",
        routePath: "/broken",
        routeType: "route",
        renderSource: undefined,
        revalidateReason: undefined,
        renderType: undefined,
      }),
    ).resolves.toBeUndefined();
  });

  test("captureError and onRequestError contain a foreign provider whose tracer throws", async () => {
    trace.disable();

    trace.setGlobalTracerProvider(BROKEN_TRACER_PROVIDER);

    expect(() => {
      captureError(new Error("customer failure"));
    }).not.toThrow();

    await expect(
      onRequestError(new Error("customer failure"), undefined, {
        routerKind: "App Router",
        routePath: "/broken",
        routeType: "route",
        renderSource: undefined,
        revalidateReason: undefined,
        renderType: undefined,
      }),
    ).resolves.toBeUndefined();
  });

  test("each contained failure warns once per site", () => {
    captureError(revoked());

    captureError(revoked());

    const warnings = vi
      .mocked(console.warn)
      .mock.calls.filter(([message]) =>
        String(message).startsWith("[interfere] captureError() failed"),
      );

    expect(warnings.length).toBeLessThanOrEqual(1);
  });
});
