/* oxlint-disable typescript/only-throw-error, typescript/require-await */
import { SpanStatusCode, context, trace } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";

import { notFound, redirect } from "next/navigation";

import { onRequestError } from "../capture/capture.js";
import type { OnRequestErrorContext } from "../types.js";
import { withInterfereServerAction } from "./wrap-server-action.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

let exporter: InMemorySpanExporter;

let provider: BasicTracerProvider;

const REQUEST_CONTEXT = {
  routePath: "/test",
  routeType: "action",
  routerKind: "App Router",
} satisfies OnRequestErrorContext;

function exceptionEvents() {
  return exporter
    .getFinishedSpans()
    .flatMap((s) =>
      s.events.flatMap((e) =>
        e.name === "exception" ? [e.attributes ?? {}] : [],
      ),
    );
}

function actionSpan(name: string) {
  return exporter
    .getFinishedSpans()
    .find((s) => s.name === `serverAction ${name}`);
}

describe(withInterfereServerAction, () => {
  beforeEach(() => {
    vi.unstubAllEnvs();

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_FORCE_ENABLE", "1");

    vi.stubEnv("INTERFERE_API_KEY", "interfere_pk_test_123");

    exporter = new InMemorySpanExporter();

    provider = new BasicTracerProvider({
      spanProcessors: [new SimpleSpanProcessor(exporter)],
    });

    trace.disable();

    context.disable();

    trace.setGlobalTracerProvider(provider);

    context.setGlobalContextManager(
      new AsyncLocalStorageContextManager().enable(),
    );
  });

  afterEach(async () => {
    context.disable();

    trace.disable();

    await provider.shutdown();

    vi.restoreAllMocks();
  });

  test("emits no event of its own; the rethrown error is annotated and the action span records the error outcome", async () => {
    let thrown: unknown;

    try {
      await withInterfereServerAction(
        "createUser",
        () => {
          throw new Error("Server action error!");
        },
        { args: [new Map(), "id-1"] },
      );
    } catch (error) {
      thrown = error;
    }

    expect(thrown).toBeInstanceOf(Error);
    expect(exceptionEvents()).toHaveLength(0);

    expect(
      actionSpan("createUser")?.attributes["interfere.server_action.outcome"],
    ).toBe("error");
  });

  test("onRequestError tags the annotated error server_action with identity + digest — a single event", async () => {
    let thrown: Error & { digest?: string } = new Error("unset");

    try {
      await withInterfereServerAction(
        "createUser",
        () => {
          throw new Error("Server action error!");
        },
        { args: [new Map(), "id-1"] },
      );
    } catch (error) {
      if (!(error instanceof Error)) {
        throw error;
      }

      thrown = error;
    }

    thrown.digest = "digest-abc";

    await onRequestError(thrown, undefined, REQUEST_CONTEXT);

    const events = exceptionEvents();

    expect(events).toHaveLength(1);

    expect(events[0]).toMatchObject({
      "exception.message": "Server action error!",
      "interfere.error.digest": "digest-abc",
      "interfere.exception.handled": false,
      "interfere.exception.mechanism": "auto.function.nextjs.server_action",
      "interfere.server_action.args_shape": "Map,string",
      "interfere.server_action.name": "createUser",
    });
  });

  test.each([
    ["redirect", () => redirect("/done")],
    ["not_found", () => notFound()],
    [
      "forbidden",
      () => {
        throw Object.assign(new Error("NEXT_HTTP_ERROR_FALLBACK;403"), {
          digest: "NEXT_HTTP_ERROR_FALLBACK;403",
        });
      },
    ],
    [
      "unauthorized",
      () => {
        throw Object.assign(new Error("NEXT_HTTP_ERROR_FALLBACK;401"), {
          digest: "NEXT_HTTP_ERROR_FALLBACK;401",
        });
      },
    ],
  ] as const)(
    "records %s control flow as a non-error outcome and rethrows it unannotated",
    async (outcome, body) => {
      let thrown: unknown;

      try {
        await withInterfereServerAction("navigate", body);
      } catch (error) {
        thrown = error;
      }

      expect(thrown).toBeInstanceOf(Error);

      const span = actionSpan("navigate");

      expect(span?.attributes["interfere.server_action.outcome"]).toBe(outcome);

      expect(span?.status.code).toBe(SpanStatusCode.UNSET);

      await onRequestError(thrown, undefined, REQUEST_CONTEXT);

      expect(exceptionEvents()).toHaveLength(0);
    },
  );

  test("an unannotated error reported for an action route is tagged server_action", async () => {
    await onRequestError(
      new Error("plain server error"),
      undefined,
      REQUEST_CONTEXT,
    );

    const events = exceptionEvents();

    expect(events).toHaveLength(1);

    expect(events[0]?.["interfere.exception.mechanism"]).toBe(
      "auto.function.nextjs.server_action",
    );

    expect(events[0]?.["interfere.server_action.name"]).toBeUndefined();
  });

  test("returns the result and records a success outcome", async () => {
    const result = await withInterfereServerAction("loadData", () => 42);

    expect(result).toBe(42);
    expect(exceptionEvents()).toHaveLength(0);

    expect(
      actionSpan("loadData")?.attributes["interfere.server_action.outcome"],
    ).toBe("success");
  });

  test("records argument structure without argument values", async () => {
    const secret = "customer-secret";

    await withInterfereServerAction("redacted", () => "ok", {
      args: [
        null,
        undefined,
        secret,
        42,
        42n,
        true,
        Symbol(secret),
        () => secret,
        [secret],
        { secret },
        new Map([[secret, secret]]),
      ],
    });

    const span = actionSpan("redacted");

    expect(span?.attributes["interfere.server_action.args_shape"]).toBe(
      "null,undefined,string,number,bigint,boolean,symbol,function,Array,Object,Map",
    );

    expect(JSON.stringify(span?.attributes)).not.toContain(secret);
  });

  test.each([null, undefined, "failure", 42, false])(
    "preserves rejected primitive %j without adding an exception event",
    async (value) => {
      await expect(
        withInterfereServerAction("primitive", async () => {
          throw value;
        }),
      ).rejects.toBe(value);

      expect(exceptionEvents()).toHaveLength(0);

      expect(
        actionSpan("primitive")?.attributes["interfere.server_action.outcome"],
      ).toBe("error");
    },
  );

  test("rethrows a frozen customer error unchanged", async () => {
    const frozen = Object.freeze(new Error("frozen failure"));

    await expect(
      withInterfereServerAction("frozen", () => {
        throw frozen;
      }),
    ).rejects.toBe(frozen);
  });

  test("runs the action when its arguments cannot be inspected", async () => {
    const { proxy, revoke } = Proxy.revocable({}, {});
    let runs = 0;

    revoke();

    await expect(
      withInterfereServerAction(
        "revoked",
        () => {
          runs += 1;

          return "ok";
        },
        { args: [proxy] },
      ),
    ).resolves.toBe("ok");

    expect(runs).toBe(1);
  });

  test("runs the action once when the tracer fails", async () => {
    let runs = 0;

    trace.disable();

    trace.setGlobalTracerProvider({
      getTracer() {
        throw new Error("injected tracer failure");
      },
    });

    await expect(
      withInterfereServerAction("untraced", () => {
        runs += 1;

        return "ok";
      }),
    ).resolves.toBe("ok");

    expect(runs).toBe(1);
  });
});
