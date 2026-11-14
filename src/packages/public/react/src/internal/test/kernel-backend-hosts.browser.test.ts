import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { context, propagation, trace } from "@opentelemetry/api";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, test } from "vitest";

import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { worker } from "../../../test/msw.js";
import { createKernel } from "../kernel.js";
import type { Kernel } from "../kernel.js";

let ownedKernel: Kernel | undefined;

beforeEach(() => {
  ownedKernel = undefined;

  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  setInjectedConfiguration("__INTERFERE_BACKEND_HOSTS__", undefined);

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

  setInjectedConfiguration("__INTERFERE_INGEST_URL__", undefined);
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

test.each([false, true])(
  "propagates to backend hosts with baked config=%s without explicit propagation targets",
  async (baked) => {
    expect.hasAssertions();

    if (baked) {
      setInjectedConfiguration("__INTERFERE_PLUGINS__", DISABLE_ALL_PLUGINS);

      setInjectedConfiguration("__INTERFERE_BACKEND_HOSTS__", [
        "backend.example.test",
      ]);
    }

    const headers = new Map<string, string | null>();
    const backendUrl = "https://backend.example.test/graphql";
    const thirdPartyUrl = "https://unrelated.example.test/resource";

    worker.use(
      http.get("https://kernel.invalid/v3/config", () =>
        HttpResponse.json({
          backendHosts: ["backend.example.test"],
          plugins: DISABLE_ALL_PLUGINS,
        }),
      ),
      http.all(
        "https://kernel.invalid/*",
        () => new HttpResponse(null, { status: 204 }),
      ),
      ...[backendUrl, thirdPartyUrl].map((url) =>
        http.get(url, ({ request }) => {
          headers.set(url, request.headers.get("traceparent"));

          return new HttpResponse(null, { status: 204 });
        }),
      ),
    );

    ownedKernel = await createKernel({
      opts: { apiHost: "https://kernel.invalid", consent: true },
    });

    await fetch(backendUrl);

    await fetch(thirdPartyUrl);

    expect(headers.get(backendUrl)).toMatch(
      /^00-[0-9a-f]{32}-[0-9a-f]{16}-01$/u,
    );

    expect(headers.get(thirdPartyUrl)).toBeNull();
  },
);
