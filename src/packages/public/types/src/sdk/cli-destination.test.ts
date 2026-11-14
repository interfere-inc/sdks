import { releaseSlugSchema } from "../releases/slug.ts";
import { resolveCliReleaseDestination } from "./cli-destination.ts";
import { describe, expect, test } from "vitest";

describe(resolveCliReleaseDestination, () => {
  test("builds CLI destination metadata from release and deployment environment variables", () => {
    expect(
      resolveCliReleaseDestination(
        {
          INTERFERE_DEPLOYMENT_URL: " https://deploy.example.com/release/123 ",
          INTERFERE_ENVIRONMENT: " production ",
        },
        releaseSlugSchema.parse("rel_1234567890abcdef"),
      ),
    ).toStrictEqual({
      deploymentUrl: "https://deploy.example.com/release/123",
      destinationReleaseId: "rel_1234567890abcdef",
      environment: "production",
      provider: "cli",
    });
  });

  test("normalizes empty CLI destination environment variables to null", () => {
    expect(
      resolveCliReleaseDestination(
        {
          INTERFERE_DEPLOYMENT_URL: "",
          INTERFERE_ENVIRONMENT: "   ",
        },
        releaseSlugSchema.parse("rel_fedcba0987654321"),
      ),
    ).toStrictEqual({
      deploymentUrl: null,
      destinationReleaseId: "rel_fedcba0987654321",
      environment: null,
      provider: "cli",
    });
  });
});
