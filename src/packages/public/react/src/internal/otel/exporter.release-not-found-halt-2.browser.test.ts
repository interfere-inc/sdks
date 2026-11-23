/* oxlint-disable typescript/no-unsafe-assignment */
import { SpanKind, SpanStatusCode } from "@opentelemetry/api";
import type { ExportResult } from "@opentelemetry/core";
import { resourceFromAttributes } from "@opentelemetry/resources";
import type { ReadableSpan } from "@opentelemetry/sdk-trace";

import {
  createTraceExporter,
  isIngestHalted,
  resetIngestHalt,
} from "./exporter.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";
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
