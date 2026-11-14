import { createHash } from "node:crypto";
import { deriveReleaseSlug } from "@interfere/types/releases/slug";
import { expect, test } from "vitest";
import { resolveBuildReleaseSlug } from "./release-slug.ts";

const commitSha = "e0613916dca66bed7989f948820462298897ea15";

test("separates same-commit Vercel artifacts and reproduces the original identity", () => {
  const first = resolveBuildReleaseSlug({
    commitSha,
    deploymentId: "dpl_first",
  });

  const second = resolveBuildReleaseSlug({
    commitSha,
    deploymentId: "dpl_second",
  });

  expect(first).toMatch(/^rel_[a-f0-9]{16}$/u);
  expect(second).not.toBe(first);

  expect(
    resolveBuildReleaseSlug({ commitSha, deploymentId: "dpl_first" }),
  ).toBe(first);

  expect(
    resolveBuildReleaseSlug({
      commitSha,
      deploymentId: "dpl_first",
      variant: "desktop",
    }),
  ).not.toBe(first);
});

test("preserves legacy commit and variant identities without a deployment ID", () => {
  expect(resolveBuildReleaseSlug({ commitSha })).toBe(
    deriveReleaseSlug(commitSha),
  );

  expect(resolveBuildReleaseSlug({ commitSha, deploymentId: " " })).toBe(
    deriveReleaseSlug(commitSha),
  );

  const variant = "desktop";

  const hash = createHash("sha256")
    .update(JSON.stringify([commitSha, variant]))
    .digest("hex");

  expect(resolveBuildReleaseSlug({ commitSha, variant })).toBe(
    deriveReleaseSlug(hash),
  );

  expect(
    resolveBuildReleaseSlug({ commitSha: null, deploymentId: "dpl_first" }),
  ).toBeNull();
});
