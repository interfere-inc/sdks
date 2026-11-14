import { nativeBrowserFixture } from "@interfere/test-config/native-browser";
import { HttpResponse, http } from "msw";
import { beforeEach, expect, test } from "vitest";

interface SeenRequest {
  path: string;
  method: string;
  body: string;
  traceparent: string | null;
}

let seen: SeenRequest[];
let slow: PromiseWithResolvers<Response>;

const createPage = nativeBrowserFixture(
  new URL("../../../../test/xhr-cors.fixture.ts", import.meta.url),
  [
    http.all("*", async ({ request }) => {
      const path = new URL(request.url).pathname;

      seen.push({
        path,
        method: request.method,
        body: await request.text(),
        traceparent: request.headers.get("traceparent"),
      });

      const headers = new Headers({
        "access-control-allow-origin": request.headers.get("origin") ?? "*",
        "access-control-allow-methods": "POST",
      });

      if (path !== "/strict") {
        headers.set(
          "access-control-allow-headers",
          "traceparent,tracestate,baggage",
        );
      }

      if (request.method === "OPTIONS") {
        return new HttpResponse(null, { status: 204, headers });
      }

      if (path === "/slow") {
        return slow.promise;
      }

      return new HttpResponse("accepted", {
        headers: path === "/unreadable" ? {} : headers,
      });
    }),
  ],
);

beforeEach(() => {
  seen = [];

  slow = Promise.withResolvers<Response>();
});

test("automatic cross-origin XHR sends its POST once without trace headers and preserves completion events", async () => {
  const runtime = await createPage();

  const backend = await createPage();

  const target = new URL("/strict", backend.page.url()).href;

  await runtime.page.evaluate(() =>
    globalThis.xhrCorsFixture.install({ backendHosts: ["127.0.0.1"] }),
  );

  const result = await runtime.page.evaluate(
    async (url) => globalThis.xhrCorsFixture.request(url),
    target,
  );

  expect(result.status).toBe(200);
  expect(result.body).toBe("accepted");

  expect(
    result.events.filter((event) =>
      ["load", "error", "abort", "timeout", "loadend"].includes(event),
    ),
  ).toStrictEqual(["load", "loadend"]);

  expect(result.events[0]).toBe("readystatechange:1");
  expect(result.events).toContain("readystatechange:4");

  expect(seen).toStrictEqual([
    {
      path: "/strict",
      method: "POST",
      body: "customer-order",
      traceparent: null,
    },
  ]);
});

test("same-origin XHR retains tracing", async () => {
  const runtime = await createPage();

  await runtime.page.evaluate(() => globalThis.xhrCorsFixture.install({}));

  const result = await runtime.page.evaluate(async () =>
    globalThis.xhrCorsFixture.request("/open"),
  );

  expect(result.status).toBe(200);
  expect(seen).toHaveLength(1);
  expect(seen[0]?.traceparent).toMatch(/^00-[0-9a-f]{32}-[0-9a-f]{16}-01$/u);
});

test("preserves an explicit exact-origin cross-origin opt-in", async () => {
  const runtime = await createPage();

  const backend = await createPage();

  const target = new URL("/open", backend.page.url()).href;

  await runtime.page.evaluate((url) => {
    globalThis.xhrCorsFixture.install({ propagateTo: [new URL(url).origin] });
  }, target);

  const result = await runtime.page.evaluate(
    async (url) => globalThis.xhrCorsFixture.request(url),
    target,
  );

  expect(result.status).toBe(200);

  expect(seen.map((request) => request.method)).toStrictEqual([
    "OPTIONS",
    "POST",
  ]);

  expect(seen[1]?.traceparent).toMatch(/^00-[0-9a-f]{32}-[0-9a-f]{16}-01$/u);
});

test("does not replay an explicitly traced POST whose response fails CORS after being processed", async () => {
  const runtime = await createPage();

  const backend = await createPage();

  const target = new URL("/unreadable", backend.page.url()).href;

  await runtime.page.evaluate(
    (url) =>
      globalThis.xhrCorsFixture.install({ propagateTo: [new URL(url).origin] }),
    target,
  );

  const result = await runtime.page.evaluate(
    async (url) => globalThis.xhrCorsFixture.request(url),
    target,
  );

  expect(result.status).toBe(0);

  expect(
    result.events.filter((event) =>
      ["load", "error", "loadend"].includes(event),
    ),
  ).toStrictEqual(["error", "loadend"]);

  expect(seen.filter((request) => request.method === "POST")).toHaveLength(1);
});

test.each(["abort", "timeout"])(
  "preserves native %s without retry",
  async (mode) => {
    const runtime = await createPage();

    const backend = await createPage();

    const target = new URL("/slow", backend.page.url()).href;

    await runtime.page.evaluate(() =>
      globalThis.xhrCorsFixture.install({ backendHosts: ["127.0.0.1"] }),
    );

    const result = await runtime.page.evaluate(
      async ({ url, selected }) =>
        globalThis.xhrCorsFixture.request(
          url,
          selected === "abort" ? { abort: true } : { timeout: 100 },
        ),
      { url: target, selected: mode },
    );

    slow.resolve(new HttpResponse("late"));

    expect(result.status).toBe(0);

    expect(
      result.events.filter((event) =>
        ["load", "error", "abort", "timeout", "loadend"].includes(event),
      ),
    ).toStrictEqual([mode, "loadend"]);

    expect(seen.length).toBeLessThanOrEqual(1);
  },
);

test("does not broaden configured HTTPS backend origins to HTTP or other ports", async () => {
  const runtime = await createPage();

  const backend = await createPage();

  const target = new URL("/open", backend.page.url()).href;

  await runtime.page.evaluate(() =>
    globalThis.xhrCorsFixture.install({ backendHosts: ["127.0.0.1"] }),
  );

  const status = await runtime.page.evaluate(
    async (url) =>
      (await fetch(url, { method: "POST", body: "customer-order" })).status,
    target,
  );

  expect(status).toBe(200);

  expect(seen.map((request) => request.method)).toStrictEqual(["POST"]);

  expect(seen[0]?.traceparent).toBeNull();
});

test("leaves trace headers supplied by the application untouched", async () => {
  const runtime = await createPage();

  const backend = await createPage();

  const target = new URL("/open", backend.page.url()).href;

  await runtime.page.evaluate(() =>
    globalThis.xhrCorsFixture.install({ backendHosts: ["127.0.0.1"] }),
  );

  const result = await runtime.page.evaluate(
    async (url) =>
      globalThis.xhrCorsFixture.request(url, { traceHeader: true }),
    target,
  );

  expect(result.status).toBe(200);

  expect(seen[1]?.traceparent).toBe(
    "00-12345678901234567890123456789012-1234567890123456-01",
  );
});
