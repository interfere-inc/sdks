import { afterEach, beforeEach, expect, test, vi } from "vitest";

import { exportedSpans } from "./otlp.js";
import { receiver } from "./telemetry-receiver.js";

interface ServerAdapter {
  register: () => Promise<void>;
  capture: (error: Error) => void;
  recordUrl: (url: string) => void;
  flush: () => Promise<void>;
  dispose: () => Promise<void>;
  installHost: () => {
    run: (capture: () => void) => void;
    captured: () => number;
    ownsTracing: () => boolean;
  };
}

/** Exercises the same lifecycle, privacy and delivery promises through each server adapter. */
export function serverContract(load: (url: string) => Promise<ServerAdapter>) {
  let collector: Awaited<ReturnType<typeof receiver>>;
  let adapter: ServerAdapter;
  const originalConsole = { ...console };

  beforeEach(async () => {
    vi.resetModules();

    collector = await receiver();

    adapter = await load(collector.url);
  });

  afterEach(async () => {
    Object.assign(console, originalConsole);

    await adapter.dispose();

    await collector.close();

    vi.unstubAllEnvs();

    vi.restoreAllMocks();
  });

  test("concurrent registration has one owner and exports each log once", async () => {
    expect.hasAssertions();

    await Promise.all([
      adapter.register(),
      adapter.register(),
      adapter.register(),
    ]);

    // oxlint-disable-next-line eslint/no-console -- Exercise the real console bridge.
    console.info("server-contract-once");

    await Promise.resolve();

    await adapter.flush();

    const bodies = collector.requests
      .filter((request) => request.url.includes("logs"))
      .map((request) => request.body)
      .join("\n");

    expect(bodies.match(/server-contract-once/gu)).toHaveLength(1);
  });

  test("preserves application ownership while delivering captured errors once", async () => {
    const host = adapter.installHost();

    await Promise.all([adapter.register(), adapter.register()]);

    host.run(() => adapter.capture(new Error("host-contract-error")));

    await adapter.flush();

    expect(host.ownsTracing()).toBe(true);
    expect(host.captured()).toBe(1);

    expect(
      JSON.stringify(exportedSpans(collector.requests)).match(
        /host-contract-error/gu,
      )?.length,
    ).toBeGreaterThan(0);

    expect(exportedSpans(collector.requests)).toHaveLength(1);
  });

  test("redacts sensitive URLs before exporting spans and captured exceptions", async () => {
    expect.hasAssertions();

    await adapter.register();

    adapter.recordUrl(
      "https://service.example/orders?token=contract-secret&sort=asc",
    );

    adapter.capture(
      new Error(
        "failed https://service.example/orders?token=contract-secret&sort=asc",
      ),
    );

    await adapter.flush();

    const spans = exportedSpans(collector.requests);
    const body = JSON.stringify(spans);

    expect(spans.length).toBeGreaterThanOrEqual(2);
    expect(body).not.toContain("contract-secret");
    expect(body).toContain("REDACTED");
    expect(body).toContain("sort=asc");
  });
}
