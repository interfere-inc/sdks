import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { pairSourceMapsForCleanup } from "./source-maps";
import { afterEach, beforeEach, expect, test } from "vitest";

let distDir: string;

beforeEach(async () => {
  distDir = join(
    await mkdtemp(join(tmpdir(), "interfere-vite-cleanup-pairs-")),
    "dist",
  );

  await mkdir(join(distDir, "assets"), { recursive: true });
});

afterEach(async () => {
  await rm(join(distDir, ".."), { force: true, recursive: true });
});

test("pairs an unparseable map with every referencing chunk and ignores inline or escaping maps", async () => {
  await writeFile(join(distDir, "assets/app.js.map"), "{");

  for (const chunk of ["assets/app.js", "assets/twin.js"]) {
    // oxlint-disable-next-line eslint/no-await-in-loop
    await writeFile(
      join(distDir, chunk),
      "console.log('chunk');\n//# sourceMappingURL=app.js.map",
    );
  }

  await writeFile(
    join(distDir, "inline.js"),
    "console.log('inline');\n//# sourceMappingURL=data:application/json;base64,e30=",
  );

  await writeFile(
    join(distDir, "escape.js"),
    "console.log('escape');\n//# sourceMappingURL=../outside.js.map",
  );

  const pairs = await pairSourceMapsForCleanup(distDir);

  expect(
    pairs.map(({ absolute, chunkAbsolutes }) => ({
      absolute,
      chunkAbsolutes: chunkAbsolutes.toSorted(),
    })),
  ).toStrictEqual([
    {
      absolute: join(distDir, "assets/app.js.map"),
      chunkAbsolutes: [
        join(distDir, "assets/app.js"),
        join(distDir, "assets/twin.js"),
      ],
    },
  ]);
});
