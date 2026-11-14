import type { UserConfig } from "tsdown";
import babel from "@rolldown/plugin-babel";
import reactCompiler from "babel-plugin-react-compiler";

const isWatch = process.argv.includes("--watch") || process.argv.includes("-w");

export default {
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
    "TypeScript 7.0 does not yet have a stable API and is experimental.",
  unbundle: false,
} satisfies UserConfig;
