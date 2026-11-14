import { PLUGIN_MANIFEST } from "@interfere/types/sdk/plugins/manifest";

import { LOADERS, resolveFeatures } from "./loader.js";
import { expect, test } from "vitest";

test("resolves defaults and applies overrides", () => {
  expect(
    resolveFeatures({ pageEvents: false, rageClick: false }),
  ).toStrictEqual({
    device: true,
    errors: true,
    logs: true,
    pageEvents: false,
    rageClick: false,
    replay: true,
  });
});

test.each([
  [{ interactions: false }, true],
  [{}, true],
  [true, true],
  [false, false],
])("treats pageEvents %j as enabled: %s", (pageEvents, enabled) => {
  expect(resolveFeatures({ pageEvents }).pageEvents).toBe(enabled);
});

test("has a loader for every manifest entry", () => {
  expect.hasAssertions();

  expect(Object.keys(LOADERS).toSorted()).toStrictEqual(
    PLUGIN_MANIFEST.map((plugin) => plugin.name).toSorted(),
  );

  for (const plugin of PLUGIN_MANIFEST) {
    expect(LOADERS[plugin.name]).toBeTypeOf("function");
  }
});

test.each([
  ["device", "device"],
  ["errors", "errors"],
  ["logs", "logs"],
  ["pageEvents", "pageEvents"],
  ["rageClick", "rage-clicks"],
  ["replay", "replay"],
] as const)("loads the actual %s plugin module", async (key, name) => {
  const loaded = await LOADERS[key]();

  const plugin = "default" in loaded ? loaded.default : loaded;

  expect(plugin.name).toBe(name);
  expect(plugin.setup).toBeTypeOf("function");
});
