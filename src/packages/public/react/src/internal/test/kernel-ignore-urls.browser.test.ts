import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";

import { context, propagation, trace } from "@opentelemetry/api";
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
  }
});

test("skips fetch spans for customer ignoreUrls", async () => {
  expect.hasAssertions();

  const spans = new InMemorySpanExporter();

  worker.use(
    http.get(`${location.origin}/ignored-test`, () => HttpResponse.json({})),
    http.get(`${location.origin}/tracked-test`, () => HttpResponse.json({})),
    http.get("https://kernel.invalid/v3/config", () =>
      HttpResponse.json({ plugins: DISABLE_ALL_PLUGINS }),
    ),
    http.all(
      "https://kernel.invalid/*",
      () => new HttpResponse(null, { status: 204 }),
    ),
  );

  ownedKernel = await createKernel({
    opts: {
      apiHost: "https://kernel.invalid",
      _internalAdditionalSpanProcessors: [new SimpleSpanProcessor(spans)],
      tracing: { ignoreUrls: [/\/ignored-test$/u] },
    },
  });

  await (await fetch(`${location.origin}/ignored-test`)).json();

  await (await fetch(`${location.origin}/tracked-test`)).json();

  const fetchedUrls = () =>
    spans.getFinishedSpans().map((span) => String(span.attributes["url.full"]));

  await vi.waitFor(() => {
    expect(fetchedUrls()).toContain(`${location.origin}/tracked-test`);
  });

  expect(fetchedUrls()).not.toContain(`${location.origin}/ignored-test`);
});
