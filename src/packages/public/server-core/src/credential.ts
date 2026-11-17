import { PUBLIC_KEY_HEADER } from "@interfere/constants/api";

export type ServerCredential =
  | { readonly apiKey: string; readonly publicKey?: string }
  | { readonly publicKey: string };

/** Selects the server credential: the workspace API key, else the legacy public key alias. */
export function serverCredential(
  apiKey: string | null,
  publicKey: string | null,
) {
  if (apiKey) {
    return (
      publicKey ? { apiKey, publicKey } : { apiKey }
    ) satisfies ServerCredential;
  }

  return publicKey ? ({ publicKey } satisfies ServerCredential) : null;
}

export function serverCredentialHeaders(credential: ServerCredential) {
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
