import { cpSync, globSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import type { UserConfig } from "tsdown";
import babel from "@rolldown/plugin-babel";
import reactCompiler from "babel-plugin-react-compiler";
import { z } from "zod";

const entryMap = z.record(z.string(), z.string());

const isWatch = process.argv.includes("--watch") || process.argv.includes("-w");

// oxlint-disable-next-line node/no-process-env -- build:types selects its declaration-only output through the environment.
const declarations = process.env["TSDOWN_DECLARATIONS"] === "1";

const staging = "node_modules/.cache/declarations";

function publishDeclarations() {
  for (const file of globSync("**/*.d.mts", { cwd: staging })) {
    mkdirSync(dirname(join("dist", file)), { recursive: true });

    cpSync(join(staging, file), join("dist", file));
  }
}

function cleaning() {
  if (declarations) {
    return true;
  }

  return isWatch ? false : ["dist/**/*"];
}

const config: UserConfig = {
  checks: { moduleLevelDirective: false },
  clean: cleaning(),
  deps: {
    alwaysBundle: [
      "rrweb",
      "@rrweb/types",
      "@opentelemetry/browser-instrumentation/experimental/fetch",
      "@opentelemetry/browser-instrumentation/experimental/xhr",
    ],
    dts: { alwaysBundle: ["@rrweb/types"] },
    neverBundle: (id) => id.startsWith("#"),
  },
  dts: { sourcemap: false },
  entry: ["src/**/*.{ts,tsx}", "!**/*.{test,spec,fixture,test-utils}.{ts,tsx}"],
  failOnWarn: "ci-only",
  hooks: declarations ? { "build:done": publishDeclarations } : {},
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
  outDir: declarations ? staging : "dist",
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
