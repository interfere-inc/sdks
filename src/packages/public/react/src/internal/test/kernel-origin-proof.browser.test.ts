import { browserStorage } from "@interfere/test-config/browser-storage";
import { HttpResponse, http } from "msw";
import { expect, test, vi } from "vitest";

import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { worker } from "../../../test/msw.js";
import {
  API,
  OWN_TRACEPARENT,
  PAGE_URL,
  exporters,
  ownershipHarness,
  sendXhr,
  settle,
} from "./kernel-ownership.test-utils.js";

const harness = ownershipHarness();

const BACKEND = "https://backend.test";

const EXPLICIT = "https://explicit.test";

const STRICT = "https://strict.test";

interface Wire {
  baggage: string | null;
  method: string;
  traceparent: string | null;
}

function record(origin: string, strict = false) {
  const seen: Wire[] = [];

  worker.use(
    http.all(`${origin}/*`, ({ request }) => {
      const entry = {
        baggage: request.headers.get("baggage"),
        method: request.method,
        traceparent: request.headers.get("traceparent"),
      };

      seen.push(entry);

      return strict && entry.traceparent !== null
        ? HttpResponse.error()
        : new HttpResponse(null, { status: 204 });
    }),
  );

  return seen;
}

function carried(seen: Wire[]) {
  return seen.map((entry) => [
    entry.method,
    OWN_TRACEPARENT.test(entry.traceparent ?? ""),
    entry.baggage !== null,
  ]);
}

function withBackendHosts() {
  worker.use(
    http.get(`${API}/v3/config`, () =>
      HttpResponse.json({
        backendHosts: ["backend.test"],
        plugins: DISABLE_ALL_PLUGINS,
      }),
    ),
  );
}

async function post(url: string) {
  await fetch(url, { body: "{}", method: "POST" });
}

test("a discovered backend host carries headers on GET from the start and on POST only after a header-bearing GET answered; an explicit origin always carries them", async () => {
  expect.hasAssertions();

  withBackendHosts();

  const backend = record(BACKEND);
  const explicit = record(EXPLICIT);

  const { observations, options } = exporters();

  await harness.boot({
    ...options,
    tracing: { mode: "application", propagateTo: [EXPLICIT] },
  });

  await post(`${BACKEND}/orders`);

  await sendXhr(`${BACKEND}/orders`, {}, "POST");

  await post(`${EXPLICIT}/orders`);

  await fetch(`${BACKEND}/orders`);

  await post(`${BACKEND}/orders`);

  await sendXhr(`${BACKEND}/orders`, {}, "POST");

  await settle();

  expect(carried(backend)).toStrictEqual([
    ["POST", false, false],
    ["POST", false, false],
    ["GET", true, true],
    ["POST", true, true],
    ["POST", true, true],
  ]);

  expect(carried(explicit)).toStrictEqual([["POST", true, true]]);

  expect(
    observations().map((entry) => entry.attributes["url.full"]),
  ).toStrictEqual([
    `${BACKEND}/orders`,
    `${BACKEND}/orders`,
    `${EXPLICIT}/orders`,
    `${BACKEND}/orders`,
    `${BACKEND}/orders`,
    `${BACKEND}/orders`,
  ]);
});

test("an origin that rejects a header-bearing request at the network layer is marked header-less for the session, warned once and never retried", async () => {
  expect.hasAssertions();

  const warn = vi.spyOn(console, "warn");

  const strict = record(STRICT, true);

  const { observations, options } = exporters();

  await harness.boot({ ...options, tracing: { propagateTo: [STRICT] } });

  await expect(fetch(`${STRICT}/orders`)).rejects.toThrow(TypeError);

  await post(`${STRICT}/orders`);

  await sendXhr(`${STRICT}/orders`);

  await fetch(`${STRICT}/orders`);

  await settle();

  expect(carried(strict)).toStrictEqual([
    ["GET", true, true],
    ["POST", false, false],
    ["GET", false, false],
    ["GET", false, false],
  ]);

  expect(
    warn.mock.calls.filter(([text]) =>
      String(text).includes("Access-Control-Allow-Headers"),
    ),
  ).toHaveLength(1);

  expect(observations()).toHaveLength(0);

  expect(
    JSON.parse(sessionStorage.getItem("interfere:propagation_denied") ?? "[]"),
  ).toStrictEqual([STRICT]);
});

test("the proof memo survives a reload: a backend host proven in this browser session carries headers on its first POST", async () => {
  expect.hasAssertions();

  withBackendHosts();

  browserStorage.session.set(
    "interfere:propagation_proven",
    JSON.stringify([BACKEND]),
  );

  const backend = record(BACKEND);

  const { options } = exporters();

  await harness.boot(options);

  await post(`${BACKEND}/orders`);

  await fetch(PAGE_URL);

  await settle();

  expect(carried(backend)).toStrictEqual([["POST", true, true]]);
});
