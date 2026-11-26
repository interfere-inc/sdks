import { nativeBrowserFixture } from "@interfere/test-config/native-browser";
import { HttpResponse, http } from "msw";
import { beforeEach, expect, test } from "vitest";

interface Seen {
  origin: string;
  path: string;
  method: string;
  body: string;
  trace: string | null;
  marker: string | null;
}

let seen: Seen[];

const failureBody = JSON.stringify({
  code: "PAYMENT_DECLINED",
  message: "private-customer@example.com",
});

const createPage = nativeBrowserFixture(
  new URL("../../../../test/network-semantics.fixture.ts", import.meta.url),
  [
    http.all("*", async ({ request }) => {
      const url = new URL(request.url);

      seen.push({
        origin: url.origin,
        path: url.pathname,
        method: request.method,
        body: await request.text(),
        trace: request.headers.get("traceparent"),
        marker: request.headers.get("x-customer"),
      });

      const headers = new Headers({
        "access-control-allow-origin": request.headers.get("origin") ?? "*",
        "access-control-allow-methods": "GET,HEAD,POST,PUT,PATCH,DELETE",
        "access-control-allow-headers":
          url.pathname === "/strict"
            ? "x-customer"
            : "traceparent,tracestate,baggage,x-customer,content-type",
      });

      if (request.method === "OPTIONS") {
        return new HttpResponse(null, { status: 204, headers });
      }

      if (url.pathname === "/unreadable") {
        return new HttpResponse("processed", { status: 200 });
      }

      if (url.pathname === "/failure") {
        headers.set("content-type", "application/json");

        headers.set(
          "content-length",
          String(new TextEncoder().encode(failureBody).length),
        );

        return new HttpResponse(failureBody, { status: 422, headers });
      }

      return new HttpResponse(request.method === "HEAD" ? null : "accepted", {
        headers,
      });
    }),
  ],
);

beforeEach(() => {
  seen = [];
});

async function pair() {
  const client = await createPage();

  const backend = await createPage();

  const { origin } = new URL(backend.page.url());

  await client.page.evaluate(
    async (target) =>
      globalThis.networkSemanticsFixture.install({ propagateTo: [target] }),
    origin,
  );

  return { page: client.page, origin };
}

test.each(["GET", "HEAD"])(
  "native %s CORS failures keep one request per call and diagnose the origin once",
  async (method) => {
    expect.hasAssertions();

    const { page, origin } = await pair();

    const warnings: string[] = [];

    page.on("console", (message) => {
      if (message.type() === "warning") {
        warnings.push(message.text());
      }
    });

    const results = await page.evaluate(
      async ({ target, verb }) => {
        async function attempt() {
          try {
            await fetch(`${target}/unreadable`, { method: verb });

            return "success";
          } catch (error) {
            return error instanceof Error ? error.name : "unknown";
          }
        }

        return [await attempt(), await attempt()];
      },
      { target: origin, verb: method },
    );

    expect(results).toStrictEqual(["TypeError", "TypeError"]);

    await expect(
      page.evaluate(async () =>
        globalThis.networkSemanticsFixture.suspectCors(1),
      ),
    ).resolves.toBe(1);

    const requests = seen.filter((item) => item.method !== "OPTIONS");

    expect(requests.map((item) => item.method)).toStrictEqual([method, method]);

    expect(requests[0]?.trace).toMatch(/^00-[a-f0-9]{32}-[a-f0-9]{16}-01$/u);

    expect(Boolean(requests[1]?.trace)).toBe(false);

    expect(
      warnings.filter((text) => text.startsWith("[interfere] Requests to")),
    ).toStrictEqual([
      `[interfere] Requests to ${origin} fail at the network layer while carrying trace headers. If ${origin} does not allow "traceparent, tracestate, baggage" in Access-Control-Allow-Headers, add them there or give ${origin} a narrower headers list in tracing.propagateTo. Interfere did not retry the request and sends no trace headers to ${origin} for the rest of this browser session.`,
    ]);
  },
);

test("an allowed origin does not authorize another port on the same hostname", async () => {
  const { page, origin } = await pair();

  const second = await createPage();

  const other = new URL(second.page.url()).origin;

  await page.evaluate(
    async ({ allowed, unlisted }) => {
      await (await fetch(`${allowed}/open`)).text();

      await (await fetch(`${unlisted}/strict`)).text();
    },
    { allowed: origin, unlisted: other },
  );

  expect(
    seen.find((item) => item.origin === origin && item.method === "GET")?.trace,
  ).toMatch(/^00-[a-f0-9]{32}-[a-f0-9]{16}-01$/u);

  expect(
    seen.find((item) => item.origin === other && item.method === "GET")?.trace,
  ).toBeNull();
});

test("backend hostnames do not authorize an HTTP origin or its port", async () => {
  const { page, origin } = await pair();

  await page.evaluate(async (target) => {
    await globalThis.networkSemanticsFixture.install({
      backendHosts: [new URL(target).hostname],
    });

    await (await fetch(`${target}/strict`)).text();
  }, origin);

  expect(seen).toMatchObject([{ origin, method: "GET", trace: null }]);
});

