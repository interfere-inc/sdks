import { omitUndefined } from "@interfere/helpers/omit-undefined";
import type { ReleaseSlug } from "@interfere/types/releases/slug";
import { ENVIRONMENT_GLOBAL } from "@interfere/types/sdk/injected-globals";
import { resolveEnvironment } from "@interfere/types/sdk/runtime";

import { getStringGlobal } from "../../util/global.js";
import { SDK_NAME, SDK_VERSION } from "../version.js";

export interface ResourceAttributesInput {
  releaseSlug?: ReleaseSlug | null | undefined;
  sdkStack: string[];
  serviceName?: string | undefined;
  serviceNamespace?: string | undefined;
}

function readBundledEnv(read: () => string | undefined) {
  try {
    return read();
  } catch {
    return;
  }
}

/** The resource every browser export carries, shared by the tracing providers and the capture-only encoder. */
export function buildResourceAttributes(input: ResourceAttributesInput) {
  // WARNING: Aliasing process.env or guarding on `typeof process` prevents bundler substitution and silently selects development in browser builds.
  const environment = resolveEnvironment({
    explicit: getStringGlobal(ENVIRONMENT_GLOBAL),
    env: {
      INTERFERE_ENVIRONMENT: readBundledEnv(
        // oxlint-disable-next-line node/no-process-env
        () => process.env["INTERFERE_ENVIRONMENT"],
      ),
      NODE_ENV: readBundledEnv(() => process.env["NODE_ENV"]),
      // oxlint-disable-next-line node/no-process-env
      VERCEL_ENV: readBundledEnv(() => process.env["VERCEL_ENV"]),
    },
  });

  return omitUndefined({
    "service.name": input.serviceName || "unknown_service",
    "service.namespace": input.serviceNamespace,
    "deployment.environment.name": environment,
    "telemetry.sdk.language": "webjs",
    "interfere.sdk.name": SDK_NAME,
    "interfere.sdk.version": SDK_VERSION,
    "interfere.sdk.stack": input.sdkStack.join(", "),
    "release.slug": input.releaseSlug || undefined,
  });
}
