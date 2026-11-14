import { omitUndefined } from "@interfere/helpers/omit-undefined";
import {
  API_URL_GLOBAL,
  BUILD_MARKER_GLOBAL,
  ENVIRONMENT_GLOBAL,
  PUBLIC_KEY_GLOBAL,
  RELEASE_SLUG_GLOBAL,
} from "@interfere/types/sdk/injected-globals";
import { SERVICE_NAME_GLOBAL } from "@interfere/types/sdk/service-name";
import type { ResolvedBuildConfig } from "../../config.js";

export function interfereInjectedValues(
  config: ResolvedBuildConfig,
  apiHost: string | undefined,
  serviceName: string | undefined,
) {
  return omitUndefined({
    [SERVICE_NAME_GLOBAL]: serviceName,
    [RELEASE_SLUG_GLOBAL]: config.releaseSlug,
    [BUILD_MARKER_GLOBAL]: true,
    [PUBLIC_KEY_GLOBAL]: config.publicKey || undefined,
    [API_URL_GLOBAL]: config.publicKey && apiHost ? apiHost : undefined,
    [ENVIRONMENT_GLOBAL]: config.environment || undefined,
  });
}
