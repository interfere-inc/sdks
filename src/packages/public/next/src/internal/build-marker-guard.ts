import {
  BUILD_MARKER_GLOBAL,
  DISABLED_GLOBAL,
} from "@interfere/types/sdk/injected-globals";

export function buildIntegrationRan() {
  const globals: object = globalThis;

  return (
    BUILD_MARKER_GLOBAL in globals && globals[BUILD_MARKER_GLOBAL] !== undefined
  );
}

export function interfereDisabled() {
  const globals: object = globalThis;

  return DISABLED_GLOBAL in globals && globals[DISABLED_GLOBAL] === true;
}

export function hasEnvCredentials() {
  if (typeof process === "undefined") {
    return false;
  }

  return (
    // oxlint-disable-next-line node/no-process-env
    Boolean(process.env["INTERFERE_PUBLIC_KEY"])
  );
}

export function warnBuildIntegrationMissing() {
  console.warn(
    "[interfere] withInterfere() did not run for this build, so Interfere is inactive and will send no events. Another Next config wrapper can silently drop it when wrapping withInterfere(). Make withInterfere() the outermost wrapper in next.config, or upgrade the wrapping tool.",
  );
}

export function warnBuildIntegrationDegraded() {
  console.warn(
    "[interfere] withInterfere() did not run for this build. Interfere will still send events using credentials from the environment, but release association and sourcemaps are unavailable. Make withInterfere() the outermost wrapper in next.config to restore them.",
  );
}
