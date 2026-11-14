import { protectionBypassHeaders } from "./protection-bypass.js";
import { expect, test, vi } from "vitest";

test.each([
  undefined,
  "http://localhost:3000",
  "http://127.0.0.1:3000",
  "http://0.0.0.0:3000",
  "http://[::1]:3000",
])("does not require credentials for %s", (baseURL) => {
  vi.stubEnv("CF_ACCESS_CLIENT_ID", undefined);

  vi.stubEnv("CF_ACCESS_CLIENT_SECRET", undefined);

  expect(protectionBypassHeaders(baseURL, "cloudflare-access")).toBeUndefined();
});

test("uses the configured service token for remote deployments", () => {
  vi.stubEnv("CF_ACCESS_CLIENT_ID", "client-id");

  vi.stubEnv("CF_ACCESS_CLIENT_SECRET", "client-secret");

  expect(
    protectionBypassHeaders("https://preview.example.com", "cloudflare-access"),
  ).toStrictEqual({
    "CF-Access-Client-Id": "client-id",
    "CF-Access-Client-Secret": "client-secret",
  });
});

test.each(["CF_ACCESS_CLIENT_ID", "CF_ACCESS_CLIENT_SECRET"])(
  "explains how to configure missing %s without private infrastructure instructions",
  (missing) => {
    vi.stubEnv("CF_ACCESS_CLIENT_ID", "client-id");

    vi.stubEnv("CF_ACCESS_CLIENT_SECRET", "client-secret");

    vi.stubEnv(missing, undefined);

    expect(() =>
      protectionBypassHeaders(
        "https://preview.example.com",
        "cloudflare-access",
      ),
    ).toThrow(
      "Missing CF_ACCESS_CLIENT_ID / CF_ACCESS_CLIENT_SECRET while baseURL points at a remote Cloudflare deployment. baseURL=https://preview.example.com Set both environment variables to a Cloudflare Access service token authorized for this deployment.",
    );
  },
);
