import type { NextConfig } from "next";
import type { Compiler } from "webpack";

import type { ProductionCompileContext } from "../../config.js";

/** Publishes after Next's final client compilation, which follows both server compilers even when build workers run in parallel. */
export function withLegacyWebpackPublication(
  existingWebpack: NextConfig["webpack"],
  context: ProductionCompileContext,
  publish: (context: ProductionCompileContext) => Promise<void>,
) {
  let publication: Promise<void> | undefined;

  return ((webpackConfig, options) => {
    const configured: unknown = existingWebpack
      ? existingWebpack(webpackConfig, options)
      : webpackConfig;

    if (options.dev || options.isServer) {
      return configured;
    }

    if (
      configured === null ||
      // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Customer webpack callbacks return arbitrary values; require an object before inspecting plugins.
      typeof configured !== "object" ||
      Array.isArray(configured)
    ) {
      throw new TypeError(
        "The webpack hook must return a configuration object.",
      );
    }

    const plugins: unknown =
      "plugins" in configured ? configured.plugins : undefined;

    if (plugins !== undefined && !Array.isArray(plugins)) {
      throw new TypeError("Webpack plugins must be an array.");
    }

    const configuredPlugins: unknown[] = plugins ?? [];

    Reflect.set(configured, "plugins", configuredPlugins);

    configuredPlugins.push({
      apply(compiler: Compiler) {
        compiler.hooks.done.tapPromise(
          {
            name: "InterfereSourceMapPublication",
            stage: Number.POSITIVE_INFINITY,
          },
          async (stats) => {
            if (stats.hasErrors()) {
              return;
            }

            publication ??= publish(context);

            await publication;
          },
        );
      },
    });

    return configured;
  }) satisfies NonNullable<NextConfig["webpack"]>;
}
