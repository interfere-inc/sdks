import type { UserConfig } from "tsdown";

const isWatch = process.argv.includes("--watch") || process.argv.includes("-w");

// WARNING: Identifier mangling collides with consumer Turbopack/webpack renaming and can crash SDK initialization.
const baseConfig = {
  checks: { moduleLevelDirective: false },
  clean: !isWatch,
  deps: {
    neverBundle: (id) => id.startsWith("#") || id.startsWith("cloudflare:"),
  },
  dts: { sourcemap: false },
  entry: ["src/**/*.{ts,tsx}", "!**/*.{test,spec}.{ts,tsx}"],
  failOnWarn: "ci-only",
  logLevel: "warn",
  minify: { codegen: true, compress: true, mangle: false },
  sourcemap: false,
  suppressWarnings:
    "TypeScript 7.0 does not yet have a stable API and is experimental.",
  unbundle: true,
} satisfies UserConfig;

export default baseConfig;
