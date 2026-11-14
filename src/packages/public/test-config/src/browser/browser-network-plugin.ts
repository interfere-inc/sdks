import { readFileSync } from "node:fs";
import type { Plugin } from "vitest/config";

const worker = readFileSync(
  new URL(import.meta.resolve("msw/mockServiceWorker.js")),
);

export const browserNetworkPlugin = {
  name: "test-network-worker",
  configureServer(server) {
    server.middlewares.use("/mockServiceWorker.js", (_request, response) => {
      response.setHeader("Content-Type", "application/javascript");

      response.end(worker);
    });
  },
} satisfies Plugin;