test("an invalid fetch receiver keeps its native rejection without sending a request", async () => {
  const { page, origin } = await pair();

  const result = await page.evaluate(async (target) => {
    const receiver = {};

    try {
      await fetch.call(receiver, new URL("/open", target));

      return "resolved";
    } catch (error) {
      return error instanceof Error ? error.name : "unknown";
    }
  }, origin);

  expect(result).toBe("TypeError");
  expect(seen).toStrictEqual([]);
});

test("request input overrides preserve the customer's method, headers and body", async () => {
  const { page, origin } = await pair();

  const result = await page.evaluate(async (target) => {
    const request = new Request(`${target}/open`, {
      headers: { "x-customer": "original" },
    });

    const response = await fetch(request, {
      method: "POST",
      headers: { "x-customer": "override" },
      body: "order=one",
    });

    return { status: response.status, body: await response.text() };
  }, origin);

  expect(result).toStrictEqual({ status: 200, body: "accepted" });

  expect(seen.filter((item) => item.method !== "OPTIONS")).toMatchObject([
    { method: "POST", marker: "override", body: "order=one" },
  ]);
});

test.each(["POST", "PUT", "PATCH", "DELETE"])(
  "never duplicates a %s already processed before an unreadable CORS response",
  async (method) => {
    const { page, origin } = await pair();

    const result = await page.evaluate(
      async ({ target, verb }) => {
        try {
          await fetch(`${target}/unreadable`, {
            method: verb,
            body: "customer-mutation",
          });

          return "success";
        } catch (error) {
          return error instanceof Error ? error.name : "unknown";
        }
      },
      { target: origin, verb: method },
    );

    expect(result).toBe("TypeError");
    expect(seen.filter((item) => item.method === method)).toHaveLength(1);
  },
);

test("an already aborted request never reaches the customer backend", async () => {
  const { page, origin } = await pair();

  const result = await page.evaluate(async (target) => {
    const controller = new AbortController();

    controller.abort();

    try {
      await fetch(`${target}/open`, { signal: controller.signal });

      return "success";
    } catch (error) {
      return error instanceof Error ? error.name : "unknown";
    }
  }, origin);

  expect(result).toBe("AbortError");
  expect(seen).toHaveLength(0);
});

test("hTTP failure evidence preserves the response and excludes private payload fields", async () => {
  const { page, origin } = await pair();

  const result = await page.evaluate(async (target) => {
    const response = await fetch(`${target}/failure`, {
      method: "POST",
      body: "private-order-value",
    });

    return { status: response.status, body: await response.text() };
  }, origin);

  expect(result).toStrictEqual({ status: 422, body: failureBody });

  await expect
    .poll(async () =>
      page.evaluate(() =>
        globalThis.networkSemanticsFixture
          .spans()
          .filter((span) =>
            span.events.some((event) => event.name === "exception"),
          ),
      ),
    )
    .toHaveLength(1);

  const spans = await page.evaluate(() =>
    globalThis.networkSemanticsFixture.spans(),
  );

  expect(
    spans.find((span) =>
      span.events.some((event) => event.name === "exception"),
    )?.attributes["http.response.error_codes"],
  ).toStrictEqual(["PAYMENT_DECLINED"]);

  expect(JSON.stringify(spans)).not.toMatch(
    /private-customer|private-order-value/u,
  );

  expect(seen.filter((item) => item.method === "POST")).toHaveLength(1);
});

test("disposal restores native fetch and reinstall produces a single failure event", async () => {
  const { page, origin } = await pair();

  await page.evaluate(async (target) => {
    globalThis.networkSemanticsFixture.dispose();

    await (await fetch(`${target}/open`)).text();

    await globalThis.networkSemanticsFixture.install({
      propagateTo: [target],
    });

    await (await fetch(`${target}/failure`)).text();
  }, origin);

  expect(seen.find((item) => item.path === "/open")?.trace).toBeNull();

  await expect
    .poll(async () =>
      page.evaluate(
        () =>
          globalThis.networkSemanticsFixture
            .spans()
            .flatMap((span) => span.events)
            .filter((event) => event.name === "exception").length,
      ),
    )
    .toBe(1);

  expect(
    seen.filter((item) => item.path === "/failure" && item.method === "GET"),
  ).toHaveLength(1);
});

test("disabling failed-request capture leaves native error responses readable", async () => {
  const { page, origin } = await pair();

  const status = await page.evaluate(async (target) => {
    await globalThis.networkSemanticsFixture.install({
      propagateTo: [target],
      captureFailedRequests: false,
    });

    const response = await fetch(`${target}/failure`);

    await response.text();

    return response.status;
  }, origin);

  expect(status).toBe(422);

  await expect
    .poll(async () =>
      page.evaluate(() => globalThis.networkSemanticsFixture.spans().length),
    )
    .toBeGreaterThan(0);

  await expect(
    page.evaluate(() =>
      globalThis.networkSemanticsFixture
        .spans()
        .flatMap((span) => span.events)
        .filter((event) => event.name === "exception"),
    ),
  ).resolves.toStrictEqual([]);
});
