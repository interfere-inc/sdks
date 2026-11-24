import { resolveBuildReleaseSlug } from "@interfere/build/release-slug";
import { resolveBuildServiceName } from "@interfere/build/service-name";
import type { ReleaseSlug } from "@interfere/types/releases/slug";
import { resolveDeployEnvironment } from "@interfere/types/sdk/deploy-environment";

import { runGitCommand } from "./git";
import {
  BAKED_RELEASE_SLUG_DEFINE,
  BAKED_RELEASE_TOKEN_DEFINE,
} from "../baked-release";
import type { InterfereViteOptions } from "../plugin-options";
import { resolveCommitSha } from "../release-slug";

export interface BrowserConfig {
  apiHost: string | undefined;
  commitSha: string | null;
  environment: string | null;
  publicKey: string | undefined;
  releaseSlug: ReleaseSlug | null;
  serviceName: string | undefined;
}

function normalizeOrigin(host: string) {
  try {
    return new URL(host).origin;
  } catch {
    return null;
  }
}

function resolveApiHost(options: InterfereViteOptions) {
  const host = options.apiHost;

  if (!host) {
    return;
  }

  const origin = normalizeOrigin(host);

  if (!origin) {
    throw new Error(
      'interfere({ apiHost }) requires a full URL, e.g. apiHost: "https://in.yourapp.com"',
    );
  }

  return origin;
}

export function resolveBrowserConfig(
  options: InterfereViteOptions,
  env: Record<string, string | undefined>,
  command: "build" | "serve",
  root: string,
) {
  const commitSha =
    options.commitSha ??
    resolveCommitSha((gitCommand) => runGitCommand(gitCommand, root));

  return {
    apiHost: resolveApiHost(options),
    commitSha,
    environment:
      command === "build"
        ? // oxlint-disable-next-line node/no-process-env
          resolveDeployEnvironment({ ...env, ...process.env })
        : null,
    publicKey: env["VITE_INTERFERE_PUBLIC_KEY"],
    serviceName: resolveBuildServiceName({
      option: options.serviceName,
      env,
      root,
    }),
    releaseSlug: resolveBuildReleaseSlug({
      commitSha,
      deploymentId:
        command === "build"
          ? // oxlint-disable-next-line node/no-process-env
            (process.env["VERCEL_DEPLOYMENT_ID"] ?? env["VERCEL_DEPLOYMENT_ID"])
          : undefined,
      variant: options.buildVariant,
    }),
  } satisfies BrowserConfig;
}

export function browserDefine(
  browser: BrowserConfig,
  releaseToken: string | null,
  command: "build" | "serve",
) {
  const baked = command === "build";

  return {
    "import.meta.env.VITE_INTERFERE_API_HOST": JSON.stringify(
      browser.apiHost ?? null,
    ),
    "import.meta.env.VITE_INTERFERE_ENVIRONMENT": JSON.stringify(
      browser.environment,
    ),
    [baked
      ? BAKED_RELEASE_SLUG_DEFINE
      : "import.meta.env.VITE_INTERFERE_RELEASE_SLUG"]: JSON.stringify(
      browser.releaseSlug,
    ),
    [baked
      ? BAKED_RELEASE_TOKEN_DEFINE
      : "import.meta.env.VITE_INTERFERE_RELEASE_TOKEN"]:
      JSON.stringify(releaseToken),
    "import.meta.env.VITE_INTERFERE_SERVICE_NAME": JSON.stringify(
      browser.serviceName ?? null,
    ),
  };
}
