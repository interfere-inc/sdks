import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { beforeEach, expect, test, vi } from "vitest";

import { hasIngestCredential, resolveTargets } from "./config";

beforeEach(() => {
  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", undefined);

  setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", undefined);

  setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", undefined);

  setInjectedConfiguration("__INTERFERE_API_URL__", undefined);

  setInjectedConfiguration("__INTERFERE_INGEST_URL__", undefined);

  vi.stubEnv("INTERFERE_PUBLIC_KEY", undefined);
});

test("passes the native process environment key through generated authentication", () => {
  vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_process");

  const targets = resolveTargets();

  expect(targets.ingest.publicKeyHeader).toStrictEqual({
    apiKey: "interfere_pk_process",
  });

  expect(targets.config.publicKeyHeader).toStrictEqual(
    targets.ingest.publicKeyHeader,
  );

  expect(targets.ingest.baseUrl).toBe("https://api.interfere.com");

  expect(targets.config.baseUrl).toBe("https://api.interfere.com");
});

test("prefers the injected public key over the native process environment", () => {
  vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_process");

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_global");

  expect(resolveTargets().ingest.publicKeyHeader).toStrictEqual({
    apiKey: "interfere_pk_global",
  });
});

test.each([undefined, ""])(
  "omits generated authentication for an absent process key %j",
  (key) => {
    vi.stubEnv("INTERFERE_PUBLIC_KEY", key);

    expect(resolveTargets().ingest.publicKeyHeader).toBeUndefined();
  },
);

test.each([undefined, ""])("reports no credential for %j", (key) => {
  vi.stubEnv("INTERFERE_PUBLIC_KEY", key);

  expect(hasIngestCredential()).toBe(false);
});

test("reports a public key from the native process environment", () => {
  vi.stubEnv("INTERFERE_PUBLIC_KEY", "interfere_pk_process");

  expect(hasIngestCredential()).toBe(true);
});

test("reports an injected public key", () => {
  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_global");

  expect(hasIngestCredential()).toBe(true);
});

test("a baked release token alone enables ingest and rides every request beside no public key", () => {
  setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", "irt_baked");

  expect(hasIngestCredential()).toBe(true);

  const targets = resolveTargets();

  expect(targets.ingest.publicKeyHeader).toBeUndefined();

  expect(targets.ingest.releaseTokenHeader).toStrictEqual({
    apiKey: "irt_baked",
  });

  expect(targets.ingest.headers).toStrictEqual({
    "x-interfere-release-token": "irt_baked",
  });

  expect(targets.config.headers).toStrictEqual(targets.ingest.headers);
});

test("a build with both credentials sends the release token and keeps the public key", () => {
  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_global");

  setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", "irt_baked");

  const { ingest } = resolveTargets();

  expect(ingest.publicKeyHeader).toStrictEqual({
    apiKey: "interfere_pk_global",
  });

  expect(ingest.releaseTokenHeader).toBeUndefined();

  expect(ingest.headers).toStrictEqual({
    "x-interfere-release-token": "irt_baked",
  });
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
