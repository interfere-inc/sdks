import { HttpResponse, http } from "msw";
import { setupServer } from "msw/node";
import { afterAll, afterEach, beforeAll, expect, test, vi } from "vitest";
import { createBuildConfigResolver } from "./config";

const server = setupServer();

beforeAll(() => server.listen({ onUnhandledFrame: "error" }));

afterEach(() => server.resetHandlers());

afterAll(() => server.close());

test.each([
  { status: 503, config: { plugins: undefined } },
  { status: 200, config: { platform: null, plugins: undefined } },
])(
  "records malformed or unavailable config as fallback: $status",
  async ({ status, config }) => {
    server.use(
      http.get("https://config.test/v3/config", () =>
        HttpResponse.json({ plugins: { errors: "invalid" } }, { status }),
      ),
    );

    const observe =
      vi.fn<NonNullable<Parameters<typeof createBuildConfigResolver>[0]>>();

    await createBuildConfigResolver(observe)("https://config.test", {
      releaseToken: "public-key",
    });

    expect(observe).toHaveBeenCalledWith({ config, status: "fallback" });
  },
);

test("distinguishes a valid empty config from a failed fetch", async () => {
  server.use(
    http.get("https://config.test/v3/config", () => HttpResponse.json({})),
  );

  const observe =
    vi.fn<NonNullable<Parameters<typeof createBuildConfigResolver>[0]>>();

  await createBuildConfigResolver(observe)("https://config.test", {
    releaseToken: "public-key",
  });

  expect(observe).toHaveBeenCalledWith({
    config: { platform: null },
    status: "fetched",
  });
});
