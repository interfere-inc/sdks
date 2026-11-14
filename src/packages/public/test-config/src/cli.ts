import { createRequire } from "node:module";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { z } from "zod";

if (process.env["TESTCONTAINERS_RYUK_DISABLED"] === "true") {
  throw new Error(
    "Testcontainers cleanup is disabled. Unset TESTCONTAINERS_RYUK_DISABLED before running tests so Ryuk can remove containers and host-port forwarders after the test process exits.",
  );
}

const args = z
  .array(z.string())
  .parse(JSON.parse(process.env["VITEST_ARGS"] ?? "[]"));

const packageWorkers = process.argv.some(
  (argument) =>
    argument === "--maxWorkers" || argument.startsWith("--maxWorkers="),
);

process.argv.push(
  ...args.filter(
    (argument) => !(packageWorkers && argument.startsWith("--maxWorkers")),
  ),
);

const manifest = createRequire(resolve("package.json")).resolve(
  "vitest/package.json",
);

await import(new URL("vitest.mjs", pathToFileURL(manifest)).href);
