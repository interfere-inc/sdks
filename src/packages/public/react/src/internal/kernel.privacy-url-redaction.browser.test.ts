import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { context, propagation, trace } from "@opentelemetry/api";
import {
  InMemoryLogRecordExporter,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace";

import { DISABLE_ALL_PLUGINS } from "../../test/fixtures/disable-all-plugins.js";
import { createKernel } from "./kernel.js";
import type { Kernel, KernelOptions } from "./kernel.js";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

import { worker } from "../../test/msw.js";

const API_HOST = "https://kernel.invalid";

const SECRET = "fixture-secret";

const WAIT_OPTS = { timeout: 5000 };

let ownedKernel: Kernel | undefined;
let host: HTMLDivElement;
let priorUrl: string;
let priorState: unknown;

beforeEach(() => {
  priorUrl = location.href;

  priorState = history.state;

  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

  trace.disable();

  context.disable();

  propagation.disable();

  host = document.createElement("div");

  document.body.append(host);
});

afterEach(async () => {
  try {
    await ownedKernel?.dispose();
  } finally {
    ownedKernel = undefined;

    host.remove();

    history.replaceState(priorState, "", priorUrl);

    trace.disable();

    context.disable();

    propagation.disable();
  }
});

async function startKernel(opts: KernelOptions) {
  const spans = new InMemorySpanExporter();
  const logs = new InMemoryLogRecordExporter();

  worker.use(
    http.get(`${API_HOST}/v3/config`, () =>
      HttpResponse.json({
        plugins: { ...DISABLE_ALL_PLUGINS, pageEvents: true, rageClick: true },
      }),
    ),
    http.all(`${API_HOST}/*`, () => new HttpResponse(null, { status: 204 })),
  );

  const kernel = await createKernel({
    onCreated: (created) => {
      created.consent.set(true);
    },
    opts: {
      ...opts,
      apiHost: API_HOST,
      _internalAdditionalSpanProcessors: [
        new SimpleSpanProcessor({ exporter: spans }),
      ],
      _internalAdditionalLogRecordProcessors: [
        new SimpleLogRecordProcessor({ exporter: logs }),
      ],
    },
  });

  ownedKernel = kernel;

  await vi.waitFor(() => {
    expect(navigationLogs(logs)).not.toHaveLength(0);
  }, WAIT_OPTS);

  return { kernel, logs, spans };
}

function navigationLogs(logs: InMemoryLogRecordExporter) {
  return logs
    .getFinishedLogRecords()
    .filter((record) => "browser.navigation.type" in record.attributes);
}

function serialized(
  spans: InMemorySpanExporter,
  logs: InMemoryLogRecordExporter,
) {
  return JSON.stringify([
    ...spans
      .getFinishedSpans()
      .map(({ attributes, events, name }) => ({ attributes, events, name })),
    ...logs.getFinishedLogRecords().map(({ attributes }) => ({ attributes })),
  ]);
}

function render(html: string) {
  const fragment = document.createRange().createContextualFragment(html);

  for (const node of fragment.querySelectorAll(
    "button, a, input, label, select, form",
  )) {
    node.addEventListener(
      node instanceof HTMLFormElement ? "submit" : "click",
      (event) => {
        event.preventDefault();
      },
    );
  }

  host.append(fragment);
}

function fixture(selector: string) {
  const found = host.querySelector<HTMLElement>(selector);

  if (!found) {
    throw new Error(`Missing fixture element ${selector}`);
  }

  return found;
}

describe("URL redaction", () => {
  test("privacy.urls redacts every exported URL suffix while requests keep the original URL", async () => {
    expect.hasAssertions();

    const received: { traceparent: string | null; url: string }[] = [];

    worker.use(
      http.get(`${location.origin}/api/:resource`, ({ params, request }) => {
        received.push({
          traceparent: request.headers.get("traceparent"),
          url: request.url,
        });

        return new HttpResponse(null, {
          status: params["resource"] === "orders" ? 500 : 503,
        });
      }),
    );

    history.replaceState({}, "", `/landing?q=${SECRET}#${SECRET}`);

    const { kernel, logs, spans } = await startKernel({
      privacy: { urls: true },
    });

    render(`<button id="go">Go</button>`);

    kernel.syncProviderConfig({});

    fixture("#go").click();

    await fetch(`/api/orders?q=${SECRET}`);

    kernel.recordException(
      new Error(`checkout failed at https://pay.test/confirm?q=${SECRET}`, {
        cause: new Error(`upstream https://api.test/charge#${SECRET}`),
      }),
      { mechanism: { handled: true, type: "browser.onerror" } },
    );

    const xhrDone = Promise.withResolvers<undefined>();
    const xhr = new XMLHttpRequest();

    xhr.open("GET", `/api/xhr?q=${SECRET}`);

    xhr.addEventListener("loadend", () => xhrDone.resolve(undefined), {
      once: true,
    });

    xhr.send();

    await xhrDone.promise;

    history.pushState({}, "", `/next?q=${SECRET}#${SECRET}`);

    const [fetchSpan, xhrSpan] = await vi.waitFor(() => {
      const requests = ["/api/orders", "/api/xhr"].map((path) =>
        spans
          .getFinishedSpans()
          .find((record) => record.attributes["url.path"] === path),
      );

      expect(requests).not.toContain(undefined);

      return requests;
    }, WAIT_OPTS);

    await kernel.flush();

    expect(received.map(({ url }) => url)).toStrictEqual([
      `${location.origin}/api/orders?q=${SECRET}`,
      `${location.origin}/api/xhr?q=${SECRET}`,
    ]);

    for (const { traceparent } of received) {
      expect(traceparent).toMatch(/^00-/u);
    }

    expect(
      [fetchSpan, xhrSpan].map((span) => span?.attributes["url.full"]),
    ).toStrictEqual([
      `${location.origin}/api/orders?q=REDACTED`,
      `${location.origin}/api/xhr?q=REDACTED`,
    ]);

    expect(
      spans
        .getFinishedSpans()
        .flatMap((span) => span.events)
        .map((event) => event.attributes?.["exception.message"]),
    ).toStrictEqual(
      expect.arrayContaining([
        `HTTP 500 GET ${location.origin}/api/orders?q=REDACTED`,
        `HTTP 503 XHR ${location.origin}/api/xhr?q=REDACTED`,
        "checkout failed at https://pay.test/confirm?q=REDACTED",
      ]),
    );

    expect(
      navigationLogs(logs).map((record) => record.attributes["url.full"]),
    ).toStrictEqual([
      `${location.origin}/landing?q=REDACTED`,
      `${location.origin}/next?q=REDACTED`,
    ]);

    expect(serialized(spans, logs)).not.toContain(SECRET);
  });
});
