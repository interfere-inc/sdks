/* oxlint-disable typescript/no-unsafe-assignment */
import { SpanKind, SpanStatusCode } from "@opentelemetry/api";
import { isUrlIgnored } from "@opentelemetry/core";
import type { ExportResult } from "@opentelemetry/core";
import { resourceFromAttributes } from "@opentelemetry/resources";
import type { ResourceMetrics } from "@opentelemetry/sdk-metrics";
import type { ReadableSpan } from "@opentelemetry/sdk-trace";
import { z } from "zod";

import { PRODUCER_VERSION } from "../version.js";
import {
  INGEST_IGNORE_RE,
  createMetricExporter,
  createTraceExporter,
  isIngestHalted,
  resetIngestHalt,
} from "./exporter.js";
import {
  afterEach,
  assert,
  beforeEach,
  describe,
  expect,
  test,
  vi,
} from "vitest";
import { HttpResponse, http } from "msw";
import type { HttpResponseResolver } from "msw";

import { createCountSignal } from "../../../test/fixtures/upload-signal.js";
import { worker } from "../../../test/msw.js";

const requests: Request[] = [];

const requestSignal = createCountSignal(requests);

function resultCallback() {
  const called = Promise.withResolvers<true>();
  const spy = vi.fn<(result: ExportResult) => void>();

  const callback = (result: ExportResult) => {
    spy(result);

    called.resolve(true);
  };

  return { callback, spy: Object.assign(spy, { called: called.promise }) };
}

function collector(respond: HttpResponseResolver) {
  return http.post("https://in.example.com/drain/v1/:signal", async (info) => {
    requests.push(info.request.clone());

    requestSignal.notify();

    return respond(info);
  });
}

describe("INGEST_IGNORE_RE contract", () => {
  test.each([
    ["traces", "https://api.interfere.com"],
    ["logs", "https://api.interfere.com/"],
    ["metrics", "https://api.interfere.com"],
    ["traces", "https://in.example.com/api/interfere"],
  ] as const)(
    "ignores the %s URL the exporter actually posts to",
    (signal, base) => {
      expect(
        isUrlIgnored(`${base.replace(/\/$/u, "")}/drain/v1/${signal}`, [
          INGEST_IGNORE_RE,
        ]),
      ).toBe(true);
    },
  );

  test("leaves customer traffic instrumented", () => {
    expect(isUrlIgnored("https://app.test/api/items", [INGEST_IGNORE_RE])).toBe(
      false,
    );
  });
});

const CLIENT_OPTIONS = {
  baseUrl: "https://in.example.com",
  publicKeyHeader: { apiKey: "test-key" },
};

function exporterUnderTest() {
  return createTraceExporter({ clientOptions: CLIENT_OPTIONS });
}

function exportSpan(exporter: ReturnType<typeof exporterUnderTest>) {
  const { callback, spy } = resultCallback();

  exporter.export([makeSpan()], callback);

  return spy;
}

function incompressible(length: number) {
  return btoa(
    String.fromCodePoint(...crypto.getRandomValues(new Uint8Array(length))),
  ).slice(0, length);
}

function makeSpan() {
  return {
    attributes: {},
    droppedAttributesCount: 0,
    droppedEventsCount: 0,
    droppedLinksCount: 0,
    duration: [0, 0],
    endTime: [0, 0],
    ended: true,
    events: [],
    instrumentationScope: { name: "test" },
    kind: SpanKind.INTERNAL,
    links: [],
    name: "test-span",
    resource: resourceFromAttributes({}),
    spanContext: () => ({
      traceId: "a".repeat(32),
      spanId: "b".repeat(16),
      traceFlags: 1,
    }),
    startTime: [0, 0],
    status: { code: SpanStatusCode.UNSET },
  } satisfies ReadableSpan;
}

