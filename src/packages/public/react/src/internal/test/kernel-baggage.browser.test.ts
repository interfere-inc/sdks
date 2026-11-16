import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";

import {
  ROOT_CONTEXT,
  context,
  defaultTextMapGetter,
  defaultTextMapSetter,
  propagation,
  trace,
} from "@opentelemetry/api";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";

import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { createKernel } from "../kernel.js";
import type { Kernel } from "../kernel.js";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

import { worker } from "../../../test/msw.js";

let ownedKernel: Kernel | undefined;

beforeEach(() => {
  ownedKernel = undefined;

  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

  setInjectedConfiguration("__INTERFERE_INGEST_URL__", undefined);

  trace.disable();

  context.disable();

  propagation.disable();
});

afterEach(async () => {
  try {
    await ownedKernel?.dispose();
  } finally {
    trace.disable();

    context.disable();

    propagation.disable();

    vi.unstubAllGlobals();
  }
});

async function sendXhr(url: string) {
  return new Promise<void>((resolve, reject) => {
    const xhr = new XMLHttpRequest();

    xhr.open("GET", url);

    xhr.addEventListener("load", () => resolve(), { once: true });

    xhr.addEventListener("error", () => reject(new Error("XHR failed")), {
      once: true,
    });

    xhr.send();
  });
}

test.each([{ extracted: undefined }] as const)(
  "baggage opt-out keeps browser session attributes and trace context without registering a propagator",
  async ({ extracted }) => {
    expect.hasAssertions();

    const spans = new InMemorySpanExporter();
    const requests: Request[] = [];
    const url = `${location.origin}/baggage-disabled`;

    worker.use(
      http.get("https://kernel.invalid/v3/config", () =>
        HttpResponse.json({ plugins: DISABLE_ALL_PLUGINS }),
      ),
      http.all(
        "https://kernel.invalid/*",
        () => new HttpResponse(null, { status: 204 }),
      ),
      http.get(url, ({ request }) => {
        requests.push(request);

        return new HttpResponse(null, { status: 204 });
      }),
    );

    const kernel = await createKernel({
      onCreated: (created) => {
        created.consent.set(true);
      },
      opts: {
        apiHost: "https://kernel.invalid",
        _internalAdditionalSpanProcessors: [new SimpleSpanProcessor(spans)],
        tracing: { propagateTraceBaggage: false },
      },
    });

    ownedKernel = kernel;

    const inbound = propagation.extract(
      ROOT_CONTEXT,
      { baggage: "vendor=untouched" },
      defaultTextMapGetter,
    );

    expect(propagation.getBaggage(inbound)?.getEntry("vendor")?.value).toBe(
      extracted,
    );

    const existingBaggage = propagation.createBaggage({
      vendor: { value: "untouched" },
    });

    const parent = propagation.setBaggage(context.active(), existingBaggage);
    const carrier = { baggage: "caller=untouched" };

    propagation.inject(parent, carrier, defaultTextMapSetter);

    expect(carrier.baggage).toBe("caller=untouched");

    expect(propagation.getBaggage(parent)?.getEntry("vendor")?.value).toBe(
      "untouched",
    );

    const span = kernel
      .getTracer("baggage-disabled")
      .startSpan("browser-action");

    await context.with(trace.setSpan(context.active(), span), async () => {
      await fetch(url);

      await sendXhr(url);
    });

    span.end();

    await kernel.flush();

    expect(requests).toHaveLength(2);

    for (const request of requests) {
      expect(request.headers.get("baggage")).toBeNull();

      expect(request.headers.get("traceparent")).toMatch(
        /^00-[0-9a-f]{32}-[0-9a-f]{16}-01$/u,
      );
    }

    const finished = spans
      .getFinishedSpans()
      .find((record) => record.name === "browser-action");

    expect(finished?.attributes["session.id"]).toBe(kernel.session.getId());
  },
);
