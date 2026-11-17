import { defineConfig } from "vite";

const config = defineConfig({
  build: { ssr: "server.ts", outDir: "server-dist" },
  ssr: { noExternal: ["@interfere/vite"] },
});

export default config;
