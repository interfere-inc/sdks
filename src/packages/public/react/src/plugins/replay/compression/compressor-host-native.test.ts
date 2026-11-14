import { execFile } from "node:child_process";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import { expect, test } from "vitest";

const execute = promisify(execFile);

test("fails open in a native JavaScript realm without compression streams", async () => {
  const { stdout: source } = await execute("bun", [
    "build",
    fileURLToPath(new URL("compressor-host.ts", import.meta.url)),
    "--target=browser",
    "--format=esm",
  ]);

  const { stdout } = await execute(process.execPath, [
    "--experimental-vm-modules",
    "--input-type=module",
    "-e",
    `import { createContext, runInContext, SourceTextModule } from "node:vm";
const context = createContext();
const module = new SourceTextModule(${JSON.stringify(source)}, { context });
await module.link(() => { throw new Error("Unexpected external module"); });
await module.evaluate();
const compressor = module.namespace.createCompressor();
console.log(JSON.stringify({ host: runInContext("typeof CompressionStream", context), gzip: compressor.gzip, disposeType: typeof compressor.dispose, disposed: compressor.dispose() === undefined }));`,
  ]);

  expect(stdout.trim()).toBe(
    JSON.stringify({
      host: "undefined",
      gzip: null,
      disposeType: "function",
      disposed: true,
    }),
  );
});
