import type { ReleaseSlug } from "../releases/slug.ts";

export const API_KEY_PREFIXES = [
  "interfere_secret_us_",
  "interfere_secret_eu_",
  "interfere_ak_",
] as const;

const API_KEY_PREVIEW_LENGTH = 12;

const DASHBOARD_KEYS_URL = "https://interfere.com/settings/api-keys";

const STATUS_URL = "https://status.interfere.com";

const SUPPORT_EMAIL = "support@interfere.com";

const DASHBOARD_SECURITY_URL = "https://interfere.com/settings/security";

const API_KEY_PREFIX_HINT = API_KEY_PREFIXES.map(
  (prefix) => `\`${prefix}\``,
).join(" or ");

const BUILD_CONTINUES =
  "The build continues, but this release has no source maps in Interfere.";

export interface ReleaseDiagnosticContext {
  readonly publicKeyEnvVar: string;
  readonly docsUrl: string;
}

export function formatApiKeyPreview(apiKey: string) {
  const preview = apiKey.slice(0, API_KEY_PREVIEW_LENGTH);

  return apiKey.length > API_KEY_PREVIEW_LENGTH ? `${preview}...` : preview;
}

export function isInterfereApiKey(apiKey: string) {
  return API_KEY_PREFIXES.some((prefix) => apiKey.startsWith(prefix));
}

function publicKeyHint(ctx: ReleaseDiagnosticContext) {
  return `If you set a public key (\`interfere_public_us_*\`, \`interfere_public_eu_*\`, or legacy \`interfere_pub_*\`/\`interfere_pk_*\`), put it in \`${ctx.publicKeyEnvVar}\` instead.`;
}

export function checkReleaseConfig(
  ctx: ReleaseDiagnosticContext,
  input: {
    readonly apiKey: string | null | undefined;
    readonly commitSha: string | null;
    readonly releaseSlug: ReleaseSlug | null;
  },
) {
  const { apiKey, commitSha, releaseSlug } = input;

  if (!apiKey) {
    return {
      status: "skip" as const,
      title: "Skipping release",
      details: [
        "INTERFERE_API_KEY is not set, so this build has no release or source maps in Interfere.",
        "Set INTERFERE_API_KEY on builds that you deploy.",
        `See: ${ctx.docsUrl}`,
      ],
    };
  }

  if (!isInterfereApiKey(apiKey)) {
    return {
      status: "fail" as const,
      title: "INTERFERE_API_KEY has wrong prefix",
      details: [
        `Expected a key starting with ${API_KEY_PREFIX_HINT} but got \`${formatApiKeyPreview(apiKey)}\`.`,
        publicKeyHint(ctx),
        `Generate a new secret key at ${DASHBOARD_KEYS_URL}.`,
      ],
    };
  }

  if (!(commitSha && releaseSlug)) {
    return {
      status: "warn" as const,
      title: "Commit SHA missing",
      details: [
        "Could not resolve a commit SHA from environment variables or git.",
        "Set `INTERFERE_SOURCE_ID`, ensure your CI exposes its git SHA env var (e.g. `VERCEL_GIT_COMMIT_SHA`, `GITHUB_SHA`), or run inside a git checkout.",
        BUILD_CONTINUES,
      ],
    };
  }

  return { status: "ok" as const, apiKey, commitSha, releaseSlug };
}

export type ReleaseDiagnosticCode =
  | "API_KEY_UNREADABLE"
  | "API_KEY_NETWORK_DENIED"
  | "API_KEY_REJECTED"
  | "API_KEY_SCOPE_MISMATCH"
  | "API_KEY_SURFACE_REQUIRED"
  | "RELEASE_PIPELINE_FAILED";

export interface ReleaseDiagnostic {
  readonly code: ReleaseDiagnosticCode;
  readonly details: readonly string[];
  readonly severity: "error" | "warn";
  readonly title: string;
}

const CATALOG: Record<
  ReleaseDiagnosticCode,
  (ctx: ReleaseDiagnosticContext, errorMessage: string) => ReleaseDiagnostic
