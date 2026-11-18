import { setupNetwork } from "@msw/cloudflare";
import { exports } from "cloudflare:workers";
import { delay, http } from "msw";
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
