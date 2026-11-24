import { startNodeRuntime } from "@interfere/server-core/node";
import { W3CTraceContextPropagator } from "@interfere/server-core/otel/core/propagation";
import { setupNetwork } from "@msw/cloudflare";
import { exports } from "cloudflare:workers";
import { HttpResponse, delay, http } from "msw";
import { afterAll, beforeAll, expect, test } from "vitest";

const network = setupNetwork();

beforeAll(() => {
  network.configure({ onUnhandledFrame: "error" });

  network.enable();

  network.use(
    http.get("*/v3/config", async () => {
      await delay("infinite");
    }),
  );
});

afterAll(() => {
  network.disable();
});

test("on workerd, a register never awaits startup owned by a request that has since ended (website-production 2026-11-18T14:52:17Z)", async () => {
  await exports.default.abandonStartup();

  await expect(
    exports.default.registerDuringStartup(),
  ).resolves.toBeUndefined();
});

test("on workerd, a register initialises the runtime while another request that has since ended still owns the remote config fetch", async () => {
  await exports.default.abandonRegister();

  await expect(exports.default.register()).resolves.toBe(1);
});

test("workerd sends server traces through its fetch transport", async () => {
  const requests: string[] = [];

  network.use(
    http.post("https://collector.test/drain/v1/traces", async ({ request }) => {
      requests.push(await request.text());

      return HttpResponse.json({});
    }),
  );

  const pipeline = await startNodeRuntime({
    name: "workerd-test",
    version: "1",
    apiUrl: "https://collector.test",
    credential: { publicKey: "test-key" },
    resourceAttributes: {},
    options: { consoleBridge: false, tracing: "application" },
    drain: false,
    logDrain: true,
    vercel: false,
    propagator: new W3CTraceContextPropagator(),
  });

  try {
    pipeline.provider
      .getTracer("workerd-test")
      .startSpan("edge-collector")
      .end();

    await pipeline.flush();

    expect(requests).toHaveLength(1);
    expect(requests[0]).toContain("edge-collector");
  } finally {
    await pipeline.shutdown();
  }
});
