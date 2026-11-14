import { omitUndefined } from "@interfere/helpers/omit-undefined";

import { existsSync } from "node:fs";
import { basename, join, normalize, resolve } from "node:path";

import type { NextConfig } from "next";

import type { surfaceGlobals } from "./preflight.js";

/** Globals the loader assigns before the instrumentation module body: the build-time values plus the surface configuration snapshot. */
interface InterfereInjectedValues {
  readonly __INTERFERE_API_URL__?: string;
  readonly __INTERFERE_BUILD__?: boolean;
  readonly __INTERFERE_DISABLED__?: boolean;
  readonly __INTERFERE_ENVIRONMENT__?: string;
  readonly __INTERFERE_PUBLIC_KEY__?: string;
  readonly __INTERFERE_RELEASE_SLUG__?: string | null;
  readonly __INTERFERE_SERVICE_NAME__?: string;
}

/** The build-time values plus the surface snapshot globals the loader assigns. */
export type InjectedValues = InterfereInjectedValues &
  Partial<ReturnType<typeof surfaceGlobals>>;

const INSTRUMENTATION_EXTENSIONS = ["ts", "tsx", "js", "jsx"] as const;

function instrumentationCandidates(base: string) {
  return ["", "src/"].flatMap((dir) =>
    INSTRUMENTATION_EXTENSIONS.map((ext) => `${dir}${base}.${ext}`),
  );
}

const LOADER_FILE_CANDIDATES = ["ts", "js", "mjs", "cjs"].map(
  (ext) => `value-injection-loader.${ext}`,
);

interface ConfigureBuildInput {
  readonly existingTurbopack: NextConfig["turbopack"] | undefined;
  readonly existingWebpack: NextConfig["webpack"] | undefined;
  readonly nextMajor: number | null;
  readonly projectDir: string;
  readonly values: InjectedValues;
}

interface ConfigureBuildOutput {
  readonly clientInstrumentationApplies: boolean;
  readonly turbopack: NextConfig["turbopack"] | undefined;
  readonly webpack: NextConfig["webpack"] | undefined;
}

export function configureBuild({
  existingWebpack,
  existingTurbopack,
  nextMajor,
  projectDir,
  values,
}: ConfigureBuildInput) {
  const clientPath = resolveFirstExisting(
    projectDir,
    instrumentationCandidates("instrumentation-client"),
  );

  const serverPath = resolveFirstExisting(
    projectDir,
    instrumentationCandidates("instrumentation"),
  );

  const targets = [clientPath, serverPath].filter(
    (path): path is string => path !== null,
  );

  const hasMetadata =
    Boolean(values.__INTERFERE_RELEASE_SLUG__) ||
    values.__INTERFERE_PUBLIC_KEY__ !== undefined;

  const injectionApplies = targets.length > 0;
  const clientInstrumentationApplies = Boolean(clientPath) && hasMetadata;

  return {
    clientInstrumentationApplies,
    turbopack: buildTurbopackConfig(
      existingTurbopack,
      targets,
      values,
      injectionApplies,
      nextMajor,
    ),
    webpack: buildWebpackHook(
      existingWebpack,
      targets,
      values,
      injectionApplies,
    ),
  } satisfies ConfigureBuildOutput;
}

function buildWebpackHook(
  existingWebpack: NextConfig["webpack"] | undefined,
  targets: string[],
  values: InjectedValues,
  injectionApplies: boolean,
) {
  if (!injectionApplies) {
    return existingWebpack;
  }

  const normalizedTargets = new Set(targets.map((path) => normalize(path)));
  const injectionLoaderPath = resolveLoaderPath();

  return ((webpackConfig, options) => {
    const outputConfig: unknown = existingWebpack
      ? existingWebpack(webpackConfig, options)
      : webpackConfig;

    if (!isWebpackConfig(outputConfig)) {
      throw new TypeError(
        "The webpack hook must return a configuration object with an optional module.rules array.",
      );
    }

    const mutableConfig = outputConfig;

    mutableConfig.module ??= {};

    mutableConfig.module.rules ??= [];

    mutableConfig.module.rules.push({
      test: (resource: string) =>
        Boolean(resource) && normalizedTargets.has(normalize(resource)),
      use: [
        {
          loader: injectionLoaderPath,
          options: { serializedValues: JSON.stringify(values) },
        },
      ],
    });

    return outputConfig;
  }) satisfies NextConfig["webpack"] | undefined;
}

function isWebpackConfig(
  value: unknown,
): value is { module?: { rules?: unknown[] | null } | null } {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    return false;
  }

  if (
    !("module" in value) ||
    value.module === null ||
    value.module === undefined
  ) {
    return true;
  }

  const { module } = value;

  return (
    typeof module === "object" &&
    !Array.isArray(module) &&
    (!("rules" in module) ||
      module.rules === null ||
      module.rules === undefined ||
      Array.isArray(module.rules))
  );
}

const TURBOPACK_DEBUG_IDS_MIN_MAJOR = 16;

function buildTurbopackConfig(
  existingTurbopack: NextConfig["turbopack"] | undefined,
  targets: string[],
  values: InjectedValues,
  injectionApplies: boolean,
  nextMajor: number | null,
) {
  const baseConfig = existingTurbopack ?? {};

  const debugIdsSupported =
    nextMajor === null || nextMajor >= TURBOPACK_DEBUG_IDS_MIN_MAJOR;

  const debugIds = debugIdsSupported
    ? (baseConfig.debugIds ?? true)
    : baseConfig.debugIds;

  if (!injectionApplies) {
    return omitUndefined({
      ...baseConfig,
      debugIds,
    });
  }

  const rules: NonNullable<NonNullable<NextConfig["turbopack"]>["rules"]> = {
    ...baseConfig.rules,
  };

  const loaderPath = resolveLoaderPath();

  for (const target of targets) {
    const ruleKey = `**/${basename(target)}`;

    rules[ruleKey] = appendTurbopackLoader(rules[ruleKey], {
      loader: loaderPath,
      options: { serializedValues: JSON.stringify(values) },
    });
  }

  return omitUndefined({
    ...baseConfig,
    debugIds,
    rules,
  });
}

function appendTurbopackLoader(
  existing:
    | NonNullable<NonNullable<NextConfig["turbopack"]>["rules"]>[string]
    | undefined,
  loader: { loader: string; options: Record<string, string> },
) {
  if (Array.isArray(existing)) {
    return [...existing, { loaders: [loader] }];
  }

  return { ...existing, loaders: [...(existing?.loaders ?? []), loader] };
}

function resolveFirstExisting(
  projectDir: string,
  candidates: readonly string[],
) {
  for (const candidate of candidates) {
    const filePath = resolve(projectDir, candidate);

    if (existsSync(filePath)) {
      return filePath;
    }
  }

  return null;
}

function resolveLoaderPath() {
  const directory = resolve(import.meta.dirname);

  for (const candidate of LOADER_FILE_CANDIDATES) {
    const filePath = join(directory, candidate);

    if (existsSync(filePath)) {
      return filePath;
    }
  }

  return join(directory, "value-injection-loader.js");
}
