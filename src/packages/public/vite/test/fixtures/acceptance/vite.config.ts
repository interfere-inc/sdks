import { defineConfig } from "vite";

import { interfere } from "@interfere/vite/plugin";

const config = defineConfig({
  plugins: [
    interfere({
      apiHost: process.env["ACCEPTANCE_INGEST_URL"] ?? "http://127.0.0.1",
    }),
  ],
  build: {
    sourcemap: true,
    rolldownOptions: {
      input: { index: "index.html", sw: "sw.ts" },
      output: {
        entryFileNames: (chunk) =>
          chunk.name === "sw" ? "sw.js" : "assets/[name]-[hash].js",
      },
    },
  },
});

export default config;
