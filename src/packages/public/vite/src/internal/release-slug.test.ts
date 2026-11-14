import { execFileSync } from "node:child_process";
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";
import { afterEach, beforeAll, beforeEach, expect, test, vi } from "vitest";
import { resolveReleaseSlug } from "./release-slug.ts";

const commitSha = "e0613916dca66bed7989f948820462298897ea15";
const bakedSlug = "rel_0123456789abcdef";

let nativeResult: unknown;

beforeAll(() => {
  const entry = new URL("../../dist/internal/release-slug.mjs", import.meta.url)
    .href;

  const output = execFileSync(
    process.execPath,
    [
      "--input-type=module",
      "--eval",
      `import { resolveReleaseSlug } from ${JSON.stringify(entry)};
    process.stdout.write(JSON.stringify({ viteEnv: typeof import.meta.env, release: resolveReleaseSlug() }));`,
    ],
    {
      env: { ...process.env, INTERFERE_SOURCE_ID: commitSha },
      encoding: "utf8",
    },
  );

  nativeResult = JSON.parse(output);
});

beforeEach(() => {
  for (const key of releaseSourceIdEnvKeys) {
    vi.stubEnv(key, undefined);
  }

  vi.stubEnv("VITE_INTERFERE_RELEASE_SLUG", undefined);
});

afterEach(() => vi.unstubAllEnvs());

test("uses the bundled identity after promotion changes the deployment and commit environment", () => {
  vi.stubEnv("VITE_INTERFERE_RELEASE_SLUG", bakedSlug);

  vi.stubEnv("VERCEL_DEPLOYMENT_ID", "dpl_promoted");

  vi.stubEnv("INTERFERE_SOURCE_ID", commitSha);

  const runGit = vi.fn<() => null>();

  expect(resolveReleaseSlug(runGit)).toStrictEqual({
    commitSha: null,
    slug: bakedSlug,
  });

  expect(runGit).not.toHaveBeenCalled();
});

test.each([undefined, "invalid"])(
  "falls back for an absent or invalid bundled identity: %s",
  (slug) => {
    vi.stubEnv("VITE_INTERFERE_RELEASE_SLUG", slug);

    vi.stubEnv("INTERFERE_SOURCE_ID", commitSha);

    expect(resolveReleaseSlug()).toStrictEqual({
      commitSha,
      slug: "rel_e0613916dca66bed",
    });
  },
);

test("the built resolver preserves commit fallback in native Node without Vite env injection", () => {
  expect(nativeResult).toStrictEqual({
    viteEnv: "undefined",
    release: { commitSha, slug: "rel_e0613916dca66bed" },
  });
});
