import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";
import {
  ROOT_CONTEXT,
  TraceFlags,
  context,
  createTraceState,
  propagation,
  trace,
} from "@opentelemetry/api";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { StackContextManager } from "@opentelemetry/sdk-trace-web";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, test } from "vitest";

import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { worker } from "../../../test/msw.js";
import { createKernel } from "../kernel.js";
import type { Kernel, KernelInternalOptions } from "../kernel.js";

const API = "https://kernel.invalid";

let ownedKernel: Kernel | undefined;

beforeEach(() => {
  ownedKernel = undefined;

  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

  trace.disable();

  context.disable();

  propagation.disable();

  worker.use(
    http.get(`${API}/v3/config`, () =>
      HttpResponse.json({ plugins: DISABLE_ALL_PLUGINS }),
    ),
    http.all(`${API}/*`, () => new HttpResponse(null, { status: 204 })),
  );
});

afterEach(async () => {
  try {
    await ownedKernel?.dispose();
  } finally {
    trace.disable();

    context.disable();

    propagation.disable();
  }
});

async function boot(opts: Omit<KernelInternalOptions, "apiHost"> = {}) {
  const kernel = await createKernel({ opts: { ...opts, apiHost: API } });

  ownedKernel = kernel;

  return kernel;
}

test("a context manager another SDK registered stays registered, and Interfere registers no propagator", async () => {
  expect.hasAssertions();

  const foreign = new StackContextManager().enable();

  expect(context.setGlobalContextManager(foreign)).toBe(true);

  const kernel = await boot();

  expect(propagation.fields()).toStrictEqual([]);

  await kernel.dispose();

  ownedKernel = undefined;

  expect(propagation.fields()).toStrictEqual([]);

  expect(context.setGlobalContextManager(new StackContextManager())).toBe(
    false,
  );
});

test("an exception under an unsampled host context exports as its own root linked to that context", async () => {
  expect.hasAssertions();

  const spans = new InMemorySpanExporter();

  context.setGlobalContextManager(new StackContextManager().enable());

  const kernel = await boot({
    _internalAdditionalSpanProcessors: [new SimpleSpanProcessor(spans)],
  });

  kernel.syncProviderConfig({});

  const host = {
    spanId: "00f067aa0ba902b7",
    traceFlags: TraceFlags.NONE,
    traceId: "4bf92f3577b34da6a3ce929d0e0e4736",
    traceState: createTraceState("vendor=abc"),
  };

  context.with(trace.setSpanContext(ROOT_CONTEXT, host), () => {
    kernel.recordException(new Error("unsampled host"), {
      mechanism: { handled: true, type: MECHANISM_TYPE.manual.capture },
    });
  });

  const exported = spans
    .getFinishedSpans()
    .filter((span) => span.events.some((event) => event.name === "exception"));

  expect(exported).toHaveLength(1);

  const [span] = exported;

  expect(span?.parentSpanContext).toBeUndefined();
  expect(span?.spanContext().traceId).not.toBe(host.traceId);
  expect(span?.spanContext().traceFlags).toBe(TraceFlags.SAMPLED);
  expect(span?.links.map((link) => link.context)).toStrictEqual([host]);
});
