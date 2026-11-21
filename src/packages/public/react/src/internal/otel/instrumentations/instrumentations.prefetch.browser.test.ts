import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";
import type { ReadableSpan } from "@opentelemetry/sdk-trace";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, test } from "vitest";

import { worker } from "../../../../test/msw.js";
import { settledRequestSpans } from "../../../../test/request-spans.test-utils.js";
import { registerBundledInstrumentations } from "./instrumentations.js";

const PAGE_URL = `${location.origin}/projects`;

const PREFETCH_HEADERS = { RSC: "1", "Next-Router-Prefetch": "1" };

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
    http.get(PAGE_URL, ({ request }) => {
      const mode = new URL(request.url).searchParams.get("mode");

      if (mode === "offline") {
        return HttpResponse.error();
      }

      return new HttpResponse(null, { status: 400 });
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

test("a router prefetch that fails in transport records its span without an exception", async () => {
  await expect(
    fetch(`${PAGE_URL}?mode=offline&_rsc=a1`, { headers: PREFETCH_HEADERS }),
  ).rejects.toThrow(TypeError);

  const spans = await settledSpans(1);

  expect(exceptionMessages(spans)).toStrictEqual([]);
  expect(spans[0]?.attributes["http.request.prefetch"]).toBe(true);
});

test("a router prefetch answered with an error status records its span without an exception", async () => {
  await fetch(`${PAGE_URL}?mode=reject&_rsc=b2`, {
    headers: PREFETCH_HEADERS,
  });

  const spans = await settledSpans(1);

  expect(exceptionMessages(spans)).toStrictEqual([]);
  expect(spans[0]?.attributes["http.request.prefetch"]).toBe(true);
});

test("the same navigation request without a prefetch header still reports its failure", async () => {
  await fetch(`${PAGE_URL}?mode=reject&_rsc=c3`, { headers: { RSC: "1" } });

  expect(exceptionMessages(await settledSpans(1))).toStrictEqual([
    `HTTP 400 GET ${PAGE_URL}?mode=reject&_rsc=c3`,
  ]);
});
