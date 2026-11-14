#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const platform = `${process.platform}-${process.arch}`;

const supported = new Set([
  "darwin-arm64",
  "darwin-x64",
  "linux-arm64",
  "linux-x64",
  "win32-x64",
]);

if (!supported.has(platform)) {
  process.stderr.write(`Interfere CLI does not support ${platform}.\n`);

  process.exit(1);
}

let binary: string;

try {
  binary = require.resolve(
    `@interfere/cli-${platform}/bin/interfere${process.platform === "win32" ? ".exe" : ""}`,
  );
} catch {
  process.stderr.write(
    `Missing @interfere/cli-${platform}. Reinstall @interfere/cli with optional dependencies enabled.\n`,
  );

  process.exit(1);
}

const result = spawnSync(binary, process.argv.slice(2), { stdio: "inherit" });

if (result.error) {
  process.stderr.write(
    `Unable to start Interfere CLI: ${result.error.message}\n`,
  );

  process.exit(1);
}

if (result.signal) {
  process.kill(process.pid, result.signal);
} else {
  process.exit(result.status ?? 1);
}
