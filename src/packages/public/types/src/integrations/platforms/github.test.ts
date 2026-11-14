import {
  detectGithubHostFlavor,
  normalizeGithubHost,
  providerFromHost,
  resolveGithubHostUrls,
} from "./github.ts";
import { describe, expect, test } from "vitest";

describe(normalizeGithubHost, () => {
  test("trims, lowercases, and strips protocol and path", () => {
    expect(normalizeGithubHost("  HTTPS://GitHub.Acme.com/foo/bar  ")).toBe(
      "github.acme.com",
    );

    expect(normalizeGithubHost("http://github.com")).toBe("github.com");
    expect(normalizeGithubHost("github.acme.com")).toBe("github.acme.com");
  });
});

describe(detectGithubHostFlavor, () => {
  test("detects github.com", () => {
    expect(detectGithubHostFlavor("https://github.com")).toBe("github_com");
  });

  test("detects ghe.com cloud hosts", () => {
    expect(detectGithubHostFlavor("acme.ghe.com")).toBe("ghe_cloud");
  });

  test("falls back to ghe_server for self-hosted hosts", () => {
    expect(detectGithubHostFlavor("github.acme.com")).toBe("ghe_server");
  });
});

describe(providerFromHost, () => {
  test("maps github.com to the github provider", () => {
    expect(providerFromHost("github.com")).toBe("github");
  });

  test("maps ghe.com hosts to ghe_cloud", () => {
    expect(providerFromHost("acme.ghe.com")).toBe("ghe_cloud");
  });

  test("maps self-hosted hosts to ghe_server", () => {
    expect(providerFromHost("github.acme.com")).toBe("ghe_server");
  });
});

describe(resolveGithubHostUrls, () => {
  test("resolves github.com urls", () => {
    expect(resolveGithubHostUrls({ host: "github.com" })).toStrictEqual({
      apiBaseUrl: "https://api.github.com",
      flavor: "github_com",
      host: "github.com",
      webBaseUrl: "https://github.com",
    });
  });

  test("resolves ghe_cloud urls", () => {
    expect(resolveGithubHostUrls({ host: "acme.ghe.com" })).toStrictEqual({
      apiBaseUrl: "https://api.acme.ghe.com",
      flavor: "ghe_cloud",
      host: "acme.ghe.com",
      webBaseUrl: "https://acme.ghe.com",
    });
  });

  test("resolves ghe_server urls and normalizes the host", () => {
    expect(
      resolveGithubHostUrls({ host: "https://github.acme.com/" }),
    ).toStrictEqual({
      apiBaseUrl: "https://github.acme.com/api/v3",
      flavor: "ghe_server",
      host: "github.acme.com",
      webBaseUrl: "https://github.acme.com",
    });
  });

  test("honors an explicit flavor override", () => {
    expect(
      resolveGithubHostUrls({ flavor: "ghe_cloud", host: "github.acme.com" }),
    ).toStrictEqual({
      apiBaseUrl: "https://api.github.acme.com",
      flavor: "ghe_cloud",
      host: "github.acme.com",
      webBaseUrl: "https://github.acme.com",
    });
  });
});
