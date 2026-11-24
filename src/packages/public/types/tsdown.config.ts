/// <reference types="node" />
import type { UserConfig } from "tsdown";
// @boundaries-ignore the SDK packages share one OpenTelemetry vendoring plugin, which the SDK mirror exports beside them
import {
  otelNeverBundle,
  otelVendored,
  vendorOpenTelemetry,
} from "../otel-vendor.ts";

const isWatch = process.argv.includes("--watch") || process.argv.includes("-w");

const config: UserConfig = vendorOpenTelemetry({
  clean: !isWatch,
  deps: {
    alwaysBundle: [otelVendored],
    neverBundle: otelNeverBundle,
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
});

export default config;
