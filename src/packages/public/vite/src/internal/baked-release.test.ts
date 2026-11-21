import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { afterEach, expect, test, vi } from "vitest";

import {
  BAKED_RELEASE_SLUG_DEFINE,
  BAKED_RELEASE_TOKEN_DEFINE,
  bakedReleaseSlug,
  bakedReleaseToken,
} from "./baked-release";

afterEach(() => {
  vi.unstubAllEnvs();
});

test("a build-time identifier wins over the Vite environment", () => {
  vi.stubEnv("VITE_INTERFERE_RELEASE_SLUG", "rel_fromenv");

  vi.stubEnv("VITE_INTERFERE_RELEASE_TOKEN", "irt_fromenv");

  setInjectedConfiguration(BAKED_RELEASE_SLUG_DEFINE, "rel_baked");

  setInjectedConfiguration(BAKED_RELEASE_TOKEN_DEFINE, "irt_baked");

  expect([bakedReleaseSlug(), bakedReleaseToken()]).toStrictEqual([
    "rel_baked",
    "irt_baked",
  ]);
});

test("without a build-time identifier the Vite environment supplies the release", () => {
  vi.stubEnv("VITE_INTERFERE_RELEASE_SLUG", "rel_fromenv");

  vi.stubEnv("VITE_INTERFERE_RELEASE_TOKEN", "irt_fromenv");

  expect([bakedReleaseSlug(), bakedReleaseToken()]).toStrictEqual([
    "rel_fromenv",
    "irt_fromenv",
  ]);
});
