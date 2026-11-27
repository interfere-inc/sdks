import { HttpResponse } from "msw";
import type { ReceivedRequest } from "@interfere/test-utils/vitest/telemetry-receiver";

const ACCEPTANCE_RELEASE_TOKEN = "irt_acceptance";

export function acceptanceRelease(request: ReceivedRequest) {
  return request.method === "POST" &&
    new URL(request.url, "http://receiver").pathname === "/v3/releases"
    ? HttpResponse.json({
        build: { hash: "0123456789abcdef0123456789abcdef01234567" },
        destination: {
          environment: "preview",
          id: "release_acceptance",
          slug: "rel_0123456789abcdef",
        },
        org: { id: "org_acceptance", slug: "acceptance" },
        source: { id: "source_acceptance" },
        surface: { id: "surface_acceptance", slug: "web" },
        token: ACCEPTANCE_RELEASE_TOKEN,
      })
    : undefined;
}
