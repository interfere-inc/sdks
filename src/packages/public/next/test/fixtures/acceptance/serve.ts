/* oxlint-disable typescript/no-misused-promises */
import assert from "node:assert/strict";
import { createServer } from "node:http";
import next from "next";

const app = next({ dev: false, hostname: "127.0.0.1" });

await app.prepare();

const server = createServer(app.getRequestHandler());

server.listen(0, "127.0.0.1", () => {
  const address = server.address();

  // oxlint-disable-next-line anti-slop/no-runtime-typeof
  assert.ok(address !== null && typeof address !== "string");

  process.send?.(`http://127.0.0.1:${address.port}`);
});

process.once("SIGTERM", async () => {
  server.closeAllConnections();

  server.close();

  await app.close();
});
