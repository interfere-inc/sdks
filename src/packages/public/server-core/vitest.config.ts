import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { node } from "@interfere/test-config/node";

export default defineConfig({
  root: import.meta.dirname,
  test: {
    projects: [
      node({
        resolve: {
          alias: {
            "#otlp-http": fileURLToPath(
              new URL("src/otlp/node.ts", import.meta.url),
            ),
          },
        },
        test: { name: "node" },
      }),
    ],
  },
});
