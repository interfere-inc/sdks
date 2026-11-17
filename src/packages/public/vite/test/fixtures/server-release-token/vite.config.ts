import { defineConfig } from "vite";

import { interfere } from "@interfere/vite/plugin";

const config = defineConfig({
  plugins: [interfere()],
  build: { ssr: "server.ts", outDir: "server-dist" },
  ssr: { noExternal: ["@interfere/vite"] },
});

export default config;