> = {
  API_KEY_NETWORK_DENIED: () => ({
    code: "API_KEY_NETWORK_DENIED",
    severity: "error",
    title: "Build machine blocked by network policy",
    details: [
      "INTERFERE_API_KEY was recognized, but the workspace network policy blocks this build machine.",
      `Allow your CI egress IPs or ASN under Network access at ${DASHBOARD_SECURITY_URL}.`,
      BUILD_CONTINUES,
    ],
  }),
  API_KEY_REJECTED: () => ({
    code: "API_KEY_REJECTED",
    severity: "error",
    title: "Secret key rejected by server",
    details: [
      "INTERFERE_API_KEY was sent but the server did not recognize it.",
      `Verify it matches an active key at ${DASHBOARD_KEYS_URL}.`,
      "If you recently rotated keys, restart your build with the new value.",
      BUILD_CONTINUES,
    ],
  }),
  API_KEY_SCOPE_MISMATCH: () => ({
    code: "API_KEY_SCOPE_MISMATCH",
    severity: "error",
    title: "Secret key missing release scope",
    details: [
      "INTERFERE_API_KEY was recognized but cannot publish release metadata.",
      `Create or update the key with \`release:write\` scope at ${DASHBOARD_KEYS_URL}.`,
      "If you recently changed key scopes, restart your build with the new value.",
      BUILD_CONTINUES,
    ],
  }),
  API_KEY_SURFACE_REQUIRED: (ctx) => ({
    code: "API_KEY_SURFACE_REQUIRED",
    severity: "error",
    title: "Service needed for a workspace secret key",
    details: [
      "INTERFERE_API_KEY is a workspace secret key, so the build must name the service it publishes for, and register that service's release before uploading source maps.",
      `Set \`serviceName\` (or \`OTEL_SERVICE_NAME\`) to that service, or set \`${ctx.publicKeyEnvVar}\` on the build machine.`,
      BUILD_CONTINUES,
    ],
  }),
  API_KEY_UNREADABLE: (ctx) => ({
    code: "API_KEY_UNREADABLE",
    severity: "error",
    title: "Secret key could not be read",
    details: [
      "INTERFERE_API_KEY was sent but the server could not read it.",
      `Expected a key starting with ${API_KEY_PREFIX_HINT}.`,
      publicKeyHint(ctx),
      `Generate a new secret key at ${DASHBOARD_KEYS_URL}.`,
      BUILD_CONTINUES,
    ],
  }),
  RELEASE_PIPELINE_FAILED: (_ctx, errorMessage) => ({
    code: "RELEASE_PIPELINE_FAILED",
    severity: "warn",
    title: "Skipping release pipeline",
    details: [
      "Interfere failed to process this release.",
      "Events from this release will be ingested, but ignored as finding when generating insights.",
      `Please check ${STATUS_URL} for any outages, or contact ${SUPPORT_EMAIL} if the problem persists.`,
      `Error: ${errorMessage}`,
    ],
  }),
};

const API_CODE_TO_DIAGNOSTIC = new Map(
  Object.entries({
    AUTH_AK_INVALID: "API_KEY_REJECTED",
    AUTH_AK_SCOPE_MISMATCH: "API_KEY_SCOPE_MISMATCH",
    AUTH_AK_SURFACE_REQUIRED: "API_KEY_SURFACE_REQUIRED",
    AUTH_MISSING: "API_KEY_UNREADABLE",
    NETWORK_ACCESS_DENIED: "API_KEY_NETWORK_DENIED",
    RELEASE_SERVICE_REQUIRED: "API_KEY_SURFACE_REQUIRED",
  } satisfies Record<string, ReleaseDiagnosticCode>),
);

export function classifyReleasePipelineError(
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Build failures may be arbitrary thrown values; only recognized provider codes receive a specific diagnostic.
  error: unknown,
) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Primitive failures have no provider error-code field and use the generic diagnostic.
  if (typeof error !== "object" || error === null) {
    return "RELEASE_PIPELINE_FAILED";
  }

  const body = "body" in error ? error.body : undefined;

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- API failures can carry arbitrary response bodies.
  if (typeof body !== "object" || body === null) {
    return "RELEASE_PIPELINE_FAILED";
  }

  const code = "code" in body ? body.code : undefined;

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Non-string provider codes must not be coerced into diagnostic lookup keys.
  if (typeof code !== "string") {
    return "RELEASE_PIPELINE_FAILED";
  }

  return API_CODE_TO_DIAGNOSTIC.get(code) ?? "RELEASE_PIPELINE_FAILED";
}

export function diagnoseRelease(
  ctx: ReleaseDiagnosticContext,
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Build failures may be arbitrary thrown values; only recognized provider codes receive a specific diagnostic.
  error: unknown,
) {
  const errorMessage = error instanceof Error ? error.message : String(error);

  return CATALOG[classifyReleasePipelineError(error)](ctx, errorMessage);
}
