import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { context, propagation, trace } from "@opentelemetry/api";
import type { TracerProvider } from "@opentelemetry/sdk-trace";

import { ingestHandlers } from "../../test/fixtures/kernel.js";
import { isBotUserAgent } from "../util/bot.js";
import { getLogLevel, setLogLevel } from "../util/log.js";
import { createKernel, readReleaseSlugFromGlobals } from "./kernel.js";
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

import { addBaselineHandlers } from "../../test/msw.js";

beforeAll(() => {
  addBaselineHandlers(
    http.get("https://api.interfere.com/v3/config", () =>
      HttpResponse.json({ plugins: {} }),
    ),
    ...ingestHandlers("https://api.interfere.com"),
    ...ingestHandlers("https://in.example.com"),
  );
});

const ownedKernels = new Set<Awaited<ReturnType<typeof createKernel>>>();

const providers = new Set<TracerProvider>();

let priorUrl: string;

let priorHistory: unknown;

let priorCookie: Awaited<ReturnType<CookieStore["get"]>>;

let priorLogLevel: ReturnType<typeof getLogLevel>;

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

describe(readReleaseSlugFromGlobals, () => {
  afterEach(() => vi.unstubAllGlobals());

  test.each([undefined, null, false, 42, {}])(
    "ignores non-string host global %j",
    (value) => {
      setInjectedConfiguration("__INTERFERE_RELEASE_SLUG__", value);

      expect(readReleaseSlugFromGlobals()).toBeNull();
    },
  );

  test("validates the injected release slug", () => {
    setInjectedConfiguration(
      "__INTERFERE_RELEASE_SLUG__",
      "rel_0123456789abcdef",
    );

    expect(readReleaseSlugFromGlobals()).toBe("rel_0123456789abcdef");

    setInjectedConfiguration("__INTERFERE_RELEASE_SLUG__", "malformed");

    expect(() => readReleaseSlugFromGlobals()).toThrow(/Invalid/u);
  });
});

test("detects the actual automated browser", () => {
  expect(navigator.webdriver).toBe(true);

  expect(isBotUserAgent()).toBe(true);
});

describe("createKernel injected-global wiring", () => {
  const FORCE_ENABLE_GLOBAL = "__INTERFERE_FORCE_ENABLE__";
  const SDK_STACK_GLOBAL = "__INTERFERE_SDK_STACK__";

  const FORCE_ENABLE_WARNING =
    "FORCE_ENABLE is active. Events will be dropped by production ingest. This should only be set during local development.";

  afterEach(() => {
    Reflect.deleteProperty(globalThis, FORCE_ENABLE_GLOBAL);

    Reflect.deleteProperty(globalThis, SDK_STACK_GLOBAL);
  });

  test("warns exactly once per initialization when FORCE_ENABLE is truthy", async () => {
    setLogLevel("warn");

    const warn = vi.spyOn(globalThis.console, "warn");

    setInjectedConfiguration(FORCE_ENABLE_GLOBAL, true);

    const kernel = await createKernel({
      opts: { tracing: false },
    });

    ownedKernels.add(kernel);

    expect(
      warn.mock.calls.filter(
        (call) =>
          call[0] === "[Interfere:kernel]" && call[1] === FORCE_ENABLE_WARNING,
      ),
    ).toHaveLength(1);

    const second = await createKernel({
      opts: { tracing: false },
    });

    ownedKernels.add(second);

    expect(
      warn.mock.calls.filter(
        (call) =>
          call[0] === "[Interfere:kernel]" && call[1] === FORCE_ENABLE_WARNING,
      ),
    ).toHaveLength(2);

    await kernel.dispose();
  });

  test("does not warn about FORCE_ENABLE when the global is unset", async () => {
    setLogLevel("warn");

    const warn = vi.spyOn(globalThis.console, "warn");

    const kernel = await createKernel({
      opts: { tracing: false },
    });

    ownedKernels.add(kernel);

    expect(warn).not.toHaveBeenCalledWith(
      "[Interfere:kernel]",
      FORCE_ENABLE_WARNING,
    );

    await kernel.dispose();
  });

  test("stamps _wrapperVersions onto the SDK_STACK global by reference", async () => {
    const versions = ["next@1.2.3"];

    const kernel = await createKernel({
      opts: {
        _wrapperVersions: versions,
        tracing: false,
      },
    });

    ownedKernels.add(kernel);

    expect(globalThis).toHaveProperty(SDK_STACK_GLOBAL, versions);

    expect(
      Object.getOwnPropertyDescriptor(globalThis, SDK_STACK_GLOBAL)?.value,
    ).toBe(versions);

    await kernel.dispose();
  });

  test("rejects a non-writable SDK_STACK global rather than silently dropping wrapper versions", async () => {
    Object.defineProperty(globalThis, SDK_STACK_GLOBAL, {
      configurable: true,
      value: undefined,
      writable: false,
    });

    await expect(
      createKernel({
        opts: {
          _wrapperVersions: ["next@1.2.3"],
          tracing: false,
        },
      }),
    ).rejects.toThrow(TypeError);
  });

  test("leaves the SDK_STACK global unset when _wrapperVersions is absent", async () => {
    const kernel = await createKernel({
      opts: { tracing: false },
    });

    ownedKernels.add(kernel);

    expect(globalThis).not.toHaveProperty(SDK_STACK_GLOBAL);

    await kernel.dispose();
  });
});
