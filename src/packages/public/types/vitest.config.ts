import { defineConfig } from "vitest/config";
import { browser } from "@interfere/test-config/browser";
import { node } from "@interfere/test-config/node";

export default defineConfig({
  root: import.meta.dirname,
  test: {
    projects: [
      node({ test: { name: "node" } }),
      browser({ test: { name: "browser" } }),
    ],
  },
});
