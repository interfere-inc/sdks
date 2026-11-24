import { cloudflare } from "@cloudflare/vite-plugin";
import { defineConfig } from "vite";

import { interfere } from "@interfere/vite/plugin";

const config = defineConfig({
  plugins: [cloudflare(), interfere()],
  environments: { packed_worker: { build: { sourcemap: "hidden" } } },
});

export default config;
