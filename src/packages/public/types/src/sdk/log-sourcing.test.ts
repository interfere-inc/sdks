import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import {
  DEFAULT_LOG_SOURCING_MODE,
  LOG_SOURCING_GLOBAL,
  readLogSourcingMode,
  resolveLogSourcingMode,
} from "./log-sourcing.ts";
import { afterEach, describe, expect, test, vi } from "vitest";

describe(resolveLogSourcingMode, () => {
  test("follows the injected surface configuration", () => {
    expect(resolveLogSourcingMode("drain")).toBe("drain");
    expect(resolveLogSourcingMode(" SDK ")).toBe("sdk");
  });

  test("defaults to sdk when nothing valid is injected", () => {
    expect(resolveLogSourcingMode()).toBe(DEFAULT_LOG_SOURCING_MODE);
    expect(resolveLogSourcingMode("nope")).toBe("sdk");
  });
});

describe(readLogSourcingMode, () => {
  afterEach(() => {
    vi.unstubAllGlobals();

    vi.unstubAllEnvs();
  });

  test.each([undefined, null, false, 42, {}, [], "invalid"])(
    "defaults to sdk for invalid injected mode %j",
    (value) => {
      setInjectedConfiguration(LOG_SOURCING_GLOBAL, value);

      expect(readLogSourcingMode()).toBe("sdk");
    },
  );

  test("reads the build-injected global", () => {
    setInjectedConfiguration(LOG_SOURCING_GLOBAL, "drain");

    expect(readLogSourcingMode()).toBe("drain");
  });

  test("ignores the removed INTERFERE_LOG_SOURCING env var", () => {
    vi.stubEnv("INTERFERE_LOG_SOURCING", "drain");

    expect(readLogSourcingMode()).toBe("sdk");
  });
});
