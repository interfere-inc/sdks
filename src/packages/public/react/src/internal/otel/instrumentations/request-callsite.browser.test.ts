import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { WebTracerProvider } from "@opentelemetry/sdk-trace-web";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, test } from "vitest";

import { worker } from "../../../../test/msw.js";
import { settledRequestSpans } from "../../../../test/request-spans.test-utils.js";
import { failedSpan, response } from "./failed-response.fixture.js";
import { registerBundledInstrumentations } from "./instrumentations.js";
import type { BundledInstrumentations } from "./instrumentations.js";

let exporter: InMemorySpanExporter;
let provider: WebTracerProvider;
let handle: BundledInstrumentations;

const base = `${globalThis.location.origin}/request-callsite`;

beforeEach(() => {
  exporter = new InMemorySpanExporter();

  provider = new WebTracerProvider({
    spanProcessors: [new SimpleSpanProcessor(exporter)],
  });

  handle = registerBundledInstrumentations({
    ignoreUrls: [],
    tracerProvider: provider,
    redactUrl: (url) => `${new URL(url).origin}/[redacted]`,
  });
});

afterEach(async () => {
  handle.dispose();

  await provider.shutdown();
});

async function customerFetchRequest(url: string) {
  return fetch(url, { headers: { "Next-Action": "private-action-id" } });
}

async function customerXhrRequest(url: string) {
  const xhr = new XMLHttpRequest();

  const settled = new Promise<XMLHttpRequest>((resolve, reject) => {
    xhr.addEventListener("load", () => resolve(xhr));

    xhr.addEventListener("error", () => reject(new TypeError("XHR failed")));
  });

  xhr.open("GET", url);

  xhr.setRequestHeader("Next-Action", "private-action-id");

  xhr.send();

  return settled;
}

const SDK_FRAMES = [
  "observedFetch",
  "tracedFetch",
  "observedSend",
  "captureRequestEvidence",
  "onStart",
];

test.each([
  ["fetch", customerFetchRequest, "customerFetchRequest"],
  ["xhr", customerXhrRequest, "customerXhrRequest"],
] as const)(
  "%s failed responses record a stack whose first frame is the application caller",
  async (kind, request, name) => {
    expect.hasAssertions();

    worker.use(
      http.get(`${base}/first-frame-${kind}`, () =>
        response('{"code":"FAILED"}'),
      ),
    );

    await request(`${base}/first-frame-${kind}`);

    const span = await failedSpan(exporter);

    const event = span.events.find((item) => item.name === "exception");

    const frames = String(event?.attributes?.["exception.stacktrace"]).split(
      "\n",
    );

    expect(event?.attributes?.["interfere.exception.mechanism"]).toBe(
      `auto.browser.${kind}.failed_response`,
    );

    expect(frames[0]).toContain(name);
    expect(frames[0]).not.toContain("HTTP request call site");

    for (const sdkFrame of SDK_FRAMES) {
      expect(frames.join("\n")).not.toContain(sdkFrame);
    }
  },
);

test("xHR failed responses retain the initiating callsite and only header presence", async () => {
  expect.hasAssertions();

  worker.use(http.get(`${base}/response`, () => response('{"code":"FAILED"}')));

  const xhr = await customerXhrRequest(`${base}/response`);

  expect(xhr.status).toBe(422);
  expect(xhr.responseText).toBe('{"code":"FAILED"}');

  const span = await failedSpan(exporter);

  const event = span.events.find((item) => item.name === "exception");

  expect(event?.attributes).toMatchObject({
    "http.request.next_action": true,
    "interfere.exception.mechanism": "auto.browser.xhr.failed_response",
  });

  expect(event?.attributes?.["exception.stacktrace"]).toContain(
    "customerXhrRequest",
  );

  expect(JSON.stringify(span.events)).not.toContain("private-action-id");
});

test.each([
  ["fetch", customerFetchRequest, "customerFetchRequest"],
  ["xhr", customerXhrRequest, "customerXhrRequest"],
] as const)(
  "%s transport failures retain the initiating stack in the event and chain",
  async (kind, request, name) => {
    expect.hasAssertions();

    worker.use(http.get(`${base}/${kind}`, () => HttpResponse.error()));

    await expect(request(`${base}/${kind}`)).rejects.toBeInstanceOf(TypeError);

    const [span] = await settledRequestSpans(exporter, 1);

    const event = span?.events.find((item) => item.name === "exception");
    const stack = String(event?.attributes?.["exception.stacktrace"]);

    expect(event?.attributes).toMatchObject({
      "http.request.next_action": true,
      "exception.type": "NetworkError",
      "interfere.exception.mechanism": `auto.browser.${kind}.network_error`,
    });

    expect(stack.split("\n")[0]).toContain(name);
    expect(stack).toContain("/[redacted]");
    expect(stack).not.toContain("request-callsite.browser.test.ts");
    expect(stack.length).toBeLessThanOrEqual(8192);
    expect(stack.split("\n").length).toBeLessThanOrEqual(20);

    expect(
      JSON.parse(String(event?.attributes?.["interfere.exception.chain"])),
    ).toStrictEqual(
      expect.arrayContaining([
        expect.objectContaining({ type: "NetworkError", stack }),
      ]),
    );

    expect(JSON.stringify(span?.events)).not.toContain("private-action-id");
  },
);
