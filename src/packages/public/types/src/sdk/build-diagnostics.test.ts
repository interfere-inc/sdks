import { deriveReleaseSlug } from "../releases/slug.ts";
import {
  API_KEY_PREFIXES,
  checkReleaseConfig,
  classifyReleasePipelineError,
  diagnoseRelease,
  formatApiKeyPreview,
  isInterfereApiKey,
} from "./build-diagnostics.ts";
import { describe, expect, test } from "vitest";

const NEXT_CONTEXT = {
  publicKeyEnvVar: "INTERFERE_PUBLIC_KEY",
  docsUrl: "https://interfere.com/docs/sdk/next-js",
} as const;

const VITE_CONTEXT = {
  publicKeyEnvVar: "VITE_INTERFERE_PUBLIC_KEY",
  docsUrl: "https://interfere.com/docs/sdk/vite",
} as const;

const COMMIT_SHA = "abcd0000abcd0000abcd0000abcd0000abcd0000";

const RELEASE = {
  commitSha: COMMIT_SHA,
  releaseSlug: deriveReleaseSlug(COMMIT_SHA),
};

describe(classifyReleasePipelineError, () => {
  test.each([
    ["AUTH_MISSING", "API_KEY_UNREADABLE"],
    ["AUTH_AK_INVALID", "API_KEY_REJECTED"],
    ["AUTH_AK_SCOPE_MISMATCH", "API_KEY_SCOPE_MISMATCH"],
    ["AUTH_AK_SURFACE_REQUIRED", "API_KEY_SURFACE_REQUIRED"],
    ["NETWORK_ACCESS_DENIED", "API_KEY_NETWORK_DENIED"],
  ] as const)("maps %s to %s", (code, expected) => {
    expect(
      classifyReleasePipelineError({ body: { code }, statusCode: 401 }),
    ).toBe(expected);
  });

  test("falls back to RELEASE_PIPELINE_FAILED for unrecognized codes", () => {
    expect(
      classifyReleasePipelineError({ body: { code: "INGEST_BACKPRESSURE" } }),
    ).toBe("RELEASE_PIPELINE_FAILED");
  });

  test.each(["constructor", "__proto__", "toString", "hasOwnProperty"])(
    "does not interpret inherited object key %s as a API code",
    (code) => {
      expect(classifyReleasePipelineError({ body: { code } })).toBe(
        "RELEASE_PIPELINE_FAILED",
      );
    },
  );

  test.each([null, 42, "error", {}])(
    "ignores malformed API response bodies %j",
    (body) => {
      expect(classifyReleasePipelineError({ body })).toBe(
        "RELEASE_PIPELINE_FAILED",
      );
    },
  );

  test.each([null, undefined, "boom", 42, new Error("nope"), {}])(
    "falls back to RELEASE_PIPELINE_FAILED for non-API error %s",
    (value) => {
      expect(classifyReleasePipelineError(value)).toBe(
        "RELEASE_PIPELINE_FAILED",
      );
    },
  );
});

