/// <reference types="node" />
import type { UserConfig } from "tsdown";

const isWatch = process.argv.includes("--watch") || process.argv.includes("-w");

export default {
  clean: !isWatch,
  deps: {
    neverBundle: (id) => id.startsWith("#"),
  },
  dts: { generator: "tsrs", sourcemap: false },
  entry: ["src/**/*.{ts,tsx}", "!**/*.{test,spec}.{ts,tsx}"],
  failOnWarn: "ci-only",
  checks: { legacyCjs: false },
  format: ["esm", "cjs"],
  logLevel: "warn",
  minify: { codegen: true, compress: true, mangle: false },
  sourcemap: false,
  suppressWarnings:
    "tsrs support is highly experimental and may change or break without notice. Some options will be unavailable.",
  unbundle: true,
} satisfies UserConfig;
