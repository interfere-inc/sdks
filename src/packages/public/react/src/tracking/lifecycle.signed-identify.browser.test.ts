import {
  SOURCE,
  identifies,
  identifiesArrived,
  setupLifecycle,
  started,
  teardownLifecycle,
} from "./lifecycle.fixture.js";
import { afterEach, beforeEach, describe, expect, test } from "vitest";

beforeEach(setupLifecycle);

afterEach(teardownLifecycle);

describe("A72 signed identify", () => {
  const signature = "a".repeat(64);

  test("posts the server-computed signature with the identify params", async () => {
    expect.hasAssertions();

    const tracker = await started();

    await tracker.identify({
      identifier: "user-a72",
      signature,
      source: SOURCE,
    });

    await identifiesArrived(1);

    expect(identifies()[0]).toMatchObject({
      identifier: "user-a72",
      signature,
    });
  });

  test("re-sends identify when the same user gains a signature", async () => {
    expect.hasAssertions();

    const tracker = await started();

    await tracker.identify({ identifier: "user-a72b", source: SOURCE });

    await identifiesArrived(1);

    await tracker.identify({
      identifier: "user-a72b",
      signature,
      source: SOURCE,
    });

    await identifiesArrived(2);

    expect(identifies()[0]).not.toHaveProperty("signature");

    expect(identifies()[1]).toMatchObject({
      identifier: "user-a72b",
      signature,
    });
  });
});
