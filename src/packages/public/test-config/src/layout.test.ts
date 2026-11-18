import { expect, test } from "vitest";

import { layout, splitFiles } from "./layout.ts";

const packages = [
  { name: "@fixture/large", files: splitFiles * 3 },
  { name: "@fixture/threshold", files: splitFiles },
  { name: "@fixture/medium", files: 60 },
  { name: "@fixture/small", files: 12 },
  { name: "@fixture/tiny", files: 1 },
  { name: "@fixture/empty", files: 0 },
];

const shards = (total: number) =>
  Array.from({ length: total }, (_, index) =>
    layout(packages, index + 1, total),
  );

test.each([1, 2, 3, 8])(
  "runs each package below the split threshold once, whole, across %i shards",
  (total) => {
    expect.hasAssertions();

    const runs = shards(total).flatMap(({ filters, args }) =>
      filters.map((filter) => ({
        name: filter.slice("--filter=".length),
        args: args[filter.slice("--filter=".length)] ?? [],
      })),
    );

    for (const name of [
      "@fixture/medium",
      "@fixture/small",
      "@fixture/tiny",
      "@fixture/empty",
    ]) {
      expect(runs.filter((run) => run.name === name)).toStrictEqual([
        { name, args: [] },
      ]);
    }

    for (const name of ["@fixture/large", "@fixture/threshold"]) {
      expect(runs.filter((run) => run.name === name)).toStrictEqual(
        Array.from({ length: total }, (_, index) => ({
          name,
          args: [`--shard=${index + 1}/${total}`],
        })),
      );
    }
  },
);

test("balances whole packages by test file count", () => {
  expect(shards(3).map(({ filters }) => filters)).toStrictEqual([
    [
      "--filter=@fixture/large",
      "--filter=@fixture/medium",
      "--filter=@fixture/threshold",
    ],
    [
      "--filter=@fixture/large",
      "--filter=@fixture/small",
      "--filter=@fixture/threshold",
    ],
    [
      "--filter=@fixture/empty",
      "--filter=@fixture/large",
      "--filter=@fixture/threshold",
      "--filter=@fixture/tiny",
    ],
  ]);
});
