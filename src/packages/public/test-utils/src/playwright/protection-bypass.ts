/* oxlint-disable typescript/no-unnecessary-condition */
const LOCAL_HOSTNAMES = new Set(["localhost", "127.0.0.1", "0.0.0.0", "[::1]"]);

export type ProtectionProvider = "cloudflare-access";

function isLocal(baseURL: string) {
  try {
    return LOCAL_HOSTNAMES.has(new URL(baseURL).hostname);
  } catch {
    return false;
  }
}

function cloudflareAccessHeaders(baseURL: string) {
  // oxlint-disable-next-line node/no-process-env
  const clientId = process.env["CF_ACCESS_CLIENT_ID"];
  // oxlint-disable-next-line node/no-process-env
  const clientSecret = process.env["CF_ACCESS_CLIENT_SECRET"];

  if (!(clientId && clientSecret)) {
    throw new Error(
      [
        "Missing CF_ACCESS_CLIENT_ID / CF_ACCESS_CLIENT_SECRET while baseURL points at a remote Cloudflare deployment.",
        `baseURL=${baseURL}`,
        "Set both environment variables to a Cloudflare Access service token authorized for this deployment.",
      ].join(" "),
    );
  }

  return {
    "CF-Access-Client-Id": clientId,
    "CF-Access-Client-Secret": clientSecret,
  };
}

export function protectionBypassHeaders(
  baseURL: string | undefined,
  provider: ProtectionProvider,
) {
  if (!baseURL || isLocal(baseURL)) {
    return;
  }

  if (provider === "cloudflare-access") {
    return cloudflareAccessHeaders(baseURL);
  }

  throw new Error(`Unsupported protection provider: ${String(provider)}`);
}
