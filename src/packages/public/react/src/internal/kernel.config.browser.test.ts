import { HttpResponse, http } from "msw";
import { worker } from "../../test/msw.js";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { INGEST_URL_GLOBAL } from "@interfere/types/sdk/injected-globals";
import type { Span } from "@opentelemetry/api";
import {
  ROOT_CONTEXT,
  TraceFlags,
  context,
  propagation,
  trace,
} from "@opentelemetry/api";
import { hrTimeToMilliseconds } from "@opentelemetry/core";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  StackContextManager,
  TracerProvider,
} from "@opentelemetry/sdk-trace";
import {
  InMemoryLogRecordExporter,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import { DISABLE_ALL_PLUGINS } from "../../test/fixtures/disable-all-plugins.js";
import { createKernel } from "./kernel.js";
import type { Kernel } from "./kernel.js";
import { afterEach, expect, onTestFinished, test, vi } from "vitest";

const MILLIS_PER_SECOND = 1000;

let ownedKernel: Kernel | undefined;

const ownedMeta: HTMLMetaElement[] = [];

afterEach(async () => {
  try {
    await ownedKernel?.dispose();
  } finally {
    ownedKernel = undefined;

    for (const meta of ownedMeta.splice(0)) {
      meta.remove();
    }

    trace.disable();

    context.disable();

    propagation.disable();

    vi.unstubAllGlobals();
  }
});

function holdConfig() {
  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  setInjectedConfiguration(INGEST_URL_GLOBAL, undefined);

  setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", "test-key");

  const held = Promise.withResolvers<Response>();
  const requested = Promise.withResolvers<Request>();
  const sessionPosts: string[] = [];

  worker.use(
    http.get("https://kernel.invalid/v3/config", async ({ request }) => {
      requested.resolve(request);

      return held.promise;
    }),
    http.post("*/v3/session", ({ request }) => {
      sessionPosts.push(new URL(request.url).origin);

      return new HttpResponse(null, { status: 204 });
    }),
    http.all(
      /^https:\/\/(?<ingest>ingest\.)?kernel\.invalid\//u,
      () => new HttpResponse(null, { status: 204 }),
    ),
  );

  return { held, requested: requested.promise, sessionPosts };
}

const UNSAMPLED_HOST = {
  spanId: "00f067aa0ba902b7",
  traceFlags: TraceFlags.NONE,
  traceId: "4bf92f3577b34da6a3ce929d0e0e4736",
};

function hostContextFor(host: string, hostSpan: Span | undefined) {
  if (hostSpan) {
    return trace.setSpan(ROOT_CONTEXT, hostSpan);
  }

  if (host === "unsampled") {
    return trace.setSpanContext(ROOT_CONTEXT, UNSAMPLED_HOST);
  }

  return ROOT_CONTEXT;
}

test.each([
  { errors: true, exported: 1, host: "none", links: [] },
  { errors: true, exported: 1, host: "recording", links: [] },
  { errors: false, exported: 0, host: "recording", links: [] },
  {
    errors: true,
    exported: 1,
    host: "unsampled",
    links: [UNSAMPLED_HOST.traceId],
  },
])(
  "necessary capture starts while the config request is open; the late config (errors $errors) decides delivery under host context $host",
  async ({ errors, exported, host, links }) => {
    expect.hasAssertions();

    context.setGlobalContextManager(new StackContextManager().enable());

    const hostSpans = new InMemorySpanExporter();

    const hostSpan =
      host === "recording"
        ? new TracerProvider({
            spanProcessors: [new SimpleSpanProcessor({ exporter: hostSpans })],
          })
            .getTracer("host")
            .startSpan("host-work")
        : undefined;

    const hostContext = hostContextFor(host, hostSpan);

    vi.useFakeTimers({ toFake: ["Date"] });

    onTestFinished(() => {
      vi.useRealTimers();
    });

    const { held, requested, sessionPosts } = holdConfig();
    const exporter = new InMemorySpanExporter();

    const creating = createKernel({
      opts: {
        _internalAdditionalSpanProcessors: [
          new SimpleSpanProcessor({ exporter }),
        ],
        apiHost: "https://kernel.invalid",
      },
    });

    await requested;

    const capturedAt = Date.UTC(2026, 0, 1);

    vi.setSystemTime(capturedAt);

    context.with(hostContext, () => {
      globalThis.onerror?.(
        "startup boom",
        "https://app.invalid/main.js",
        1,
        1,
        new Error("startup boom"),
      );
    });

    hostSpan?.end();

    expect(sessionPosts).toStrictEqual([]);

    vi.setSystemTime(capturedAt + 60_000);

    held.resolve(
      HttpResponse.json({
        ingestUrl: "https://ingest.kernel.invalid",
        plugins: { ...DISABLE_ALL_PLUGINS, errors },
      }),
    );

    const kernel = await creating;

    await kernel.flush();

    const spans = exporter
      .getFinishedSpans()
      .filter((span) => span.events.some((e) => e.name === "exception"));

    expect(spans).toHaveLength(exported);

    expect(
      hostSpans.getFinishedSpans().flatMap((span) => span.events),
    ).toStrictEqual([]);

    for (const span of spans) {
      expect(Math.round(hrTimeToMilliseconds(span.startTime))).toBe(capturedAt);

      expect(span.parentSpanContext?.spanId).toBe(
        hostSpan?.spanContext().spanId,
      );

      expect(span.links.map((link) => link.context.traceId)).toStrictEqual(
        links,
      );
    }

    await vi.waitFor(() => {
      expect(sessionPosts).toStrictEqual(["https://ingest.kernel.invalid"]);
    });

    await kernel.dispose();
  },
);

const SSR_TRACE_ID = "0af7651916cd43dd8448eb211c80319c";

function setMeta(name: string, content: string) {
  const meta = document.createElement("meta");

  meta.setAttribute("name", name);

  meta.setAttribute("content", content);

  document.head.append(meta);

  ownedMeta.push(meta);
}

test.each([
  { flags: "01", stamped: true, linked: true },
  { flags: "00", stamped: true, linked: true },
  { flags: "01", stamped: false, linked: false },
])(
  "never parents page work under SSR meta with flags $flags (stamped: $stamped); the initial page view carries it only when fresh: $linked",
  async ({ flags, stamped, linked }) => {
    expect.hasAssertions();

    setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

    setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", "test-key");

    setMeta("traceparent", `00-${SSR_TRACE_ID}-b7ad6b7169203331-${flags}`);

    if (stamped) {
      setMeta(
        "tracestate",
        `interfere=${Math.floor(Date.now() / MILLIS_PER_SECOND)}`,
      );
    }

    worker.use(
      http.get("https://kernel.invalid/v3/config", () =>
        HttpResponse.json({
          plugins: { ...DISABLE_ALL_PLUGINS, pageEvents: true },
        }),
      ),
      http.all(
        "https://kernel.invalid/*",
        () => new HttpResponse(null, { status: 204 }),
      ),
    );

    const logs = new InMemoryLogRecordExporter();

    const kernel = await createKernel({
      opts: {
        _internalAdditionalLogRecordProcessors: [
          new SimpleLogRecordProcessor({ exporter: logs }),
        ],
        apiHost: "https://kernel.invalid",
      },
    });

    ownedKernel = kernel;

    kernel.syncProviderConfig({});

    const span = kernel.getTracer("ssr-seed-test").startSpan("page-work");

    expect(span.isRecording()).toBe(true);
    expect(span.spanContext().traceId).not.toBe(SSR_TRACE_ID);

    span.end();

    const pageView = await vi.waitFor(() => {
      const found = logs
        .getFinishedLogRecords()
        .find((record) => record.eventName === "browser.navigation");

      expect(found).toBeDefined();

      return found;
    });

    expect(pageView?.spanContext?.traceId === SSR_TRACE_ID).toBe(linked);
  },
);
