import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import type { ReadableSpan } from "@opentelemetry/sdk-trace-base";
import { WebTracerProvider } from "@opentelemetry/sdk-trace-web";
import { HttpResponse, delay, http } from "msw";
import { afterEach, beforeEach, expect, test } from "vitest";

import { worker } from "../../../../test/msw.js";
import { settledRequestSpans } from "../../../../test/request-spans.test-utils.js";
import { requestCancellation, requestRecovered } from "../../../api.js";
import { registerBundledInstrumentations } from "./instrumentations.js";

const SEARCH_URL = `${location.origin}/api/search`;

let exporter: InMemorySpanExporter;
let provider: WebTracerProvider;
let dispose: () => void;

beforeEach(() => {
  exporter = new InMemorySpanExporter();

  provider = new WebTracerProvider({
    spanProcessors: [new SimpleSpanProcessor(exporter)],
  });

  ({ dispose } = registerBundledInstrumentations({
    ignoreUrls: [],
    tracerProvider: provider,
  }));

  worker.use(
    http.get(SEARCH_URL, async ({ request }) => {
      const query = new URL(request.url).searchParams.get("q");

      if (query === "offline") {
        return HttpResponse.error();
      }

      await delay(query === "slow" ? 500 : 0);

      return HttpResponse.json({ query });
    }),
  );
});

afterEach(async () => {
  dispose();

  await provider.shutdown();
});

async function settledSpans(count: number) {
  return settledRequestSpans(exporter, count);
}

function exceptionMessages(spans: ReadableSpan[]) {
  return spans.flatMap((span) =>
    span.events
      .filter((event) => event.name === "exception")
      .map((event) => event.attributes?.["exception.message"]),
  );
}

function cancellations(spans: ReadableSpan[]) {
  return spans.flatMap((span) => {
    const value = span.attributes["interfere.http.cancelled"];

    return value === undefined ? [] : [value];
  });
}

function openXhr(query: string) {
  const xhr = new XMLHttpRequest();

  xhr.open("GET", `${SEARCH_URL}?q=${query}`);

  const { promise: settled, resolve } = Promise.withResolvers<string>();

  for (const type of ["load", "abort", "timeout", "error"] as const) {
    xhr.addEventListener(type, () => resolve(type), { once: true });
  }

  return { settled, xhr };
}

test("typeahead aborting a superseded XHR reports no exception (DataDog/browser-sdk#362)", async () => {
  const first = openXhr("slow");

  first.xhr.send();

  const second = openXhr("sl");

  first.xhr.abort();

  second.xhr.send();

  await expect(first.settled).resolves.toBe("abort");

  await expect(second.settled).resolves.toBe("load");

  const spans = await settledSpans(2);

  expect(exceptionMessages(spans)).toStrictEqual([]);
  expect(cancellations(spans)).toStrictEqual(["abort"]);
});

test("an XHR timeout expiry reports no exception", async () => {
  const request = openXhr("slow");

  request.xhr.timeout = 20;

  request.xhr.send();

  await expect(request.settled).resolves.toBe("timeout");

  const spans = await settledSpans(1);

  expect(exceptionMessages(spans)).toStrictEqual([]);
  expect(cancellations(spans)).toStrictEqual(["timeout"]);
});

test("fetch aborted by its AbortController reports no exception", async () => {
  const controller = new AbortController();
  const pending = fetch(`${SEARCH_URL}?q=slow`, { signal: controller.signal });

  controller.abort();

  await expect(pending).rejects.toHaveProperty("name", "AbortError");

  const spans = await settledSpans(1);

  expect(exceptionMessages(spans)).toStrictEqual([]);
  expect(cancellations(spans)).toStrictEqual(["abort"]);
});

test("fetch aborted with a non-Error reason reports no exception", async () => {
  const controller = new AbortController();

  const pending = fetch(
    new Request(`${SEARCH_URL}?q=slow`, { signal: controller.signal }),
  );

  controller.abort("superseded");

  await expect(pending).rejects.toBe("superseded");

  const spans = await settledSpans(1);

  expect(exceptionMessages(spans)).toStrictEqual([]);
  expect(cancellations(spans)).toStrictEqual(["abort"]);
});

test("fetch past its AbortSignal.timeout deadline reports no exception", async () => {
  const pending = fetch(`${SEARCH_URL}?q=slow`, {
    signal: AbortSignal.timeout(20),
  });

  await expect(pending).rejects.toHaveProperty("name", "TimeoutError");

  const spans = await settledSpans(1);

  expect(exceptionMessages(spans)).toStrictEqual([]);
  expect(cancellations(spans)).toStrictEqual(["timeout"]);
});

test("fetch transport failure still reports one NetworkError with the browser message despite a long task during the hold", async () => {
  await expect(fetch(`${SEARCH_URL}?q=offline`)).rejects.toThrow(
    new TypeError("Failed to fetch"),
  );

  const stallStart = performance.now();

  while (performance.now() - stallStart < 120) {
    continue;
  }

  const spans = await settledSpans(1);

  expect(exporter.getFinishedSpans().map((span) => span.name)).toContain(
    "longtask",
  );

  expect(exceptionMessages(spans)).toStrictEqual([
    `Network network GET ${SEARCH_URL}?q=offline`,
  ]);

  expect(
    JSON.parse(
      String(
        spans[0]?.events.at(-1)?.attributes?.["interfere.exception.chain"],
      ),
    ),
  ).toContainEqual({
    kind: "error",
    stack: "",
    type: "TypeError",
    value: "Failed to fetch",
  });

  expect(cancellations(spans)).toStrictEqual([]);
});

