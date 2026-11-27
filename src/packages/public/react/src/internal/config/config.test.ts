import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { beforeEach, expect, test } from "vitest";

import { hasIngestCredential, resolveTargets } from "./config";

beforeEach(() => {
  setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", undefined);

  setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", undefined);

  setInjectedConfiguration("__INTERFERE_API_URL__", undefined);

  setInjectedConfiguration("__INTERFERE_INGEST_URL__", undefined);
});

test("reports no credential without a release token", () => {
  expect(hasIngestCredential()).toBe(false);
});

test("a baked release token alone enables ingest and rides every request", () => {
  setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", "irt_baked");

  expect(hasIngestCredential()).toBe(true);

  const targets = resolveTargets();

  expect(targets.ingest.releaseTokenHeader).toStrictEqual({
    apiKey: "irt_baked",
  });

  expect(targets.ingest.headers).toStrictEqual({
    "x-interfere-release-token": "irt_baked",
  });

  expect(targets.config.headers).toStrictEqual(targets.ingest.headers);
});

test.each([
  [
    { serviceName: "web", serviceNamespace: "shop" },
    { "x-interfere-service": "web", "x-interfere-service-namespace": "shop" },
  ],
  [{ serviceName: "web" }, { "x-interfere-service": "web" }],
  [{ serviceNamespace: "shop" }, {}],
])(
  "shares service headers across config and ingest targets: %j",
  (service, expected) => {
    const targets = resolveTargets(undefined, undefined, service);

    expect(targets.config.headers).toStrictEqual(expected);
    expect(targets.ingest.headers).toStrictEqual(expected);
  },
);
