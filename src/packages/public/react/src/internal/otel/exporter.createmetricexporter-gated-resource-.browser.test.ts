/* oxlint-disable typescript/no-unsafe-assignment */
import type { ExportResult } from "@opentelemetry/core";
import { resourceFromAttributes } from "@opentelemetry/resources";
import type { ResourceMetrics } from "@opentelemetry/sdk-metrics";

import { createMetricExporter } from "./exporter.js";
import { assert, beforeEach, describe, expect, test, vi } from "vitest";
import { HttpResponse, http } from "msw";
import type { HttpResponseResolver } from "msw";

import { createCountSignal } from "../../../test/fixtures/upload-signal.js";
import { worker } from "../../../test/msw.js";

const requests: Request[] = [];

const requestSignal = createCountSignal(requests);

function resultCallback() {
  const called = Promise.withResolvers<true>();
  const spy = vi.fn<(result: ExportResult) => void>();

  const callback = (result: ExportResult) => {
    spy(result);

    called.resolve(true);
  };

  return { callback, spy: Object.assign(spy, { called: called.promise }) };
}

function collector(respond: HttpResponseResolver) {
  return http.post("https://in.example.com/drain/v1/:signal", async (info) => {
    requests.push(info.request.clone());

    requestSignal.notify();

    return respond(info);
  });
}

const CLIENT_OPTIONS = {
  baseUrl: "https://in.example.com",
  publicKeyHeader: { apiKey: "test-key" },
};

function makeResourceMetrics() {
  return {
    resource: resourceFromAttributes({ "service.name": "test" }),
    scopeMetrics: [],
  } satisfies ResourceMetrics;
}

async function exportMetrics(
  exporter: ReturnType<typeof createMetricExporter>,
) {
  const { callback, spy } = resultCallback();

  exporter.export(makeResourceMetrics(), callback);

  await spy.called;

  expect(spy).toHaveBeenCalledWith({ code: 0 });

  const [request] = requests.splice(0);

  assert.exists(request);

  return request;
}

describe("createMetricExporter gated resource attributes", () => {
  beforeEach(() => {
    requests.length = 0;

    worker.use(collector(() => new HttpResponse(null)));
  });

  test("merges the gated attrs into the payload resource while enabled", async () => {
    expect.hasAssertions();

    const exporter = createMetricExporter(
      { clientOptions: CLIENT_OPTIONS },
      { attributes: { "browser.platform": "macOS" }, isEnabled: () => true },
    );

    await expect((await exportMetrics(exporter)).json()).resolves.toMatchObject(
      {
        resourceMetrics: [
          {
            resource: {
              attributes: expect.arrayContaining([
                { key: "service.name", value: { stringValue: "test" } },
                { key: "browser.platform", value: { stringValue: "macOS" } },
              ]),
            },
          },
        ],
      },
    );
  });

  test("strips the gated attrs while disabled, re-reading the gate per export", async () => {
    expect.hasAssertions();

    let allowed = false;

    const exporter = createMetricExporter(
      { clientOptions: CLIENT_OPTIONS },
      {
        attributes: { "browser.platform": "macOS" },
        isEnabled: () => allowed,
      },
    );

    await expect((await exportMetrics(exporter)).json()).resolves.toMatchObject(
      {
        resourceMetrics: [
          {
            resource: {
              attributes: [
                { key: "service.name", value: { stringValue: "test" } },
              ],
            },
          },
        ],
      },
    );

    allowed = true;

    await expect((await exportMetrics(exporter)).json()).resolves.toMatchObject(
      {
        resourceMetrics: [
          {
            resource: {
              attributes: expect.arrayContaining([
                { key: "browser.platform", value: { stringValue: "macOS" } },
              ]),
            },
          },
        ],
      },
    );
  });

  test("leaves the payload untouched when no gate is supplied", async () => {
    expect.hasAssertions();

    const exporter = createMetricExporter({ clientOptions: CLIENT_OPTIONS });

    await expect((await exportMetrics(exporter)).json()).resolves.toMatchObject(
      {
        resourceMetrics: [
          {
            resource: {
              attributes: [
                { key: "service.name", value: { stringValue: "test" } },
              ],
            },
          },
        ],
      },
    );
  });
});
