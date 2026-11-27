import { expect, test } from "vitest";

import { serverCredential, serverCredentialHeaders } from "./credential.js";

test.each([
  [
    "interfere_secret_us_key",
    "irt_token",
    { apiKey: "interfere_secret_us_key" },
  ],
  ["interfere_secret_us_key", null, { apiKey: "interfere_secret_us_key" }],
  [null, "irt_token", { releaseToken: "irt_token" }],
  [null, null, null],
] as const)(
  "api key %s and release token %s select %o",
  (apiKey, releaseToken, expected) => {
    expect(serverCredential(apiKey, releaseToken)).toStrictEqual(expected);
  },
);

test("sends the workspace API key as a bearer token and the release token in its header", () => {
  expect(serverCredentialHeaders({ apiKey: "key" })).toStrictEqual({
    authorization: "Bearer key",
  });

  expect(serverCredentialHeaders({ releaseToken: "irt_token" })).toStrictEqual({
    "x-interfere-release-token": "irt_token",
  });
});
