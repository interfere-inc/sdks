import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { beforeAll, expect, test } from "vitest";

const execute = promisify(execFile);

let stdout: string;

beforeAll(async () => {
  const source = new URL("sequence.ts", import.meta.url).href;

  ({ stdout } = await execute(process.execPath, [
    "--no-experimental-webstorage",
    "--input-type=module",
    "-e",
    `import { createSegmentSequence } from ${JSON.stringify(source)};
const sequence = createSegmentSequence("win_nostore");
console.log(JSON.stringify({ storage: typeof globalThis.sessionStorage, ids: [sequence.take(), sequence.take()] }));`,
  ]));
});

test("allocates sequence ids in a native host with web storage disabled", () => {
  expect(stdout.trim()).toBe(
    JSON.stringify({ storage: "undefined", ids: [0, 1] }),
  );
});
