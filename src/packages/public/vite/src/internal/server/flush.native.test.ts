import { exports } from "cloudflare:workers";
import { expect, test } from "vitest";

test("on workerd, a flush never awaits a flush started by a request that has since ended (website-production 2026-11-18T02:51:05Z)", async () => {
  await exports.default.abandon();

  await expect(exports.default.flush()).resolves.toBeUndefined();
});
