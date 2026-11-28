import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, test } from "vitest";

import { worker } from "../../../../../test/msw.js";
import { settledRequestSpans } from "../../../../../test/request-spans.test-utils.js";
import { transportContextAttrs } from "../../../capture/transport-context.js";
import { registerBundledInstrumentations } from "../instrumentations.js";

const OFFLINE_URL = `${location.origin}/api/search?q=offline`;

let exporter: InMemorySpanExporter;
let provider: TracerProvider;
let dispose: () => void;

beforeEach(() => {
  exporter = new InMemorySpanExporter();

  provider = new TracerProvider({
    spanProcessors: [new SimpleSpanProcessor({ exporter })],
  });

  ({ dispose } = registerBundledInstrumentations({
    ignoreUrls: [],
    tracerProvider: provider,
  }));

  worker.use(
    http.get(`${location.origin}/api/search`, () => HttpResponse.error()),
  );
});

afterEach(async () => {
  dispose();

  await provider.shutdown();
});

async function exceptionAttributes() {
  const [span] = await settledRequestSpans(exporter, 1);

  return {
    event: span?.events.find((event) => event.name === "exception")?.attributes,
    span: span?.attributes,
  };
}

test("unannotated transport failures retain their callsite and load outcome without cancellation attribution", async () => {
  await expect(fetch(OFFLINE_URL)).rejects.toThrow(TypeError);

  const { event, span } = await exceptionAttributes();

  const message = `Network network GET ${OFFLINE_URL}`;
  const stack = event?.["exception.stacktrace"];
  const duration = event?.["interfere.load.duration_ms"];

  expect(stack).toContain("network-failure.browser.test.ts");

  expect(duration).toBeTypeOf("number");

  expect(event).toStrictEqual({
    "exception.stacktrace": stack,
    "http.request.next_action": false,
    "exception.message": message,
    "exception.type": "NetworkError",
    "interfere.exception.chain": JSON.stringify([
      {
        kind: "error",
        mechanism: {
          handled: false,
          type: MECHANISM_TYPE.browser.fetchNetworkError,
        },
        stack,
        type: "NetworkError",
        value: message,
      },
      { kind: "error", stack: "", type: "TypeError", value: "Failed to fetch" },
    ]),
    "interfere.exception.handled": false,
    "interfere.exception.kind": "error",
    "interfere.exception.mechanism": MECHANISM_TYPE.browser.fetchNetworkError,
    "interfere.exception.subtype": "network",
    "interfere.load.duration_ms": duration,
    "interfere.load.failure": "no_status",
    "interfere.load.transfer_size": 0,
    ...transportContextAttrs(),
  });

  expect(span?.["interfere.load.failure"]).toBe("no_status");

  expect(span?.["interfere.http.capture_status"]).toBeUndefined();
});

test("an XHR transport failure records the browser's load outcome", async () => {
  const xhr = new XMLHttpRequest();

  const settled = new Promise<void>((resolve) => {
    xhr.addEventListener("loadend", () => resolve(), { once: true });
  });

  xhr.open("GET", OFFLINE_URL);

  xhr.send();

  await settled;

  const { event } = await exceptionAttributes();

  expect(event).toMatchObject({
    "exception.message": `Network network XHR ${OFFLINE_URL}`,
    "interfere.exception.mechanism": MECHANISM_TYPE.browser.xhrNetworkError,
    "interfere.load.failure": "no_status",
    "interfere.load.transfer_size": 0,
  });
});
