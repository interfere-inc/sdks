import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { beforeAll, expect, test } from "vitest";

const runNode = promisify(execFile);

const entries = ["config", "release-request", "service-name"] as const;

const loaded = new Map<string, { stderr: string; stdout: string }>();

beforeAll(async () => {
  await Promise.all(
    entries.map(async (entry) => {
      const result = await runNode(
        process.execPath,
        [
          "--input-type=module",
          "--eval",
          `await import("@interfere/build/${entry}")`,
        ],
        { cwd: new URL("../", import.meta.url), timeout: 10_000 },
      );

      loaded.set(entry, { stderr: result.stderr, stdout: result.stdout });
    }),
  );
});

test.each(entries)("loads the %s source export with Node", (entry) => {
  expect(loaded.get(entry)).toMatchObject({ stderr: "", stdout: "" });
});
