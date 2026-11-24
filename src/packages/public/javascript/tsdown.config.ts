import { fileURLToPath } from "node:url";
import type { UserConfig } from "tsdown";

const shared = {
  entry: ["src/index.ts"],
  format: ["esm", "cjs"],
  dts: { generator: "tsrs", sourcemap: false },
  minify: { codegen: true, compress: true, mangle: false },
  sourcemap: false,
  fixedExtension: true,
  failOnWarn: "ci-only",
  checks: { legacyCjs: false },
  logLevel: "warn",
  suppressWarnings:
    "tsrs support is highly experimental and may change or break without notice. Some options will be unavailable.",
} satisfies UserConfig;

export default [
  {
    ...shared,
    name: "node",
    platform: "node",
    outDir: "dist/node",
  },
  {
    ...shared,
    name: "portable",
    platform: "neutral",
    outDir: "dist/portable",
    dts: false,
    alias: {
      fs: fileURLToPath(new URL("runtime/portable-fs.ts", import.meta.url)),
    },
    deps: { alwaysBundle: ["fs"] },
    outputOptions: { codeSplitting: false },
  },
] satisfies UserConfig[];
