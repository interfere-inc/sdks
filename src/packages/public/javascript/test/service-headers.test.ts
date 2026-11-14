import { InterfereClient } from "../src/index";
import { HttpResponse, http } from "msw";
import { setupServer } from "msw/node";
import { afterAll, beforeAll, expect, test } from "vitest";

const requests: Request[] = [];

const server = setupServer(
  http.all("https://service.test/*", ({ request }) => {
    requests.push(request);

    return HttpResponse.json({});
  }),
);

beforeAll(() => server.listen({ onUnhandledRequest: "error" }));

afterAll(() => server.close());

test("preserves service headers across generated gateway clients", async () => {
  expect.hasAssertions();

  const client = new InterfereClient({
    baseUrl: "https://service.test",
    publicKeyHeader: { apiKey: "workspace" },
    apiKey: { token: "secret" },
    headers: {
      "x-interfere-service": "web",
      "x-interfere-service-namespace": "shop",
    },
    maxRetries: 0,
  });

  await client.config.getSdkConfig();

  await client.sessions.createSession({ sessionId: "session" });

  await client.sessions.identifySession({
    sessionId: "session",
    identifier: "user",
    source: { type: "custom", name: "test" },
  });

  await client.releases.getReleasesConfig();

  await client.fetch("/v3/replay/upload/session", {
    method: "POST",
    body: new Uint8Array([1]),
  });

  expect(requests).toHaveLength(5);

  for (const request of requests) {
    expect(request.headers.get("x-interfere-service")).toBe("web");
    expect(request.headers.get("x-interfere-service-namespace")).toBe("shop");
  }
});
