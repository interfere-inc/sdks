import { parseRoutePrefixInput } from "./internal/route-prefix";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import {
  DEFAULT_ROUTE_PREFIX,
  normalizeRoutePrefix,
  resolveRoutePrefix,
} from "./route-prefix.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

afterEach(() => {
  vi.unstubAllGlobals();

  vi.unstubAllEnvs();
});

describe(resolveRoutePrefix, () => {
  beforeEach(() => {
    setInjectedConfiguration("__INTERFERE_ROUTE_PREFIX__", undefined);

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_ROUTE_PREFIX", "");
  });

  test("uses the default when neither source is configured", () => {
    expect(resolveRoutePrefix()).toBe(DEFAULT_ROUTE_PREFIX);
  });

  test.each([undefined, null, false, 42, {}])(
    "ignores non-string environment input %j",
    (value) => {
      expect(parseRoutePrefixInput(value)).toBeUndefined();
    },
  );

  test("ignores the removed environment prefix when the global property is absent", () => {
    Reflect.deleteProperty(globalThis, "__INTERFERE_ROUTE_PREFIX__");

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_ROUTE_PREFIX", "from-env/");

    expect(resolveRoutePrefix()).toBe(DEFAULT_ROUTE_PREFIX);
  });

  test("prefers the host-injected prefix over the environment", () => {
    setInjectedConfiguration("__INTERFERE_ROUTE_PREFIX__", " from-global/ ");

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_ROUTE_PREFIX", "/from-env");

    expect(resolveRoutePrefix()).toBe("/from-global");
  });

  test.each([undefined, null, false, 42, "", {}, [], () => "/not-a-prefix"])(
    "ignores malformed or empty global override %j",
    (value) => {
      setInjectedConfiguration("__INTERFERE_ROUTE_PREFIX__", value);

      vi.stubEnv("NEXT_PUBLIC_INTERFERE_ROUTE_PREFIX", "/from-env");

      expect(resolveRoutePrefix()).toBe(DEFAULT_ROUTE_PREFIX);
    },
  );

  test("retains the default for a whitespace-only global instead of using environment fallback", () => {
    setInjectedConfiguration("__INTERFERE_ROUTE_PREFIX__", " ");

    vi.stubEnv("NEXT_PUBLIC_INTERFERE_ROUTE_PREFIX", "/from-env");

    expect(resolveRoutePrefix()).toBe(DEFAULT_ROUTE_PREFIX);
  });
});

test.each([
  { prefix: "", expected: DEFAULT_ROUTE_PREFIX },
  { prefix: " ", expected: DEFAULT_ROUTE_PREFIX },
  { prefix: "/", expected: DEFAULT_ROUTE_PREFIX },
  { prefix: "route", expected: "/route" },
  { prefix: " route/ ", expected: "/route" },
  { prefix: "/route/", expected: "/route" },
  { prefix: "/route//", expected: "/route/" },
])(
  "normalizes $prefix without changing established slash semantics",
  ({ prefix, expected }) => {
    expect(normalizeRoutePrefix(prefix)).toBe(expected);
  },
);
