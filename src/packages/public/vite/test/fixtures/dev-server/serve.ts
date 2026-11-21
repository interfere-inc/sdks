import assert from "node:assert/strict";
import { createServer } from "vite";

const server = await createServer({
  logLevel: "warn",
  server: { host: "127.0.0.1", port: 0, strictPort: false },
});

await server.listen();

const [url] = server.resolvedUrls?.local ?? [];

assert.ok(url);

process.send?.(url);
