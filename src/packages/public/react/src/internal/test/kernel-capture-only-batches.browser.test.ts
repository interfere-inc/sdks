import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";
import { context, propagation, trace } from "@opentelemetry/api";
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

import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { worker } from "../../../test/msw.js";
import { MAX_EXPORT_BODY_BYTES } from "../capture/capture-only-sink.js";
import { createKernel } from "../kernel.js";
import type { Kernel } from "../kernel.js";
import { decodedBodyText } from "../../../test/fixtures/request-body.js";

const API = "https://capture-only-batches.invalid";

const HANDLED = {
  mechanism: { handled: true, type: MECHANISM_TYPE.manual.capture },
} as const;

const keyValue = z.object({
  key: z.string(),
  value: z.looseObject({ stringValue: z.string().optional() }),
});

const exportBody = z.object({
  resourceSpans: z.tuple([
    z.object({
      scopeSpans: z.tuple([
        z.object({
          spans: z.array(
            z.looseObject({
              events: z.tuple([z.object({ attributes: z.array(keyValue) })]),
            }),
          ),
        }),
      ]),
    }),
  ]),
});

const logsBody = z.object({
  resourceLogs: z.tuple([
    z.object({
      scopeLogs: z.tuple([
        z.object({
          logRecords: z.array(z.object({ attributes: z.array(keyValue) })),
        }),
      ]),
    }),
  ]),
});

interface LogExport {
  bytes: number;
  keepalive: boolean;
  paths: string[];
}

interface Export {
  bytes: number;
  firstSpanBytes: number;
  keepalive: boolean;
  messages: string[];
}

const textEncoder = new TextEncoder();

let ownedKernel: Kernel | undefined;

let posts: Export[];

let logPosts: LogExport[];

let firstArrived: PromiseWithResolvers<undefined>;

let hold: PromiseWithResolvers<undefined>;

function messageOf(attributes: z.infer<typeof keyValue>[]) {
  return (
    attributes.find(({ key }) => key === "exception.message")?.value
      .stringValue ?? ""
  );
}

function pathOf(attributes: z.infer<typeof keyValue>[]) {
  return (
    attributes.find(({ key }) => key === "url.path")?.value.stringValue ?? ""
  );
}

function largeMessage(index: number) {
  return `large ${String(index).padStart(2, "0")} ${"x".repeat(900)}`;
}

