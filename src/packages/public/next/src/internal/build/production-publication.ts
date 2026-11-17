import { omitUndefined } from "@interfere/helpers/omit-undefined";
import type { NextConfig } from "next";
import type {
  ProductionCompileContext,
  ResolvedBuildConfig,
} from "../../config.js";
import { FatalError, log } from "../logger.js";

export function interfereCompilerConfig(
  compiler: NextConfig["compiler"],
  config: ResolvedBuildConfig,
  publish: (
    context: ProductionCompileContext,
    config: ResolvedBuildConfig,
  ) => Promise<void>,
) {
  const existingHook = compiler?.runAfterProductionCompile;

  return omitUndefined({
    ...compiler,
    async runAfterProductionCompile(context: ProductionCompileContext) {
      if (existingHook) {
        await existingHook(context);
      }

      if (config.publicKey === null && !config.registeredRelease?.token) {
        log.warn("No browser credential", [
          "This build has no release token and no INTERFERE_PUBLIC_KEY, so it sends no browser events.",
          "Set INTERFERE_API_KEY on builds that you deploy and withInterfere() bakes in a release token.",
        ]);
      }

      try {
        await publish(context, config);
      } catch (error) {
        if (error instanceof FatalError) {
          throw error;
        }

        const message = error instanceof Error ? error.message : String(error);

        log.warn("Skipping release pipeline", [
          "Interfere encountered an unexpected error.",
          "Build will continue, but this release's data will be ingested, but ignored as finding when generating insights.",
          `Error: ${message}`,
        ]);
      }
    },
  });
}