describe(createTraceExporter, () => {
  beforeEach(() => {
    requests.length = 0;

    worker.use(collector(() => new HttpResponse(null)));
  });

  test("dispatches via a keepalive fetch that never attaches credentials", async () => {
    expect.hasAssertions();

    const callback = exportSpan(exporterUnderTest());

    await requestSignal.reached(1);

    expect(requests).toHaveLength(1);

    const [request] = requests;

    if (!request) {
      throw new Error("Expected an exported request");
    }

    const { url } = request;

    expect(url).toContain("/drain/v1/traces");
    expect(new URL(url).searchParams.has("_pv")).toBe(false);

    expect(request.headers.get("x-interfere-producer-version")).toBe(
      PRODUCER_VERSION,
    );

    expect(request.credentials).toBe("omit");
    expect(request.keepalive).toBe(true);
    expect(request.method).toBe("POST");

    await callback.called;

    expect(callback).toHaveBeenCalledWith({ code: 0 });
  });

  test.each([429, 503])(
    "retries %s without reporting success before acceptance",
    async (status) => {
      expect.hasAssertions();

      worker.use(
        collector(() =>
          requests.length === 1
            ? new HttpResponse(null, {
                status,
                headers: { "Retry-After": "0" },
              })
            : new HttpResponse(null),
        ),
      );

      const callback = exportSpan(exporterUnderTest());

      await callback.called;

      expect(callback).toHaveBeenCalledWith({ code: 0 });

      expect(requests).toHaveLength(2);
      expect(callback).toHaveBeenCalledTimes(1);

      const second = await requests[1]?.text();

      await expect(requests[0]?.text()).resolves.toBe(second);
    },
  );

  test("stamps each attempt with the device clock at dispatch", async () => {
    expect.hasAssertions();

    vi.useFakeTimers({ toFake: ["Date"] });

    onTestFinished(() => {
      vi.useRealTimers();
    });

    const firstAttemptAt = Date.UTC(2026, 10, 4, 12);
    const retryAt = firstAttemptAt + 7000;

    vi.setSystemTime(firstAttemptAt);

    worker.use(
      collector(() => {
        if (requests.length > 1) {
          return new HttpResponse(null);
        }

        vi.setSystemTime(retryAt);

        return new HttpResponse(null, {
          status: 503,
          headers: { "Retry-After": "0" },
        });
      }),
    );

    const callback = exportSpan(exporterUnderTest());

    await callback.called;

    expect(callback).toHaveBeenCalledWith({ code: 0 });

    expect(
      requests.map((request) => request.headers.get("x-interfere-sent-at")),
    ).toStrictEqual([String(firstAttemptAt), String(retryAt)]);
  });

  test("stamps metric exports with the device clock", async () => {
    expect.hasAssertions();

    vi.useFakeTimers({ toFake: ["Date"] });

    onTestFinished(() => {
      vi.useRealTimers();
    });

    const sentAt = Date.UTC(2026, 10, 4, 12);

    vi.setSystemTime(sentAt);

    const { callback, spy } = resultCallback();

    createMetricExporter({ clientOptions: CLIENT_OPTIONS }).export(
      {
        resource: resourceFromAttributes({}),
        scopeMetrics: [],
      } satisfies ResourceMetrics,
      callback,
    );

    await spy.called;

    expect(requests[0]?.url).toContain("/drain/v1/metrics");

    expect(requests[0]?.headers.get("x-interfere-sent-at")).toBe(
      String(sentAt),
    );
  });

  test("short-circuits empty span batches without dispatching", () => {
    const callback = vi.fn<() => void>();

    exporterUnderTest().export([], callback);

    expect(requests).toHaveLength(0);
    expect(callback).toHaveBeenCalledWith({ code: 0 });
  });

  test("does not report a result until the request settles", async () => {
    expect.hasAssertions();

    const response = Promise.withResolvers<Response>();

    onTestFinished(() => response.resolve(new Response(null)));

    worker.use(collector(async () => response.promise));

    const callback = exportSpan(exporterUnderTest());

    await requestSignal.reached(1);

    expect(requests).toHaveLength(1);

    expect(callback).not.toHaveBeenCalled();

    response.resolve(new Response(null));

    await callback.called;

    expect(callback).toHaveBeenCalledExactlyOnceWith({ code: 0 });
  });

  test("reports FAILED with the cause when the fetch rejects", async () => {
    expect.hasAssertions();

    worker.use(collector(() => HttpResponse.error()));

    const callback = exportSpan(exporterUnderTest());

    await callback.called;

    expect(callback).toHaveBeenCalledWith({
      code: 1,
      error: expect.objectContaining({ cause: expect.any(TypeError) }),
    });
  });

  test("sends nothing after a network failure, confirmed without keepalive, until the backoff expires", async () => {
    expect.hasAssertions();

    vi.useFakeTimers({ toFake: ["Date"] });

    onTestFinished(() => {
      vi.useRealTimers();
    });

    worker.use(
      collector(() =>
        requests.length <= 2 ? HttpResponse.error() : new HttpResponse(null),
      ),
    );

    const exporter = exporterUnderTest();

    const blocked = exportSpan(exporter);

    await blocked.called;

    const skipped = exportSpan(exporter);

    await skipped.called;

    expect(skipped).toHaveBeenCalledWith(expect.objectContaining({ code: 1 }));

    expect(requests.map((request) => request.keepalive)).toStrictEqual([
      true,
      false,
    ]);

    vi.setSystemTime(Date.now() + 5000);

    const resumed = exportSpan(exporter);

    await resumed.called;

    expect(resumed).toHaveBeenCalledWith({ code: 0 });
    expect(requests).toHaveLength(3);
  });

  test("reports FAILED when the collector permanently rejects the payload", async () => {
    expect.hasAssertions();

    worker.use(collector(() => new HttpResponse(null, { status: 400 })));

    const callback = exportSpan(exporterUnderTest());

    await callback.called;

    expect(callback).toHaveBeenCalledWith({
      code: 1,
      error: expect.objectContaining({
        statusCode: 400,
      }),
    });
  });

  test("treats the service worker's synthetic 202 Queued as delivered", async () => {
    expect.hasAssertions();

    worker.use(
      collector(
        () => new HttpResponse(null, { status: 202, statusText: "Queued" }),
      ),
    );

    const callback = exportSpan(exporterUnderTest());

    await callback.called;

    expect(callback).toHaveBeenCalledWith({ code: 0 });
  });

  test.for(["forceFlush", "shutdown"] as const)(
    "%s waits for an export that is still in flight",
    async (method) => {
      expect.hasAssertions();

      const response = Promise.withResolvers<null>();

      onTestFinished(() => response.resolve(null));

      worker.use(
        collector(async () => {
          await response.promise;

          return new HttpResponse(null);
        }),
      );

      const exporter = exporterUnderTest();
      const { callback, spy } = resultCallback();

      exporter.export([makeSpan()], callback);

      let settled = false;

      const flushing = (async () => {
        await exporter[method]();

        settled = true;
      })();

      await requestSignal.reached(1);

      expect(settled).toBe(false);

      response.resolve(null);

      await flushing;

      expect(spy).toHaveBeenCalledWith({ code: 0 });
    },
  );

  test("keeps overlapping exports within half the browser keepalive quota and sends the rest without keepalive", async () => {
    expect.hasAssertions();

    const response = Promise.withResolvers<null>();

    onTestFinished(() => response.resolve(null));

    worker.use(
      collector(async () => {
        await response.promise;

        return new HttpResponse(null);
      }),
    );

    const exporter = exporterUnderTest();

    const callbacks = [1, 2, 3].map(() => {
      const { callback, spy } = resultCallback();

      exporter.export(
        [{ ...makeSpan(), attributes: { payload: incompressible(25_000) } }],
        callback,
      );

      return spy;
    });

    await requestSignal.reached(3);

    expect(requests).toHaveLength(3);

    expect(requests.filter((request) => request.keepalive)).toHaveLength(1);

    response.resolve(null);

    await Promise.all(callbacks.map(async (callback) => callback.called));

    for (const callback of callbacks) {
      expect(callback).toHaveBeenCalledWith({ code: 0 });
    }

    const recovered = exportSpan(exporter);

    await requestSignal.reached(4);

    expect(requests).toHaveLength(4);

    expect(requests[3]?.keepalive).toBe(true);

    await recovered.called;

    expect(recovered).toHaveBeenCalledWith({ code: 0 });
  });
});

