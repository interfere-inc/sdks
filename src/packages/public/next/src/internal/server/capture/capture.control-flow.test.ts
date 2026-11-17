import { context, trace } from "@opentelemetry/api";
import { AsyncLocalStorageContextManager } from "@opentelemetry/context-async-hooks";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { DynamicServerError } from "next/dist/client/components/hooks-server-context.js";
import { StaticGenBailoutError } from "next/dist/client/components/static-generation-bailout.js";
import { BailoutToCSRError } from "next/dist/shared/lib/lazy-dynamic/bailout-to-csr.js";
import { ResponseAborted } from "next/dist/server/web/spec-extension/adapters/next-request.js";
import { notFound, redirect } from "next/navigation";

import { captureError, onRequestError } from "./capture.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

let exporter: InMemorySpanExporter;

let provider: BasicTracerProvider;

function withDigest(digest: string) {
  return Object.assign(new Error("control flow"), { digest });
}

function thrownBy(fn: () => void) {
  try {
    fn();
  } catch (error) {
    if (error instanceof Error) {
      return error;
    }
  }

  throw new Error("expected an Error throw");
}

const CONTROL_FLOW_THROWS: readonly (readonly [string, () => object])[] = [
  ["redirect()", () => thrownBy(() => redirect("/login"))],
  ["notFound()", () => thrownBy(() => notFound())],
  ["forbidden()", () => withDigest("NEXT_HTTP_ERROR_FALLBACK;403")],
  ["unauthorized()", () => withDigest("NEXT_HTTP_ERROR_FALLBACK;401")],
  ["dynamic server usage", () => new DynamicServerError("cookies")],
  [
    "bailout to client-side rendering",
    () => new BailoutToCSRError("useSearchParams()"),
  ],
  ["prerender interrupted", () => withDigest("NEXT_PRERENDER_INTERRUPTED")],
  ["hanging promise rejection", () => withDigest("HANGING_PROMISE_REJECTION")],
  ["client hook dynamic", () => withDigest("CLIENT_HOOK_DYNAMIC")],
  [
    "static generation bailout",
    () => new StaticGenBailoutError("Page couldn't be rendered statically"),
  ],
  [
    "React postpone",
    () => ({ $$typeof: Symbol.for("react.postpone"), message: "postponed" }),
  ],
  [
    "AbortError",
    () => new DOMException("The operation was aborted.", "AbortError"),
  ],
  ["ResponseAborted", () => new ResponseAborted()],
];

function exceptionEvents() {
  return exporter
    .getFinishedSpans()
    .flatMap((s) =>
      s.events.flatMap((e) =>
        e.name === "exception" ? [e.attributes ?? {}] : [],
      ),
    );
}

describe("framework control flow is never captured as an error", () => {
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

  test.each(CONTROL_FLOW_THROWS)(
    "captureError skips %s and records nothing",
    (_label, make) => {
      captureError(make());

      expect(exceptionEvents()).toHaveLength(0);
    },
  );

  test.each(CONTROL_FLOW_THROWS)(
    "onRequestError skips %s and records nothing",
    async (_label, make) => {
      await onRequestError(
        make(),
        { method: "GET", path: "/dash" },
        { routePath: "/dash", routeType: "render", routerKind: "App Router" },
      );

      expect(exceptionEvents()).toHaveLength(0);
    },
  );

  test("still captures errors that only resemble a sentinel", () => {
    captureError(Object.assign(new Error("boom"), { code: "ECONNRESET" }));

    captureError(new Error("DYNAMIC_SERVER_USAGE"));

    captureError(Object.assign(new Error("boom"), { name: "TimeoutError" }));

    captureError({ $$typeof: Symbol.for("react.element") });

    expect(exceptionEvents()).toHaveLength(4);
  });
});
