import { createReleaseRequest, createReleaseResponse } from "./definition.ts";
import { describe, expect, test } from "vitest";
import { z } from "zod";

describe("createReleaseRequestSchema", () => {
  const validSource = {
    branch: "main",
    commitMessage: "feat: init",
    commitSha: "abc123",
    provider: "github",
  };

  test("accepts a valid request with buildId", () => {
    const result = createReleaseRequest.safeParse({
      buildId: "abc123",
      destination: null,
      source: validSource,
    });

    expect(result.success).toBe(true);
  });

  test("rejects request with null buildId", () => {
    const result = createReleaseRequest.safeParse({
      buildId: null,
      destination: null,
      source: validSource,
    });

    expect(result.success).toBe(false);
  });

  test("rejects request with missing buildId", () => {
    const result = createReleaseRequest.safeParse({
      destination: null,
      source: validSource,
    });

    expect(result.success).toBe(false);
  });

  test("accepts request with empty string buildId", () => {
    const result = createReleaseRequest.safeParse({
      buildId: "",
      destination: null,
      source: validSource,
    });

    expect(result.success).toBe(true);
  });
});

describe("createReleaseResponseSchema", () => {
  test("parses a valid response matching the server shape", () => {
    const serverResponse = {
      build: { hash: "abc123" },
      destination: {
        environment: "production",
        id: "release_99",
        slug: "rel_0123456789abcdef",
      },
      org: { id: "org_1", slug: "my-org" },
      source: { id: "abc123" },
      surface: { id: "surface_42", slug: "my-surface" },
    };

    const result = createReleaseResponse.safeParse(serverResponse);

    expect(result.success).toBe(true);
    expect(result.data).toStrictEqual(serverResponse);
  });

  test("answers with the service and documents surface as deprecated", () => {
    const serverResponse = {
      build: { hash: "abc123" },
      destination: {
        environment: "production",
        id: "release_99",
        slug: "rel_0123456789abcdef",
      },
      org: { id: "org_1", slug: "my-org" },
      service: { id: "service_7", name: "checkout", namespace: null },
      source: { id: "abc123" },
      surface: null,
      token: null,
    };

    expect(createReleaseResponse.parse(serverResponse)).toStrictEqual(
      serverResponse,
    );

    const { $defs } = z.toJSONSchema(createReleaseResponse, { io: "input" });

    expect($defs?.["CreateReleaseResponse"]?.description).toContain(
      "surface is deprecated: read service instead",
    );

    expect($defs?.["CreateReleaseResponse"]).toMatchObject({
      properties: {
        service: {
          anyOf: [{ $ref: "#/$defs/ReleaseServiceRef" }, { type: "null" }],
        },
      },
    });

    expect($defs?.["ReleaseServiceRef"]).toMatchObject({
      properties: {
        id: { type: "string" },
        name: { type: "string" },
        namespace: { type: ["string", "null"] },
      },
      required: ["id", "name", "namespace"],
    });
  });

  test("rejects response with flat fields instead of nested", () => {
    const legacyResponse = {
      environment: "production",
      organizationId: "org_123",
      releaseId: "release_99",
      releaseSlug: "happy-tiger-sunset",
      sourceId: "abc123",
      surfaceId: "surface_42",
      surfaceSlug: "my-surface",
    };

    const result = createReleaseResponse.safeParse(legacyResponse);

    expect(result.success).toBe(false);
  });

  test("rejects response with org.id as number", () => {
    const wrongType = {
      build: { hash: "abc123" },
      destination: {
        environment: "production",
        id: "release_99",
        slug: "rel_0123456789abcdef",
      },
      org: { id: 1, slug: "my-org" },
      source: { id: "abc123" },
      surface: { id: "surface_42", slug: "my-surface" },
    };

    const result = createReleaseResponse.safeParse(wrongType);

    assert.isFalse(result.success);

    expect(result.error.issues.map((issue) => issue.path)).toStrictEqual([
      ["org", "id"],
    ]);
  });

  test("rejects response with null build.hash", () => {
    const result = createReleaseResponse.safeParse({
      build: { hash: null },
      destination: {
        environment: "production",
        id: "release_99",
        slug: "rel_0123456789abcdef",
      },
      org: { id: "org_1", slug: "my-org" },
      source: { id: "abc123" },
      surface: { id: "surface_42", slug: "my-surface" },
    });

    assert.isFalse(result.success);

    expect(result.error.issues.map((issue) => issue.path)).toStrictEqual([
      ["build", "hash"],
    ]);
  });

  test("rejects response with missing build.hash", () => {
    const result = createReleaseResponse.safeParse({
      build: {},
      destination: {
        environment: "production",
        id: "release_99",
        slug: "rel_0123456789abcdef",
      },
      org: { id: "org_1", slug: "my-org" },
      source: { id: "abc123" },
      surface: { id: "surface_42", slug: "my-surface" },
    });

    assert.isFalse(result.success);

    expect(result.error.issues.map((issue) => issue.path)).toStrictEqual([
      ["build", "hash"],
    ]);
  });
});
