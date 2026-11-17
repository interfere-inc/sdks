import { context, trace } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";

import { onRequestError } from "./capture.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

let exporter: InMemorySpanExporter;

let provider: BasicTracerProvider;

const ROUTE = {
  routePath: "/api/x",
  routeType: "route",
  routerKind: "App Router",
} as const;

function exceptionEvents() {
  return exporter
    .getFinishedSpans()
    .flatMap((s) =>
      s.events.flatMap((e) =>
        e.name === "exception" ? [e.attributes ?? {}] : [],
      ),
    );
}

describe("onRequestError request context", () => {
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

  test("records Next's route context as attributes", async () => {
    await onRequestError(
      new Error("isr boom"),
      { method: "GET", path: "/blog/hello" },
      {
        renderSource: "react-server-components",
        revalidateReason: "stale",
        routePath: "/blog/[slug]",
        routeType: "render",
        routerKind: "App Router",
      },
    );

    const [attrs] = exceptionEvents();

    expect(attrs).toMatchObject({
      "interfere.nextjs.render_source": "react-server-components",
      "interfere.nextjs.revalidate_reason": "stale",
      "interfere.nextjs.route_type": "render",
      "interfere.nextjs.router_kind": "App Router",
    });
  });

  test("omits route context attributes Next leaves undefined", async () => {
    await onRequestError(new Error("boom"), undefined, {
      revalidateReason: undefined,
      routePath: "/dash",
      routeType: "render",
      routerKind: "App Router",
    });

    const [attrs] = exceptionEvents();

    expect(attrs).not.toHaveProperty("interfere.nextjs.revalidate_reason");
    expect(attrs).not.toHaveProperty("interfere.nextjs.render_source");
  });

  test("tags an unwrapped server action failure with the server action mechanism", async () => {
    await onRequestError(
      new Error("action boom"),
      { method: "POST", path: "/checkout" },
      { routePath: "/checkout", routeType: "action", routerKind: "App Router" },
    );

    const [attrs] = exceptionEvents();

    expect(attrs?.["interfere.exception.mechanism"]).toBe(
      "auto.function.nextjs.server_action",
    );
  });

  test.each(["proxy", "middleware"] as const)(
    "tags a %s failure reported by Next's template with the proxy mechanism",
    async (routeType) => {
      await onRequestError(
        new Error("proxy boom"),
        { method: "GET", path: "/dash" },
        { routePath: "/proxy", routeType, routerKind: "Pages Router" },
      );

      const [attrs] = exceptionEvents();

      expect(attrs?.["interfere.exception.mechanism"]).toBe(
        "auto.function.nextjs.proxy",
      );
    },
  );

  test("captures the client IP from x-forwarded-for by default", async () => {
    await onRequestError(
      new Error("boom"),
      {
        headers: {
          "x-forwarded-for": "203.0.113.7, 10.0.0.1",
          "x-forwarded-host": "app.example.com",
        },
        method: "GET",
        path: "/api/x",
      },
      ROUTE,
    );

    const [attrs] = exceptionEvents();

    expect(attrs?.["http.request.header.x-forwarded-for"]).toBe(
      "203.0.113.7, 10.0.0.1",
    );

    expect(attrs?.["http.request.header.x-forwarded-host"]).toBe(
      "app.example.com",
    );
  });

  test("redacts sensitive query values in the captured referer", async () => {
    await onRequestError(
      new Error("boom"),
      {
        headers: {
          referer: "https://app.example.com/reset?token=abc123&step=2",
        },
        method: "GET",
        path: "/api/x",
      },
      ROUTE,
    );

    const [attrs] = exceptionEvents();

    expect(attrs?.["http.request.header.referer"]).toBe(
      "https://app.example.com/reset?token=REDACTED&step=2",
    );
  });
});
