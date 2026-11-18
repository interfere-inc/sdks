import { trace } from "@opentelemetry/api";
import { BasicTracerProvider } from "@opentelemetry/sdk-trace-base";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

await import("./ownership.js");

beforeEach(() => {
  vi.resetModules();
});

afterEach(() => {
  trace.disable();

  vi.unstubAllGlobals();

  vi.restoreAllMocks();
});

test("auto owns tracing only when no provider is registered and warns once on adoption", async () => {
  const { resolveTracingMode } = await import("./ownership.js");

  const warn = vi.spyOn(console, "warn");

  expect(resolveTracingMode(undefined)).toBe("managed");
  expect(resolveTracingMode("auto")).toBe("managed");
  expect(warn).not.toHaveBeenCalled();

  const host = new BasicTracerProvider();

  trace.setGlobalTracerProvider(host);

  expect(resolveTracingMode(undefined)).toBe("application");
  expect(resolveTracingMode("auto")).toBe("application");
  expect(resolveTracingMode("managed")).toBe("managed");

  expect(warn.mock.calls).toStrictEqual([
    [
      expect.stringContaining(
        "[interfere] Adopted the OpenTelemetry tracer provider",
      ),
    ],
  ]);

  await host.shutdown();
});

test("explicit application tracing adopts a registered provider silently", async () => {
  const { resolveTracingMode } = await import("./ownership.js");

  const warn = vi.spyOn(console, "warn");

  const host = new BasicTracerProvider();

  trace.setGlobalTracerProvider(host);

  expect(resolveTracingMode("application")).toBe("application");
  expect(warn).not.toHaveBeenCalled();

  await host.shutdown();
});
