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

const PROVEN_KEY = "interfere:propagation_proven";

const DENIED_KEY = "interfere:propagation_denied";

interface Wire {
  baggage: string | null;
  method: string;
  path: string;
  traceparent: string | null;
  tracestate: string | null;
}

interface Backend {
  probeGate?: Promise<undefined>;
  rejects?: (entry: Wire) => boolean;
}

function record(origin: string, backend: Backend = {}) {
  const seen: Wire[] = [];

  worker.use(
    http.all(`${origin}/*`, async ({ request }) => {
      const url = new URL(request.url);

      const entry = {
        baggage: request.headers.get("baggage"),
        method: request.method,
        path: `${url.pathname}${url.search}`,
        traceparent: request.headers.get("traceparent"),
        tracestate: request.headers.get("tracestate"),
      };

      seen.push(entry);

      if (entry.method === "HEAD") {
        await backend.probeGate;
      }

      return backend.rejects?.(entry) === true
        ? HttpResponse.error()
        : new HttpResponse(null, { status: 204 });
    }),
  );

  return seen;
}

function requests(seen: Wire[]) {
  return seen.flatMap((entry) =>
    entry.method === "HEAD"
      ? []
      : [
          [
            entry.method,
            OWN_TRACEPARENT.test(entry.traceparent ?? ""),
            entry.baggage !== null,
          ],
        ],
  );
}

