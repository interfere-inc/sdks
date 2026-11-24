import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";
import {
  ROOT_CONTEXT,
  TraceFlags,
  context,
  propagation,
  trace,
} from "@opentelemetry/api";
import { StackContextManager } from "@opentelemetry/sdk-trace";
import { HttpResponse, http } from "msw";
import {
  afterEach,
  beforeEach,
  expect,
  onTestFinished,
  test,
  vi,
} from "vitest";
import { z } from "zod";

import { DISABLE_ALL_PLUGINS } from "../../test/fixtures/disable-all-plugins.js";
import { worker } from "../../test/msw.js";
import { createKernel } from "./kernel.js";
import type { Kernel } from "./kernel.js";
import {
  decodedBodyJson,
  decodedBodyText,
} from "../../test/fixtures/request-body.js";

const API = "https://capture-only.invalid";

const VIEW_ID = "interfere.view.id";

const keyValue = z.object({
  key: z.string(),
  value: z.record(z.string(), z.json()),
});

const exportBody = z.object({
  resourceSpans: z.tuple([
    z.object({
      resource: z.object({ attributes: z.array(keyValue) }),
      scopeSpans: z.tuple([
        z.object({
          scope: z.object({ name: z.string() }),
          spans: z.tuple([
            z.object({
              traceId: z.string().regex(/^[\da-f]{32}$/u),
              spanId: z.string().regex(/^[\da-f]{16}$/u),
              name: z.string(),
              startTimeUnixNano: z.string().regex(/^\d{19}$/u),
              attributes: z.array(keyValue),
              events: z.tuple([
                z.object({
                  name: z.literal("exception"),
                  attributes: z.array(keyValue),
                }),
              ]),
              links: z.array(
                z.object({
                  traceId: z.string(),
                  spanId: z.string(),
                  flags: z.number(),
                }),
              ),
              status: z.object({ code: z.number().optional() }),
            }),
          ]),
        }),
      ]),
    }),
  ]),
});

const batchBody = z.object({
  resourceSpans: z.tuple([
    z.object({
      scopeSpans: z.tuple([z.object({ spans: z.array(z.unknown()) })]),
    }),
  ]),
});

let ownedKernel: Kernel | undefined;

let exported: PromiseWithResolvers<Request>;

function attrs(list: z.infer<typeof keyValue>[]) {
  return Object.fromEntries(
    list.map(({ key, value }) => [key, Object.values(value)[0]]),
  );
}

