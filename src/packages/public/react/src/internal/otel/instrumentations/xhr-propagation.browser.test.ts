import { WebTracerProvider } from "@opentelemetry/sdk-trace-web";
import { HttpResponse, http } from "msw";
import { expect, onTestFinished, test } from "vitest";

import { worker } from "../../../../test/msw.js";
import { w3cTraceHeaders } from "../../../../test/trace-headers.test-utils.js";
import { registerBundledInstrumentations } from "./instrumentations.js";

function install(options: { propagateTo?: string[] } = {}) {
  const provider = new WebTracerProvider();

  const handle = registerBundledInstrumentations({
    ...options,
    backendHosts: ["discovered.backend.test"],
    captureFailedRequests: false,
    ignoreUrls: [],
    traceHeaders: w3cTraceHeaders(),
    tracerProvider: provider,
  });

  onTestFinished(async () => {
    handle.dispose();

    await provider.shutdown();
  });
}

async function carriesTraceparent(transport: "fetch" | "xhr", url: string) {
  const seen = Promise.withResolvers<string | null>();

  worker.use(
    http.get(url, ({ request }) => {
      seen.resolve(request.headers.get("traceparent"));

      return new HttpResponse(null, { status: 204 });
    }),
  );

  if (transport === "fetch") {
    await fetch(url);
  } else {
    await new Promise<void>((resolve) => {
      const xhr = new XMLHttpRequest();

      xhr.addEventListener("loadend", () => resolve());

      xhr.open("GET", url);

      xhr.send();
    });
  }

  return (await seen.promise) !== null;
}

const sibling = new URL(globalThis.location.origin);

sibling.port = sibling.port === "8444" ? "8445" : "8444";

test.each(["fetch", "xhr"] as const)(
  "%s: headers reach exact page, configured and explicit origins only, never same-site siblings",
  async (transport) => {
    expect.hasAssertions();

    install({
      propagateTo: [
        "https://explicit.backend.test",
        "https://custom.backend.test",
      ],
    });

    for (const [url, expected] of [
      [`${sibling.origin}/orders`, false],
      [`${globalThis.location.origin}/orders`, true],
      ["https://discovered.backend.test/orders", true],
      ["https://explicit.backend.test/orders", true],
      ["https://custom.backend.test/orders", true],
      ["https://unrelated.test/orders", false],
      ["https://api.stripe.com/v1/charges", false],
    ] as const) {
      // oxlint-disable-next-line eslint/no-await-in-loop -- Each URL's handler is registered and observed before the next request.
      await expect(carriesTraceparent(transport, url)).resolves.toBe(expected);
    }
  },
);
