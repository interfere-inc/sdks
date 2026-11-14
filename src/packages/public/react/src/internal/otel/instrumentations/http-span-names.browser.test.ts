import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { WebTracerProvider } from "@opentelemetry/sdk-trace-web";
import { HttpResponse, http } from "msw";
import { expect, onTestFinished, test } from "vitest";

import { worker } from "../../../../test/msw.js";
import { requestSpans } from "../../../../test/request-spans.test-utils.js";
import { registerBundledInstrumentations } from "./instrumentations.js";

async function patchWithXhr(url: string) {
  await new Promise<void>((resolve, reject) => {
    const xhr = new XMLHttpRequest();

    xhr.addEventListener("loadend", () => resolve());

    xhr.addEventListener("error", () => reject(new Error("XHR failed")));

    xhr.open("PATCH", url);

    xhr.send();
  });
}

test.each([
  { transport: "fetch", template: undefined, route: "/api/items/{id}" },
  { transport: "xhr", template: undefined, route: "/api/items/{id}" },
  { transport: "fetch", template: "/api/items/[id]", route: "/api/items/[id]" },
  { transport: "xhr", template: "/api/items/[id]", route: "/api/items/[id]" },
])(
  "names $transport spans with route $route",
  async ({ transport, template, route }) => {
    expect.hasAssertions();

    const exporter = new InMemorySpanExporter();

    const provider = new WebTracerProvider({
      spanProcessors: [new SimpleSpanProcessor(exporter)],
    });

    const handle = registerBundledInstrumentations({
      ignoreUrls: [],
      captureFailedRequests: false,
      resolveRoute: () => template,
      tracerProvider: provider,
    });

    onTestFinished(async () => {
      handle.dispose();

      await provider.shutdown();
    });

    worker.use(
      http.patch(
        `${location.origin}/api/items/:id`,
        () => new HttpResponse(null, { status: 204 }),
      ),
    );

    for (const id of [
      "0af76519-16cd-43dd-8448-eb211c80319c",
      "1bf76519-16cd-43dd-8448-eb211c80319c",
    ]) {
      const path = `/api/items/${id}`;
      const url = `${location.origin}${path}?secret=value`;

      // oxlint-disable-next-line eslint/no-await-in-loop -- Each request must finish before the latest exported span is read.
      await (transport === "fetch"
        ? fetch(url, { method: "PATCH" })
        : patchWithXhr(url));

      // oxlint-disable-next-line eslint/no-await-in-loop -- The span ends once its outcome settles, after the response is returned.
      await expect
        .poll(() => requestSpans(exporter).at(-1))
        .toMatchObject({
          name: `PATCH ${route}`,
          attributes: {
            "http.route": route,
            "url.path": path,
            "url.full": url,
          },
        });
    }
  },
);