beforeEach(() => {
  ownedKernel = undefined;

  exported = Promise.withResolvers<Request>();

  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

  trace.disable();

  context.disable();

  propagation.disable();

  worker.use(
    http.get(`${API}/v3/config`, () =>
      HttpResponse.json({ plugins: DISABLE_ALL_PLUGINS }),
    ),
    http.post(`${API}/drain/v1/traces`, ({ request }) => {
      exported.resolve(request.clone());

      return new HttpResponse(null, { status: 204 });
    }),
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

async function captured() {
  const request = await exported.promise;

  return {
    body: exportBody.parse(await decodedBodyJson(request)),
    producer: request.headers.get("x-interfere-producer-version"),
  };
}

test("tracing: false exports an unhandled exception as one hand-encoded OTLP span", async () => {
  expect.hasAssertions();

  const kernel = await createKernel({ opts: { apiHost: API, tracing: false } });

  ownedKernel = kernel;

  expect(kernel.getTracer("off").startSpan("probe").isRecording()).toBe(false);

  kernel.recordException(new TypeError("capture-only boom"), {
    mechanism: { handled: false, type: MECHANISM_TYPE.manual.capture },
  });

  const { body, producer } = await captured();

  const [resourceSpan] = body.resourceSpans;
  const [span] = resourceSpan.scopeSpans[0].spans;

  expect(attrs(resourceSpan.resource.attributes)).toMatchObject({
    "telemetry.sdk.language": "webjs",
    "interfere.sdk.name": "@interfere/react",
  });

  expect(attrs(span.attributes)).toStrictEqual({
    "interfere.capture.kind": "exception",
    "session.id": kernel.session.getId(),
    [VIEW_ID]: kernel.views.viewId(),
  });

  expect(attrs(span.events[0].attributes)).toMatchObject({
    "exception.type": "TypeError",
    "exception.message": "capture-only boom",
    "interfere.exception.handled": false,
  });

  expect(span.status.code).toBe(2);

  expect(span.links).toStrictEqual([]);

  expect(producer).toMatch(/^@interfere\/react@/u);
});

test("tracing: false links the captured exception to the host's active span context", async () => {
  expect.hasAssertions();

  context.setGlobalContextManager(new StackContextManager().enable());

  const kernel = await createKernel({ opts: { apiHost: API, tracing: false } });

  ownedKernel = kernel;

  const host = {
    spanId: "00f067aa0ba902b7",
    traceFlags: TraceFlags.SAMPLED,
    traceId: "4bf92f3577b34da6a3ce929d0e0e4736",
  };

  context.with(trace.setSpanContext(ROOT_CONTEXT, host), () => {
    kernel.recordException(new Error("under host span"), {
      mechanism: { handled: true, type: MECHANISM_TYPE.manual.capture },
    });
  });

  const [span] = (await captured()).body.resourceSpans[0].scopeSpans[0].spans;

  expect(span.traceId).not.toBe(host.traceId);

  expect(span.links).toStrictEqual([
    { traceId: host.traceId, spanId: host.spanId, flags: TraceFlags.SAMPLED },
  ]);

  expect(span.status).toStrictEqual({});
});

test("tracing: false redacts URL secrets from the message, stack, chain and status before export", async () => {
  expect.hasAssertions();

  const kernel = await createKernel({ opts: { apiHost: API, tracing: false } });

  ownedKernel = kernel;

  kernel.recordException(
    new Error("Failed https://example.test/?token=secret"),
    { mechanism: { handled: false, type: MECHANISM_TYPE.manual.capture } },
  );

  const raw = await decodedBodyText(await exported.promise);

  expect(raw).not.toContain("secret");

  const [span] = exportBody.parse(JSON.parse(raw)).resourceSpans[0]
    .scopeSpans[0].spans;

  expect(attrs(span.events[0].attributes)).toMatchObject({
    "exception.message": "Failed https://example.test/?token=REDACTED",
  });

  expect(raw).toContain("token=REDACTED");
});

test("tracing: false keeps one export in flight, caps the queue behind it and drains on flush and shutdown", async () => {
  expect.hasAssertions();

  const bodies: { spans: number }[] = [];
  const firstArrived = Promise.withResolvers<undefined>();

  let hold = Promise.withResolvers<undefined>();

  worker.use(
    http.post(`${API}/drain/v1/traces`, async ({ request }) => {
      const body = batchBody.parse(await decodedBodyJson(request));

      bodies.push({
        spans: body.resourceSpans[0].scopeSpans[0].spans.length,
      });

      firstArrived.resolve(undefined);

      await hold.promise;

      return new HttpResponse(null, { status: 204 });
    }),
  );

  const kernel = await createKernel({ opts: { apiHost: API, tracing: false } });

  ownedKernel = kernel;

  const throwBurst = (count: number) => {
    for (let index = 0; index < count; index += 1) {
      kernel.recordException(new Error(`burst ${index}`), {
        mechanism: { handled: true, type: MECHANISM_TYPE.manual.capture },
      });
    }
  };

  const delivered = () => bodies.reduce((sum, body) => sum + body.spans, 0);

  throwBurst(1);

  await firstArrived.promise;

  throwBurst(100);

  expect(bodies).toStrictEqual([{ spans: 1 }]);

  hold.resolve(undefined);

  await kernel.flush();

  expect(delivered()).toBe(33);

  hold = Promise.withResolvers<undefined>();

  throwBurst(1);

  const disposed = kernel.dispose();

  hold.resolve(undefined);

  await disposed;

  ownedKernel = undefined;

  expect(bodies.at(-1)).toStrictEqual({ spans: 1 });

  expect(delivered()).toBe(34);
});

test("tracing: false releases an exception queued while the config was pending, with its capture time and host link", async () => {
  expect.hasAssertions();

  context.setGlobalContextManager(new StackContextManager().enable());

  const held = Promise.withResolvers<Response>();

  worker.use(http.get(`${API}/v3/config`, async () => held.promise));

  const creating = createKernel({ opts: { apiHost: API, tracing: false } });

  const host = {
    spanId: "00f067aa0ba902b7",
    traceFlags: TraceFlags.NONE,
    traceId: "4bf92f3577b34da6a3ce929d0e0e4736",
  };

  const capturedAt = Date.UTC(2026, 0, 1);

  vi.useFakeTimers({ now: capturedAt, toFake: ["Date"] });

  onTestFinished(() => {
    vi.useRealTimers();
  });

  context.with(trace.setSpanContext(ROOT_CONTEXT, host), () => {
    globalThis.onerror?.("pending boom", "", 1, 1, new Error("pending boom"));
  });

  held.resolve(
    HttpResponse.json({ plugins: { ...DISABLE_ALL_PLUGINS, errors: true } }),
  );

  ownedKernel = await creating;

  const [span] = (await captured()).body.resourceSpans[0].scopeSpans[0].spans;

  expect(span.startTimeUnixNano).toBe(`${capturedAt}000000`);

  expect(span.links).toStrictEqual([
    { traceId: host.traceId, spanId: host.spanId, flags: TraceFlags.NONE },
  ]);

  expect(attrs(span.attributes)[VIEW_ID]).toBe(ownedKernel.views.viewId());

  expect(attrs(span.events[0].attributes)[VIEW_ID]).toBe(
    ownedKernel.views.viewId(),
  );
});
