import { readProcessEnv } from "@interfere/types/sdk/runtime";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const SCOPE_PREFIX = /^@[^/]+\//u;

function nonEmpty(value: string | undefined) {
  return value?.trim() || undefined;
}

function packageServiceName(root: string) {
  let manifest: unknown;

  try {
    manifest = JSON.parse(readFileSync(join(root, "package.json"), "utf8"));
  } catch {
    return;
  }

  if (
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Package manifests are untyped JSON; only a string name can identify an application.
    typeof manifest !== "object" ||
    manifest === null ||
    !("name" in manifest)
  ) {
    return;
  }

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Reject non-string package names before normalization.
  return typeof manifest.name === "string"
    ? nonEmpty(manifest.name.trim().replace(SCOPE_PREFIX, ""))
    : undefined;
}

/** Resolve a build's service name from an option, OTEL_SERVICE_NAME, or its unscoped app package name. */
export function resolveBuildServiceName({
  root,
  option,
  env = readProcessEnv() ?? {},
}: {
  root: string;
  option?: string | undefined;
  env?: Readonly<Record<string, string | undefined>>;
}) {
  return (
    nonEmpty(option) ??
    nonEmpty(env["OTEL_SERVICE_NAME"]) ??
    packageServiceName(root)
  );
}
