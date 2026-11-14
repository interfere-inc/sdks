import {
  DEPLOY_ENVIRONMENT_MAX_LENGTH,
  DeployEnvironmentError,
  applyDeployEnvironment,
  resolveDeployEnvironment,
} from "./deploy-environment.ts";
import { describe, expect, test } from "vitest";

describe(resolveDeployEnvironment, () => {
  test("resolves INTERFERE_ENVIRONMENT over the provider signal", () => {
    expect(
      resolveDeployEnvironment({
        INTERFERE_ENVIRONMENT: "Staging",
        VERCEL_ENV: "production",
      }),
    ).toBe("staging");
  });

  test("resolves VERCEL_ENV", () => {
    expect(resolveDeployEnvironment({ VERCEL_ENV: "preview" })).toBe("preview");
  });

  test("prefers VERCEL_TARGET_ENV over VERCEL_ENV", () => {
    expect(
      resolveDeployEnvironment({
        VERCEL_ENV: "preview",
        VERCEL_TARGET_ENV: "staging",
      }),
    ).toBe("staging");
  });

  test("resolves CLOUDFLARE_ENV", () => {
    expect(resolveDeployEnvironment({ CLOUDFLARE_ENV: "production" })).toBe(
      "production",
    );
  });

  test("returns null when no deploy environment is detected", () => {
    expect(resolveDeployEnvironment({})).toBeNull();
  });

  test("treats an empty signal value as absent", () => {
    expect(resolveDeployEnvironment({ VERCEL_ENV: "" })).toBeNull();
  });

  test("accepts an environment at the maximum length", () => {
    const name = "a".repeat(DEPLOY_ENVIRONMENT_MAX_LENGTH);

    expect(resolveDeployEnvironment({ INTERFERE_ENVIRONMENT: name })).toBe(
      name,
    );
  });

  test("throws when the resolved environment exceeds the maximum length", () => {
    const name = "a".repeat(DEPLOY_ENVIRONMENT_MAX_LENGTH + 1);

    expect(() =>
      resolveDeployEnvironment({ INTERFERE_ENVIRONMENT: name }),
    ).toThrow(DeployEnvironmentError);
  });
});

describe(applyDeployEnvironment, () => {
  test("returns destination metadata unchanged when no environment is resolved", () => {
    const destination = {
      deploymentId: "dpl_123",
      deploymentUrl: "https://example.vercel.app",
      destinationReleaseId: "dpl_123",
      environment: null,
      environmentName: null,
      environmentTarget: null,
      provider: "vercel" as const,
    };

    expect(applyDeployEnvironment(destination, null)).toBe(destination);
  });

  test("sets only environment for CLI destination metadata", () => {
    expect(
      applyDeployEnvironment(
        {
          deploymentUrl: null,
          destinationReleaseId: "rel_1234567890abcdef",
          environment: null,
          provider: "cli",
        },
        "preview",
      ),
    ).toStrictEqual({
      deploymentUrl: null,
      destinationReleaseId: "rel_1234567890abcdef",
      environment: "preview",
      provider: "cli",
    });
  });

  test("sets environment aliases for platform destination metadata", () => {
    expect(
      applyDeployEnvironment(
        {
          accountId: "account_123",
          deploymentUrl: "https://worker.example.com",
          destinationReleaseId: "version_123",
          environment: null,
          environmentName: null,
          environmentTarget: null,
          provider: "cloudflare",
          scriptName: "worker",
          versionId: "version_123",
        },
        "production",
      ),
    ).toMatchObject({
      environment: "production",
      environmentName: "production",
      environmentTarget: "production",
    });
  });
});
