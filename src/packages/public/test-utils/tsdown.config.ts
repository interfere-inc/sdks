import type { UserConfig } from "tsdown";

const isWatch = process.argv.includes("--watch") || process.argv.includes("-w");

export default {
  checks: { moduleLevelDirective: false },
  clean: !isWatch,
  deps: {
    neverBundle: (id) =>
      id.startsWith("#") || id.startsWith("cloudflare:") || id === "vitest",
  },
  dts: { sourcemap: false },
  entry: ["src/**/*.{ts,tsx}", "!**/*.{test,spec}.{ts,tsx}"],
  failOnWarn: "ci-only",
  logLevel: "warn",
  minify: { codegen: true, compress: true, mangle: false },
  sourcemap: false,
  suppressWarnings:
    "TypeScript Go is experimental. Some options will be unavailable.",
  unbundle: true,
} satisfies UserConfig;
