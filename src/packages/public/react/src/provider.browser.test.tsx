import type { PropsWithChildren } from "react";
import { page } from "vitest/browser";
import { ErrorBoundary } from "./error-boundary.js";
import { render, renderHook } from "vitest-browser-react";

import { DISABLE_ALL_PLUGINS } from "../test/fixtures/disable-all-plugins.js";
import { buildKernel } from "../test/fixtures/kernel.js";
import type { Kernel } from "./internal/kernel.js";
import { NULL_CONTEXT_VALUE } from "./internal/react-context.js";
import { InterfereProvider, useInterfere, useSession } from "./provider.js";
import { beforeEach, describe, expect, onTestFinished, test, vi } from "vitest";

interface Harness {
  kernel: Kernel;
}

function makeHarness(pluginOverrides = DISABLE_ALL_PLUGINS) {
  const built = buildKernel({ pluginOverrides });

  onTestFinished(async () => {
    await built.kernel.dispose();
  });

  built.tracker.start();

  built.runtime.start();

  return {
    kernel: built.kernel,
  } satisfies Harness;
}

function providerWrapper(kernel: Kernel) {
  return function wrapper({ children }: PropsWithChildren) {
    return (
      <InterfereProvider kernel={kernel} plugins={DISABLE_ALL_PLUGINS}>
        {children}
      </InterfereProvider>
    );
  };
}

const SOURCE = { name: "Test", type: "custom" } as const;

describe(InterfereProvider, () => {
  let harness: Harness;

  beforeEach(() => {
    harness = makeHarness();
  });

  async function renderWithSession() {
    const { result } = await renderHook(() => useInterfere(), {
      wrapper: providerWrapper(harness.kernel),
    });

    await vi.waitFor(() => {
      expect(result.current.session.getId()).toMatch(/\S/u);
    });

    return result;
  }

  test("preserves context identity when the provider rerenders with the same kernel", async () => {
    const wrapper = providerWrapper(harness.kernel);

    const { result, rerender } = await renderHook(() => useInterfere(), {
      wrapper,
    });

    const initial = result.current;

    await rerender();

    expect(result.current).toBe(initial);
  });

  test("exposes consent, session, identity, and device via context", async () => {
    const wrapper = providerWrapper(harness.kernel);

    const { result } = await renderHook(() => useInterfere(), { wrapper });

    expect(result.current.session.getId()).toMatch(/\S/u);
    expect(result.current.consent.get).toBeTypeOf("function");
    expect(result.current.consent.set).toBeTypeOf("function");
    expect(result.current.identity.get).toBeTypeOf("function");
    expect(result.current.identity.set).toBeTypeOf("function");
    expect(result.current.identity.clear).toBeTypeOf("function");
    expect(result.current.device.getDeviceId).toBeTypeOf("function");
    expect(result.current.kernel).toBe(harness.kernel);
  });

  test("preserves imperative denial across provider mounting and plugin updates", async () => {
    harness.kernel.consent.set(false);

    const { rerender } = await render(
      <InterfereProvider kernel={harness.kernel} plugins={{ replay: false }} />,
    );

    await rerender(
      <InterfereProvider
        kernel={harness.kernel}
        plugins={{ replay: true, pageEvents: true }}
      />,
    );

    expect(harness.kernel.consent.get()).toStrictEqual({
      analytics: false,
      replay: false,
    });

    expect(harness.kernel.isPageEventsCaptureEnabled()).toBe(false);
    expect(harness.kernel.isDeviceInfoCaptureEnabled()).toBe(false);

    harness.kernel.consent.set(true);

    expect(harness.kernel.consent.get()).toStrictEqual({
      analytics: true,
      replay: true,
    });
  });

  test("updates identity when the current user changes", async () => {
    expect.hasAssertions();

    const result = await renderWithSession();

    await result.current.identity.set({
      identifier: "react-unit-1",
      source: SOURCE,
    });

    await result.current.identity.set({
      identifier: "react-unit-2",
      source: SOURCE,
    });

    expect(harness.kernel.identity.get()).toStrictEqual({
      identifier: "react-unit-2",
      source: SOURCE,
    });
  });

  test("clears identity through context", async () => {
    expect.hasAssertions();

    const result = await renderWithSession();

    await result.current.identity.set({
      identifier: "react-clear",
      source: SOURCE,
    });

    result.current.identity.clear();

    expect(harness.kernel.identity.get()).toBeNull();
  });

  test("throws when useInterfere is used outside provider", async () => {
    await expect(renderHook(() => useInterfere())).rejects.toThrow(
      "useInterfere must be used within <InterfereProvider>",
    );
  });
});

describe("NULL_CONTEXT_VALUE contract", () => {
  test("no-ops when identity.clear is called before initialization", () => {
    expect(() => {
      NULL_CONTEXT_VALUE.identity.clear();
    }).not.toThrow();
  });
});

function ThrowingChild({ error }: { error: Error }): never {
  throw error;
}

test.each([{}, { captureRenderErrors: true }] as const)(
  "enabled render capture %j preserves the host fallback",
  async (props) => {
    vi.spyOn(console, "error");

    const { kernel } = makeHarness();
    const recordSpy = vi.spyOn(kernel, "recordException");
    const error = new Error("provider-rethrow");

    await render(
      <ErrorBoundary fallback={<div>upstream caught</div>}>
        <InterfereProvider
          kernel={kernel}
          plugins={DISABLE_ALL_PLUGINS}
          {...props}
        >
          <ThrowingChild error={error} />
        </InterfereProvider>
      </ErrorBoundary>,
    );

    await expect.element(page.getByText("upstream caught")).toBeVisible();

    expect(recordSpy.mock.calls.length).toBeGreaterThan(0);
    expect(recordSpy.mock.calls[0]?.[0]).toBe(error);

    expect(recordSpy.mock.calls[0]?.[1]?.mechanism.handled).toBe(false);
  },
);

test.each([{ captureRenderErrors: false }] as const)(
  "disabled render capture %j preserves the host fallback",
  async (props) => {
    vi.spyOn(console, "error");

    const { kernel } = makeHarness();
    const recordSpy = vi.spyOn(kernel, "recordException");
    const error = new Error("provider-rethrow");

    await render(
      <ErrorBoundary fallback={<div>upstream caught</div>}>
        <InterfereProvider
          kernel={kernel}
          plugins={DISABLE_ALL_PLUGINS}
          {...props}
        >
          <ThrowingChild error={error} />
        </InterfereProvider>
      </ErrorBoundary>,
    );

    await expect.element(page.getByText("upstream caught")).toBeVisible();

    expect(recordSpy).toHaveBeenCalledTimes(0);
    expect(recordSpy.mock.calls[0]?.[0]).toBeUndefined();

    expect(recordSpy.mock.calls[0]?.[1]?.mechanism.handled).toBeUndefined();
  },
);

describe(useSession, () => {
  test("returns the current session id after rerender", async () => {
    expect.hasAssertions();

    const harness = makeHarness();

    const wrapper = providerWrapper(harness.kernel);

    const { result, rerender } = await renderHook(() => useSession(), {
      wrapper,
    });

    await vi.waitFor(async () => {
      await rerender();

      expect(result.current).toMatch(/\S/u);
    });

    const previousId = result.current;

    harness.kernel.identity.clear();

    await rerender();

    expect(result.current).toBe(harness.kernel.session.getId());
    expect(result.current).not.toBe(previousId);
  });
});