function probes(seen: Wire[]) {
  return seen.flatMap((entry) =>
    entry.method === "HEAD"
      ? [
          [
            entry.path,
            entry.traceparent !== null,
            entry.tracestate !== null,
            entry.baggage !== null,
          ],
        ]
      : [],
  );
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

function proven() {
  return JSON.parse(sessionStorage.getItem(PROVEN_KEY) ?? "{}") as unknown;
}

async function provenSettled(origin: string) {
  await vi.waitFor(() => {
    expect(proven()).toHaveProperty([origin]);
  });
}

async function post(url: string) {
  await fetch(url, { body: "{}", method: "POST" });
}

test("a discovered backend host carries no trace header on any request until Interfere's own HEAD probe passed CORS with that header set; an explicit origin always carries them", async () => {
  expect.hasAssertions();

  withBackendHosts();

  const gate = Promise.withResolvers<undefined>();

  const backend = record(BACKEND, { probeGate: gate.promise });
  const explicit = record(EXPLICIT);

  const { clientSpans, observations, options } = exporters();

  await harness.boot({
    ...options,
    tracing: { propagateTo: [EXPLICIT] },
  });

  await fetch(`${BACKEND}/orders?token=secret`);

  await post(`${BACKEND}/orders`);

  await sendXhr(`${BACKEND}/orders`, {}, "POST");

  await post(`${EXPLICIT}/orders`);

  gate.resolve(undefined);

  await provenSettled(BACKEND);

  await fetch(`${BACKEND}/orders`);

  await post(`${BACKEND}/orders`);

  await sendXhr(`${BACKEND}/orders`, {}, "POST");

  await settle();

  expect(requests(backend)).toStrictEqual([
    ["GET", false, false],
    ["POST", false, false],
    ["POST", false, false],
    ["GET", true, true],
    ["POST", true, true],
    ["POST", true, true],
  ]);

  expect(probes(backend)).toStrictEqual([["/orders", true, true, true]]);

  expect(requests(explicit)).toStrictEqual([["POST", true, true]]);

  expect(observations()).toStrictEqual([]);

  expect(
    clientSpans().map((span) => span.attributes["url.full"]),
  ).toStrictEqual([
    `${BACKEND}/orders?token=REDACTED`,
    `${BACKEND}/orders`,
    `${BACKEND}/orders`,
    `${EXPLICIT}/orders`,
    `${BACKEND}/orders`,
    `${BACKEND}/orders`,
    `${BACKEND}/orders`,
  ]);
});

test("the Profound shape: a discovered host that allows traceparent and tracestate but rejects baggage completes every request and links after a baggage-less probe", async () => {
  expect.hasAssertions();

  withBackendHosts();

  const backend = record(BACKEND, {
    rejects: (entry) => entry.baggage !== null,
  });

  const { options } = exporters();

  await harness.boot(options);

  await post(`${BACKEND}/slides`);

  await sendXhr(`${BACKEND}/slides`, {}, "PUT");

  await provenSettled(BACKEND);

  await post(`${BACKEND}/slides`);

  await sendXhr(`${BACKEND}/slides`, {}, "PUT");

  await settle();

  expect(requests(backend)).toStrictEqual([
    ["POST", false, false],
    ["PUT", false, false],
    ["POST", true, false],
    ["PUT", true, false],
  ]);

  expect(probes(backend)).toStrictEqual([
    ["/slides", true, true, true],
    ["/slides", true, true, false],
  ]);

  expect(proven()).toStrictEqual({
    [BACKEND]: ["traceparent", "tracestate"],
  });
});

test("a discovered host that rejects every trace header never sees one on an application request, is warned once and is not probed again this session", async () => {
  expect.hasAssertions();

  withBackendHosts();

  const warn = vi.spyOn(console, "warn");

  const backend = record(BACKEND, {
    rejects: (entry) => entry.traceparent !== null,
  });

  const { options } = exporters();

  await harness.boot(options);

  await fetch(`${BACKEND}/orders`);

  await vi.waitFor(() => {
    expect(
      JSON.parse(sessionStorage.getItem(DENIED_KEY) ?? "[]"),
    ).toStrictEqual([BACKEND]);
  });

  await post(`${BACKEND}/orders`);

  await sendXhr(`${BACKEND}/orders`, {}, "POST");

  await settle();

  expect(requests(backend)).toStrictEqual([
    ["GET", false, false],
    ["POST", false, false],
    ["POST", false, false],
  ]);

  expect(probes(backend)).toHaveLength(2);

  expect(
    warn.mock.calls.filter(([text]) =>
      String(text).includes("Access-Control-Allow-Headers"),
    ),
  ).toHaveLength(1);
});

test("an explicit origin that rejects a header-bearing request at the network layer is marked header-less for the session, warned once and never retried", async () => {
  expect.hasAssertions();

  const warn = vi.spyOn(console, "warn");

  const strict = record(STRICT, {
    rejects: (entry) => entry.traceparent !== null,
  });

  const { observations, options } = exporters();

  await harness.boot({ ...options, tracing: { propagateTo: [STRICT] } });

  await expect(fetch(`${STRICT}/orders`)).rejects.toThrow(TypeError);

  await post(`${STRICT}/orders`);

  await sendXhr(`${STRICT}/orders`);

  await fetch(`${STRICT}/orders`);

  await settle();

  expect(requests(strict)).toStrictEqual([
    ["GET", true, true],
    ["POST", false, false],
    ["GET", false, false],
    ["GET", false, false],
  ]);

  expect(probes(strict)).toHaveLength(0);

  expect(
    warn.mock.calls.filter(([text]) =>
      String(text).includes("Access-Control-Allow-Headers"),
    ),
  ).toHaveLength(1);

  expect(observations()).toHaveLength(0);

  expect(JSON.parse(sessionStorage.getItem(DENIED_KEY) ?? "[]")).toStrictEqual([
    STRICT,
  ]);
});

test("the proof memo survives a reload: a backend host proven in this browser session carries its proven headers on its first request without a new probe", async () => {
  expect.hasAssertions();

  withBackendHosts();

  browserStorage.session.set(
    PROVEN_KEY,
    JSON.stringify({ [BACKEND]: ["traceparent", "tracestate"] }),
  );

  const backend = record(BACKEND);

  const { options } = exporters();

  await harness.boot(options);

  await post(`${BACKEND}/orders`);

  await fetch(PAGE_URL);

  await settle();

  expect(requests(backend)).toStrictEqual([["POST", true, false]]);

  expect(probes(backend)).toHaveLength(0);
});
