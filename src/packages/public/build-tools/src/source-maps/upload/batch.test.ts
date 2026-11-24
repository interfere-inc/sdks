import { sourceMapUploadJobs } from "./batch";
import { expect, test } from "vitest";

const limits = { version: 1, maxBodyBytes: 524_288, maxFiles: 32 };

const file = {
  path: "日本語.js.map",
  content: "é",
  chunkUrl: "a.js",
  hash: "hash",
  debugId: "debug",
};

test("packs exact UTF-8 JSON limits and preserves raw byte accounting", () => {
  const chunk = { path: file.path, offset: 0, bytes: "w6k=" };

  const maxBodyBytes = new TextEncoder().encode(
    JSON.stringify({ chunks: [chunk] }),
  ).length;

  const jobs = [
    ...sourceMapUploadJobs([file, { ...file, path: "b.js.map" }], {
      ...limits,
      maxBodyBytes,
    }),
  ];

  expect(jobs).toHaveLength(2);
  expect(jobs[0]).toStrictEqual({ chunks: [chunk], rawBytes: 2 });

  expect([
    ...sourceMapUploadJobs([file], {
      ...limits,
      maxBodyBytes: maxBodyBytes - 1,
    }),
  ]).toStrictEqual([{ file }]);
});

test("limits each batch to the advertised file count", () => {
  const files = Array.from({ length: 65 }, (_, index) => ({
    ...file,
    path: `${index}.map`,
  }));

  const jobs = [...sourceMapUploadJobs(files, limits)];

  expect(jobs.map((job) => job.chunks?.length)).toStrictEqual([32, 32, 1]);
  expect(jobs.reduce((sum, job) => sum + (job.rawBytes ?? 0), 0)).toBe(130);
});

test("does not read the next file until its batch is requested", () => {
  let reads = 0;

  const files = Array.from({ length: 100 }, (_, index) => ({
    ...file,
    path: `${index}.map`,
    get content() {
      reads += 1;

      return "é";
    },
  }));

  const jobs = sourceMapUploadJobs(files, { ...limits, maxFiles: 2 });

  expect(reads).toBe(0);
  expect(jobs.next().value?.chunks).toHaveLength(2);
  expect(reads).toBeLessThanOrEqual(6);
  expect(jobs.next().value?.chunks).toHaveLength(2);
  expect(reads).toBeLessThanOrEqual(10);
});

test("preserves legacy jobs when capability is absent or a file cannot fit", () => {
  const large = { ...file, content: "a".repeat(524_288) };
  const empty = { ...file, content: "" };

  expect([
    ...sourceMapUploadJobs([file, large, empty], undefined),
  ]).toStrictEqual([{ file }, { file: large }, { file: empty }]);

  expect([...sourceMapUploadJobs([file, large], limits)]).toStrictEqual([
    { chunks: [{ path: file.path, offset: 0, bytes: "w6k=" }], rawBytes: 2 },
    { file: large },
  ]);
});

test.each([
  null,
  {},
  { ...limits, version: 2 },
  { ...limits, maxFiles: 0 },
  { ...limits, maxFiles: 33 },
  { ...limits, maxBodyBytes: 524_289 },
])(
  "rejects present invalid capability before consuming files: %j",
  (capability) => {
    // @ts-expect-error -- Malformed wire metadata must fail runtime validation.
    expect(() => sourceMapUploadJobs([], capability)).toThrow(
      /Invalid|Too (?:big|small)/u,
    );
  },
);