test("unannotated fetch transport failure followed by pagehide keeps unload evidence on its span", async () => {
  await expect(fetch(`${SEARCH_URL}?q=offline`)).rejects.toThrow(TypeError);

  globalThis.dispatchEvent(
    new PageTransitionEvent("pagehide", { persisted: true }),
  );

  const spans = await settledSpans(1);

  expect(exceptionMessages(spans)).toStrictEqual([]);

  expect(spans[0]?.attributes["interfere.http.cancellation_initiator"]).toBe(
    "unload",
  );
});

test("an XHR transport failure still reports one NetworkError", async () => {
  const request = openXhr("offline");

  request.xhr.send();

  await expect(request.settled).resolves.toBe("error");

  const spans = await settledSpans(1);

  expect(exceptionMessages(spans)).toStrictEqual([
    `Network network XHR ${SEARCH_URL}?q=offline`,
  ]);

  expect(cancellations(spans)).toStrictEqual([]);
});

test("a failed HTTP status is reported unchanged", async () => {
  worker.use(
    http.get(SEARCH_URL, () => new HttpResponse(null, { status: 503 })),
  );

  await fetch(`${SEARCH_URL}?q=down`);

  expect(exceptionMessages(await settledSpans(1))).toStrictEqual([
    `HTTP 503 GET ${SEARCH_URL}?q=down`,
  ]);
});

test.each([
  ["user", "abort"],
  ["navigation", "abort"],
  ["unload", "abort"],
  ["timeout", "timeout"],
] as const)(
  "explicit %s cancellation reaches the exception with request-scoped evidence",
  async (initiator, cancellation) => {
    const controller = new AbortController();

    const pending = fetch(`${SEARCH_URL}?q=slow`, {
      signal: controller.signal,
    });

    const reason = requestCancellation(initiator);

    controller.abort(reason);

    await expect(pending).rejects.toBe(reason);

    const spans = await settledSpans(1);

    const attributes = spans[0]?.events.find(
      (event) => event.name === "exception",
    )?.attributes;

    expect(attributes).toMatchObject({
      "interfere.http.cancellation_initiator": initiator,
    });

    expect(attributes?.["interfere.http.request_outcome"]).toBeUndefined();

    expect(attributes?.["exception.message"]).toBe(
      `Network ${cancellation} GET ${SEARCH_URL}?q=slow`,
    );
  },
);

test("reports recovery only after an observed cancellation and a successful retry", async () => {
  const reason = requestCancellation("user");

  expect(requestRecovered(reason)).toBe(false);

  const controller = new AbortController();
  const pending = fetch(`${SEARCH_URL}?q=slow`, { signal: controller.signal });

  controller.abort(reason);

  await expect(pending).rejects.toBe(reason);

  const cancelled = await settledSpans(1);

  expect(
    cancelled[0]?.events[0]?.attributes?.["interfere.http.request_outcome"],
  ).toBeUndefined();

  await expect(
    (await fetch(`${SEARCH_URL}?q=retry`)).json(),
  ).resolves.toStrictEqual({ query: "retry" });

  expect(requestRecovered(reason)).toBe(true);
  expect(requestRecovered(reason)).toBe(false);

  const spans = await settledSpans(3);

  const recovery = spans
    .flatMap((span) => span.events)
    .filter(
      (event) =>
        event.name === "exception" &&
        event.attributes?.["interfere.http.request_outcome"] === "recovered",
    );

  expect(recovery).toHaveLength(1);

  expect(
    recovery[0]?.attributes?.["interfere.http.cancellation_initiator"],
  ).toBe("user");
});

test("unannotated transport failures retain their callsite without cancellation attribution", async () => {
  await expect(fetch(`${SEARCH_URL}?q=offline`)).rejects.toThrow(TypeError);

  const spans = await settledSpans(1);

  const message = `Network network GET ${SEARCH_URL}?q=offline`;

  const attributes = spans[0]?.events.find(
    (event) => event.name === "exception",
  )?.attributes;

  const stack = attributes?.["exception.stacktrace"];

  expect(stack).toContain("instrumentations.cancellation.browser.test.ts");

  expect(attributes).toStrictEqual({
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
  });

  expect(spans[0]?.attributes["interfere.http.capture_status"]).toBeUndefined();
});

test("unannotated XHR transport failure followed by pagehide emits no exception", async () => {
  const request = openXhr("offline");

  request.xhr.send();

  // WARNING: Register after send so OTel timestamps the failure before pagehide; the earlier promise listener can resume between browser event listeners.
  request.xhr.addEventListener(
    "error",
    () => {
      globalThis.dispatchEvent(
        new PageTransitionEvent("pagehide", { persisted: true }),
      );
    },
    { once: true },
  );

  await expect(request.settled).resolves.toBe("error");

  const spans = await settledSpans(1);

  expect(exceptionMessages(spans)).toStrictEqual([]);

  expect(spans[0]?.attributes["interfere.http.cancellation_initiator"]).toBe(
    "unload",
  );
});
