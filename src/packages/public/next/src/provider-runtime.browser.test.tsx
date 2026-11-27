import { network } from "@interfere/test-config/browser-network";
import { HttpResponse, http } from "msw";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { renderHook } from "vitest-browser-react";
import { close, getKernelOrNull, init } from "./instrument-client.js";
import { InterfereProvider, useInterfere, useSession } from "./provider.js";
import { assert, expect, test, vi } from "vitest";

test("updates mounted consumers from null through delayed init, close and reinitialization", async ({
  onTestFinished,
}) => {
  expect.hasAssertions();

  await close();

  setInjectedConfiguration("__INTERFERE_BUILD__", true);

  setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", true);

  setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", "irt_test");

  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  setInjectedConfiguration("__INTERFERE_INGEST_URL__", undefined);

  setInjectedConfiguration("__INTERFERE_SDK_STACK__", undefined);

  const plugins = {
    device: false,
    errors: false,
    logs: false,
    pageEvents: false,
    rageClick: false,
    replay: false,
  };

  const held = Promise.withResolvers<Response>();
  const requested = Promise.withResolvers<Request>();
  const requests: Request[] = [];
  const unexpected: string[] = [];

  const saved = [localStorage, sessionStorage].flatMap((storage) =>
    [
      "interfere:session_id",
      "interfere:session_started",
      "interfere:last_activity",
      "interfere:window_id",
    ].map((key) => ({ storage, key, value: storage.getItem(key) })),
  );

  network.use(
    http.get("https://next-provider.invalid/v3/config", async ({ request }) => {
      requests.push(request);

      requested.resolve(request);

      return requests.filter((entry) => entry.method === "GET").length === 1
        ? held.promise
        : HttpResponse.json({ plugins });
    }),
    http.post("https://next-provider.invalid/v3/session", ({ request }) => {
      requests.push(request);

      return new HttpResponse(null, { status: 204 });
    }),
    http.all("https://next-provider.invalid/*", ({ request }) => {
      requests.push(request);

      unexpected.push(`${request.method} ${request.url}`);

      return HttpResponse.error();
    }),
  );

  let initializing: ReturnType<typeof init> | undefined;

  onTestFinished(async () => {
    held.resolve(HttpResponse.json({ plugins }));

    try {
      await initializing;

      await close();
    } finally {
      vi.unstubAllGlobals();

      for (const { storage, key, value } of saved) {
        if (value === null) {
          storage.removeItem(key);
        } else {
          storage.setItem(key, value);
        }
      }

      expect(unexpected).toStrictEqual([]);
    }
  });

  const view = await renderHook(
    () => ({ context: useInterfere(), session: useSession() }),
    {
      wrapper: ({ children }) => (
        <InterfereProvider plugins={plugins}>{children}</InterfereProvider>
      ),
    },
  );

  onTestFinished(async () => view.unmount());

  expect(view.result.current.context.kernel).toBeNull();
  expect(view.result.current.session).toBeNull();
  expect(view.result.current.context.identity.get()).toBeNull();
  expect(view.result.current.context.device.getDeviceId()).toBeNull();
  expect(view.result.current.context.consent.get()).toBeNull();

  const options = {
    apiHost: "https://next-provider.invalid",
    enabled: true,
    tracing: false as const,
  };

  const creating = init(options);

  initializing = creating;

  const request = await requested.promise;

  expect(request.headers.get("x-interfere-force-enable")).toBe("1");
  expect(getKernelOrNull()).toBeNull();
  expect(view.result.current.session).toBeNull();

  await view.act(async () => {
    held.resolve(HttpResponse.json({ plugins }));

    await creating;
  });

  const first = await creating;

  expect(first).not.toBeNull();
  expect(view.result.current.context.kernel).toBe(first);

  assert.exists(first);

  expect(view.result.current.session).toBe(first.session.getId());

  expect(view.result.current.session).toBe(
    localStorage.getItem("interfere:session_id"),
  );

  expect(view.result.current.session).toStrictEqual(expect.any(String));

  await view.act(close);

  expect(getKernelOrNull()).toBeNull();
  expect(view.result.current.context.kernel).toBeNull();
  expect(view.result.current.session).toBeNull();

  assert.exists(first);

  expect(first.session.getId()).toBeNull();

  await view.act(async () => {
    await init(options);
  });

  const second = getKernelOrNull();

  expect(second).not.toBeNull();
  expect(second).not.toBe(first);
  expect(view.result.current.context.kernel).toBe(second);

  assert.exists(second);

  expect(view.result.current.session).toBe(second.session.getId());

  assert.exists(second);

  expect(view.result.current.context.session).toBe(second.session);
  expect(requests.filter((entry) => entry.method === "GET")).toHaveLength(2);

  await vi.waitFor(() => {
    expect(requests.filter((entry) => entry.method === "POST")).toHaveLength(2);
  });

  expect(unexpected).toStrictEqual([]);
});
