import { execFileSync } from "node:child_process";
import { z } from "zod";

export const splitFiles = 100;

export interface LayoutPackage {
  name: string;
  files: number;
}

export function layout(
  packages: LayoutPackage[],
  shard: number,
  total: number,
) {
  const split = packages.filter((pkg) => pkg.files >= splitFiles);

  const loads = Array.from({ length: total }, () => 0);
  const owners = new Map<string, number>();

  for (const pkg of packages
    .filter((candidate) => candidate.files < splitFiles)
    .toSorted((a, b) => b.files - a.files || a.name.localeCompare(b.name))) {
    const owner = loads.indexOf(Math.min(...loads));

    loads[owner] = (loads[owner] ?? 0) + Math.max(pkg.files, 1);

    owners.set(pkg.name, owner + 1);
  }

  const whole = [...owners].flatMap(([name, owner]) =>
    owner === shard ? [name] : [],
  );

  const names = [...split.map((pkg) => pkg.name), ...whole].toSorted();

  return {
    filters: names.map((name) => `--filter=${name}`),
    args: Object.fromEntries(
      split.map((pkg) => [pkg.name, [`--shard=${shard}/${total}`]]),
    ),
  };
}

const position = z
  .string()
  .regex(/^(?<shard>\d+)\/(?<total>\d+)$/u)
  .transform((value) => {
    const [shard, total] = value.split("/").map(Number);

    return { shard: shard ?? 0, total: total ?? 0 };
  })
  .refine(({ shard, total }) => shard >= 1 && shard <= total);

const tasks = z.object({
  data: z.object({
    packages: z.object({
      items: z.array(
        z.object({
          name: z.string(),
          path: z.string(),
          tasks: z.object({
            items: z.array(
              z.object({
                name: z.string(),
                script: z.string().nullable(),
                tags: z.array(z.string()).nullable(),
              }),
            ),
          }),
        }),
      ),
    }),
  }),
});

function tagged(tag: string) {
  const output = execFileSync(
    "bunx",
    [
      "--no-install",
      "turbo",
      "--skip-infer",
      "query",
      "{ packages { items { name path tasks { items { name script tags } } } } }",
    ],
    { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 },
  );

  return tasks
    .parse(JSON.parse(output))
    .data.packages.items.filter((pkg) =>
      pkg.tasks.items.some(
        (task) =>
          task.name === "test" &&
          task.script !== null &&
          (task.tags ?? []).includes(tag),
      ),
    );
}

function testFiles(path: string) {
  return execFileSync("git", ["ls-files", "-z", "--", path], {
    encoding: "utf8",
    maxBuffer: 64 * 1024 * 1024,
  })
    .split("\0")
    .filter(
      (file) =>
        /\.test\.[cm]?[jt]sx?$/u.test(file) &&
        !/\.(?:browser|chromium)\.test\./u.test(file),
    ).length;
}

if (import.meta.main) {
  const { shard, total } = position.parse(process.argv[2]);

  const [filter] = z
    .tuple([z.string().regex(/^--filter=tag:[\w-]+$/u)])
    .parse(process.argv.slice(3));

  const packages = tagged(filter.slice("--filter=tag:".length)).map((pkg) => ({
    name: pkg.name,
    files: testFiles(pkg.path),
  }));

  if (packages.length === 0) {
    throw new Error(`${filter} selects no packages with a test script`);
  }

  process.stdout.write(`${JSON.stringify(layout(packages, shard, total))}\n`);
}
