const TRANSPORT_FAILURE_MESSAGES = [
  "Failed to fetch",
  "Load failed",
  "NetworkError when attempting to fetch resource.",
  "fetch failed",
  "Network request failed",
  "The network connection was lost.",
  "The Internet connection appears to be offline.",
] as const;

const HOST_SUFFIX_RE = / \([^()\s]+\)$/u;

function isBareTransportFailure(message: string) {
  const trimmed = message.trim().replace(HOST_SUFFIX_RE, "");

  return TRANSPORT_FAILURE_MESSAGES.some((known) => trimmed === known);
}

/**
 * True when an error message is one of the browser or runtime messages for a
 * request that never got a response: Chromium `Failed to fetch`, WebKit
 * `Load failed`, Firefox `NetworkError when attempting to fetch resource.`,
 * undici `fetch failed` and React Native `Network request failed`. It also
 * matches the same message with a host suffix, `Failed to fetch (api.example.com)`,
 * and a library message that ends with one after a colon, such as Zero's
 * `Fetch from API server threw error: fetch failed`.
 */
export function isTransportFailureMessage(message: string | null | undefined) {
  if (!message) {
    return false;
  }

  if (isBareTransportFailure(message)) {
    return true;
  }

  const separator = message.lastIndexOf(": ");

  return (
    separator !== -1 && isBareTransportFailure(message.slice(separator + 2))
  );
}

export const TRANSPORT_FAILURE_ERROR_TYPE = "transport_failure";

export const NETWORK_UNAVAILABLE = "unavailable";

/**
 * Exception event attributes the browser SDK records with a transport failure.
 * `errorType` is the semconv `error.type` and `connectionType` the semconv
 * `network.connection.type`; the page attributes have no semconv equivalent.
 */
export const TRANSPORT_CONTEXT_ATTRIBUTE = {
  connectionType: "network.connection.type",
  errorType: "error.type",
  pageLeaving: "interfere.page.leaving",
  visibility: "interfere.page.visibility",
} as const;
