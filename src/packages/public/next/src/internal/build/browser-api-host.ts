import { API_URL } from "@interfere/constants/api";

import { log } from "../logger.js";

export function resolveBrowserApiHost(
  apiHost: string | undefined,
  apiUrl: string,
) {
  const host = apiHost ?? apiUrl;

  if (host === API_URL) {
    return;
  }

  return normalizeHost(host);
}

function normalizeHost(host: string) {
  try {
    return new URL(host).origin;
  } catch {
    return log.fatal("Invalid apiHost", [
      `interfere.apiHost must be a full URL, e.g. "https://in.yourapp.com".`,
      `Received: ${host}`,
    ]);
  }
}
