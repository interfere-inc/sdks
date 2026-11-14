import type { UserConfig } from "tsdown";

const isWatch = process.argv.includes("--watch") || process.argv.includes("-w");

export default {
  clean: !isWatch,
  deps: {
    neverBundle: (id) => id.startsWith("#"),
  },
  dts: { sourcemap: false },
  entry: ["src/**/*.{ts,tsx}", "!**/*.{test,spec}.{ts,tsx}"],
  failOnWarn: "ci-only",
  checks: { legacyCjs: false },
  format: ["esm", "cjs"],
  logLevel: "warn",
  minify: { codegen: true, compress: true, mangle: false },
  sourcemap: false,
  suppressWarnings:
    "TypeScript 7.0 does not yet have a stable API and is experimental.",
  unbundle: true,
} satisfies UserConfig;
