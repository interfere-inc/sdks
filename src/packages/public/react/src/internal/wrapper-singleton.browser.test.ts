import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, describe, expect, test } from "vitest";

import { ingestHandlers } from "../../test/fixtures/kernel.js";
import { worker } from "../../test/msw.js";
import { createWrapperSingleton } from "./wrapper-singleton.js";
import type { Kernel } from "./kernel.js";

const API_HOST = "https://wrapper-singleton.invalid";

const requests: Request[] = [];

function recordRequest({ request }: { request: Request }) {
  if (new URL(request.url).origin === API_HOST) {
    requests.push(request.clone());
  }
}

function createWrapper() {
  return createWrapperSingleton({
    initEntryName: "test",
    producerVersion: "@interfere/test@0.0.0",
  });
}

beforeEach(() => {
  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", undefined);

  setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", true);

  worker.use(
    http.get(`${API_HOST}/v3/config`, () => HttpResponse.json({ plugins: {} })),
    ...ingestHandlers(API_HOST),
  );

  worker.events.on("request:start", recordRequest);
});

afterEach(() => {
  worker.events.removeListener("request:start", recordRequest);

  requests.splice(0);

  setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", undefined);
});

describe(createWrapperSingleton, () => {
  test("no-ops when identity.clear is called before initialization", () => {
    const wrapper = createWrapper();

    expect(() => {
      wrapper.identity.clear();
    }).not.toThrow();
  });

  test.each([undefined, ""])(
    "sends nothing without a public key (%j)",
    async (key) => {
      setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", key);

      const wrapper = createWrapper();

      await expect(
        wrapper.init({ apiHost: API_HOST, enabled: true, tracing: false }),
      ).resolves.toBeNull();

      expect(wrapper.getKernelOrNull()).toBeNull();

      expect(requests).toStrictEqual([]);
    },
  );

  test("authenticates every request with an injected public key", async () => {
    setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

    const wrapper = createWrapper();

    try {
      await expect(
        wrapper.init({ apiHost: API_HOST, enabled: true, tracing: false }),
      ).resolves.not.toBeNull();

      expect(requests.length).toBeGreaterThan(0);

      expect(
        requests.map((request) =>
          request.headers.get("x-interfere-public-key"),
        ),
      ).toStrictEqual(requests.map(() => "interfere_pk_test"));
    } finally {
      await wrapper.close();
    }
  });

  test.each([
    { restart: false, configRequests: 1, publications: 0 },
    { restart: true, configRequests: 2, publications: 1 },
  ])(
    "closes pending initialization without publishing it, with restart $restart",
    async ({ restart, configRequests, publications }) => {
      setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

      const requested = Promise.withResolvers<undefined>();
      const response = Promise.withResolvers<undefined>();
      const wrapper = createWrapper();
      const published: (Kernel | null)[] = [];

      const unsubscribe = wrapper.subscribeToKernel(() => {
        published.push(wrapper.getKernelOrNull());
      });

      worker.use(
        http.get(`${API_HOST}/v3/config`, async () => {
          requested.resolve(undefined);

          await response.promise;

          return HttpResponse.json({ plugins: {} });
        }),
      );

      const options = {
        apiHost: API_HOST,
        enabled: true,
        tracing: false,
      } as const;

      const initial = wrapper.init(options);
      const shared = wrapper.init(options);

      try {
        await requested.promise;

        const closing = wrapper.close();
        const repeatedClose = wrapper.close();
        const next = restart ? wrapper.init(options) : null;

        expect(wrapper.getKernelOrNull()).toBeNull();

        response.resolve(undefined);

        await expect(initial).resolves.toBeNull();

        await expect(shared).resolves.toBeNull();

        await Promise.all([closing, repeatedClose]);

        const kernel = await next;

        expect(wrapper.getKernelOrNull()).toBe(kernel);
        expect(published).toHaveLength(publications);
        expect(published).not.toContain(null);

        expect(
          requests.filter((request) => request.url.endsWith("/v3/config")),
        ).toHaveLength(configRequests);
      } finally {
        response.resolve(undefined);

        await initial;

        await wrapper.close();

        unsubscribe();
      }
    },
  );

  test("failed initialization releases capture and navigation before a fresh start", async () => {
    setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

    setInjectedConfiguration("__INTERFERE_RELEASE_SLUG__", "invalid");

    const wrapper = createWrapper();
    const originalError = globalThis.onerror;
    // oxlint-disable-next-line typescript/unbound-method
    const originalPushState = history.pushState;

    const options = {
      apiHost: API_HOST,
      enabled: true,
      tracing: false,
    } as const;

    try {
      await expect(wrapper.init(options)).rejects.toThrow(/Invalid/u);

      expect(wrapper.getKernelOrNull()).toBeNull();
      expect(globalThis.onerror).toBe(originalError);
      // oxlint-disable-next-line typescript/unbound-method
      expect(history.pushState).toBe(originalPushState);

      setInjectedConfiguration("__INTERFERE_RELEASE_SLUG__", undefined);

      const kernel = await wrapper.init(options);

      expect(kernel).not.toBeNull();

      const closing = wrapper.close();

      expect(wrapper.getKernelOrNull()).toBeNull();

      await closing;

      expect(globalThis.onerror).toBe(originalError);
      // oxlint-disable-next-line typescript/unbound-method
      expect(history.pushState).toBe(originalPushState);
    } finally {
      setInjectedConfiguration("__INTERFERE_RELEASE_SLUG__", undefined);

      await wrapper.close();
    }
  });

  test.each(["before", "during"])(
    "preserves consent set %s initialization when the provider configures plugins",
    async (timing) => {
      setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

      const wrapper = createWrapper();

      if (timing === "before") {
        wrapper.consent.set(false);
      }

      const pending = wrapper.init({
        apiHost: API_HOST,
        enabled: true,
        tracing: false,
      });

      if (timing === "during") {
        wrapper.consent.set(false);
      }

      try {
        const kernel = await pending;

        kernel?.syncProviderConfig({
          plugins: { replay: true, pageEvents: true },
        });

        expect(wrapper.consent.get()).toStrictEqual({
          analytics: false,
          replay: false,
        });

        expect(kernel?.isPageEventsCaptureEnabled()).toBe(false);
        expect(kernel?.isDeviceInfoCaptureEnabled()).toBe(false);
      } finally {
        await wrapper.close();
      }
    },
  );
});