function makeResourceMetrics() {
  return {
    resource: resourceFromAttributes({ "service.name": "test" }),
    scopeMetrics: [],
  } satisfies ResourceMetrics;
}

async function exportMetrics(
  exporter: ReturnType<typeof createMetricExporter>,
) {
  const { callback, spy } = resultCallback();

  exporter.export(makeResourceMetrics(), callback);

  await spy.called;

  expect(spy).toHaveBeenCalledWith({ code: 0 });

  const [request] = requests.splice(0);

  assert.exists(request);

  return request;
}

describe("createMetricExporter gated resource attributes", () => {
  beforeEach(() => {
    requests.length = 0;

    worker.use(collector(() => new HttpResponse(null)));
  });

  test("merges the gated attrs into the payload resource while enabled", async () => {
    expect.hasAssertions();

    const exporter = createMetricExporter(
      { clientOptions: CLIENT_OPTIONS },
      { attributes: { "browser.platform": "macOS" }, isEnabled: () => true },
    );

    await expect((await exportMetrics(exporter)).json()).resolves.toMatchObject(
      {
        resourceMetrics: [
          {
            resource: {
              attributes: expect.arrayContaining([
                { key: "service.name", value: { stringValue: "test" } },
                { key: "browser.platform", value: { stringValue: "macOS" } },
              ]),
            },
          },
        ],
      },
    );
  });

  test("strips the gated attrs while disabled, re-reading the gate per export", async () => {
    expect.hasAssertions();

    let allowed = false;

    const exporter = createMetricExporter(
      { clientOptions: CLIENT_OPTIONS },
      {
        attributes: { "browser.platform": "macOS" },
        isEnabled: () => allowed,
      },
    );

    await expect((await exportMetrics(exporter)).json()).resolves.toMatchObject(
      {
        resourceMetrics: [
          {
            resource: {
              attributes: [
                { key: "service.name", value: { stringValue: "test" } },
              ],
            },
          },
        ],
      },
    );

    allowed = true;

    await expect((await exportMetrics(exporter)).json()).resolves.toMatchObject(
      {
        resourceMetrics: [
          {
            resource: {
              attributes: expect.arrayContaining([
                { key: "browser.platform", value: { stringValue: "macOS" } },
              ]),
            },
          },
        ],
      },
    );
  });

  test("leaves the payload untouched when no gate is supplied", async () => {
    expect.hasAssertions();

    const exporter = createMetricExporter({ clientOptions: CLIENT_OPTIONS });

    await expect((await exportMetrics(exporter)).json()).resolves.toMatchObject(
      {
        resourceMetrics: [
          {
            resource: {
              attributes: [
                { key: "service.name", value: { stringValue: "test" } },
              ],
            },
          },
        ],
      },
    );
  });
});