describe(checkReleaseConfig, () => {
  test("accepts a regional secret key with a release identity", () => {
    expect(
      checkReleaseConfig(NEXT_CONTEXT, {
        apiKey: "interfere_secret_us_test_123",
        ...RELEASE,
      }),
    ).toStrictEqual({
      status: "ok",
      apiKey: "interfere_secret_us_test_123",
      ...RELEASE,
    });
  });

  test.each([undefined, null, ""])(
    "skips the release without failing when the secret key is %j",
    (apiKey) => {
      expect(
        checkReleaseConfig(VITE_CONTEXT, { apiKey, ...RELEASE }),
      ).toMatchObject({ status: "skip" });
    },
  );

  test("skips the release for a missing secret key even without a commit SHA", () => {
    expect(
      checkReleaseConfig(VITE_CONTEXT, {
        apiKey: undefined,
        commitSha: null,
        releaseSlug: null,
      }),
    ).toMatchObject({ status: "skip" });
  });

  test("lists current prefixes and redacts a wrong-prefix key", () => {
    const result = checkReleaseConfig(NEXT_CONTEXT, {
      apiKey: "interfere_pk_super_secret_value",
      ...RELEASE,
    });

    expect(result).toMatchObject({
      status: "fail",
      title: "INTERFERE_API_KEY has wrong prefix",
    });

    const details = result.status === "ok" ? "" : result.details.join("\n");

    expect(details).toContain("`interfere_secret_us_`");
    expect(details).toContain("`interfere_secret_eu_`");
    expect(details).toContain("`interfere_pk...`");
    expect(details).toContain("`INTERFERE_PUBLIC_KEY`");
    expect(details).not.toContain("super_secret_value");
  });

  test.each([
    { commitSha: null, releaseSlug: null },
    { commitSha: COMMIT_SHA, releaseSlug: null },
  ])("warns about a missing release identity %j", (release) => {
    expect(
      checkReleaseConfig(NEXT_CONTEXT, {
        apiKey: "interfere_ak_test_123",
        ...release,
      }),
    ).toMatchObject({ status: "warn", title: "Commit SHA missing" });
  });
});

describe(diagnoseRelease, () => {
  test.each([
    ["AUTH_MISSING", "Secret key could not be read"],
    ["AUTH_AK_INVALID", "Secret key rejected by server"],
    ["AUTH_AK_SCOPE_MISMATCH", "Secret key missing release scope"],
    [
      "AUTH_AK_SURFACE_REQUIRED",
      "Public key needed for a workspace secret key",
    ],
    ["NETWORK_ACCESS_DENIED", "Build machine blocked by network policy"],
  ])("reports %s as a non-fatal error", (code, title) => {
    const diagnostic = diagnoseRelease(VITE_CONTEXT, { body: { code } });

    expect(diagnostic).toMatchObject({ severity: "error", title });

    expect(diagnostic.details.at(-1)).toBe(
      "The build continues, but this release has no source maps in Interfere.",
    );
  });

  test.each(["AUTH_MISSING", "AUTH_AK_SURFACE_REQUIRED"])(
    "renders the host public-key env var into %s diagnostics",
    (code) => {
      expect(
        diagnoseRelease(VITE_CONTEXT, { body: { code } }).details,
      ).toContainEqual(expect.stringContaining("`VITE_INTERFERE_PUBLIC_KEY`"));
    },
  );

  test("reports other failures as a warning with the underlying message", () => {
    const diagnostic = diagnoseRelease(
      NEXT_CONTEXT,
      new Error("socket hang up"),
    );

    expect(diagnostic.severity).toBe("warn");
    expect(diagnostic.details.at(-1)).toBe("Error: socket hang up");
  });

  test("points a network-denied build at the workspace security settings", () => {
    expect(
      diagnoseRelease(VITE_CONTEXT, { body: { code: "NETWORK_ACCESS_DENIED" } })
        .details,
    ).toContainEqual(
      expect.stringContaining("https://interfere.com/settings/security"),
    );
  });
});

describe(formatApiKeyPreview, () => {
  test("exports current and legacy API key prefixes", () => {
    expect(API_KEY_PREFIXES).toStrictEqual([
      "interfere_secret_us_",
      "interfere_secret_eu_",
      "interfere_ak_",
    ]);
  });

  test.each([
    "interfere_secret_us_test_123",
    "interfere_secret_eu_test_123",
    "interfere_ak_test_123",
  ])("accepts API key prefix %s", (apiKey) => {
    expect(isInterfereApiKey(apiKey)).toBe(true);
  });

  test.each(["interfere_pk_test_123", "interfere_pub_us_test_123", "secret"])(
    "rejects non-API key prefix %s",
    (apiKey) => {
      expect(isInterfereApiKey(apiKey)).toBe(false);
    },
  );

  test("returns short keys verbatim", () => {
    expect(formatApiKeyPreview("short")).toBe("short");
  });

  test("truncates long keys with an ellipsis", () => {
    expect(formatApiKeyPreview("interfere_ak_0123456789")).toBe(
      "interfere_ak...",
    );
  });
});
