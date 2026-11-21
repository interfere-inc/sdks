import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";
import { HttpResponse, http } from "msw";
import { expect, onTestFinished, test } from "vitest";

import { worker } from "../../../../test/msw.js";
import { settledRequestSpans } from "../../../../test/request-spans.test-utils.js";
import { registerBundledInstrumentations } from "./instrumentations.js";

const RESOURCE_TIMING_EVENTS = new Set([
  "fetchStart",
  "domainLookupStart",
  "domainLookupEnd",
  "connectStart",
  "secureConnectionStart",
  "connectEnd",
  "requestStart",
  "responseStart",
  "responseEnd",
]);

async function getWithXhr(url: string) {
  await new Promise<void>((resolve, reject) => {
    const xhr = new XMLHttpRequest();

    xhr.addEventListener("loadend", () => resolve());

    xhr.addEventListener("error", () => reject(new Error("XHR failed")));

    xhr.open("GET", url);

    xhr.send();
  });
}

test.each(["fetch", "xhr"])(
  "exports %s request spans without resource-timing events",
  async (transport) => {
    const exporter = new InMemorySpanExporter();

    const provider = new TracerProvider({
      spanProcessors: [new SimpleSpanProcessor({ exporter })],
    });

    const handle = registerBundledInstrumentations({
      ignoreUrls: [],
      captureFailedRequests: false,
      tracerProvider: provider,
    });

    onTestFinished(async () => {
      handle.dispose();

      await provider.shutdown();
    });

    worker.use(
      http.get(`${location.origin}/api/ping`, () =>
        HttpResponse.json({ ok: true }),
      ),
    );

    const url = `${location.origin}/api/ping`;

    await (transport === "fetch" ? fetch(url) : getWithXhr(url));

    const [span] = await settledRequestSpans(exporter, 1);

    expect(span?.attributes["url.full"]).toBe(url);

    expect(
      span?.events
        .map((event) => event.name)
        .filter((name) => RESOURCE_TIMING_EVENTS.has(name)),
    ).toStrictEqual([]);
  },
);
