import { expect, test } from "vitest";

import { selection } from "./selection.ts";

const app = [".browser.test.", "--project=*(browser)", "--coverage"];

const entries = [
  "src/sites/app/src/a.browser.test.tsx",
  "src/sites/app/src/b.test.ts",
  "src/sites/app/src/routes/",
  "src/sites/design/src/c.browser.test.tsx",
];

const expand = (directory: string) => [
  `${directory}c.browser.test.tsx`,
  `${directory}d.test.ts`,
  `${directory}helper.ts`,
];

test("runs only the package files that match the suite patterns", () => {
  expect.hasAssertions();

  expect(
    selection(app, entries, "/repo", "src/sites/app", expand),
  ).toStrictEqual([
    "--project=*(browser)",
    "--coverage",
    "/repo/src/sites/app/src/a.browser.test.tsx",
    "/repo/src/sites/app/src/routes/c.browser.test.tsx",
  ]);
});

test("runs nothing when no selected file matches the package and suite", () => {
  expect.hasAssertions();

  expect(
    selection(
      app,
      ["src/sites/app/src/b.test.ts"],
      "/repo",
      "src/sites/app",
      expand,
    ),
  ).toBeUndefined();

  expect(
    selection(app, entries, "/repo", "src/sites/website", expand),
  ).toBeUndefined();
});

test("runs the whole suite for a package selected whole", () => {
  expect.hasAssertions();

  expect(
    selection(
      app,
      ["src/sites/app/", ...entries],
      "/repo",
      "src/sites/app",
      () => {
        throw new Error("a whole package needs no file list");
      },
    ),
  ).toStrictEqual(app);
});

test("keeps the package entries for suites without patterns", () => {
  expect.hasAssertions();

  expect(
    selection(
      ["--exclude=**/*.browser.test.*"],
      entries,
      "/repo",
      "src/sites/app",
      expand,
    ),
  ).toStrictEqual([
    "--exclude=**/*.browser.test.*",
    "/repo/src/sites/app/src/a.browser.test.tsx",
    "/repo/src/sites/app/src/b.test.ts",
    "/repo/src/sites/app/src/routes",
  ]);
});
