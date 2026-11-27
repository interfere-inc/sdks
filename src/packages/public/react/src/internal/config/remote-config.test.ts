import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { ANONYMOUS_REPLAY_GLOBAL } from "@interfere/types/sdk/remote-config/anonymous-replay";
import type { remoteConfigInputSchema } from "@interfere/types/sdk/remote-config";
import type { z } from "zod";
import { RELEASE_TOKEN_GLOBAL } from "@interfere/types/sdk/release/token";
import { afterEach, beforeEach, expect, test } from "vitest";

import { resolveRemoteConfig } from "./remote-config";

const API_HOST = "https://config.invalid";

function serveConfig(body: z.input<typeof remoteConfigInputSchema>) {
  return async () => Response.json(await Promise.resolve(body));
}

beforeEach(() => {
  setInjectedConfiguration(RELEASE_TOKEN_GLOBAL, "interfere_pk_test");
});

afterEach(() => {
  setInjectedConfiguration(RELEASE_TOKEN_GLOBAL, undefined);

  setInjectedConfiguration(ANONYMOUS_REPLAY_GLOBAL, undefined);
});

test("reads the anonymous replay policy from the fetched config", async () => {
  const resolved = await resolveRemoteConfig(
    API_HOST,
    serveConfig({ anonymousReplayPolicy: "on_error", plugins: {} }),
    null,
  );

  expect(resolved.anonymousReplayPolicy).toBe("on_error");
});

test("falls back to the baked policy when the fetched config has none", async () => {
  setInjectedConfiguration(ANONYMOUS_REPLAY_GLOBAL, "never");

  const resolved = await resolveRemoteConfig(
    API_HOST,
    serveConfig({ plugins: {} }),
    null,
  );

  expect(resolved.anonymousReplayPolicy).toBe("never");
});

test("uses the baked policy alongside baked plugin settings", async () => {
  setInjectedConfiguration(ANONYMOUS_REPLAY_GLOBAL, "on_error");

  const resolved = await resolveRemoteConfig(
    API_HOST,
    serveConfig({ anonymousReplayPolicy: "always", plugins: {} }),
    { replay: true },
  );

  expect(resolved.anonymousReplayPolicy).toBe("on_error");
});

test("returns null when neither the fetched config nor the build carries a policy", async () => {
  const resolved = await resolveRemoteConfig(
    API_HOST,
    serveConfig({ plugins: {} }),
    null,
  );

  expect(resolved.anonymousReplayPolicy).toBeNull();
});