const releaseNotFound = () =>
  HttpResponse.json(
    {
      code: "RELEASE_NOT_FOUND",
      message: "Surface `web` has no release `rel_dead`.",
      service: "collector",
    },
    { status: 422 },
  );

const authInvalid = () =>
  HttpResponse.json({ code: "AUTH_PK_INVALID" }, { status: 401 });

async function exportOnce(exporter: ReturnType<typeof exporterUnderTest>) {
  const callback = exportSpan(exporter);

  await callback.called;

  expect(callback).toHaveBeenCalledWith(
    expect.objectContaining({ code: expect.any(Number) }),
  );
}

async function haltOnAuth(exporter: ReturnType<typeof exporterUnderTest>) {
  await exportOnce(exporter);

  await exportOnce(exporter);

  await exportOnce(exporter);

  expect(isIngestHalted()).toBe(true);
}

describe("release-not-found halt", () => {
  beforeEach(() => {
    resetIngestHalt();

    requests.length = 0;

    vi.spyOn(console, "warn");
  });

  afterEach(() => {
    resetIngestHalt();

    vi.restoreAllMocks();
  });

  test("halts every subsequent export after a RELEASE_NOT_FOUND 422", async () => {
    expect.hasAssertions();

    worker.use(collector(releaseNotFound));

    const exporter = exporterUnderTest();
    const first = exportSpan(exporter);

    await first.called;

    expect(first).toHaveBeenCalledWith(expect.objectContaining({ code: 1 }));

    expect(isIngestHalted()).toBe(true);

    const second = exportSpan(exporter);

    await requestSignal.reached(1);

    expect(requests).toHaveLength(1);

    expect(second).toHaveBeenCalledWith(expect.objectContaining({ code: 1 }));

    expect(second.mock.calls[0]?.[0].error?.message).toBe(
      "ingest halted: the ingest endpoint does not recognize this build's release",
    );
  });

  test("halts every subsequent export after the collector filters the environment", async () => {
    expect.hasAssertions();

    worker.use(
      collector(() =>
        HttpResponse.json(
          {
            code: 7,
            message: "Surface `web` filters the `development` environment.",
          },
          { status: 403 },
        ),
      ),
    );

    const exporter = exporterUnderTest();

    await exportSpan(exporter).called;

    expect(isIngestHalted()).toBe(true);

    const second = exportSpan(exporter);

    await second.called;

    await requestSignal.reached(1);

    expect(requests).toHaveLength(1);

    expect(second.mock.calls[0]?.[0].error?.message).toBe(
      "ingest halted: the ingest endpoint filters this page's environment",
    );

    expect(console.warn).toHaveBeenCalledExactlyOnceWith(
      expect.stringContaining("filters the `development` environment"),
    );
  });

  test("keeps exporting after a 403 that is not a collector status", async () => {
    worker.use(collector(() => new HttpResponse("Forbidden", { status: 403 })));

    await exportOnce(exporterUnderTest());

    expect(isIngestHalted()).toBe(false);
  });

  test.each([
    "Release is unknown",
    undefined,
    null,
    42,
    false,
    { detail: "invalid message" },
  ])("warns once when rejection message is %s", async (message) => {
    expect.hasAssertions();

    worker.use(
      collector(() =>
        HttpResponse.json(
          { code: "RELEASE_NOT_FOUND", message },
          { status: 422 },
        ),
      ),
    );

    const exporter = exporterUnderTest();

    exporter.export([makeSpan()], vi.fn());

    await vi.waitFor(() => expect(isIngestHalted()).toBe(true));

    const second = exportSpan(exporter);

    await second.called;

    expect(second).toHaveBeenCalledWith(expect.objectContaining({ code: 1 }));

    await requestSignal.reached(1);

    expect(requests).toHaveLength(1);

    expect(console.warn).toHaveBeenCalledTimes(1);

    expect(console.warn).toHaveBeenCalledWith(
      expect.stringContaining("source-map upload and finalization succeeded"),
    );

    expect(vi.mocked(console.warn).mock.calls[0]?.[0]).not.toContain(
      "undefined",
    );
  });

  test("tolerates auth-invalid rejections below the threshold", async () => {
    worker.use(collector(authInvalid));

    const exporter = exporterUnderTest();

    await exportOnce(exporter);

    await exportOnce(exporter);

    expect(isIngestHalted()).toBe(false);
    expect(requests).toHaveLength(2);
  });

  test("halts after three consecutive auth-invalid rejections", async () => {
    worker.use(collector(authInvalid));

    const exporter = exporterUnderTest();

    await exportOnce(exporter);

    await exportOnce(exporter);

    await exportOnce(exporter);

    expect(isIngestHalted()).toBe(true);

    expect(console.warn).toHaveBeenCalledWith(
      expect.stringContaining("INTERFERE_API_KEY"),
    );

    exporter.export([makeSpan()], vi.fn());

    expect(requests).toHaveLength(3);
  });

  test("names the public key, not the release, while auth-halted", async () => {
    worker.use(collector(authInvalid));

    const exporter = exporterUnderTest();

    await exportOnce(exporter);

    await exportOnce(exporter);

    await exportOnce(exporter);

    const halted = exportSpan(exporter);

    await halted.called;

    expect(halted.mock.calls[0]?.[0].error?.message).toBe(
      "ingest paused: the ingest endpoint rejected this page's public key",
    );

    expect(requests).toHaveLength(3);
  });

  describe("auth-halt cool-down", () => {
    beforeEach(() => {
      vi.useFakeTimers({ toFake: ["Date"] });
    });

    afterEach(() => {
      vi.useRealTimers();
    });

    test("stays halted until five minutes pass", async () => {
      worker.use(collector(authInvalid));

      const warn = vi.spyOn(console, "warn");
      const exporter = exporterUnderTest();

      await haltOnAuth(exporter);

      const rejections = warn.mock.calls
        .map(([message]) => String(message))
        .filter((message) => message.includes("rejected this page"));

      expect(rejections).toHaveLength(1);
      expect(rejections[0]).not.toMatch(/\bsurface\b|\bApps?\b/u);

      vi.advanceTimersByTime(5 * 60 * 1000 - 1);

      expect(isIngestHalted()).toBe(true);

      await exportOnce(exporter);

      expect(requests).toHaveLength(3);
    });

    test("re-halts after a single rejected probe once the cool-down ends", async () => {
      worker.use(collector(authInvalid));

      const exporter = exporterUnderTest();

      await haltOnAuth(exporter);

      vi.advanceTimersByTime(5 * 60 * 1000);

      expect(isIngestHalted()).toBe(false);

      await exportOnce(exporter);

      expect(requests).toHaveLength(4);
      expect(isIngestHalted()).toBe(true);

      await exportOnce(exporter);

      expect(requests).toHaveLength(4);
    });

    test("resumes export when the probe after the cool-down succeeds", async () => {
      worker.use(
        collector(() =>
          requests.length >= 4 ? new HttpResponse(null) : authInvalid(),
        ),
      );

      const exporter = exporterUnderTest();

      await haltOnAuth(exporter);

      vi.advanceTimersByTime(5 * 60 * 1000);

      await exportOnce(exporter);

      await exportOnce(exporter);

      expect(isIngestHalted()).toBe(false);
      expect(requests).toHaveLength(5);
    });

    test("keeps a release-not-found halt for the page lifetime", async () => {
      worker.use(collector(releaseNotFound));

      const exporter = exporterUnderTest();

      await exportOnce(exporter);

      vi.advanceTimersByTime(60 * 60 * 1000);

      expect(isIngestHalted()).toBe(true);
    });
  });

  test("resets the auth-rejection count on a successful export", async () => {
    worker.use(
      collector(() =>
        requests.length === 3 ? new HttpResponse(null) : authInvalid(),
      ),
    );

    const exporter = exporterUnderTest();

    await exportOnce(exporter);

    await exportOnce(exporter);

    await exportOnce(exporter);

    await exportOnce(exporter);

    await exportOnce(exporter);

    expect(isIngestHalted()).toBe(false);
    expect(requests).toHaveLength(5);
  });

  test("does not halt on a 401 with a different code", async () => {
    worker.use(
      collector(() =>
        HttpResponse.json({ code: "AUTH_MISSING" }, { status: 401 }),
      ),
    );

    const exporter = exporterUnderTest();

    await exportOnce(exporter);

    await exportOnce(exporter);

    await exportOnce(exporter);

    expect(isIngestHalted()).toBe(false);
  });

  test.each([
    [
      "does not halt on a 422 with a different code",
      () =>
        HttpResponse.json({ code: "OTLP_INVALID_SEMCONV" }, { status: 422 }),
    ],
    [
      "tolerates a non-JSON 422 body",
      () => new HttpResponse("bad gateway", { status: 422 }),
    ],
  ])("%s", async (_name, respond) => {
    expect.hasAssertions();

    worker.use(collector(respond));

    const callback = exportSpan(exporterUnderTest());

    await callback.called;

    expect(callback).toHaveBeenCalledWith(expect.objectContaining({ code: 1 }));

    expect(isIngestHalted()).toBe(false);
  });

  test.each([null, false, 42, "RELEASE_NOT_FOUND", []])(
    "does not halt for a non-object rejection body: %j",
    async (body) => {
      worker.use(collector(() => HttpResponse.json(body, { status: 422 })));

      const exporter = exporterUnderTest();

      await exportOnce(exporter);

      expect(isIngestHalted()).toBe(false);
      expect(console.warn).not.toHaveBeenCalled();
    },
  );
});

