import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { releaseSourceIdEnvKeys } from "@interfere/types/integrations";

import { withTempProject } from "../../test/fixtures/build";
import { runGitCommand } from "./build/release/git";
import { deriveReleaseSlug } from "@interfere/types/releases/slug";

import { resolveReleaseSlug } from "./release-slug.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

const SLUG_GLOBAL = "__INTERFERE_RELEASE_SLUG__";

const SHA = "01e037e5dc3f6a013124dab44863d87a9be93029";

const SHA_SLUG = `rel_${SHA.slice(0, 16)}`;

const BAKED_SLUG = "rel_0123456789abcdef";

beforeEach(() => {
  setInjectedConfiguration(SLUG_GLOBAL, undefined);

  for (const key of releaseSourceIdEnvKeys) {
    vi.stubEnv(key, undefined);
  }
});

afterEach(() => {
  vi.unstubAllGlobals();

  vi.unstubAllEnvs();
});

describe(resolveReleaseSlug, () => {
  test("prefers the build-baked global and never consults env or git", () => {
    setInjectedConfiguration(SLUG_GLOBAL, BAKED_SLUG);

    vi.stubEnv("INTERFERE_SOURCE_ID", SHA);

    vi.stubEnv("VERCEL_DEPLOYMENT_ID", "dpl_promoted");

    const runGit = vi.fn<() => string>();
    const { slug, commitSha } = resolveReleaseSlug(runGit);

    expect(slug).toBe(BAKED_SLUG);
    expect(commitSha).toBeNull();
    expect(runGit).not.toHaveBeenCalled();
  });

  test("falls back to the env SHA when no global is baked", () => {
    vi.stubEnv("INTERFERE_SOURCE_ID", SHA);

    const runGit = vi.fn<() => null>();
    const { slug, commitSha } = resolveReleaseSlug(runGit);

    expect(commitSha).toBe(SHA);
    expect(slug).toBe(SHA_SLUG);
    expect(runGit).not.toHaveBeenCalled();
  });

  test("falls back to the actual git source when neither global nor env is set", () => {
    const runGit = vi.spyOn({ runGitCommand }, "runGitCommand");
    const expected = runGitCommand("git rev-parse HEAD");
    const { slug, commitSha } = resolveReleaseSlug(runGit);

    expect(expected).toMatch(/^[a-f0-9]{40}$/u);
    expect(runGit).toHaveBeenCalledWith("git rev-parse HEAD");
    expect(commitSha).toBe(expected);
    expect(slug).toBe(expected ? deriveReleaseSlug(expected) : null);
  });

  test("never spawns a git subprocess when called without a git source", () => {
    const { slug, commitSha } = resolveReleaseSlug();

    expect(slug).toBeNull();
    expect(commitSha).toBeNull();
  });

  test("ignores a malformed baked global and falls through to env", () => {
    setInjectedConfiguration(SLUG_GLOBAL, "not-a-real-slug");

    vi.stubEnv("INTERFERE_SOURCE_ID", SHA);

    const { slug } = resolveReleaseSlug();

    expect(slug).toBe(SHA_SLUG);
  });

  test("returns null slug when git cannot resolve a commit", async () => {
    expect.assertions(2);

    await withTempProject("next-release-no-git-", (projectDir) => {
      process.chdir(projectDir);

      const { slug, commitSha } = resolveReleaseSlug(runGitCommand);

      expect(slug).toBeNull();
      expect(commitSha).toBeNull();
    });
  });

  test.each([null, undefined, 42, false, {}, [], Symbol("release")])(
    "ignores a non-string baked global %s",
    (value) => {
      setInjectedConfiguration(SLUG_GLOBAL, value);

      vi.stubEnv("INTERFERE_SOURCE_ID", SHA);

      expect(resolveReleaseSlug()).toStrictEqual({
        commitSha: SHA,
        slug: SHA_SLUG,
      });
    },
  );

  test("falls back when the baked global property is absent", () => {
    Reflect.deleteProperty(globalThis, SLUG_GLOBAL);

    vi.stubEnv("INTERFERE_SOURCE_ID", SHA);

    expect(resolveReleaseSlug()).toStrictEqual({
      commitSha: SHA,
      slug: SHA_SLUG,
    });
  });
});
