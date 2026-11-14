import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import {
  ANONYMOUS_REPLAY_GLOBAL,
  defaultAnonymousReplayPolicy,
  readBakedAnonymousReplayPolicy,
} from "./anonymous-replay.ts";
import { remoteConfigInputSchema } from "../remote-config.ts";
import { describe, expect, test } from "vitest";

describe(readBakedAnonymousReplayPolicy, () => {
  test("returns null when the build baked no policy", () => {
    setInjectedConfiguration(ANONYMOUS_REPLAY_GLOBAL, undefined);

    expect(readBakedAnonymousReplayPolicy()).toBeNull();
  });

  test("returns the baked policy", () => {
    setInjectedConfiguration(ANONYMOUS_REPLAY_GLOBAL, "on_error");

    expect(readBakedAnonymousReplayPolicy()).toBe("on_error");
  });

  test("ignores a baked value it does not recognize", () => {
    setInjectedConfiguration(ANONYMOUS_REPLAY_GLOBAL, "sometimes");

    expect(readBakedAnonymousReplayPolicy()).toBeNull();
  });
});

test("defaults public websites to on_error and signed-in apps to always", () => {
  expect(defaultAnonymousReplayPolicy(true)).toBe("on_error");
  expect(defaultAnonymousReplayPolicy(false)).toBe("always");
});

test("keeps the rest of a remote config when its replay policy is unknown", () => {
  const parsed = remoteConfigInputSchema.parse({
    anonymousReplayPolicy: "sometimes",
    plugins: { replay: true },
  });

  expect(parsed.anonymousReplayPolicy).toBeUndefined();
  expect(parsed.plugins).toStrictEqual({ replay: true });
});
