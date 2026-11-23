import { isString } from "@interfere/types/sdk/guards/json";
import { FetchInstrumentation } from "@opentelemetry/browser-instrumentation/experimental/fetch";
import { XhrInstrumentation } from "@opentelemetry/browser-instrumentation/experimental/xhr";
import {
  InMemorySpanExporter,
  NoopSpanProcessor,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";
import type { Span } from "@opentelemetry/sdk-trace";
import { HttpResponse, http } from "msw";
import { expect, onTestFinished, test } from "vitest";

import { worker } from "../../../../test/msw.js";
import { requestSpans } from "../../../../test/request-spans.test-utils.js";
import { w3cTraceHeaders } from "../../../../test/trace-headers.test-utils.js";
import { registerBundledInstrumentations } from "./instrumentations.js";
import { applyXhrSpanAttrs } from "./request-outcome.js";

const URL_ECHO = `${location.origin}/upstream-contract`;

function provider() {
  const exporter = new InMemorySpanExporter();

  const tracerProvider = new TracerProvider({
    spanProcessors: [new SimpleSpanProcessor({ exporter })],
  });

  onTestFinished(async () => tracerProvider.shutdown());

  return { exporter, tracerProvider };
}

async function both(url: string) {
  await (await fetch(url)).text();

  await new Promise<void>((resolve) => {
    const xhr = new XMLHttpRequest();

    xhr.addEventListener("loadend", () => resolve(), { once: true });

    xhr.open("GET", url);

    xhr.send();
  });
}

function methods() {
  return [
    globalThis.fetch,
    // oxlint-disable-next-line typescript/unbound-method -- Compared by identity across installation and disposal.
    XMLHttpRequest.prototype.open,
    // oxlint-disable-next-line typescript/unbound-method -- Compared by identity across installation and disposal.
    XMLHttpRequest.prototype.send,
  ] as const;
}

test.each(["before", "after"] as const)(
  "another upstream SDK installed %s Interfere survives disposal and reinstall",
  async (order) => {
    const [fetchBefore, openBefore, sendBefore] = methods();

    onTestFinished(() => {
      // oxlint-disable-next-line landmines/no-test-doubles -- The foreign instrumentation leaves its wrappers installed when disabled.
      globalThis.fetch = fetchBefore;

      XMLHttpRequest.prototype.open = openBefore;
      XMLHttpRequest.prototype.send = sendBefore;
    });

    const mine = provider();
    const foreign = provider();
    const instruments = [new FetchInstrumentation(), new XhrInstrumentation()];

    let installedForeign = methods();

    const startForeign = () => {
      for (const instrument of instruments) {
        instrument.enable({ tracerProvider: foreign.tracerProvider });
      }

      installedForeign = methods();
    };

    onTestFinished(() => {
      for (const instrument of instruments) {
        instrument.disable();
      }
    });

    const install = () =>
      registerBundledInstrumentations({
        ignoreUrls: [],
        tracerProvider: mine.tracerProvider,
        traceHeaders: w3cTraceHeaders(),
      });

    worker.use(
      http.get(URL_ECHO, () => new HttpResponse(null, { status: 204 })),
    );

    if (order === "before") {
      startForeign();
    }

    let ours = install();

    onTestFinished(() => ours.dispose());

    if (order === "after") {
      startForeign();
    }

    await both(URL_ECHO);

    ours.dispose();

    const foreignOnly = methods();

    expect(foreignOnly).toStrictEqual(installedForeign);

    await both(URL_ECHO);

    ours = install();

    await both(URL_ECHO);

    ours.dispose();

    expect(methods()).toStrictEqual(foreignOnly);

    await both(URL_ECHO);

    await expect
      .poll(() => [
        requestSpans(mine.exporter).length,
        requestSpans(foreign.exporter).length,
      ])
      .toStrictEqual([4, 8]);
  },
);

test("sanitizeUrl removes query values before either request span reaches a processor and preserves the URL on the wire", async () => {
  const wire: string[] = [];
  const started: string[] = [];
  const url = `${URL_ECHO}?token=customer-value`;

  worker.use(
    http.get(URL_ECHO, ({ request }) => {
      wire.push(request.url);

      return new HttpResponse(null, { status: 204 });
    }),
  );

  const tracerProvider = new TracerProvider({
    spanProcessors: [
      new (class extends NoopSpanProcessor {
        override onStart(span: Span) {
          const value = span.attributes["url.full"];

          if (isString(value)) {
            started.push(value);
          }
        }
      })(),
    ],
  });

  const ours = registerBundledInstrumentations({
    tracerProvider,
    ignoreUrls: [],
    redactUrl: (value) => value.replace("customer-value", "REDACTED"),
  });

  onTestFinished(async () => {
    ours.dispose();

    await tracerProvider.shutdown();
  });

  await both(url);

  expect(wire).toStrictEqual([url, url]);

  expect(started.filter((value) => value.startsWith(URL_ECHO))).toStrictEqual([
    `${URL_ECHO}?token=REDACTED`,
    `${URL_ECHO}?token=REDACTED`,
  ]);
});

test.each([
  { attributes: { "http.request.method": "PATCH" }, method: "PATCH" },
  { attributes: { "http.method": "DELETE" }, method: "DELETE" },
  { attributes: { "http.request.method": 7 }, method: "XHR" },
])(
  "uses $method for an XHR span with $attributes",
  async ({ attributes, method }) => {
    const { tracerProvider, exporter } = provider();

    const span = tracerProvider
      .getTracer("xhr-contract")
      .startSpan("request", { attributes });

    const xhr = new XMLHttpRequest();

    Object.defineProperty(xhr, "responseURL", { value: URL_ECHO });

    await applyXhrSpanAttrs(
      span,
      xhr,
      null,
      { dispose: () => undefined, leftSince: () => false },
      () => true,
      undefined,
    );

    span.end();

    expect(exporter.getFinishedSpans().at(-1)?.name).toBe(
      `${method} /upstream-contract`,
    );
  },
);
