import { PUBLIC_KEY_HEADER } from "@interfere/constants/api";
import { RELEASE_TOKEN_HEADER } from "@interfere/types/sdk/release/token";

export type ServerCredential =
  | { readonly apiKey: string; readonly publicKey?: string }
  | { readonly releaseToken: string; readonly publicKey?: string }
  | { readonly publicKey: string };

/** Selects the server credential: the workspace API key, else the release token baked into the bundle, else the legacy public key alias. */
export function serverCredential(
  apiKey: string | null,
  releaseToken: string | null,
  publicKey: string | null,
) {
  if (apiKey) {
    return (
      publicKey ? { apiKey, publicKey } : { apiKey }
    ) satisfies ServerCredential;
  }

  if (releaseToken) {
    return (
      publicKey ? { releaseToken, publicKey } : { releaseToken }
    ) satisfies ServerCredential;
  }

  return publicKey ? ({ publicKey } satisfies ServerCredential) : null;
}

export function serverCredentialHeaders(credential: ServerCredential) {
  if ("releaseToken" in credential) {
    return credential.publicKey === undefined
      ? { [RELEASE_TOKEN_HEADER]: credential.releaseToken }
      : {
          [RELEASE_TOKEN_HEADER]: credential.releaseToken,
          [PUBLIC_KEY_HEADER]: credential.publicKey,
        };
  }

  if (!("apiKey" in credential)) {
    return { [PUBLIC_KEY_HEADER]: credential.publicKey };
  }

  return credential.publicKey === undefined
    ? { authorization: `Bearer ${credential.apiKey}` }
    : {
        authorization: `Bearer ${credential.apiKey}`,
        [PUBLIC_KEY_HEADER]: credential.publicKey,
      };
}
