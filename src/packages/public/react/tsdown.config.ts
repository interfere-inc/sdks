import type { UserConfig } from "tsdown";
import babel from "@rolldown/plugin-babel";
import reactCompiler from "babel-plugin-react-compiler";
import { z } from "zod";

const entryMap = z.record(z.string(), z.string());

const isWatch = process.argv.includes("--watch") || process.argv.includes("-w");

const config: UserConfig = {
  checks: { moduleLevelDirective: false },
  clean: isWatch ? false : ["dist/**/*"],
  deps: {
    alwaysBundle: ["rrweb", "@rrweb/types"],
    dts: { alwaysBundle: ["@rrweb/types"] },
    neverBundle: (id) => id.startsWith("#"),
  },
  dts: { sourcemap: false },
  entry: ["src/**/*.{ts,tsx}", "!**/*.{test,spec,fixture,test-utils}.{ts,tsx}"],
  failOnWarn: "ci-only",
  // WARNING: tsdown passes glob entries in filesystem-race order, and rolldown derives chunk import order and hashes from entry order.
  inputOptions: (options) => ({
    ...options,
    input: Object.fromEntries(
      Object.entries(entryMap.parse(options.input)).toSorted(([a], [b]) =>
        a < b ? -1 : 1,
      ),
    ),
  }),
  logLevel: "warn",
  minify: { codegen: true, compress: true, mangle: false },
  plugins: [
    babel({
      include: /\.tsx$/u,
      exclude: /\.(?:test|spec)\.tsx$/u,
      plugins: [reactCompiler],
    }),
  ],
  sourcemap: false,
  target: ["safari16.4", "chrome111"],
  suppressWarnings:
    "TypeScript Go is experimental. Some options will be unavailable.",
  unbundle: false,
};

export default config;
