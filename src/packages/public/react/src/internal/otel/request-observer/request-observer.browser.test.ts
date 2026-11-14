import { ROOT_CONTEXT, context, propagation, trace } from "@opentelemetry/api";
import {
  InMemoryLogRecordExporter,
  LoggerProvider,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { StackContextManager } from "@opentelemetry/sdk-trace-web";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, onTestFinished, test } from "vitest";

import { worker } from "../../../../test/msw.js";
import {
  REQUEST_OBSERVATION_EVENT_NAME,
  REQUEST_OBSERVER_LOGGER_NAME,
  requestObservationLogger,
} from "./request-observer-events.js";
import { propagationTargets } from "../instrumentations/propagation-targets.js";
import { installRequestObserver } from "./request-observer.js";

const exporter = new InMemoryLogRecordExporter();

const loggerProvider = new LoggerProvider({
  processors: [new SimpleLogRecordProcessor({ exporter })],
});

const nativeFetch = globalThis.fetch;
// oxlint-disable-next-line typescript/unbound-method -- The native functions are saved to assert restoration by identity.
const nativeOpen = XMLHttpRequest.prototype.open;
// oxlint-disable-next-line typescript/unbound-method -- The native functions are saved to assert restoration by identity.
const nativeSend = XMLHttpRequest.prototype.send;

beforeEach(() => {
  exporter.reset();

  worker.use(
    http.get(`${location.origin}/observed`, ({ request }) =>
      HttpResponse.json({ traceparent: request.headers.get("traceparent") }),
    ),
    http.post(`${location.origin}/observed`, () =>
      HttpResponse.json(null, { status: 503 }),
    ),
    http.get(`${location.origin}/ignored`, () => HttpResponse.json({})),
    http.get(`${location.origin}/broken`, () => HttpResponse.error()),
  );
});

afterEach(() => {
  context.disable();

  propagation.disable();

  // oxlint-disable-next-line interfere-testing/no-test-doubles -- Restores the real native fetch by identity after the observer's own restore ran.
  globalThis.fetch = nativeFetch;

  XMLHttpRequest.prototype.open = nativeOpen;

  XMLHttpRequest.prototype.send = nativeSend;
});

async function settle() {
  await new Promise<void>((resolve) => {
    setTimeout(resolve, 0);
  });
}

function install() {
  const redactUrl = (url: string) => url.replace("secret", "[redacted]");

  const observer = installRequestObserver({
    applicationErrors: false,
    failureRanges: null,
    ignoreUrls: [/\/ignored$/u],
    onObservation: requestObservationLogger(loggerProvider, redactUrl),
    redactUrl,
    targets: propagationTargets({ pageOrigin: location.origin }),
    traceHeaders: null,
    tracer: null,
  });

  onTestFinished(() => {
    observer.dispose();
  });

  return observer;
}

function records() {
  return exporter.getFinishedLogRecords().map((record) => ({
    attributes: record.attributes,
    eventName: record.eventName,
    scope: record.instrumentationScope.name,
    spanContext: record.spanContext,
  }));
}

async function sendXhr(method: string, url: string) {
  return new Promise<number>((resolve) => {
    const xhr = new XMLHttpRequest();

    xhr.addEventListener("loadend", () => resolve(xhr.status));

    xhr.open(method, url);

    xhr.send();
  });
}

