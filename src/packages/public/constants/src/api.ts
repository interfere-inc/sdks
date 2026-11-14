export const API_URL = "https://api.interfere.com";

export type OtlpSignal = "traces" | "logs" | "metrics";

export const PUBLIC_KEY_QUERY = "public_key";

export const PUBLIC_KEY_HEADER = "x-interfere-public-key";

export function drainPath(signal: OtlpSignal) {
  return `/drain/v1/${signal}`;
}

export function withPublicKey(url: string, publicKey: string) {
  const next = new URL(url);

  next.searchParams.set(PUBLIC_KEY_QUERY, publicKey);

  return next.toString();
}

export function drainUrl(
  apiUrl: string,
  signal: OtlpSignal,
  publicKey: string,
) {
  return withPublicKey(`${apiUrl}${drainPath(signal)}`, publicKey);
}
