import { mkdirSync, writeFileSync } from "node:fs";
import path from "node:path";
import { defineConfig } from "vite";

import { interfere } from "@interfere/vite/plugin";

const config = defineConfig({
  plugins: [
    interfere({
      apiHost: process.env["ACCEPTANCE_INGEST_URL"] ?? "http://127.0.0.1",
    }),
    {
      name: "keep-source-maps",
      writeBundle(_options, bundle) {
        for (const [name, asset] of Object.entries(bundle)) {
          if (name.endsWith(".map") && asset.type === "asset") {
            const target = path.join("maps", name);

            mkdirSync(path.dirname(target), { recursive: true });

            writeFileSync(target, asset.source);
          }
        }
      },
    },
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
