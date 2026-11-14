/* oxlint-disable typescript/no-unsafe-assignment */
import {
  access,
  mkdir,
  mkdtemp,
  readFile,
  rm,
  writeFile,
} from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";

import { cleanupSourceMaps } from "./cleanup-source-maps";
import { expect, test } from "vitest";

test("removes maps, tolerates missing files, and reports failures independently", async () => {
  const root = await mkdtemp(path.join(tmpdir(), "map-cleanup-"));

  try {
    const map = path.join(root, "app.js.map");
    const chunk = path.join(root, "app.js");
    const directory = path.join(root, "directory");

    await writeFile(map, "{}");

    await writeFile(chunk, "a();\n//# sourceMappingURL=app.js.map");

    await mkdir(directory);

    await expect(
      cleanupSourceMaps(
        [map, path.join(root, "missing"), directory].map((absolute) => ({
          absolute,
          chunkAbsolutes: [chunk],
        })),
      ),
    ).resolves.toStrictEqual([
      {
        absolute: directory,
        code: expect.any(String),
        message: expect.any(String),
      },
    ]);

    await expect(access(map)).rejects.toMatchObject({ code: "ENOENT" });

    await access(directory);
  } finally {
    await rm(root, { force: true, recursive: true });
  }
});

test("strips every chunk that shares a map and keeps the map when a chunk cannot be stripped", async () => {
  const root = await mkdtemp(path.join(tmpdir(), "map-cleanup-"));

  try {
    const shared = path.join(root, "shared.js.map");
    const kept = path.join(root, "kept.js.map");
    const a = path.join(root, "a.js");
    const b = path.join(root, "b.js");
    const unreadable = path.join(root, "unreadable.js");
    const chunk = "a();\n//# debugId=debug\n//# sourceMappingURL=x.js.map\n";

    await Promise.all(
      [shared, kept, a, b].map(async (file) => writeFile(file, chunk)),
    );

    await mkdir(unreadable);

    await expect(
      cleanupSourceMaps([
        { absolute: shared, chunkAbsolutes: [a, b] },
        { absolute: kept, chunkAbsolutes: [unreadable] },
      ]),
    ).resolves.toMatchObject([{ absolute: unreadable }]);

    await expect(readFile(a, "utf8")).resolves.toBe("a();\n//# debugId=debug");

    await expect(readFile(b, "utf8")).resolves.toBe("a();\n//# debugId=debug");

    await expect(access(shared)).rejects.toMatchObject({ code: "ENOENT" });

    await access(kept);
  } finally {
    await rm(root, { force: true, recursive: true });
  }
});

test("strips a chunk exactly once when many maps share it", async () => {
  expect.hasAssertions();

  const root = await mkdtemp(path.join(tmpdir(), "map-cleanup-"));

  try {
    const maps = Array.from({ length: 16 }, (_, index) =>
      path.join(root, `${index}.js.map`),
    );

    const chunks = [path.join(root, "a.js"), path.join(root, "b.js")];

    const kept = `${"a();\n".repeat(100_000)}//# debugId=debug${"\n//# sourceMappingURL=x.js.map".repeat(maps.length)}`;

    await Promise.all([
      ...maps.map(async (map) => writeFile(map, "{}")),
      ...chunks.map(async (chunk) =>
        writeFile(chunk, `${kept}\n//# sourceMappingURL=x.js.map\n`),
      ),
    ]);

    await expect(
      cleanupSourceMaps(
        maps.map((absolute) => ({ absolute, chunkAbsolutes: chunks })),
      ),
    ).resolves.toStrictEqual([]);

    for (const chunk of chunks) {
      // oxlint-disable-next-line eslint/no-await-in-loop -- Each chunk is read after cleanup settles.
      await expect(readFile(chunk, "utf8")).resolves.toBe(kept);
    }

    for (const map of maps) {
      // oxlint-disable-next-line eslint/no-await-in-loop -- Each map is checked after cleanup settles.
      await expect(access(map)).rejects.toMatchObject({ code: "ENOENT" });
    }
  } finally {
    await rm(root, { force: true, recursive: true });
  }
});
