import { RELEASE_TOKEN_HEADER } from "@interfere/types/sdk/release/token";

export type ServerCredential =
  | { readonly apiKey: string }
  | { readonly releaseToken: string };

/** Selects the server credential: the workspace API key, else the release token baked into the bundle. */
export function serverCredential(
  apiKey: string | null,
  releaseToken: string | null,
) {
  if (apiKey) {
    return { apiKey } satisfies ServerCredential;
  }

  return releaseToken ? ({ releaseToken } satisfies ServerCredential) : null;
}

export function serverCredentialHeaders(credential: ServerCredential) {
  return "releaseToken" in credential
    ? { [RELEASE_TOKEN_HEADER]: credential.releaseToken }
    : { authorization: `Bearer ${credential.apiKey}` };
}
