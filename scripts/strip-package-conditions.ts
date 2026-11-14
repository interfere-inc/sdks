import { resolve } from "node:path";
import process from "node:process";

import { Glob } from "bun";

import { stripPackageConditions } from "../src/packages/public/test-utils/src/vitest/pack-manifest.ts";

const root = resolve(process.argv[2] ?? ".", "src/packages/public");

const manifests = await Array.fromAsync(
  new Glob("*/package.json").scan({ absolute: true, cwd: root }),
);

await Promise.all(manifests.map(async (file) => stripPackageConditions(file)));
