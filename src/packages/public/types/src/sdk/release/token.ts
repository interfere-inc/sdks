/**
 * Global a build stamps into the page with the release token the release
 * registration returned. The browser SDK sends it as x-interfere-release-token
 * and starts without a public key when it is present.
 */
export const RELEASE_TOKEN_GLOBAL = "__INTERFERE_RELEASE_TOKEN__";

export const RELEASE_TOKEN_HEADER = "x-interfere-release-token";
