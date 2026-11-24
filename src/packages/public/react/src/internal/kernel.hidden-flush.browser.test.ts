import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";

import { context, propagation, trace } from "@opentelemetry/api";

import { DISABLE_ALL_PLUGINS } from "../../test/fixtures/disable-all-plugins.js";
import { createKernel } from "./kernel.js";
import type { Kernel } from "./kernel.js";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

import { worker } from "../../test/msw.js";
import { decodedBodyText } from "../../test/fixtures/request-body.js";

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
  vi.useRealTimers();

  globalThis.dispatchEvent(
    new PageTransitionEvent("pageshow", { persisted: true }),
  );

  try {
    await ownedKernel?.dispose();
  } finally {
    trace.disable();

    context.disable();

    propagation.disable();
  }
});

describe("B09 hide-time flush", () => {
  test("hiding the page exports queued spans with keepalive before the batch delay (B09)", async () => {
    expect.hasAssertions();

    const exported = Promise.withResolvers<Request>();

    worker.use(
      http.get("https://kernel.invalid/v3/config", () =>
        HttpResponse.json({ plugins: DISABLE_ALL_PLUGINS }),
      ),
      http.post("https://kernel.invalid/drain/v1/traces", ({ request }) => {
        exported.resolve(request.clone());

        return new HttpResponse(null, { status: 204 });
      }),
      http.all(
        "https://kernel.invalid/*",
        () => new HttpResponse(null, { status: 204 }),
      ),
    );

    const kernel = await createKernel({
      opts: { apiHost: "https://kernel.invalid" },
    });

    ownedKernel = kernel;

    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });

    kernel.getTracer("b09").startSpan("before-hide").end();

    globalThis.dispatchEvent(
      new PageTransitionEvent("pagehide", { persisted: true }),
    );

    const request = await exported.promise;

    expect(request.keepalive).toBe(true);

    await expect(decodedBodyText(request)).resolves.toContain("before-hide");
  });
});
