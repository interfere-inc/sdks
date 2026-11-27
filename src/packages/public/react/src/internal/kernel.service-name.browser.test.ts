import { getStringGlobal } from "../util/global.js";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { omitUndefined } from "@interfere/helpers/omit-undefined";

import { context, propagation, trace } from "@opentelemetry/api";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace";

import { DISABLE_ALL_PLUGINS } from "../../test/fixtures/disable-all-plugins.js";
import { createKernel } from "./kernel.js";
import type { Kernel } from "./kernel.js";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

import { worker } from "../../test/msw.js";

let ownedKernel: Kernel | undefined;

beforeEach(() => {
  ownedKernel = undefined;

  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", "interfere_pk_test");

  setInjectedConfiguration("__INTERFERE_INGEST_URL__", undefined);

  setInjectedConfiguration("__INTERFERE_SERVICE_NAME__", undefined);

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

async function resourceServiceName(serviceName: string | undefined) {
  const spans = new InMemorySpanExporter();
  const requests: Request[] = [];

  worker.use(
    http.get("https://kernel.invalid/v3/config", ({ request }) => {
      requests.push(request);

      return HttpResponse.json({ plugins: DISABLE_ALL_PLUGINS });
    }),
    http.post("https://kernel.invalid/v3/session", ({ request }) => {
      requests.push(request);

      return HttpResponse.json({});
    }),
    http.all(
      "https://kernel.invalid/*",
      () => new HttpResponse(null, { status: 204 }),
    ),
  );

  const kernel = await createKernel({
    opts: omitUndefined({
      apiHost: "https://kernel.invalid",
      _internalAdditionalSpanProcessors: [
        new SimpleSpanProcessor({ exporter: spans }),
      ],
      serviceName,
      serviceNamespace: "shop",
    }),
  });

  ownedKernel = kernel;

  kernel.getTracer("service-name-test").startSpan("service-name").end();

  await kernel.flush();

  const span = spans
    .getFinishedSpans()
    .find((record) => record.name === "service-name");

  await vi.waitFor(() =>
    expect(
      requests.some(
        (request) => new URL(request.url).pathname === "/v3/session",
      ),
    ).toBe(true),
  );

  const expectedName =
    serviceName || getStringGlobal("__INTERFERE_SERVICE_NAME__") || null;

  for (const request of requests) {
    expect(request.headers.get("x-interfere-service")).toBe(expectedName);

    expect(request.headers.get("x-interfere-service-namespace")).toBe(
      expectedName ? "shop" : null,
    );
  }

  return span?.resource.attributes["service.name"];
}

test("uses the service name the build stamped into the page when init passes none", async () => {
  setInjectedConfiguration("__INTERFERE_SERVICE_NAME__", "storefront-build");

  await expect(resourceServiceName(undefined)).resolves.toBe(
    "storefront-build",
  );
});

test("lets an explicit serviceName win over the stamped default", async () => {
  setInjectedConfiguration("__INTERFERE_SERVICE_NAME__", "storefront-build");

  await expect(resourceServiceName("storefront")).resolves.toBe("storefront");
});

test("falls back to unknown_service when nothing names the service", async () => {
  await expect(resourceServiceName(undefined)).resolves.toBe("unknown_service");
});
