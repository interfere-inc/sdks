// oxlint-disable-next-line typescript/triple-slash-reference
/// <reference path="../vite-env.d.ts" />

import {
  BACKEND_HOSTS_GLOBAL,
  isBackendHosts,
} from "@interfere/types/sdk/remote-config/backend-hosts";

import {
  API_URL_GLOBAL,
  ENVIRONMENT_GLOBAL,
  INGEST_URL_GLOBAL,
  RELEASE_SLUG_GLOBAL,
} from "@interfere/types/sdk/injected-globals";
import { RELEASE_TOKEN_GLOBAL } from "@interfere/types/sdk/release/token";
import { PLUGINS_GLOBAL } from "@interfere/types/sdk/remote-config/plugins";
import { ANONYMOUS_REPLAY_GLOBAL } from "@interfere/types/sdk/remote-config/anonymous-replay";
import { SERVICE_NAME_GLOBAL } from "@interfere/types/sdk/service-name";

import { bakedReleaseSlug, bakedReleaseToken } from "./baked-release";

// oxlint-disable-next-line anti-slop/no-unsafe-dictionary-type -- Existing host-global values have unrelated contracts and must be preserved verbatim unless nullish.
type MutableGlobal = Record<string, unknown>;

// WARNING: Aliasing import.meta.env prevents Vite from replacing VITE_INTERFERE_* values in the bundle.
export function applyBrowserGlobals() {
  const g = globalThis as MutableGlobal;

  if (import.meta.env.VITE_INTERFERE_API_HOST) {
    g[API_URL_GLOBAL] ??= import.meta.env.VITE_INTERFERE_API_HOST;
  }

  if (import.meta.env.VITE_INTERFERE_ENVIRONMENT) {
    g[ENVIRONMENT_GLOBAL] ??= import.meta.env.VITE_INTERFERE_ENVIRONMENT;
  }

  const releaseSlug = bakedReleaseSlug();

  if (releaseSlug) {
    g[RELEASE_SLUG_GLOBAL] ??= releaseSlug;
  }

  const releaseToken = bakedReleaseToken();

  if (releaseToken) {
    g[RELEASE_TOKEN_GLOBAL] ??= releaseToken;
  }

  if (import.meta.env.VITE_INTERFERE_INGEST_URL) {
    g[INGEST_URL_GLOBAL] ??= import.meta.env.VITE_INTERFERE_INGEST_URL;
  }

  if (import.meta.env.VITE_INTERFERE_SERVICE_NAME) {
    g[SERVICE_NAME_GLOBAL] ??= import.meta.env.VITE_INTERFERE_SERVICE_NAME;
  }

  const bakedHosts = parseBakedHosts(
    import.meta.env.VITE_INTERFERE_BACKEND_HOSTS,
  );

  if (bakedHosts) {
    g[BACKEND_HOSTS_GLOBAL] ??= bakedHosts;
  }

  const bakedPlugins = parseBakedPlugins(
    import.meta.env.VITE_INTERFERE_PLUGINS,
  );

  if (bakedPlugins) {
    g[PLUGINS_GLOBAL] ??= bakedPlugins;
  }

  if (import.meta.env.VITE_INTERFERE_ANONYMOUS_REPLAY) {
    g[ANONYMOUS_REPLAY_GLOBAL] ??=
      import.meta.env.VITE_INTERFERE_ANONYMOUS_REPLAY;
  }
}

function parseBakedPlugins(raw: string | null | undefined) {
  if (!raw) {
    return;
  }

  try {
    const parsed: unknown = JSON.parse(raw);

    // oxlint-disable-next-line anti-slop/no-runtime-typeof
    if (typeof parsed !== "object") {
      return;
    }

    return parsed !== null && !Array.isArray(parsed) ? parsed : undefined;
  } catch {
    return;
  }
}

function parseBakedHosts(raw: string | null | undefined) {
  if (!raw) {
    return;
  }

  try {
    const parsed: unknown = JSON.parse(raw);

    return isBackendHosts(parsed) ? parsed : undefined;
  } catch {
    return;
  }
}
