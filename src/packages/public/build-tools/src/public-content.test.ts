import { globSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { expect, test } from "vitest";

const publicRoot = resolve(import.meta.dirname, "../..");

test.each([
  ["next/README.md", "@acme/platform"],
  ["next/src/config.test.ts", "@acme/platform"],
  ["build-tools/src/service-name.test.ts", " @acme/platform "],
  [
    "react/src/internal/third-party-hosts.browser.test.ts",
    "https://acme.featurebase.app/api/v1/user/identify",
  ],
  [
    "react/src/internal/test/kernel-header-policy.browser.test.ts",
    "https://api.acme.test",
  ],
  [
    "next/test/fixtures/foreign-tracing-owner/app/journey/page.tsx",
    "example-journey-error",
  ],
])("%s uses a neutral example", (file, example) => {
  expect(readFileSync(resolve(publicRoot, file), "utf8")).toContain(example);
});

test("public source and examples omit internal issue and audit labels", () => {
  const files = globSync(
    ["*/src/**/*.{ts,tsx}", "*/test/**/*.{ts,tsx}", "*/README.md"],
    {
      cwd: publicRoot,
      exclude: ["**/node_modules/**", "**/dist/**"],
    },
  );

  const internalLabel =
    /\b(?:ENG-\d+|interfere-[a-z0-9]+(?:\.\d+)+|bl7t(?:\.\d+)+|B(?:16|44|48))\b/u;

  const violations = files.filter((file) =>
    internalLabel.test(readFileSync(resolve(publicRoot, file), "utf8")),
  );

  expect(violations).toStrictEqual([]);
});

test("regression test filenames describe behavior", () => {
  expect(
    globSync(["*/src/**/*-audit-d.*", "*/src/**/audit-*.browser.test.ts"], {
      cwd: publicRoot,
    }),
  ).toStrictEqual([]);
});