function bulkySpans(count: number) {
  return Array.from({ length: count }, () => ({
    ...makeSpan(),
    attributes: { payload: "x".repeat(2000) },
  }));
}

describe("trace export while the page is hidden", () => {
  beforeEach(() => {
    requests.length = 0;

    worker.use(collector(() => new HttpResponse(null)));
  });

  afterEach(() => {
    globalThis.dispatchEvent(
      new PageTransitionEvent("pageshow", { persisted: true }),
    );
  });

  test("sends a large batch as one request while the page is visible", async () => {
    expect.hasAssertions();

    const { callback, spy } = resultCallback();

    exporterUnderTest().export(bulkySpans(40), callback);

    await spy.called;

    expect(requests).toHaveLength(1);
    expect(spy).toHaveBeenCalledWith({ code: 0 });
  });

  test("gzips a visible page's export and marks it so the collector can decode it", async () => {
    expect.hasAssertions();

    const { callback, spy } = resultCallback();

    exporterUnderTest().export(bulkySpans(40), callback);

    await spy.called;

    expect(requests).toHaveLength(1);

    const [request] = requests;

    expect(request?.headers.get("content-encoding")).toBe("gzip");

    const compressed = await request?.clone().arrayBuffer();

    const decoded = await new Response(
      new Blob([compressed ?? new ArrayBuffer(0)])
        .stream()
        .pipeThrough(new DecompressionStream("gzip")),
    ).text();

    const exported = JSON.parse(decoded) as {
      resourceSpans: { scopeSpans: { spans: unknown[] }[] }[];
    };

    expect(
      exported.resourceSpans
        .flatMap((resourceSpans) => resourceSpans.scopeSpans)
        .reduce((total, scope) => total + scope.spans.length, 0),
    ).toBe(40);

    expect(compressed?.byteLength).toBeLessThan(
      new TextEncoder().encode(decoded).byteLength / 2,
    );
  });

  test("leaves a small export uncompressed", async () => {
    expect.hasAssertions();

    const { callback, spy } = resultCallback();

    exporterUnderTest().export([makeSpan()], callback);

    await spy.called;

    expect(requests[0]?.headers.get("content-encoding")).toBeNull();
  });

  test("sends a hidden page's export uncompressed so it can leave with the page", async () => {
    expect.hasAssertions();

    globalThis.dispatchEvent(
      new PageTransitionEvent("pagehide", { persisted: true }),
    );

    const { callback, spy } = resultCallback();

    exporterUnderTest().export(bulkySpans(40), callback);

    await spy.called;

    expect(
      requests.map((request) => request.headers.get("content-encoding")),
    ).toStrictEqual(requests.map(() => null));
  });

  test("tags a retained hidden export with its carryover id and leaves a visible export untagged", async () => {
    expect.hasAssertions();

    const exporter = exporterUnderTest();
    const visible = resultCallback();

    exporter.export([makeSpan()], visible.callback);

    await visible.spy.called;

    globalThis.dispatchEvent(
      new PageTransitionEvent("pagehide", { persisted: true }),
    );

    const hidden = resultCallback();

    exporter.export([makeSpan()], hidden.callback);

    await hidden.spy.called;

    expect(hidden.spy).toHaveBeenCalledWith({ code: 0 });

    expect(
      requests.map((request) => request.headers.get("x-interfere-carryover")),
    ).toStrictEqual([null, expect.stringMatching(/^[0-9a-f-]{36}$/u)]);
  });

  test("splits a beforeunload flush into keepalive-sized requests, sends only keepalive ones and carries the rest to the next shown page", async () => {
    expect.hasAssertions();

    globalThis.dispatchEvent(new Event("beforeunload"));

    const { callback, spy } = resultCallback();

    exporterUnderTest().export(bulkySpans(40), callback);

    await requestSignal.reached(1);

    const leaving = requests.length;

    expect([
      leaving > 0,
      requests.every((request) => request.keepalive),
    ]).toStrictEqual([true, true]);

    globalThis.dispatchEvent(
      new PageTransitionEvent("pagehide", { persisted: true }),
    );

    globalThis.dispatchEvent(
      new PageTransitionEvent("pageshow", { persisted: true }),
    );

    await spy.called;

    expect(spy).toHaveBeenCalledWith({ code: 0 });

    const delivered = async () => {
      const bodies = await Promise.all(
        requests.map(
          async (request) =>
            (await request.clone().json()) as {
              resourceSpans: { scopeSpans: { spans: unknown[] }[] }[];
            },
        ),
      );

      return bodies
        .flatMap((body) => body.resourceSpans)
        .flatMap((resourceSpans) => resourceSpans.scopeSpans)
        .reduce((total, scope) => total + scope.spans.length, 0);
    };

    let total = await delivered();

    while (total < 40) {
      // oxlint-disable-next-line no-await-in-loop -- Each wait is for the next request the carried-over drain sends; the spy resolves once exports are parked, before the drain has sent them all.
      await requestSignal.reached(requests.length + 1);

      // oxlint-disable-next-line no-await-in-loop -- The count depends on the request just recorded.
      total = await delivered();
    }

    expect(total).toBe(40);
  });

  test("puts exception spans, then request spans, first in a leave-time export", async () => {
    expect.hasAssertions();

    globalThis.dispatchEvent(new Event("beforeunload"));

    const { callback } = resultCallback();

    exporterUnderTest().export(
      [
        { ...makeSpan(), name: "resource" },
        { ...makeSpan(), kind: SpanKind.CLIENT, name: "GET" },
        {
          ...makeSpan(),
          events: [
            {
              attributes: { "exception.type": "TypeError" },
              droppedAttributesCount: 0,
              name: "exception",
              time: [0, 0],
            },
          ],
          name: "exception",
        },
      ],
      callback,
    );

    await requestSignal.reached(1);

    const [request] = requests;

    const body = z
      .object({
        resourceSpans: z.array(
          z.object({
            scopeSpans: z.array(
              z.object({ spans: z.array(z.object({ name: z.string() })) }),
            ),
          }),
        ),
      })
      .parse(await request?.clone().json());

    expect(
      body.resourceSpans
        .flatMap((resource) => resource.scopeSpans)
        .flatMap((scope) => scope.spans)
        .map((span) => span.name),
    ).toStrictEqual(["exception", "GET", "resource"]);
  });

  test("hands a visible export still in flight without keepalive to the leave path when the page starts leaving, exception first", async () => {
    expect.hasAssertions();

    const navigation = Promise.withResolvers<null>();

    onTestFinished(() => navigation.resolve(null));

    worker.use(
      collector(async ({ request }) => {
        if (!request.keepalive) {
          await navigation.promise;

          return HttpResponse.error();
        }

        return new HttpResponse(null);
      }),
    );

    const { callback, spy } = resultCallback();

    exporterUnderTest().export(
      [
        { ...makeSpan(), attributes: { payload: incompressible(60_000) } },
        {
          ...makeSpan(),
          events: [
            {
              attributes: { "exception.type": "TypeError" },
              droppedAttributesCount: 0,
              name: "exception",
              time: [0, 0],
            },
          ],
          name: "exception",
        },
      ],
      callback,
    );

    await requestSignal.reached(1);

    expect(requests.map((request) => request.keepalive)).toStrictEqual([false]);

    globalThis.dispatchEvent(new Event("beforeunload"));

    await requestSignal.reached(2);

    navigation.resolve(null);

    const [, leaving] = requests;

    const body = z
      .object({
        resourceSpans: z.array(
          z.object({
            scopeSpans: z.array(
              z.object({ spans: z.array(z.object({ name: z.string() })) }),
            ),
          }),
        ),
      })
      .parse(await leaving?.clone().json());

    expect([
      leaving?.keepalive,
      body.resourceSpans
        .flatMap((resource) => resource.scopeSpans)
        .flatMap((scope) => scope.spans)
        .map((span) => span.name),
    ]).toStrictEqual([true, ["exception"]]);

    globalThis.dispatchEvent(
      new PageTransitionEvent("pagehide", { persisted: true }),
    );

    globalThis.dispatchEvent(
      new PageTransitionEvent("pageshow", { persisted: true }),
    );

    await spy.called;

    await new Promise((resolve) => {
      setTimeout(resolve, 100);
    });

    expect(spy.mock.calls).toStrictEqual([[{ code: 0 }]]);
  });

  test("leaves a visible export already sent with keepalive alone when the page starts leaving", async () => {
    expect.hasAssertions();

    const response = Promise.withResolvers<null>();

    onTestFinished(() => response.resolve(null));

    worker.use(
      collector(async () => {
        await response.promise;

        return new HttpResponse(null);
      }),
    );

    const { callback, spy } = resultCallback();

    exporterUnderTest().export([makeSpan()], callback);

    await requestSignal.reached(1);

    globalThis.dispatchEvent(new Event("beforeunload"));

    response.resolve(null);

    await spy.called;

    await new Promise((resolve) => {
      setTimeout(resolve, 100);
    });

    expect([
      requests.map((request) => request.keepalive),
      spy.mock.calls,
    ]).toStrictEqual([[true], [[{ code: 0 }]]]);
  });

  test("carries an over-budget export over after beforeunload instead of sending it without keepalive, and delivers it once the page is shown", async () => {
    expect.hasAssertions();

    globalThis.dispatchEvent(new Event("beforeunload"));

    const { callback, spy } = resultCallback();

    exporterUnderTest().export(
      [{ ...makeSpan(), attributes: { payload: incompressible(40_000) } }],
      callback,
    );

    await new Promise((resolve) => {
      setTimeout(resolve, 100);
    });

    expect(requests.filter((request) => !request.keepalive)).toStrictEqual([]);

    globalThis.dispatchEvent(
      new PageTransitionEvent("pagehide", { persisted: true }),
    );

    globalThis.dispatchEvent(
      new PageTransitionEvent("pageshow", { persisted: true }),
    );

    await spy.called;

    await requestSignal.reached(1);

    expect(spy).toHaveBeenCalledWith({ code: 0 });

    expect(
      requests.map((request) => request.headers.get("x-interfere-carryover")),
    ).toStrictEqual([expect.stringMatching(/^[0-9a-f-]{36}$/u)]);
  });

  test("splits a large batch into keepalive-sized requests once hidden", async () => {
    expect.hasAssertions();

    globalThis.dispatchEvent(
      new PageTransitionEvent("pagehide", { persisted: true }),
    );

    const { callback, spy } = resultCallback();

    exporterUnderTest().export(bulkySpans(40), callback);

    await spy.called;

    expect(requests.length).toBeGreaterThan(1);
    expect(spy).toHaveBeenCalledTimes(1);
    expect(spy).toHaveBeenCalledWith({ code: 0 });

    const bodies = await Promise.all(
      requests.map(async (request) => request.clone().arrayBuffer()),
    );

    for (const body of bodies) {
      expect(body.byteLength).toBeLessThanOrEqual(16_000);
    }

    const exported = await Promise.all(
      requests.map(
        async (request) =>
          (await request.json()) as {
            resourceSpans: { scopeSpans: { spans: unknown[] }[] }[];
          },
      ),
    );

    const spanCount = exported
      .flatMap((body) => body.resourceSpans)
      .flatMap((resourceSpans) => resourceSpans.scopeSpans)
      .reduce((total, scope) => total + scope.spans.length, 0);

    expect(spanCount).toBe(40);
  });
});
