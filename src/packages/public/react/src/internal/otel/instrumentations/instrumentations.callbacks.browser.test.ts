import type { Span } from "@opentelemetry/api";
import { InstrumentationBase } from "@opentelemetry/instrumentation";
import { DocumentLoadInstrumentation } from "@opentelemetry/instrumentation-document-load";
import { LongTaskInstrumentation } from "@opentelemetry/instrumentation-long-task";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { WebTracerProvider } from "@opentelemetry/sdk-trace-web";
import type { MockInstance } from "vitest";

import { createExportFilterProcessor } from "../export-filter.js";
import { registerBundledInstrumentations } from "./instrumentations.js";
import type { BundledInstrumentations } from "./instrumentations.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

function resourceTiming(name: string) {
  const timing = {
    connectEnd: 0,
    connectStart: 0,
    contentType: "",
    decodedBodySize: 0,
    deliveryType: "",
    domainLookupEnd: 0,
    domainLookupStart: 0,
    duration: 0,
    encodedBodySize: 0,
    entryType: "resource",
    fetchStart: 0,
    finalResponseHeadersStart: 0,
    firstInterimResponseStart: 0,
    initiatorType: "script",
    name,
    nextHopProtocol: "h2",
    redirectEnd: 0,
    redirectStart: 0,
    requestStart: 0,
    responseEnd: 0,
    responseStart: 0,
    responseStatus: 200,
    secureConnectionStart: 0,
    serverTiming: [],
    startTime: 0,
    toJSON() {
      return {
        name: this.name,
        entryType: this.entryType,
        startTime: this.startTime,
        duration: this.duration,
      };
    },
    transferSize: 0,
    workerStart: 0,
  };

  return timing satisfies PerformanceResourceTiming;
}

describe("registered instrumentation callbacks", () => {
  let provider: WebTracerProvider;
  let exporter: InMemorySpanExporter;
  let registration: MockInstance<InstrumentationBase["setTracerProvider"]>;
  let handle: BundledInstrumentations | undefined;

  beforeEach(() => {
    exporter = new InMemorySpanExporter();

    provider = new WebTracerProvider({
      spanProcessors: [new SimpleSpanProcessor(exporter)],
    });

    registration = vi.spyOn(InstrumentationBase.prototype, "setTracerProvider");
  });

  function finishSpan(span: Span) {
    span.end();

    const finished = exporter.getFinishedSpans().at(-1);

    if (!finished) {
      throw new Error("Expected a finished span");
    }

    return finished;
  }

  function register() {
    handle = registerBundledInstrumentations({
      ignoreUrls: [],
      propagateTo: ["https://app.test", "https://cdn.test"],
      resolveRoute: () => "/test-route",
      tracerProvider: provider,
    });

    handle.setPageEventsEnabled(true);
  }

  function instrumentation<T extends InstrumentationBase>(
    constructor: abstract new (...args: never[]) => T,
  ) {
    const inst = registration.mock.contexts.find(
      (candidate): candidate is T => candidate instanceof constructor,
    );

    if (!inst) {
      throw new Error(`${constructor.name} not registered`);
    }

    return inst;
  }

  afterEach(async () => {
    handle?.dispose();

    handle = undefined;

    for (const inst of registration.mock.contexts) {
      if (inst instanceof InstrumentationBase) {
        inst.disable();
      }
    }

    await provider.shutdown();

    vi.restoreAllMocks();
  });

  test("stamps document-load resource spans, cross- and same-origin", () => {
    register();

    const resourceFetch = instrumentation(
      DocumentLoadInstrumentation,
    ).getConfig().applyCustomAttributesOnSpan?.resourceFetch;

    const cross = provider.getTracer("test").startSpan("cross");

    resourceFetch?.(cross, resourceTiming("https://cdn.test/app.js"));

    const crossResult = finishSpan(cross);

    expect(crossResult.attributes["server.address"]).toBe("cdn.test");
    expect(crossResult.name).toContain("cdn.test");

    const same = provider.getTracer("test").startSpan("same");

    resourceFetch?.(
      same,
      resourceTiming(`${globalThis.location.origin}/static/app.js`),
    );

    expect(finishSpan(same).name).toBe("resource: /static/app.js");
  });

  test("does not export document-load resource spans for ignored URLs", async () => {
    const exported = new InMemorySpanExporter();

    const filtered = new WebTracerProvider({
      spanProcessors: [
        createExportFilterProcessor(new SimpleSpanProcessor(exported)),
      ],
    });

    handle = registerBundledInstrumentations({
      ignoreUrls: ["https://ingest.test/v3/session", /\/drain\/v1\//u],
      resolveRoute: () => "/test-route",
      tracerProvider: filtered,
    });

    handle.setPageEventsEnabled(true);

    const resourceFetch = instrumentation(
      DocumentLoadInstrumentation,
    ).getConfig().applyCustomAttributesOnSpan?.resourceFetch;

    expect(resourceFetch).toBeTypeOf("function");

    for (const url of [
      "https://ingest.test/v3/session",
      "https://ingest.test/drain/v1/traces",
      "https://cdn.test/app.js",
    ]) {
      const span = filtered.getTracer("test").startSpan(url);

      resourceFetch?.(span, resourceTiming(url));

      span.end();
    }

    expect(
      exported.getFinishedSpans().map((span) => span.attributes["url.full"]),
    ).toStrictEqual(["https://cdn.test/app.js"]);

    await filtered.shutdown();
  });

  test("ignores document-load entries with a non-string or unparseable name", () => {
    register();

    const resourceFetch = instrumentation(
      DocumentLoadInstrumentation,
    ).getConfig().applyCustomAttributesOnSpan?.resourceFetch;

    const nonString = provider.getTracer("test").startSpan("non-string");
    const resource = resourceTiming("resource");

    expect(resourceFetch).toBeTypeOf("function");

    Object.defineProperty(resource, "name", { value: 123 });

    resourceFetch?.(nonString, resource);

    const unparseable = provider.getTracer("test").startSpan("unparseable");

    resourceFetch?.(unparseable, resourceTiming("::::not a url"));

    expect(finishSpan(nonString).attributes).toStrictEqual({});
    expect(finishSpan(unparseable).attributes).toStrictEqual({});
  });

  test("stamps long-task container type when present", () => {
    register();

    const observe = instrumentation(LongTaskInstrumentation).getConfig()
      .observerCallback;

    const withContainer = provider.getTracer("test").startSpan("container");

    observe?.(withContainer, {
      longtaskEntry: Object.assign(new PerformanceMark("longtask"), {
        attribution: [
          Object.assign(new PerformanceMark("attribution"), {
            containerType: "iframe",
            containerSrc: "",
            containerId: "",
            containerName: "",
          }),
        ],
      }),
    });

    expect(
      finishSpan(withContainer).attributes["longtask.container_type"],
    ).toBe("iframe");

    const withoutContainer = provider
      .getTracer("test")
      .startSpan("no-container");

    observe?.(withoutContainer, {
      longtaskEntry: Object.assign(new PerformanceMark("longtask"), {
        attribution: [],
      }),
    });

    expect(
      finishSpan(withoutContainer).attributes["longtask.container_type"],
    ).toBeUndefined();
  });
});
