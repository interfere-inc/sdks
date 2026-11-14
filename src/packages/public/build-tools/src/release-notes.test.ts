import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, test } from "vitest";

const publicRoot = resolve(import.meta.dirname, "../..");

describe.each([
  ["next", "13.0.0"],
  ["react", "13.0.0"],
  ["vite", "13.0.0"],
  ["types", "11.0.0"],
])("%s release notes", (name, version) => {
  const changelog = readFileSync(
    resolve(publicRoot, name, "CHANGELOG.md"),
    "utf8",
  );

  const release =
    changelog.split(`\n## ${version}\n`)[1]?.split("\n## ")[0] ?? "";

  test("list each source-map update and dependency revision once", () => {
    expect(release.match(/^- f584c2f:/gmu)).toHaveLength(1);

    const revisions =
      release.match(/^- Updated dependencies \[[a-f0-9]+\]/gmu) ?? [];

    expect(revisions).toHaveLength(new Set(revisions).size);
  });

  test("classify the browser tracing API change as breaking", () => {
    const major = release.split("### Major Changes")[1]?.split("### ")[0];

    expect(major).toContain("- 3ff22ac:");
  });

  test("document the canonical options object and supported shorthand", () => {
    expect(release).toContain('tracing: { mode: "auto" }');
    expect(release).toContain('tracing: "auto"');

    expect(release).not.toContain(
      'now takes `false` or an options object with `mode: "managed" | "application"`',
    );

    expect(release).not.toContain(
      "The default mode comes from the workspace surface setting",
    );
  });

  test("keep operational acceptance instructions out of release notes", () => {
    expect(release).not.toMatch(
      /packed SDK qualification|customer API preview acceptance|deploy backend identity indexing/u,
    );
  });
});
