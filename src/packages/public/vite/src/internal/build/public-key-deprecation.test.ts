import type { Logger } from "vite";
import { expect, test, vi } from "vitest";

import { warnIfPublicKeyDeprecated } from "./public-key-deprecation";

test("warns that VITE_INTERFERE_PUBLIC_KEY is deprecated", () => {
  const warn = vi.fn<Logger["warn"]>();

  warnIfPublicKeyDeprecated({ warn }, "interfere_pk_test");

  expect(warn).toHaveBeenCalledExactlyOnceWith(expect.stringContaining("14.0"));
});

test.each([undefined, ""])("stays quiet for %j", (publicKey) => {
  const warn = vi.fn<Logger["warn"]>();

  warnIfPublicKeyDeprecated({ warn }, publicKey);

  expect(warn).not.toHaveBeenCalled();
});
