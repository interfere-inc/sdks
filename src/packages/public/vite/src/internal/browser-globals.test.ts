import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { applyBrowserGlobals } from "./browser-globals";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

const GLOBAL_KEYS = [
  "__INTERFERE_PUBLIC_KEY__",
  "__INTERFERE_API_URL__",
  "__INTERFERE_ENVIRONMENT__",
  "__INTERFERE_RELEASE_SLUG__",
  "__INTERFERE_RELEASE_TOKEN__",
  "__INTERFERE_INGEST_URL__",
  "__INTERFERE_PLUGINS__",
  "__INTERFERE_BACKEND_HOSTS__",
  "__INTERFERE_FORCE_ENABLE__",
  "__INTERFERE_SERVICE_NAME__",
] as const;

beforeEach(() => {
  vi.unstubAllEnvs();

  for (const key of GLOBAL_KEYS) {
    setInjectedConfiguration(key, undefined);
  }
});

afterEach(() => {
  vi.unstubAllEnvs();

  vi.unstubAllGlobals();
});

describe(applyBrowserGlobals, () => {
  test.each([false, 0, "", {}, []])(
    "preserves existing non-nullish host value %j",
    (value) => {
      setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", value);

      vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "from_env");

      applyBrowserGlobals();

      expect(
        Object.getOwnPropertyDescriptor(globalThis, "__INTERFERE_PUBLIC_KEY__")
          ?.value,
      ).toBe(value);
    },
  );

  test("replaces a null host value", () => {
    setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", null);

    vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "from_env");

    applyBrowserGlobals();

    expect(globalThis).toHaveProperty("__INTERFERE_PUBLIC_KEY__", "from_env");
  });

  test.each(["null", "true", "42", '"plugins"', "[]"])(
    "ignores non-object plugin JSON %j",
    (value) => {
      vi.stubEnv("VITE_INTERFERE_PLUGINS", value);

      applyBrowserGlobals();

      expect(globalThis).toHaveProperty("__INTERFERE_PLUGINS__", undefined);
    },
  );

  test("bridges inlined env onto the kernel globals", () => {
    vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "interfere_pub_us_bridge");

    vi.stubEnv("VITE_INTERFERE_API_HOST", "https://in.example.com");

    vi.stubEnv("VITE_INTERFERE_ENVIRONMENT", "production");

    vi.stubEnv("VITE_INTERFERE_RELEASE_SLUG", "rel_deadbeef");

    vi.stubEnv("VITE_INTERFERE_RELEASE_TOKEN", "irt_bridge");

    applyBrowserGlobals();

    expect(globalThis).toHaveProperty(
      "__INTERFERE_PUBLIC_KEY__",
      "interfere_pub_us_bridge",
    );

    expect(globalThis).toHaveProperty(
      "__INTERFERE_API_URL__",
      "https://in.example.com",
    );

    expect(globalThis).toHaveProperty(
      "__INTERFERE_ENVIRONMENT__",
      "production",
    );

    expect(globalThis).toHaveProperty(
      "__INTERFERE_RELEASE_SLUG__",
      "rel_deadbeef",
    );

    expect(globalThis).toHaveProperty(
      "__INTERFERE_RELEASE_TOKEN__",
      "irt_bridge",
    );
  });

  test("does not overwrite a value an HTML stamp or manual set already applied", () => {
    setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "already_set");

    vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "from_env");

    applyBrowserGlobals();

    expect(globalThis).toHaveProperty(
      "__INTERFERE_PUBLIC_KEY__",
      "already_set",
    );
  });

  test("leaves the api-url global unset when the env is absent", () => {
    vi.stubEnv("VITE_INTERFERE_PUBLIC_KEY", "interfere_pub_us_x");

    applyBrowserGlobals();

    expect(globalThis).toHaveProperty("__INTERFERE_API_URL__", undefined);
  });

  test("bridges the stamped service name onto the kernel global", () => {
    vi.stubEnv("VITE_INTERFERE_SERVICE_NAME", "storefront");

    applyBrowserGlobals();

    expect(globalThis).toHaveProperty(
      "__INTERFERE_SERVICE_NAME__",
      "storefront",
    );
  });

  test("keeps a service name the host already set over the stamped one", () => {
    setInjectedConfiguration("__INTERFERE_SERVICE_NAME__", "from_host");

    vi.stubEnv("VITE_INTERFERE_SERVICE_NAME", "storefront");

    applyBrowserGlobals();

    expect(globalThis).toHaveProperty(
      "__INTERFERE_SERVICE_NAME__",
      "from_host",
    );
  });

  test("bridges the baked ingest URL onto the kernel global", () => {
    vi.stubEnv("VITE_INTERFERE_INGEST_URL", "https://in.customer.com");

    applyBrowserGlobals();

    expect(globalThis).toHaveProperty(
      "__INTERFERE_INGEST_URL__",
      "https://in.customer.com",
    );
  });

  test("parses and bridges the baked plugins JSON", () => {
    vi.stubEnv(
      "VITE_INTERFERE_PLUGINS",
      JSON.stringify({ errors: true, replay: false }),
    );

    applyBrowserGlobals();

    expect(globalThis).toHaveProperty("__INTERFERE_PLUGINS__", {
      errors: true,
      replay: false,
    });
  });

  test("ignores malformed baked plugins JSON", () => {
    vi.stubEnv("VITE_INTERFERE_PLUGINS", "not json");

    applyBrowserGlobals();

    expect(globalThis).toHaveProperty("__INTERFERE_PLUGINS__", undefined);
  });

  test.each([
    ["1", undefined],
    ["true", undefined],
    ["0", undefined],
    ["", undefined],
  ] as const)("ignores removed force-enable flag %j (%j)", (flag, expected) => {
    vi.stubEnv("VITE_INTERFERE_FORCE_ENABLE", flag);

    applyBrowserGlobals();

    expect(globalThis).toHaveProperty("__INTERFERE_FORCE_ENABLE__", expected);
  });
});

test("bridges discovered backend hosts into the browser kernel", () => {
  vi.stubEnv(
    "VITE_INTERFERE_BACKEND_HOSTS",
    JSON.stringify(["api.customer.test"]),
  );

  applyBrowserGlobals();

  expect(globalThis).toHaveProperty("__INTERFERE_BACKEND_HOSTS__", [
    "api.customer.test",
  ]);
});

test("ignores malformed backend-host JSON", () => {
  vi.stubEnv("VITE_INTERFERE_BACKEND_HOSTS", "not json");

  expect(() => applyBrowserGlobals()).not.toThrow();
  expect(globalThis).toHaveProperty("__INTERFERE_BACKEND_HOSTS__", undefined);
});
