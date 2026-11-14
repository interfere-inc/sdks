import { API_URL, drainUrl, withPublicKey } from "./api";
import { describe, expect, test } from "vitest";

describe("telemetry URLs", () => {
  test.each(["traces", "logs", "metrics"] as const)(
    "routes %s to its authenticated endpoint",
    (signal) => {
      expect(drainUrl(API_URL, signal, "key+value")).toBe(
        `${API_URL}/drain/v1/${signal}?public_key=key%2Bvalue`,
      );
    },
  );

  test("preserves existing query parameters while replacing the public key", () => {
    expect(
      withPublicKey(
        "https://events.example.com/path?custom=1&public_key=old",
        "new",
      ),
    ).toBe("https://events.example.com/path?custom=1&public_key=new");
  });
});
