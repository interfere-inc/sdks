import { defineConfig } from "vitest/config";
import { node } from "@interfere/test-config/node";
import babel from "@rolldown/plugin-babel";
import react from "@vitejs/plugin-react";
import reactCompiler from "babel-plugin-react-compiler";

import { browser } from "@interfere/test-config/browser";

const browserProject = browser({
  root: import.meta.dirname,
  publicDir: "test/public",
  optimizeDeps: {
    noDiscovery: true,
    exclude: ["fsevents", "@opentelemetry/otlp-transformer"],
    include: [
      "@opentelemetry/api",
      "@opentelemetry/sdk-trace > @opentelemetry/semantic-conventions",
      "react-dom/client",
      "react/compiler-runtime",
      "workbox-background-sync/lib/QueueStore.js",
      "workbox-background-sync/lib/StorableRequest.js",
    ],
  },
  plugins: [
    react(),
    babel({
      include: /\.tsx$/u,
      exclude: /\.test\.tsx$/u,
      plugins: [reactCompiler],
    }),
  ],
  test: {
    name: "browser",
    setupFiles: ["./test/setup.ts"],
  },
});

export default defineConfig({
  root: import.meta.dirname,
  test: {
    projects: [node({ test: { name: "node" } }), browserProject],
  },
});