beforeEach(() => {
  ownedKernel = undefined;

  posts = [];

  logPosts = [];

  firstArrived = Promise.withResolvers<undefined>();

  hold = Promise.withResolvers<undefined>();

  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

  trace.disable();

  context.disable();

  propagation.disable();

  worker.use(
    http.get(`${API}/v3/config`, () =>
      HttpResponse.json({ plugins: DISABLE_ALL_PLUGINS }),
    ),
    http.post(`${API}/drain/v1/traces`, async ({ request }) => {
      const raw = await decodedBodyText(request);

      const [
        {
          scopeSpans: [{ spans }],
        },
      ] = exportBody.parse(JSON.parse(raw)).resourceSpans;

      const [firstSpan] = spans;

      posts.push({
        bytes: textEncoder.encode(raw).byteLength,
        firstSpanBytes: textEncoder.encode(JSON.stringify(firstSpan))
          .byteLength,
        keepalive: request.keepalive,
        messages: spans.map((span) => messageOf(span.events[0].attributes)),
      });

      firstArrived.resolve(undefined);

      await hold.promise;

      return new HttpResponse(null, { status: 204 });
    }),
    http.post(`${API}/drain/v1/logs`, async ({ request }) => {
      const raw = await decodedBodyText(request);

      logPosts.push({
        bytes: textEncoder.encode(raw).byteLength,
        keepalive: request.keepalive,
        paths: logsBody
          .parse(JSON.parse(raw))
          .resourceLogs[0].scopeLogs[0].logRecords.map((record) =>
            pathOf(record.attributes),
          ),
      });

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

async function kernelWithHeldFirstExport(granted = false) {
  const kernel = await createKernel({
    onCreated: (created) => {
      if (granted) {
        created.consent.set(true);
      }
    },
    opts: { apiHost: API, tracing: false },
  });

  ownedKernel = kernel;

  kernel.recordException(new Error("first"), HANDLED);

  await firstArrived.promise;

  return kernel;
}

function throwLargeBurst(kernel: Kernel) {
  for (let index = 0; index < 32; index += 1) {
    kernel.recordException(new Error(largeMessage(index)), HANDLED);
  }
}

function expectKeepaliveSized(list: { bytes: number; keepalive: boolean }[]) {
  expect(list.map((post) => post.keepalive)).not.toContain(false);

  expect(Math.max(...list.map((post) => post.bytes))).toBeLessThanOrEqual(
    MAX_EXPORT_BODY_BYTES,
  );
}

test("tracing: false drains 32 large exceptions behind a held export as several keepalive-sized exports, each once and in order", async () => {
  expect.hasAssertions();

  const kernel = await kernelWithHeldFirstExport();

  throwLargeBurst(kernel);

  hold.resolve(undefined);

  await kernel.flush();

  expect(posts.length).toBeGreaterThan(2);

  expectKeepaliveSized(posts);

  expect(posts.flatMap((post) => post.messages)).toStrictEqual([
    "first",
    ...Array.from({ length: 32 }, (_, index) => largeMessage(index)),
  ]);

  const batches = posts.slice(1);

  expect(
    batches
      .slice(0, -1)
      .map(
        (batch, index) =>
          batch.bytes + (batches[index + 1]?.firstSpanBytes ?? 0) + 1 >
          MAX_EXPORT_BODY_BYTES,
      ),
  ).not.toContain(false);
});

test("tracing: false drains the queue behind a held export when the page hides, with keepalive on every export", async () => {
  expect.hasAssertions();

  const kernel = await kernelWithHeldFirstExport();

  throwLargeBurst(kernel);

  onTestFinished(() => {
    globalThis.dispatchEvent(
      new PageTransitionEvent("pageshow", { persisted: true }),
    );
  });

  globalThis.dispatchEvent(
    new PageTransitionEvent("pagehide", { persisted: true }),
  );

  const flushed = kernel.flush();

  expect(posts).toHaveLength(1);

  hold.resolve(undefined);

  await flushed;

  expect(posts.flatMap((post) => post.messages)).toHaveLength(33);

  expectKeepaliveSized(posts);
});

test("tracing: false truncates a 5 MB exception message and still delivers it with keepalive, keeping the queue behind it in order", async () => {
  expect.hasAssertions();

  const kernel = await kernelWithHeldFirstExport();

  kernel.recordException(
    new Error(`huge ${"x".repeat(5 * 1024 * 1024)}`),
    HANDLED,
  );

  kernel.recordException(new Error("after"), HANDLED);

  hold.resolve(undefined);

  await kernel.flush();

  expect(
    posts.flatMap((post) =>
      post.messages.map((message) => message.slice(0, 5)),
    ),
  ).toStrictEqual(["first", "huge ", "after"]);

  expectKeepaliveSized(posts);

  const delivered = posts.flatMap((post) => post.messages)[1] ?? "";

  expect(delivered.length).toBeLessThan(MAX_EXPORT_BODY_BYTES);
});

test("tracing: false drains page views queued behind a held export as keepalive-sized log exports, each once and in order", async () => {
  expect.hasAssertions();

  worker.use(
    http.get(`${API}/v3/config`, () =>
      HttpResponse.json({
        plugins: { ...DISABLE_ALL_PLUGINS, pageEvents: true },
      }),
    ),
  );

  const { href } = globalThis.location;
  const state: unknown = globalThis.history.state;

  onTestFinished(() => {
    globalThis.history.replaceState(state, "", href);
  });

  const kernel = await kernelWithHeldFirstExport(true);

  await vi.waitFor(() => {
    expect(kernel.isPageEventsCaptureEnabled()).toBe(true);
  });

  const paths = Array.from(
    { length: 24 },
    (_, index) =>
      `/capture-only-batches/${String(index).padStart(2, "0")}/${"x".repeat(2000)}`,
  );

  for (const path of paths) {
    globalThis.history.pushState(null, "", path);
  }

  hold.resolve(undefined);

  await kernel.flush();

  expect(logPosts.length).toBeGreaterThan(1);

  expectKeepaliveSized(logPosts);

  const delivered = logPosts.flatMap((post) => post.paths);

  expect(delivered.slice(1)).toStrictEqual(paths);

  expect(delivered).toHaveLength(25);
});
