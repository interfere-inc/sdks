import { releaseSlugSchema } from "@interfere/types/releases/slug";
import { describe, expect, test } from "vitest";

const { resolveReleaseRequest } = await import("./index.js");

type ReleasesConfigResponse = Parameters<typeof resolveReleaseRequest>[3];

const releaseSlug = releaseSlugSchema.parse("rel_1234567890abcdef");

function config(overrides: Partial<ReleasesConfigResponse["surface"]> = {}) {
  return {
    org: { id: "org_1", slug: "acme" },
    surface: {
      destinationProvider: "vercel",
      environmentFiltering: true,
      id: "surface_1",
      sdkConfig: null,
      slug: "web",
      sourceProvider: "github",
      ...overrides,
    },
  } satisfies ReleasesConfigResponse;
}

describe("resolveReleaseRequest", () => {
  test.each([
    ["sourceProvider", "missing_source_provider"],
    ["destinationProvider", "missing_destination_provider"],
  ] as const)("skips releases without %s", (provider, reason) => {
    expect(
      resolveReleaseRequest(
        "build_123",
        null,
        releaseSlug,
        config({ [provider]: null }),
      ),
    ).toStrictEqual({ ok: false, reason });
  });
});
