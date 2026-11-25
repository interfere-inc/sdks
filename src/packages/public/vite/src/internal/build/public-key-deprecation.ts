import type { Logger } from "vite";

export function warnIfPublicKeyDeprecated(
  logger: Pick<Logger, "warn">,
  publicKey: string | undefined,
) {
  if (!publicKey) {
    return;
  }

  logger.warn(
    "[interfere] VITE_INTERFERE_PUBLIC_KEY is deprecated and will be removed in 14.0. INTERFERE_API_KEY alone sets up a build; remove VITE_INTERFERE_PUBLIC_KEY from your environment.",
  );
}
