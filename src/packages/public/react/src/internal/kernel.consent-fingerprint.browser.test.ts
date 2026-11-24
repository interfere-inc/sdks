import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { context, propagation, trace } from "@opentelemetry/api";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, test } from "vitest";

import { DISABLE_ALL_PLUGINS } from "../../test/fixtures/disable-all-plugins.js";
import { worker } from "../../test/msw.js";
import { createKernel } from "./kernel.js";
import type { Kernel } from "./kernel.js";

const API = "https://kernel-consent-fingerprint.invalid";

let ownedKernel: Kernel | undefined;

async function idle() {
  await new Promise((resolve) => {
    requestIdleCallback(resolve, { timeout: 5000 });
  });
}

beforeEach(() => {
  ownedKernel = undefined;

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

  setInjectedConfiguration("__INTERFERE_PLUGINS__", {
    ...DISABLE_ALL_PLUGINS,
    device: true,
  });

  worker.use(
    http.get(`${API}/v3/config`, () => HttpResponse.json({ plugins: {} })),
    http.all(`${API}/*`, () => new HttpResponse(null, { status: 204 })),
  );
});

afterEach(async () => {
  try {
    await ownedKernel?.dispose();
  } finally {
    trace.disable();

    context.disable();

    propagation.disable();

    setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);
  }
});

test("a required-consent app stores no fingerprint hash until consent is granted", async () => {
  const kernel = await createKernel({
    opts: { apiHost: API, consent: "required" },
  });

  ownedKernel = kernel;

  kernel.syncProviderConfig({});

  await idle();

  expect(kernel.device.getFpHash()).toBeNull();
  expect(localStorage.getItem("interfere:fp_hash")).toBeNull();

  kernel.consent.set(true);

  await expect
    .poll(() => kernel.device.getFpHash(), { timeout: 10_000 })
    .not.toBeNull();

  const hash = kernel.device.getFpHash();

  expect(hash).toMatch(/\S/u);

  expect(localStorage.getItem("interfere:fp_hash")).toBe(
    `${kernel.device.getDeviceId()}:${hash}`,
  );

  kernel.consent.set(false);

  await expect.poll(() => localStorage.getItem("interfere:fp_hash")).toBeNull();
});
