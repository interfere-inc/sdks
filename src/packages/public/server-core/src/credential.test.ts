import { expect, test } from "vitest";

import { serverCredential, serverCredentialHeaders } from "./credential.js";

test.each([
  [
    "interfere_secret_us_key",
    "irt_token",
    "interfere_public_us_key",
    {
      apiKey: "interfere_secret_us_key",
      publicKey: "interfere_public_us_key",
    },
  ],
  [
    "interfere_secret_us_key",
    "irt_token",
    null,
    { apiKey: "interfere_secret_us_key" },
  ],
  [
    null,
    "irt_token",
    "interfere_public_us_key",
    { releaseToken: "irt_token", publicKey: "interfere_public_us_key" },
  ],
  [null, "irt_token", null, { releaseToken: "irt_token" }],
  [
    null,
    null,
    "interfere_public_us_key",
    { publicKey: "interfere_public_us_key" },
  ],
  [null, null, null, null],
] as const)(
  "api key %s, release token %s and public key %s select %o",
  (apiKey, releaseToken, publicKey, expected) => {
    expect(serverCredential(apiKey, releaseToken, publicKey)).toStrictEqual(
      expected,
    );
  },
);

test("sends the workspace API key as a bearer token, the release token and the public key alias in their headers", () => {
  expect(serverCredentialHeaders({ apiKey: "key" })).toStrictEqual({
    authorization: "Bearer key",
  });

  expect(serverCredentialHeaders({ releaseToken: "irt_token" })).toStrictEqual({
    "x-interfere-release-token": "irt_token",
  });

  expect(serverCredentialHeaders({ publicKey: "alias" })).toStrictEqual({
    "x-interfere-public-key": "alias",
  });
});

test("sends the public key alongside the workspace API key when both are configured", () => {
  expect(
    serverCredentialHeaders({ apiKey: "key", publicKey: "alias" }),
  ).toStrictEqual({
    authorization: "Bearer key",
    "x-interfere-public-key": "alias",
  });
});

test("sends the public key alongside the release token when both are configured", () => {
  expect(
    serverCredentialHeaders({ releaseToken: "irt_token", publicKey: "alias" }),
  ).toStrictEqual({
    "x-interfere-release-token": "irt_token",
    "x-interfere-public-key": "alias",
  });
});
