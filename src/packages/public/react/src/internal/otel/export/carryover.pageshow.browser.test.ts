import { browserStorage } from "@interfere/test-config/browser-storage";
import { JsonTraceSerializer } from "@opentelemetry/otlp-transformer";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, test } from "vitest";
import { z } from "zod";

import {
  CLIENT_OPTIONS,
  INGEST,
  gate,
  hidePage,
  makeSpan,
  resetSent,
  sent,
  sentSignal,
  showPage,
} from "../../../../test/fixtures/export-collector.js";
import type { Sent } from "../../../../test/fixtures/export-collector.js";
import { worker } from "../../../../test/msw.js";
import { isPageHidden } from "../../page-lifecycle/page-lifecycle.js";
import { clearExportCarryover } from "./carryover.js";
import { buildOtelProvider } from "../provider.js";
import type { OtelProviderHandle } from "../provider.js";
import { decodedBodyText } from "../../../../test/fixtures/request-body.js";

const prefix = "interfere:export:v1:";
const scope = JSON.stringify([INGEST, "test-key", "traces"]);

const traceBodySchema = z.object({
  resourceSpans: z.array(
    z.object({
      scopeSpans: z.array(
        z.object({ spans: z.array(z.object({ name: z.string() })) }),
      ),
    }),
  ),
});

function spanNames(entries: Sent[]) {
  return entries
    .filter((entry) => entry.signal === "traces")
    .flatMap((entry) =>
      traceBodySchema
        .parse(JSON.parse(entry.text))
        .resourceSpans.flatMap((resourceSpans) =>
          resourceSpans.scopeSpans.flatMap((scopeSpans) =>
            scopeSpans.spans.map((span) => span.name),
          ),
        ),
    );
}

function retained() {
  return Array.from({ length: localStorage.length }, (_, index) =>
    localStorage.key(index),
  ).filter((key): key is string => key?.startsWith(prefix) ?? false);
}

let handle: OtelProviderHandle | null = null;

beforeEach(() => {
  isPageHidden();

  clearExportCarryover();

  resetSent();
});

afterEach(async () => {
  showPage();

  await handle?.shutdown();

  handle = null;

  clearExportCarryover();
});

test("a page shown while its hide-time drain is failing still delivers the request span queued behind a drained 12.4.4 carryover", async () => {
  expect.hasAssertions();

  const held = gate();

  const v12Body = new TextDecoder().decode(
    JsonTraceSerializer.serializeRequest([
      makeSpan({ "interfere.sdk.version": "12.4.4" }),
    ]),
  );

  browserStorage.local.set(
    `${prefix}12-4-4`,
    JSON.stringify({ body: v12Body, created: Date.now() - 1000, scope }),
  );

  worker.use(
    http.post(`${INGEST}/drain/v1/:signal`, async ({ params, request }) => {
      const text = await decodedBodyText(request);

      const index = sent.length;

      sent.push({
        bytes: new TextEncoder().encode(text).byteLength,
        keepalive: request.keepalive,
        signal: String(params["signal"]),
        text,
      });

      sentSignal.notify();

      if (index === 1) {
        await held.wait();
      }

      return index >= 1 && index <= 3
        ? new HttpResponse(null, {
            status: 503,
            headers: { "retry-after": "0" },
          })
        : new HttpResponse(null);
    }),
  );

  handle = buildOtelProvider({
    clientOptions: CLIENT_OPTIONS,
    getDeviceId: () => null,
    getSessionId: () => null,
    sdkStack: ["@interfere/react@13.0.0"],
  });

  await sentSignal.reached(1);

  expect(spanNames(sent)).toStrictEqual(["test-span"]);

  handle.tracerProvider.getTracer("test").startSpan("GET /api/ping").end();

  hidePage();

  const flushed = handle.tracerProvider.forceFlush();

  await sentSignal.reached(2);

  expect(retained()).toHaveLength(1);

  showPage();

  held.open();

  await flushed;

  await sentSignal.reached(5);

  expect(spanNames(sent.slice(4))).toContain("GET /api/ping");

  await navigator.locks.request(prefix + scope, () => {
    expect(retained()).toHaveLength(0);
  });
});
