import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, describe, expect, test } from "vitest";
import { z } from "zod";

import { worker } from "../../test/msw.js";
import { createWrapperSingleton } from "./wrapper-singleton.js";
import { decodedBodyJson } from "../../test/fixtures/request-body.js";

const API_HOST = "https://audit-a-wrapper.test";

const SOURCE = { name: "Audit", type: "custom" } as const;

const recordedBody = z.looseObject({ identifier: z.string().optional() });

const identifies: z.infer<typeof recordedBody>[] = [];

const signal = { identify: Promise.withResolvers<undefined>() };

async function settle() {
  const { promise, resolve } = Promise.withResolvers<undefined>();

  setTimeout(() => {
    resolve(undefined);
  }, 100);

  await promise;
}

function createWrapper() {
  return createWrapperSingleton({
    initEntryName: "test",
    producerVersion: "@interfere/test@0.0.0",
  });
}

beforeEach(() => {
  identifies.length = 0;

  signal.identify = Promise.withResolvers<undefined>();

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

  setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", true);

  worker.use(
    http.get(`${API_HOST}/v3/config`, () => HttpResponse.json({ plugins: {} })),
    http.post(`${API_HOST}/v3/session`, () => HttpResponse.json({ ok: true })),
    http.post(
      `${API_HOST}/drain/v1/logs`,
      () => new HttpResponse(null, { status: 204 }),
    ),
    http.post(`${API_HOST}/v3/session/identify`, async ({ request }) => {
      identifies.push(recordedBody.parse(await decodedBodyJson(request)));

      signal.identify.resolve(undefined);

      return HttpResponse.json({ ok: true });
    }),
  );
});

afterEach(() => {
  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", undefined);

  setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", undefined);
});

describe("A09 identity.set before init() resolves", () => {
  test("an identify issued while init is pending is sent once the kernel is ready", async () => {
    expect.hasAssertions();

    const wrapper = createWrapper();

    try {
      const initializing = wrapper.init({
        apiHost: API_HOST,
        enabled: true,
        tracing: false,
      });

      await wrapper.identity.set({
        company: { tier: "Enterprise" },
        identifier: "user-a09",
        source: SOURCE,
      });

      await initializing;

      await signal.identify.promise;

      expect(identifies).toHaveLength(1);

      expect(identifies[0]).toMatchObject({
        company: { tier: "Enterprise" },
        identifier: "user-a09",
      });

      expect(wrapper.identity.get()).toMatchObject({
        company: { tier: "Enterprise" },
        identifier: "user-a09",
      });
    } finally {
      await wrapper.close();
    }
  });

  test("identity.clear while init is pending cancels the queued identify", async () => {
    expect.hasAssertions();

    const wrapper = createWrapper();

    try {
      const initializing = wrapper.init({
        apiHost: API_HOST,
        enabled: true,
        tracing: false,
      });

      await wrapper.identity.set({ identifier: "user-a09", source: SOURCE });

      wrapper.identity.clear();

      await expect(initializing).resolves.not.toBeNull();

      await settle();

      expect(identifies).toStrictEqual([]);
      expect(wrapper.identity.get()).toBeNull();
    } finally {
      await wrapper.close();
    }
  });

  test("identify after init() resolves is sent", async () => {
    expect.hasAssertions();

    const wrapper = createWrapper();

    try {
      await wrapper.init({ apiHost: API_HOST, enabled: true, tracing: false });

      await wrapper.identity.set({
        identifier: "user-a09-late",
        source: SOURCE,
      });

      await signal.identify.promise;

      expect(identifies).toHaveLength(1);
    } finally {
      await wrapper.close();
    }
  });
});

describe("A56 consent.set before init() resolves", () => {
  test("a revocation issued while init is pending is applied once the kernel is ready", async () => {
    expect.hasAssertions();

    const wrapper = createWrapper();

    try {
      const initializing = wrapper.init({
        apiHost: API_HOST,
        enabled: true,
        tracing: false,
      });

      wrapper.consent.set(false);

      await initializing;

      expect(wrapper.consent.get()).toStrictEqual({
        analytics: false,
        replay: false,
      });
    } finally {
      await wrapper.close();
    }
  });
});
