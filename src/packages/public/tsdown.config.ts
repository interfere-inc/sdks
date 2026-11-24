/// <reference types="node" />
import type { UserConfig } from "tsdown";
import {
  otelNeverBundle,
  otelVendored,
  vendorOpenTelemetry,
} from "./otel-vendor.ts";

const isWatch = process.argv.includes("--watch") || process.argv.includes("-w");

// WARNING: Identifier mangling collides with consumer Turbopack/webpack renaming and can crash SDK initialization.
const baseConfig: UserConfig = vendorOpenTelemetry({
  checks: { moduleLevelDirective: false },
  clean: !isWatch,
  deps: {
    alwaysBundle: [otelVendored],
    neverBundle: (id, importer) =>
      otelNeverBundle(id, importer) || id.startsWith("cloudflare:"),
  },
  dts: { generator: "tsrs", sourcemap: false },
  entry: ["src/**/*.{ts,tsx}", "!**/*.{test,spec}.{ts,tsx}"],
  failOnWarn: "ci-only",
  logLevel: "warn",
  minify: { codegen: true, compress: true, mangle: false },
  sourcemap: false,
  suppressWarnings:
    "tsrs support is highly experimental and may change or break without notice. Some options will be unavailable.",
  unbundle: true,
});

export default baseConfig;
