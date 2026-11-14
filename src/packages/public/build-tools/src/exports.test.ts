import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { expect, test } from "vitest";

const runNode = promisify(execFile);

test.each(["config", "release-request", "service-name"])(
  "loads the %s source export with Node",
  async (entry) => {
    await expect(
      runNode(
        process.execPath,
        [
          "--conditions=monorepo",
          "--input-type=module",
          "--eval",
          `await import("@interfere/build/${entry}")`,
        ],
        { cwd: new URL("../", import.meta.url), timeout: 10_000 },
      ),
    ).resolves.toMatchObject({ stderr: "", stdout: "" });
  },
);
