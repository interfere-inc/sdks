import {
  BACKEND_HOSTS_GLOBAL,
  readBakedBackendHosts,
} from "./remote-config/backend-hosts.ts";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { afterEach, describe, expect, test, vi } from "vitest";
import { PLUGIN_MANIFEST } from "./plugins/manifest.ts";
import { remoteConfigInputSchema } from "./remote-config.ts";
import { PLUGINS_GLOBAL, readBakedPlugins } from "./remote-config/plugins.ts";

function fullPluginConfig(overrides: Record<string, boolean> = {}) {
  return Object.fromEntries(
    PLUGIN_MANIFEST.map((plugin) => [
      plugin.name,
      overrides[plugin.name] ?? true,
    ]),
  );
}

afterEach(() => {
  vi.unstubAllGlobals();
});

describe("remoteConfigInputSchema contract", () => {
  test("carries known backend hosts and drops a malformed list without losing the config", () => {
    expect(
      remoteConfigInputSchema.parse({ backendHosts: ["api.acme.io"] }),
    ).toStrictEqual({ backendHosts: ["api.acme.io"] });

    expect(
      remoteConfigInputSchema.parse({
        backendHosts: "api.acme.io",
        plugins: { errors: false },
      }),
    ).toStrictEqual({ backendHosts: undefined, plugins: { errors: false } });
  });

  test("accepts partial plugin settings without inventing missing flags", () => {
    expect(
      remoteConfigInputSchema.parse({ plugins: { errors: false } }),
    ).toStrictEqual({
      plugins: { errors: false },
    });
  });

  test("accepts responses without plugin settings", () => {
    expect(
      remoteConfigInputSchema.parse({
        ingestUrl: "https://ingest.example.com",
      }),
    ).toStrictEqual({
      ingestUrl: "https://ingest.example.com",
    });
  });

  test.for([
    null,
    [],
    { plugins: [] },
    { plugins: { errors: "false" } },
    { ingestUrl: 5 },
  ])("rejects malformed responses: %j", (value) => {
    expect(remoteConfigInputSchema.safeParse(value).success).toBe(false);
  });
});

describe(readBakedPlugins, () => {
  test("returns null when no plugins global is baked", () => {
    expect(readBakedPlugins()).toBeNull();
  });

  test("returns the baked plugin config", () => {
    setInjectedConfiguration(
      PLUGINS_GLOBAL,
      fullPluginConfig({
        replay: false,
      }),
    );

    expect(readBakedPlugins()?.replay).toBe(false);
    expect(readBakedPlugins()?.errors).toBe(true);
  });

  test("keeps a baked config that predates a newer plugin key (D-RC-06)", () => {
    const withoutLogs = Object.fromEntries(
      Object.entries(fullPluginConfig({ replay: false })).filter(
        ([name]) => name !== "logs",
      ),
    );

    setInjectedConfiguration(PLUGINS_GLOBAL, withoutLogs);

    expect(readBakedPlugins()).toStrictEqual(withoutLogs);
  });

  test("returns null for a malformed baked value", () => {
    setInjectedConfiguration(PLUGINS_GLOBAL, {
      errors: "yes",
    });

    expect(readBakedPlugins()).toBeNull();
  });
});

test.each([undefined, null, "bad", [12]])(
  "ignores malformed baked hosts %j",
  (value) => {
    setInjectedConfiguration(BACKEND_HOSTS_GLOBAL, value);

    expect(readBakedBackendHosts()).toBeUndefined();
  },
);

test("retains baked backend hosts", () => {
  setInjectedConfiguration(BACKEND_HOSTS_GLOBAL, ["api.customer.test"]);

  expect(readBakedBackendHosts()).toStrictEqual(["api.customer.test"]);
});

describe("remoteConfigInputSchema protocol version", () => {
  test("keeps a protocol version it does not know and ignores unknown fields", () => {
    expect(
      remoteConfigInputSchema.parse({
        plugins: { replay: false },
        protocolVersion: 42,
        someFutureSetting: { nested: ["value"] },
      }),
    ).toStrictEqual({ plugins: { replay: false }, protocolVersion: 42 });
  });

  test("drops a malformed version without losing the config", () => {
    expect(
      remoteConfigInputSchema.parse({
        plugins: { replay: false },
        protocolVersion: "1",
      }),
    ).toStrictEqual({ plugins: { replay: false }, protocolVersion: undefined });
  });

  test("a reader without the field parses the new response and never sees it", () => {
    expect(
      remoteConfigInputSchema
        .omit({ protocolVersion: true })
        .parse({ plugins: { replay: false }, protocolVersion: 1 }),
    ).toStrictEqual({ plugins: { replay: false } });
  });
});
