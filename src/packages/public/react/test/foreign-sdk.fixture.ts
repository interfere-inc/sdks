/* oxlint-disable landmines/no-test-doubles -- Real passthrough wrappers model a foreign SDK's own patches; Interfere's behaviour is never replaced. */
import type { SessionId } from "@interfere/types/data/session";
import type { Attributes } from "@opentelemetry/api";
import {
  InMemoryLogRecordExporter,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace";

import { onConsoleCall } from "../src/internal/console-patch/console-patch.js";
import { Kernel } from "../src/internal/kernel.js";
import { registerBundledInstrumentations } from "../src/internal/otel/instrumentations/instrumentations.js";
import type { BundledInstrumentations } from "../src/internal/otel/instrumentations/instrumentations.js";
import type { TraceHeadersOption } from "../src/internal/otel/instrumentations/header-policy.js";
import { installViewTracker } from "../src/internal/otel/navigation.js";
import { traceContextWriterFor } from "../src/internal/otel/instrumentations/trace-context-writers.js";
import { buildOtelProvider } from "../src/internal/otel/provider.js";
import type { OtelProviderHandle } from "../src/internal/otel/provider.js";
import errorsPlugin from "../src/plugins/errors.js";
import { SessionTracker } from "../src/tracking/api.js";

import { foreignWriter } from "./foreign-writer.js";

const INTERFERE_SESSION_ID = "11111111-1111-4111-8111-111111111111";

export interface InterfereOptions {
  console?: boolean;
  headers?: TraceHeadersOption;
  errors?: boolean;
  propagation?: "interfere" | "external";
  navigation?: boolean;
  propagateTo?: string[];
  propagateTraceBaggage?: boolean;
}

export type RequestKind = "fetch" | "fetch-request" | "xhr";

export interface RequestSpec {
  kind: RequestKind;
  url: string;
}

export interface RequestResult {
  error: string | null;
  status: number;
}

export interface Observation {
  traceId: string | null;
  requestTraceId: string | null;
  sessionId: string | null;
  deviceId: string | null;
  kind: "event" | "span";
  url: string;
}

const foreignViews: string[] = [];
const foreignConsole: string[] = [];
const foreignErrors: string[] = [];

let foreignOnError: OnErrorEventHandler = null;

function installForeignHistory() {
  // oxlint-disable-next-line typescript/unbound-method
  const original = globalThis.history.pushState;

  const wrappedPushState: typeof original = function wrappedPushState(
    this: History,
    ...args: Parameters<typeof original>
  ) {
    original.apply(this, args);

    foreignViews.push(globalThis.location.pathname);
  };

  globalThis.history.pushState = wrappedPushState;

  return () => {
    if (globalThis.history.pushState === wrappedPushState) {
      globalThis.history.pushState = original;
    }
  };
}

function installForeignConsole() {
  const target = globalThis.console;
  const original = target.error;

  const wrappedError = (...args: unknown[]) => {
    foreignConsole.push(args.map(String).join(" "));

    original.apply(target, args);
  };

  target.error = wrappedError;

  return () => {
    if (target.error === wrappedError) {
      target.error = original;
    }
  };
}

function installForeignOnError() {
  const previous = globalThis.onerror;

  const wrappedOnError: OnErrorEventHandler = (
    event,
    source,
    lineno,
    colno,
    error,
  ) => {
    if (error instanceof Error) {
      foreignErrors.push(error.message);
    } else if (event instanceof Event) {
      foreignErrors.push(event.type);
    } else {
      foreignErrors.push(event);
    }

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- The preexisting host error handler may be null or a non-callable assignment.
    if (typeof previous !== "function") {
      return false;
    }

    const handled: unknown = previous.call(
      globalThis,
      event,
      source,
      lineno,
      colno,
      error,
    );

    return handled === true;
  };

  foreignOnError = wrappedOnError;

  // oxlint-disable-next-line unicorn/prefer-add-event-listener -- Models a foreign SDK that assigns and chains window.onerror.
  globalThis.onerror = wrappedOnError;

  return () => {
    if (globalThis.onerror === wrappedOnError) {
      // oxlint-disable-next-line unicorn/prefer-add-event-listener -- Restores the chained handler only while it is still current.
      globalThis.onerror = previous;
    }
  };
}

const spanExporter = new InMemorySpanExporter();
const logExporter = new InMemoryLogRecordExporter();
const interfereConsole: string[] = [];

let kernel: Kernel | null = null;
let handle: OtelProviderHandle | null = null;
let instrumentations: BundledInstrumentations | null = null;
let stopErrors: (() => void) | null = null;
let stopConsole: (() => void) | null = null;
let stopNavigation: (() => void) | null = null;

function installInterfere(options: InterfereOptions = {}) {
  if (handle) {
    throw new Error("Interfere already installed in this fixture");
  }

  const target = {
    baseUrl: `${globalThis.location.origin}/ingest`,
    publicKeyHeader: { apiKey: "fixture" },
  };

  const provider = buildOtelProvider({
    additionalLogRecordProcessors: [
      new SimpleLogRecordProcessor({ exporter: logExporter }),
    ],
    additionalSpanProcessors: [
      new SimpleSpanProcessor({ exporter: spanExporter }),
    ],
    canPersistExports: () => false,
    clientOptions: target,
    getDeviceId: () => "device-fixture",
    getSessionId: () => INTERFERE_SESSION_ID as SessionId,
    sdkStack: ["@interfere/react@fixture"],
  });

  handle = provider;

  kernel = new Kernel(new SessionTracker({ target }));

  instrumentations = registerBundledInstrumentations({
    ...options,
    ignoreUrls: [/\/ingest(?:[/?#]|$)/u],
    loggerProvider: provider.loggerProvider,
    propagateTo: options.propagateTo ?? [],
    traceHeaders: provider.traceHeaders,
    defersTraceContext:
      options.propagation === undefined
        ? (url: string) => traceContextWriterFor(url) !== undefined
        : undefined,
    writesTraceContext: options.propagation !== "external",
    tracerProvider: provider.tracerProvider,
  });

  kernel.attachOtel(provider, () => {
    instrumentations?.dispose();

    instrumentations = null;
  });

  if (options.errors) {
    stopErrors = errorsPlugin.setup(kernel) ?? null;
  }

  if (options.console) {
    stopConsole = onConsoleCall((level, args) => {
      interfereConsole.push(`${level}:${args.map(String).join(" ")}`);
    });
  }

  if (options.navigation) {
    const tracker = installViewTracker({
      documentContext: null,
      isEnabled: () => true,
      loggerProvider: provider.loggerProvider,
      views: kernel.views,
    });

    stopNavigation = tracker.dispose;
  }
}

async function disposeInterfere() {
  stopErrors?.();

  stopConsole?.();

  stopNavigation?.();

  stopErrors = null;

  stopConsole = null;

  stopNavigation = null;

  await kernel?.dispose();

  kernel = null;

  handle = null;

  instrumentations = null;
}

function isText(value: unknown): value is boolean | number | string {
  return ["string", "number", "boolean"].includes(typeof value);
}

function textOf(attributes: Attributes | undefined, keys: string[]) {
  for (const key of keys) {
    const value = attributes?.[key];

    if (isText(value)) {
      return String(value);
    }
  }

  return null;
}

const URL_KEYS = ["url.full", "http.url"];

function observations() {
  const spans = spanExporter.getFinishedSpans().flatMap((span) => {
    const url = textOf(span.attributes, URL_KEYS);

    return url === null
      ? []
      : [
          {
            kind: "span",
            traceId: span.spanContext().traceId,
            requestTraceId: textOf(span.attributes, [
              "interfere.request.trace_id",
            ]),
            url,
            sessionId: textOf(span.attributes, ["session.id"]),
            deviceId: textOf(span.attributes, ["device.id"]),
          } satisfies Observation,
        ];
  });

  const events = logExporter.getFinishedLogRecords().flatMap((record) => {
    const url = textOf(record.attributes, URL_KEYS);

    return url === null
      ? []
      : [
          {
            kind: "event",
            traceId: record.spanContext?.traceId ?? null,
            requestTraceId: textOf(record.attributes, [
              "interfere.request.trace_id",
            ]),
            url,
            sessionId: textOf(record.attributes, ["session.id"]),
            deviceId: textOf(record.attributes, ["device.id"]),
          } satisfies Observation,
        ];
  });

  return [...spans, ...events];
}

function exceptions() {
  return spanExporter
    .getFinishedSpans()
    .flatMap((span) =>
      span.events.filter((event) => event.name === "exception"),
    )
    .map((event) => textOf(event.attributes, ["exception.message"]) ?? "");
}

async function runFetch(input: RequestInfo) {
  try {
    const response = await fetch(input);

    return { error: null, status: response.status } satisfies RequestResult;
  } catch (error) {
    return {
      error: error instanceof Error ? error.name : String(error),
      status: 0,
    } satisfies RequestResult;
  }
}

async function runXhr(url: string) {
  return new Promise<RequestResult>((resolve) => {
    const xhr = new XMLHttpRequest();

    let error: string | null = null;

    xhr.addEventListener("error", () => {
      error = "error";
    });

    xhr.addEventListener("loadend", () => {
      resolve({ error, status: xhr.status });
    });

    xhr.open("GET", url);

    xhr.send();
  });
}

async function request(spec: RequestSpec) {
  switch (spec.kind) {
    case "xhr": {
      return runXhr(spec.url);
    }
    case "fetch-request": {
      return runFetch(new Request(spec.url, { headers: { "x-fixture": "1" } }));
    }
    default: {
      return runFetch(spec.url);
    }
  }
}

export const foreignSdkFixture = {
  foreign: {
    ...foreignWriter,
    console: () => foreignConsole,
    errors: () => foreignErrors,
    installConsole: installForeignConsole,
    installHistory: installForeignHistory,
    installOnError: installForeignOnError,
    onErrorIsCurrent: () =>
      foreignOnError !== null && globalThis.onerror === foreignOnError,
    views: () => foreignViews,
  },
  interfere: {
    console: () => interfereConsole,
    dispose: disposeInterfere,
    exceptions,
    install: installInterfere,
    observations,
  },
  pushState: (path: string) => {
    globalThis.history.pushState(null, "", path);
  },
  request,
  requestMany: async (specs: RequestSpec[]) =>
    Promise.all(specs.map(async (spec) => request(spec))),
  storageKeys: () => ({
    cookies: document.cookie
      .split(";")
      .map((entry) => entry.trim().split("=")[0] ?? "")
      .filter((name) => name !== ""),
    local: Array.from({ length: localStorage.length }, (_, index) =>
      localStorage.key(index),
    ).filter((key) => key !== null),
    session: Array.from({ length: sessionStorage.length }, (_, index) =>
      sessionStorage.key(index),
    ).filter((key) => key !== null),
  }),
  throwLater: (message: string) => {
    setTimeout(() => {
      throw new Error(message);
    }, 0);
  },
};

declare global {
  var foreignSdkFixture: typeof import("./foreign-sdk.fixture.js").foreignSdkFixture;
}

globalThis.foreignSdkFixture = foreignSdkFixture;