test("records one event per fetch and XHR with the outcome and no header writes", async () => {
  install();

  const ok = await fetch(`${location.origin}/observed?token=secret`);

  await expect(ok.json()).resolves.toStrictEqual({ traceparent: null });

  const failed = await fetch(
    new Request(`${location.origin}/observed`, {
      method: "POST",
    }),
  );

  expect(failed.status).toBe(503);

  await expect(sendXhr("GET", "/observed")).resolves.toBe(200);

  await fetch(`${location.origin}/ignored`);

  await settle();

  expect(records()).toStrictEqual([
    {
      attributes: {
        "url.full": `${location.origin}/observed?token=[redacted]`,
        "url.path": "/observed",
        "http.request.method": "GET",
        "http.response.status_code": 200,
        "interfere.request.kind": "fetch",
      },
      eventName: REQUEST_OBSERVATION_EVENT_NAME,
      scope: REQUEST_OBSERVER_LOGGER_NAME,
      spanContext: undefined,
    },
    {
      attributes: {
        "url.full": `${location.origin}/observed`,
        "url.path": "/observed",
        "http.request.method": "POST",
        "http.response.status_code": 503,
        "interfere.request.kind": "fetch",
      },
      eventName: REQUEST_OBSERVATION_EVENT_NAME,
      scope: REQUEST_OBSERVER_LOGGER_NAME,
      spanContext: undefined,
    },
    {
      attributes: {
        "url.full": `${location.origin}/observed`,
        "url.path": "/observed",
        "http.request.method": "GET",
        "http.response.status_code": 200,
        "interfere.request.kind": "xhr",
      },
      eventName: REQUEST_OBSERVATION_EVENT_NAME,
      scope: REQUEST_OBSERVER_LOGGER_NAME,
      spanContext: undefined,
    },
  ]);
});

test("records network failures with the error type and rethrows unchanged", async () => {
  install();

  await expect(fetch(`${location.origin}/broken`)).rejects.toBeInstanceOf(
    TypeError,
  );

  await expect(sendXhr("GET", "/broken")).resolves.toBe(0);

  await settle();

  expect(records().map((record) => record.attributes)).toStrictEqual([
    {
      "url.full": `${location.origin}/broken`,
      "url.path": "/broken",
      "http.request.method": "GET",
      "error.type": "TypeError",
      "interfere.request.kind": "fetch",
    },
    {
      "url.full": `${location.origin}/broken`,
      "url.path": "/broken",
      "http.request.method": "GET",
      "error.type": "NetworkError",
      "interfere.request.kind": "xhr",
    },
  ]);
});

test("carries the host's active span context when one is valid", async () => {
  const spans = new InMemorySpanExporter();

  const provider = new BasicTracerProvider({
    spanProcessors: [new SimpleSpanProcessor(spans)],
  });

  context.setGlobalContextManager(new StackContextManager().enable());

  onTestFinished(async () => provider.shutdown());

  install();

  const span = provider.getTracer("host").startSpan("host-work");

  const withHostSpan = async <T>(start: () => Promise<T>) =>
    context.with(trace.setSpan(ROOT_CONTEXT, span), start);

  await withHostSpan(async () => fetch(`${location.origin}/observed`));

  await withHostSpan(async () => sendXhr("GET", "/observed"));

  await settle();

  span.end();

  expect(records().map((record) => record.spanContext)).toStrictEqual([
    span.spanContext(),
    span.spanContext(),
  ]);
});

test("disposal restores the native functions only while they are current and goes inert beneath a later wrapper", async () => {
  const first = install();

  first.dispose();

  expect(globalThis.fetch).toBe(nativeFetch);
  // oxlint-disable-next-line typescript/unbound-method -- Compares the installed prototype method with the saved native one by identity.
  expect(XMLHttpRequest.prototype.open).toBe(nativeOpen);
  // oxlint-disable-next-line typescript/unbound-method -- Compares the installed prototype method with the saved native one by identity.
  expect(XMLHttpRequest.prototype.send).toBe(nativeSend);

  const second = install();

  const beneath = globalThis.fetch;

  const foreign: typeof fetch = async (...args) => beneath(...args);

  // oxlint-disable-next-line interfere-testing/no-test-doubles -- A real passthrough wrapper models a later SDK; the request still reaches the network.
  globalThis.fetch = foreign;

  second.dispose();

  expect(globalThis.fetch).toBe(foreign);

  await fetch(`${location.origin}/observed`);

  await sendXhr("GET", "/observed");

  await settle();

  expect(records()).toStrictEqual([]);
});
