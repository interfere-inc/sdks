import type { UserConfig } from "tsdown";

export default {
  entry: ["src/interfere.ts"],
  format: "esm",
  platform: "node",
  target: "node20",
  dts: false,
  minify: { codegen: true, compress: true, mangle: false },
  sourcemap: false,
  failOnWarn: "ci-only",
  logLevel: "warn",
} satisfies UserConfig;
