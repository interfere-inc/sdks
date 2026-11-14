import { expect, test } from "vitest";

import { sourceMapBatchDigest } from "./source-map-batch";

const file = { path: "a.js.map", hash: "abc", debugId: "id", chunkUrl: "a.js" };

const batch = {
  bundler: "rollup" as const,
  sourceFileCount: 2,
  files: [file, { ...file, path: "b.js.map" }],
};

test("canonical batch identity ignores inventory ordering", async () => {
  const digest = await sourceMapBatchDigest(batch);

  await expect(
    sourceMapBatchDigest({ ...batch, files: batch.files.toReversed() }),
  ).resolves.toBe(digest);
});

test.each(["path", "hash", "debugId", "chunkUrl"] as const)(
  "batch identity includes %s",
  async (field) => {
    const digest = await sourceMapBatchDigest(batch);

    await expect(
      sourceMapBatchDigest({
        ...batch,
        files: [
          { ...file, [field]: "changed" },
          { ...file, path: "b.js.map" },
        ],
      }),
    ).resolves.not.toBe(digest);
  },
);

test("batch identity includes bundler and declared source count", async () => {
  const digest = await sourceMapBatchDigest(batch);

  await expect(
    sourceMapBatchDigest({ ...batch, bundler: "webpack" }),
  ).resolves.not.toBe(digest);

  await expect(
    sourceMapBatchDigest({ ...batch, sourceFileCount: 3 }),
  ).resolves.not.toBe(digest);
});

test("preserves the legacy digest when aliases are absent", async () => {
  await expect(sourceMapBatchDigest(batch)).resolves.toBe(
    "9089f061953c57f146b9d8316e797e1bffcb26a3b26d4e76eebf366d26296b3f",
  );
});

test("includes canonical aliases in batch integrity", async () => {
  const withAliases = {
    ...batch,
    files: [{ ...file, chunkUrls: ["z.js", "a.js", "z.js"] }],
  };

  const digest = await sourceMapBatchDigest(withAliases);

  await expect(
    sourceMapBatchDigest({
      ...withAliases,
      files: [{ ...file, chunkUrls: ["a.js", "z.js"] }],
    }),
  ).resolves.toBe(digest);

  await expect(
    sourceMapBatchDigest({
      ...withAliases,
      files: [{ ...file, chunkUrls: ["a.js", "other.js"] }],
    }),
  ).resolves.not.toBe(digest);

  await expect(
    sourceMapBatchDigest({ ...withAliases, files: [file] }),
  ).resolves.not.toBe(digest);
});
