import { defineConfig } from "vitest/config";
import { node } from "@interfere/test-config/node";

export default defineConfig({
  root: import.meta.dirname,
  test: { projects: [node({ test: { name: "node" } })] },
});
