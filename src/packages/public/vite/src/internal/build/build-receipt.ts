import type { createBuildConfigResolver } from "@interfere/build/config";
import type { ReleaseSlug } from "@interfere/types/releases/slug";

import { hash } from "node:crypto";
import { mkdir, readFile, rm, writeFile } from "node:fs/promises";
import path from "node:path";
import type { ResolvedConfig } from "vite";
import type { runReleasePipeline } from "./pipeline";
import { canonicalBuildPath } from "./canonical-build-path";

type ConfigObservation = Parameters<
  NonNullable<Parameters<typeof createBuildConfigResolver>[0]>
>[0];

function contains(directory: string, file: string) {
  const relative = path.relative(directory, file);

  return (
    relative === "" ||
    (relative !== ".." &&
      !relative.startsWith(`..${path.sep}`) &&
      !path.isAbsolute(relative))
  );
}

export function createBuildReceipt(filename: string) {
  let destination: string | null = null;
  let root = "";
  let config: ConfigObservation | null = null;

  let identity: {
    commitSha: string | null;
    releaseSlug: ReleaseSlug | null;
    environment: string | null;
    publicKey: string | undefined;
    apiOrigin: string;
    producerVersion: string;
    serviceName?: string | undefined;
  } | null = null;

  let publication: NonNullable<
    Awaited<ReturnType<typeof runReleasePipeline>>["publication"]
  > | null = null;

  const outputs = new Set<string>();
  const directories = new Set<string>();

  function assertPrivate(file: string) {
    if ([...directories].some((directory) => contains(directory, file))) {
      throw new Error(
        "Build receipt must be outside public and build output directories",
      );
    }
  }

  return {
    observeConfig(this: void, observation: ConfigObservation) {
      config = observation;
    },
    async configure(
      resolved: ResolvedConfig,
      build: NonNullable<typeof identity>,
    ) {
      root = await canonicalBuildPath(resolved.root);

      identity = build;

      directories.add(
        await canonicalBuildPath(path.resolve(root, resolved.build.outDir)),
      );

      if (resolved.publicDir) {
        directories.add(await canonicalBuildPath(resolved.publicDir));
      }

      const next = await canonicalBuildPath(path.resolve(root, filename));

      assertPrivate(next);

      if (!contains(root, next)) {
        throw new Error("Build receipt must be inside the project directory");
      }

      if (destination !== next) {
        await rm(next, { force: true });
      }

      destination = next;
    },
    async recordOutput(directory: string, files: string[]) {
      const absolute = await canonicalBuildPath(path.resolve(root, directory));

      directories.add(absolute);

      if (destination) {
        assertPrivate(destination);
      }

      for (const file of files) {
        if (!file.endsWith(".map")) {
          outputs.add(path.resolve(absolute, file));
        }
      }
    },
    recordPublication(source: NonNullable<typeof publication>) {
      publication = source;
    },
    async write() {
      if (!destination || !identity) {
        return;
      }

      assertPrivate(destination);

      const assets = await Promise.all(
        [...outputs].toSorted().map(async (file) => ({
          path: path.relative(root, file).split(path.sep).join("/"),
          sha256: hash("sha256", await readFile(file)),
        })),
      );

      const receipt = {
        schemaVersion: 1,
        build: { ...identity, buildId: identity.commitSha },
        config: config
          ? {
              status: config.status,
              consumed: config.config,
              digest:
                config.status === "fetched"
                  ? hash("sha256", JSON.stringify(config.config))
                  : null,
            }
          : { status: "unavailable", consumed: null, digest: null },
        source: publication,
        assets,
      };

      await mkdir(path.dirname(destination), { recursive: true });

      await writeFile(destination, `${JSON.stringify(receipt, null, 2)}\n`, {
        mode: 0o600,
      });
    },
  };
}
