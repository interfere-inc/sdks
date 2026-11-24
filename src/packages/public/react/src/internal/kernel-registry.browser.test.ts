import { SessionTracker } from "../tracking/api.js";
import { setLogLevel } from "../util/log.js";
import {
  activeKernel,
  registerKernel,
  unregisterKernel,
} from "./kernel-registry.js";
import { Kernel } from "./kernel.js";
import { afterEach, describe, expect, test, vi } from "vitest";

const DOUBLE_INIT_WARN = /init.*twice/iu;

function createKernel() {
  const kernel = new Kernel(
    new SessionTracker({
      target: {
        baseUrl: "https://in.example.com",
        publicKeyHeader: { apiKey: "test-key" },
      },
    }),
  );

  onTestFinished(async () => {
    await kernel.dispose();
  });

  return kernel;
}

describe("kernel-registry", () => {
  afterEach(() => {
    const current = activeKernel();

    if (current) {
      unregisterKernel(current);
    }

    setLogLevel("none");

    vi.restoreAllMocks();
  });

  test("returns the registered kernel", () => {
    const k = createKernel();

    registerKernel(k);

    expect(activeKernel()).toBe(k);
  });

  test("unregister only clears when the instance matches", () => {
    const a = createKernel();
    const b = createKernel();

    registerKernel(a);

    unregisterKernel(b);

    expect(activeKernel()).toBe(a);

    unregisterKernel(a);

    expect(activeKernel()).toBeNull();
  });

  test("re-registering the same kernel does not warn", () => {
    setLogLevel("warn");

    const warn = vi.spyOn(globalThis.console, "warn");

    const k = createKernel();

    registerKernel(k);

    registerKernel(k);

    expect(warn).not.toHaveBeenCalled();
  });

  test("warns when a different kernel overwrites the slot", () => {
    setLogLevel("warn");

    const warn = vi.spyOn(globalThis.console, "warn");

    registerKernel(createKernel());

    registerKernel(createKernel());

    expect(warn).toHaveBeenCalledTimes(1);
    expect(warn.mock.calls[0]?.join(" ")).toMatch(DOUBLE_INIT_WARN);
  });
});
