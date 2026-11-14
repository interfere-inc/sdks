import { z } from "zod";

const HOST_REGEX = /^https?:\/\//u;

const PATH_REGEX = /\/.*$/u;

const githubSourceIdEnvKeySchema = z.enum([
  "VERCEL_GIT_COMMIT_SHA",
  "GITHUB_SHA",
]);

export const githubSourceIdEnvKeys = githubSourceIdEnvKeySchema.options;

export const githubHostFlavorSchema = z.enum([
  "github_com",
  "ghe_cloud",
  "ghe_server",
]);

export type GithubHostFlavor = z.infer<typeof githubHostFlavorSchema>;

export const GITHUB_COM_HOST = "github.com";

export const GITHUB_COM_API_BASE_URL = "https://api.github.com";

export const GITHUB_COM_WEB_BASE_URL = "https://github.com";

const githubMetadata = z.object({
  accountType: z.string(),
  avatarUrl: z.string().nullable().default(null),
  flavor: githubHostFlavorSchema.default("github_com"),
  host: z.string().default(GITHUB_COM_HOST),
  installationUrl: z.string().nullable(),
  repositories: z.record(z.string(), z.string()),
});

export const githubIntegration = z.object({
  metadata: githubMetadata,
  name: z.literal("github"),
});

export const gheCloudIntegration = z.object({
  metadata: githubMetadata,
  name: z.literal("ghe_cloud"),
});

export const gheServerIntegration = z.object({
  metadata: githubMetadata,
  name: z.literal("ghe_server"),
});

export const githubFamilyProviders = [
  githubIntegration.shape.name.value,
  gheCloudIntegration.shape.name.value,
  gheServerIntegration.shape.name.value,
] as const;

export type GithubFamilyProvider = (typeof githubFamilyProviders)[number];

export const PROVIDER_FLAVOR: Record<GithubFamilyProvider, GithubHostFlavor> = {
  ghe_cloud: "ghe_cloud",
  ghe_server: "ghe_server",
  github: "github_com",
};

export interface GithubHostUrls {
  apiBaseUrl: string;
  flavor: GithubHostFlavor;
  host: string;
  webBaseUrl: string;
}

export function normalizeGithubHost(host: string) {
  return host
    .trim()
    .toLowerCase()
    .replace(HOST_REGEX, "")
    .replace(PATH_REGEX, "");
}

export function detectGithubHostFlavor(host: string) {
  const normalized = normalizeGithubHost(host);

  if (normalized === GITHUB_COM_HOST) {
    const returnValue: GithubHostFlavor = "github_com";

    return returnValue;
  }

  if (normalized.endsWith(".ghe.com")) {
    const returnValue: GithubHostFlavor = "ghe_cloud";

    return returnValue;
  }

  const returnValue: GithubHostFlavor = "ghe_server";

  return returnValue;
}

export function providerFromHost(host: string) {
  const flavor = detectGithubHostFlavor(normalizeGithubHost(host));

  return (
    flavor === "github_com" ? "github" : flavor
  ) satisfies GithubFamilyProvider;
}

const HOST_URL_RESOLVERS: Record<
  GithubHostFlavor,
  (host: string) => GithubHostUrls
> = {
  ghe_cloud: (host) => ({
    host,
    flavor: "ghe_cloud",
    apiBaseUrl: `https://api.${host}`,
    webBaseUrl: `https://${host}`,
  }),
  ghe_server: (host) => ({
    host,
    flavor: "ghe_server",
    apiBaseUrl: `https://${host}/api/v3`,
    webBaseUrl: `https://${host}`,
  }),
  github_com: () => ({
    host: GITHUB_COM_HOST,
    flavor: "github_com",
    apiBaseUrl: GITHUB_COM_API_BASE_URL,
    webBaseUrl: GITHUB_COM_WEB_BASE_URL,
  }),
};

export function resolveGithubHostUrls(input: {
  host: string;
  flavor?: GithubHostFlavor;
}) {
  const host = normalizeGithubHost(input.host);
  const flavor = input.flavor ?? detectGithubHostFlavor(host);

  return HOST_URL_RESOLVERS[flavor](host);
}
