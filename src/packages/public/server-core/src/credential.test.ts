import { expect, test } from "vitest";

import { serverCredential, serverCredentialHeaders } from "./credential.js";

test.each([
  [
    "interfere_secret_us_key",
    "interfere_public_us_key",
    {
      apiKey: "interfere_secret_us_key",
      publicKey: "interfere_public_us_key",
    },
  ],
  ["interfere_secret_us_key", null, { apiKey: "interfere_secret_us_key" }],
  [null, "interfere_public_us_key", { publicKey: "interfere_public_us_key" }],
  [null, null, null],
] as const)(
  "api key %s and public key %s select %o",
  (apiKey, publicKey, expected) => {
    expect(serverCredential(apiKey, publicKey)).toStrictEqual(expected);
  },
);

test("sends the workspace API key as a bearer token and the public key alias in its header", () => {
  expect(serverCredentialHeaders({ apiKey: "key" })).toStrictEqual({
    authorization: "Bearer key",
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
