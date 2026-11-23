import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
/* oxlint-disable typescript/no-unsafe-assignment, unicorn/no-useless-promise-resolve-reject */
import { PLUGIN_MANIFEST } from "@interfere/types/sdk/plugins/manifest";

import { context, propagation, trace } from "@opentelemetry/api";
import type {
  LogRecordProcessor,
  ReadableLogRecord,
} from "@opentelemetry/sdk-logs";
import type { TracerProvider } from "@opentelemetry/sdk-trace";

import { ingestHandlers } from "../../test/fixtures/kernel.js";
import { getLogLevel, setLogLevel } from "../util/log.js";
import { createKernel } from "./kernel.js";
import { HttpResponse, http } from "msw";
import {
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  test,
  vi,
} from "vitest";

import { addBaselineHandlers, worker } from "../../test/msw.js";

beforeAll(() => {
  addBaselineHandlers(
    http.get("https://api.interfere.com/v3/config", () =>
      HttpResponse.json({ plugins: {} }),
    ),
    ...ingestHandlers("https://api.interfere.com"),
    ...ingestHandlers("https://in.example.com"),
  );
});

const WAIT_OPTS = { timeout: 5000 };

const ownedKernels = new Set<Awaited<ReturnType<typeof createKernel>>>();

const providers = new Set<TracerProvider>();

let priorUrl: string;

let priorHistory: unknown;

let priorCookie: Awaited<ReturnType<CookieStore["get"]>>;

let priorLogLevel: ReturnType<typeof getLogLevel>;

function serveConfig(response: Response) {
  const requests: Request[] = [];

  worker.use(
    http.get("https://api.interfere.com/v3/config", ({ request }) => {
      requests.push(request);

      return response;
    }),
  );

  return requests;
}

beforeEach(async () => {
  priorLogLevel = getLogLevel();

  priorUrl = globalThis.location.href;

  priorHistory = globalThis.history.state;

  priorCookie = await cookieStore.get("interfere_did");
});

afterEach(async () => {
  try {
    const results = await Promise.allSettled([
      ...Array.from(ownedKernels, async (kernel) => kernel.dispose()),
      ...Array.from(providers, async (provider) => provider.shutdown()),
    ]);

    const failures = results.filter((result) => result.status === "rejected");

    if (failures.length > 0) {
      throw new AggregateError(
        failures.map((result) => {
          const decodedValue: unknown = result.reason;

          return decodedValue;
        }),
        "Kernel cleanup failed",
      );
    }
  } finally {
    ownedKernels.clear();

    providers.clear();

    context.disable();

    trace.disable();

    propagation.disable();

    setLogLevel(priorLogLevel);

    globalThis.history.replaceState(priorHistory, "", priorUrl);

    await cookieStore.delete("interfere_did");

    if (priorCookie?.name && priorCookie.value !== undefined) {
      await cookieStore.set({
        ...priorCookie,
        name: priorCookie.name,
        value: priorCookie.value,
      });
    }

    vi.unstubAllGlobals();
  }
});

describe(createKernel, () => {
  test("skips the config fetch when plugins are baked at build time", async () => {
    const fullPlugins = Object.fromEntries(
      PLUGIN_MANIFEST.map((plugin) => [plugin.name, plugin.name !== "replay"]),
    );

    setInjectedConfiguration("__INTERFERE_PLUGINS__", fullPlugins);

    const configRequests = serveConfig(HttpResponse.json({ plugins: {} }));

    try {
      const kernel = await createKernel({
        opts: { tracing: false },
      });

      ownedKernels.add(kernel);

      expect(configRequests).toHaveLength(0);
      expect(kernel.getPluginConfigState("replay")).toBe("disabled");
      expect(kernel.getPluginConfigState("errors")).toBe("enabled");

      await kernel.dispose();
    } finally {
      vi.unstubAllGlobals();
    }
  });

  test.each([
    [
      "returns a failed response",
      () => new HttpResponse("unavailable", { status: 503 }),
    ],
    ["omits plugins", () => HttpResponse.json({})],
    [
      "cannot be parsed",
      () =>
        new HttpResponse("{", {
          headers: { "content-type": "application/json" },
        }),
    ],
  ])(
    "uses local plugin defaults when remote config %s",
    async (_case, response) => {
      setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "test-key");

      const configRequests = serveConfig(response());

      const kernel = await createKernel({
        opts: { tracing: false },
      });

      ownedKernels.add(kernel);

      expect(configRequests).toHaveLength(1);
      expect(kernel.getPluginConfigState("replay")).toBe("enabled");

      await kernel.dispose();
    },
  );

  test("emits the initial browser navigation log once pageEvents activates, not at boot", async () => {
    expect.hasAssertions();

    const logs: ReadableLogRecord[] = [];

    const logProcessor = {
      forceFlush: async () => Promise.resolve(),
      onEmit: (record) => {
        logs.push(record);
      },
      shutdown: async () => Promise.resolve(),
    } satisfies LogRecordProcessor;

    globalThis.history.pushState({}, "", "/login");

    const kernel = await createKernel({
      opts: { _internalAdditionalLogRecordProcessors: [logProcessor] },
    });

    ownedKernels.add(kernel);

    expect(
      logs,
      "pageview is analytics-gated; nothing may ship before consent settles",
    ).toHaveLength(0);

    kernel.syncProviderConfig({});

    await vi.waitFor(() => {
      const pageload = logs.find(
        (l) => l.attributes["browser.navigation.type"] === "pageload",
      );

      expect(pageload).toMatchObject({
        attributes: expect.objectContaining({
          "browser.navigation.type": "pageload",
          "interfere.view.id": kernel.views.viewId(),
          "interfere.view.trigger": "pageload",
          "session.id": expect.any(String),
          "url.full": globalThis.location.href,
        }),
        eventName: "browser.navigation",
        instrumentationScope: {
          name: "@interfere/browser-events",
        },
        severityNumber: 0,
      });
    }, WAIT_OPTS);

    await kernel.dispose();
  });
});
