import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { resolveTargets, setIngestHost } from "./config.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

describe(resolveTargets, () => {
  beforeEach(() => {
    setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", undefined);

    setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", undefined);

    setInjectedConfiguration("__INTERFERE_API_URL__", undefined);

    setInjectedConfiguration("__INTERFERE_INGEST_URL__", undefined);
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  test("resolves the default endpoint when no public key is set", () => {
    const targets = resolveTargets();

    expect(targets.ingest.baseUrl).toBe("https://api.interfere.com");
    expect(targets.ingest.baseUrl).toBe("https://api.interfere.com");
    expect(targets.config.baseUrl).toBe("https://api.interfere.com");
  });

  test("keeps credentials out of base URLs", () => {
    setInjectedConfiguration(
      "__INTERFERE_PUBLIC_KEY__",
      "interfere_pk_test_123",
    );

    const targets = resolveTargets();

    expect(targets.ingest.baseUrl).toBe("https://api.interfere.com");

    expect(targets.ingest.baseUrl).toBe("https://api.interfere.com");

    expect(targets.config.baseUrl).toBe("https://api.interfere.com");
  });

  test("passes public keys to native generated authentication", () => {
    setInjectedConfiguration(
      "__INTERFERE_PUBLIC_KEY__",
      "interfere_pk_test_456",
    );

    const targets = resolveTargets();

    expect(targets.ingest.publicKeyHeader).toStrictEqual({
      apiKey: "interfere_pk_test_456",
    });

    expect(targets.config.publicKeyHeader).toStrictEqual(
      targets.ingest.publicKeyHeader,
    );
  });

  test("uses globalThis.__INTERFERE_PUBLIC_KEY__ when process.env is unavailable", () => {
    setInjectedConfiguration(
      "__INTERFERE_PUBLIC_KEY__",
      "interfere_pk_vite_abc",
    );

    const targets = resolveTargets();

    expect(targets.ingest.baseUrl).toBe("https://api.interfere.com");
  });

  test("attaches x-interfere-force-enable when the global flag is set", () => {
    setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", true);

    const targets = resolveTargets();

    expect(targets.ingest.headers["x-interfere-force-enable"]).toBe("1");
    expect(targets.ingest.headers["x-interfere-force-enable"]).toBe("1");
    expect(targets.config.headers["x-interfere-force-enable"]).toBe("1");
  });

  test("omits x-interfere-force-enable when the global flag is unset", () => {
    const targets = resolveTargets();

    expect(targets.ingest.headers["x-interfere-force-enable"]).toBeUndefined();
  });

  test("resolves targets from the configured apiHost origin", () => {
    setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_sub");

    const targets = resolveTargets("https://in.example.com/ignored");

    expect(targets.ingest.baseUrl).toBe("https://in.example.com");

    expect(targets.ingest.baseUrl).toBe("https://in.example.com");
  });

  test("reads apiHost from the __INTERFERE_API_URL__ global", () => {
    setInjectedConfiguration(
      "__INTERFERE_PUBLIC_KEY__",
      "interfere_pk_override",
    );

    setInjectedConfiguration("__INTERFERE_API_URL__", "http://localhost:4001");

    const targets = resolveTargets();

    expect(targets.ingest.baseUrl).toBe("http://localhost:4001");
  });

  test("prefers an explicit apiHost over the __INTERFERE_API_URL__ global", () => {
    setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_ov");

    setInjectedConfiguration("__INTERFERE_API_URL__", "http://localhost:4001");

    const targets = resolveTargets("https://in.example.com");

    expect(targets.ingest.baseUrl).toBe("https://in.example.com");
  });

  test("falls back to the default endpoint when apiHost is not a valid URL", () => {
    setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_bad");

    const targets = resolveTargets("not a url");

    expect(targets.ingest.baseUrl).toBe("https://api.interfere.com");
  });

  test("routes ingest to the ingestHost while config stays on the config host", () => {
    setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_split");

    const targets = resolveTargets(
      "https://api.interfere.com",
      "https://int.customer.com",
    );

    expect(targets.config.baseUrl).toBe("https://api.interfere.com");

    expect(targets.ingest.baseUrl).toBe("https://int.customer.com");

    expect(targets.ingest.baseUrl).toBe("https://int.customer.com");
  });

  test("reads the ingest host from the __INTERFERE_INGEST_URL__ global", () => {
    setInjectedConfiguration(
      "__INTERFERE_INGEST_URL__",
      "https://int.customer.com",
    );

    const targets = resolveTargets();

    expect(targets.config.baseUrl).toBe("https://api.interfere.com");
    expect(targets.ingest.baseUrl).toBe("https://int.customer.com");
    expect(targets.ingest.baseUrl).toBe("https://int.customer.com");
  });

  test("prefers an explicit ingestHost over the __INTERFERE_INGEST_URL__ global", () => {
    setInjectedConfiguration(
      "__INTERFERE_INGEST_URL__",
      "https://from-global.customer.com",
    );

    const targets = resolveTargets(undefined, "https://explicit.customer.com");

    expect(targets.ingest.baseUrl).toBe("https://explicit.customer.com");
  });

  test("setIngestHost stamps the ingest global for later resolveTargets calls", () => {
    setIngestHost("https://int.customer.com");

    const targets = resolveTargets();

    expect(targets.ingest.baseUrl).toBe("https://int.customer.com");
    expect(targets.config.baseUrl).toBe("https://api.interfere.com");
  });
});
