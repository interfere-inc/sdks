import { describe, expect, test } from "vitest";

import { acceptsCorrelationBaggageEntry } from "./correlation-baggage";

const SESSION_V7 = "019f4cac-2178-768b-ac0a-5b98b34d4c58";

const DATADOG_SESSION = "9d1e3f2a-8c4b-4f6e-9a7d-1b2c3d4e5f60";

describe("correlation baggage allowlist", () => {
  test.each([
    ["interfere.session.id", SESSION_V7],
    ["interfere.device.id", DATADOG_SESSION],
    ["interfere.view.id", "anything"],
    ["session.id", SESSION_V7],
    ["session.id", `server_${SESSION_V7}`],
    ["device.id", DATADOG_SESSION],
  ])("copies %s=%s", (key, value) => {
    expect(acceptsCorrelationBaggageEntry(key, value)).toBe(true);
  });

  test.each([
    ["session.id", DATADOG_SESSION],
    ["session.id", "legacy-session"],
    ["session.id", ""],
    ["device.id", "device-1"],
    ["user.id", "42"],
    ["account.id", "acme"],
    ["tenant", "acme"],
  ])("never copies %s=%s", (key, value) => {
    expect(acceptsCorrelationBaggageEntry(key, value)).toBe(false);
  });
});
