import { builtinModules } from "node:module";
import { fileURLToPath } from "node:url";
import { cloudflareTest } from "@cloudflare/vitest-plugin";
import { defineConfig } from "vitest/config";

export default defineConfig({
  root: import.meta.dirname,
  plugins: [
    cloudflareTest({
      main: "./test/workerd/server-worker.ts",
      remoteBindings: false,
      miniflare: {
        compatibilityDate: "2026-07-06",
        compatibilityFlags: ["nodejs_compat"],
      },
    }),
  ],
  resolve: {
    alias: {
      "#request-flush": fileURLToPath(
        new URL(
          "src/internal/server/request/flush.workerd.ts",
          import.meta.url,
        ),
      ),
      "#outbound-instrumentation": fileURLToPath(
        new URL(
          "src/internal/server/outbound-instrumentation.workerd.ts",
          import.meta.url,
        ),
      ),
    },
    conditions: ["workerd"],
  },
  ssr: { resolve: { conditions: ["workerd"] } },
  test: {
    fsModuleCache: false,
    name: "@interfere/vite/native",
    env: { VITE_INTERFERE_PUBLIC_KEY: "interfere_pub_us_test_123" },
    include: ["src/**/*.native.test.ts"],
    deps: {
      optimizer: {
        ssr: {
          enabled: true,
          include: [
            "@opentelemetry/api",
            "@opentelemetry/context-async-hooks",
            "@opentelemetry/core",
            "@opentelemetry/sdk-trace",
            "zod",
          ],
          rolldownOptions: {
            external: [
              "cloudflare:workers",
              ...builtinModules,
              ...builtinModules.map((name) => `node:${name}`),
            ],
            platform: "node",
          },
        },
      },
    },
  },
});
