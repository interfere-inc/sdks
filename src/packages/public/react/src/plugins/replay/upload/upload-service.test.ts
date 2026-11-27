import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { HttpResponse, http } from "msw";
import { setupServer } from "msw/node";
import { afterAll, beforeAll, expect, test } from "vitest";
import { resolveTargets } from "../../../internal/config/config";
import { uploadReplayChunk } from "./upload";

const requests: Request[] = [];

const server = setupServer(
  http.post("https://replay.test/v3/replay/upload/session", ({ request }) => {
    requests.push(request);

    return new HttpResponse(null, { status: 204 });
  }),
);

beforeAll(() => server.listen({ onUnhandledFrame: "error" }));

afterAll(() => server.close());

test("carries service identity and authentication on replay uploads", async () => {
  setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", "workspace");

  const target = resolveTargets(undefined, "https://replay.test", {
    serviceName: "web",
    serviceNamespace: "shop",
  }).ingest;

  await uploadReplayChunk(target, "session", new Uint8Array([1]), {
    "x-interfere-sequence": "1",
  });

  expect(requests[0]?.headers.get("x-interfere-release-token")).toBe(
    "workspace",
  );

  expect(requests[0]?.headers.get("x-interfere-service")).toBe("web");

  expect(requests[0]?.headers.get("x-interfere-service-namespace")).toBe(
    "shop",
  );

  expect(requests[0]?.headers.get("x-interfere-sequence")).toBe("1");
});
