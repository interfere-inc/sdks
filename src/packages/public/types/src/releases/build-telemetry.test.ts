import {
  faultForBuildResult,
  releaseBuildResultSchema,
  releaseBuildStepSchema,
} from "./build-telemetry.ts";
import { describe, expect, test } from "vitest";

describe(faultForBuildResult, () => {
  test("blames the customer for every result they alone can fix", () => {
    expect(faultForBuildResult("api_key_wrong_prefix")).toBe("customer");
    expect(faultForBuildResult("api_key_unreadable")).toBe("customer");
    expect(faultForBuildResult("api_key_rejected")).toBe("customer");
    expect(faultForBuildResult("api_key_scope_mismatch")).toBe("customer");
    expect(faultForBuildResult("no_source_maps")).toBe("customer");
    expect(faultForBuildResult("client_maps_unmapped")).toBe("customer");
  });

  test("blames us for the catch-all failure so the error budget never flatters us", () => {
    expect(faultForBuildResult("pipeline_failed")).toBe("interfere");
  });

  test("attributes a successful run to us, keeping ok in the SLO denominator", () => {
    expect(faultForBuildResult("ok")).toBe("interfere");
  });

  test("classifies every result the schema allows", () => {
    expect.hasAssertions();

    for (const result of releaseBuildResultSchema.options) {
      expect(["interfere", "customer"]).toContain(faultForBuildResult(result));
    }
  });
});

describe("releaseBuildStepSchema contract", () => {
  test("keeps total alongside the parts so the dashboard can split them", () => {
    expect(releaseBuildStepSchema.options).toStrictEqual([
      "discover",
      "create",
      "upload",
      "preflight",
      "cleanup",
      "total",
    ]);
  });
});
