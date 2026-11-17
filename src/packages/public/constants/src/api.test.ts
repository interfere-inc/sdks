import { API_URL, drainUrl, withPublicKey } from "./api";
import { describe, expect, test } from "vitest";

describe("telemetry URLs", () => {
  test.each(["traces", "logs", "metrics"] as const)(
    "routes %s to its endpoint without a credential in the query",
    (signal) => {
      expect(drainUrl(API_URL, signal)).toBe(`${API_URL}/drain/v1/${signal}`);
    },
  );

  test("keeps the public key query for SDKs released before the server API key", () => {
    expect(drainUrl(API_URL, "traces", "key+value")).toBe(
      `${API_URL}/drain/v1/traces?public_key=key%2Bvalue`,
    );
  });

  test("preserves existing query parameters while replacing the public key", () => {
    expect(
      withPublicKey(
        "https://events.example.com/path?custom=1&public_key=old",
        "new",
      ),
    ).toBe("https://events.example.com/path?custom=1&public_key=new");
  });
});
