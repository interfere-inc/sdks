import { HttpResponse, http } from "msw";
import { worker } from "../test/msw.js";
import { useEffect } from "react";
import type { PropsWithChildren } from "react";
import { render, renderHook } from "vitest-browser-react";

import { DISABLE_ALL_PLUGINS } from "../test/fixtures/disable-all-plugins.js";
import { createKernel } from "./internal/kernel.js";
import type { Kernel } from "./internal/kernel.js";
import { InterfereProvider, useInterfere } from "./provider.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

describe("react provider browser runtime", () => {
  let kernel: Kernel;
  const unexpectedRequests: string[] = [];

  function withProvider({ children }: PropsWithChildren) {
    return (
      <InterfereProvider kernel={kernel} plugins={DISABLE_ALL_PLUGINS}>
        {children}
      </InterfereProvider>
    );
  }

  beforeEach(async () => {
    unexpectedRequests.length = 0;

    worker.use(
      http.get("https://sdk.example.test/v3/config", () =>
        HttpResponse.json({}),
      ),
      http.post("https://sdk.example.test/v3/session", () =>
        HttpResponse.json({}),
      ),
      http.all("https://sdk.example.test/*", ({ request }) => {
        unexpectedRequests.push(`${request.method} ${request.url}`);

        return HttpResponse.error();
      }),
    );

    kernel = await createKernel({
      opts: {
        apiHost: "https://sdk.example.test",
        tracing: false,
      },
    });
  });

  afterEach(async () => {
    await kernel.dispose();

    if (unexpectedRequests.length > 0) {
      throw new Error(
        `Unexpected SDK requests: ${unexpectedRequests.join(", ")}`,
      );
    }
  });

  test("session id resolves once the provider mounts", async () => {
    expect.hasAssertions();

    const { result } = await renderHook(() => useInterfere(), {
      wrapper: withProvider,
    });

    await vi.waitFor(() => {
      expect(result.current.session.getId()).toMatch(/\S/u);
    });
  });

  test.each([
    ["denied", { analytics: false, replay: false }],
    ["measurement-only", { analytics: true, replay: false }],
  ])(
    "reapplies %s child consent after the null-kernel no-op setter is replaced",
    async (_, state) => {
      expect.hasAssertions();

      const writes: ReturnType<Kernel["consent"]["get"]>[] = [];

      function ConsentSync() {
        const { consent } = useInterfere();

        useEffect(() => {
          consent.set(state);

          writes.push(consent.get());
        }, [consent]);

        return null;
      }

      const { rerender } = await render(
        <InterfereProvider kernel={null} plugins={DISABLE_ALL_PLUGINS}>
          <ConsentSync />
        </InterfereProvider>,
      );

      expect(writes).toStrictEqual([null]);

      await rerender(
        <InterfereProvider kernel={kernel} plugins={DISABLE_ALL_PLUGINS}>
          <ConsentSync />
        </InterfereProvider>,
      );

      await vi.waitFor(() => {
        expect(writes).toStrictEqual([null, state]);
        expect(kernel.consent.get()).toStrictEqual(state);
      });
    },
  );
});
