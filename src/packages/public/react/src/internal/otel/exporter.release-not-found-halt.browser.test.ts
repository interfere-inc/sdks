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
});
