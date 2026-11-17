import { expect, test } from "vitest";

import { remoteConfigInputSchema } from "../remote-config.ts";
import { parseRemoteConfigInput } from "./input.ts";

test.each([
  {},
  { plugins: { errors: false } },
  { plugins: { errors: false, replay: true, unknownPlugin: "x" } },
  { plugins: undefined },
  { plugins: { errors: undefined } },
  { plugins: null },
  { plugins: [] },
  { plugins: { errors: "false" } },
  { backendHosts: ["api.acme.io"] },
  { backendHosts: "api.acme.io", plugins: { errors: false } },
  { backendHosts: [12] },
  { backendHosts: undefined },
  { anonymousReplayPolicy: "on_error" },
  { anonymousReplayPolicy: "sometimes", plugins: { replay: true } },
  { protocolVersion: 42, someFutureSetting: { nested: ["value"] } },
  { protocolVersion: "1" },
  { protocolVersion: 0 },
  { protocolVersion: 1.5 },
  { protocolVersion: Number.MAX_SAFE_INTEGER + 2 },
  { ingestUrl: "https://ingest.example.com" },
  { ingestUrl: "  https://ingest.example.com/\tpath\n " },
  { ingestUrl: "not a url" },
  { ingestUrl: 5 },
  { ingestUrl: undefined },
  { logSourcing: "drain", platform: "vercel", serverTracing: "full" },
  { logSourcing: "console" },
  { platform: "netlify" },
  { serverTracing: "partial" },
  {
    anonymousReplayPolicy: "never",
    backendHosts: ["a.example", "b.example"],
    ingestUrl: "https://eu.ingest.example.com",
    logSourcing: "sdk",
    platform: "cloudflare",
    plugins: { logs: false },
    protocolVersion: 1,
    serverTracing: "drain",
  },
  null,
  [],
  "config",
  5,
])(
  "parseRemoteConfigInput agrees with remoteConfigInputSchema for %j",
  (body) => {
    const expected = remoteConfigInputSchema.safeParse(body);

    expect(parseRemoteConfigInput(body)).toStrictEqual(
      expected.success ? expected.data : null,
    );
  },
);
