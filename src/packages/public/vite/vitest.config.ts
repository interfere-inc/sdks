import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { browser } from "@interfere/test-config/browser";
import { node } from "@interfere/test-config/node";

export default defineConfig({
  root: import.meta.dirname,
  test: {
    projects: [
      node({
        resolve: {
          alias: {
            "#request-flush": fileURLToPath(
              new URL("src/internal/server/request-flush.ts", import.meta.url),
            ),
          },
        },
        test: { name: "node", exclude: ["**/*.native.test.ts"] },
      }),
      browser({
        optimizeDeps: {
          include: ["@opentelemetry/api"],
          exclude: ["fsevents", "@opentelemetry/otlp-transformer"],
        },
        test: { name: "browser" },
      }),
    ],
  },
});
