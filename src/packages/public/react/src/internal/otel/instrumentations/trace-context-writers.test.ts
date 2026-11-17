import { afterEach, expect, test } from "vitest";

import { traceContextWriterFor } from "./trace-context-writers.js";

const URL_UNDER_TEST = "https://api.example.com/orders";

interface SentryOptions {
  propagateTraceparent: boolean | string;
  tracePropagationTargets: string | (string | number)[];
}

type DatadogMatcher = string | RegExp | ((url: string) => boolean);

type DatadogTracingUrls =
  | string
  | undefined
  | (
      | DatadogMatcher
      | number
      | { match: DatadogMatcher; propagatorTypes?: string | string[] }
    )[];

class SentryClient {
  private readonly options: SentryOptions;

  constructor(options: SentryOptions) {
    this.options = options;
  }

  getOptions() {
    return this.options;
  }
}

class SentryScope {
  private readonly client: SentryClient;

  constructor(client: SentryClient) {
    this.client = client;
  }

  getClient() {
    return this.client;
  }
}

function installSentry(options: SentryOptions) {
  Object.assign(globalThis, {
    __SENTRY__: {
      version: "10.78.0",
      "10.78.0": {
        defaultCurrentScope: new SentryScope(new SentryClient(options)),
      },
    },
  });
}

function installDatadog(allowedTracingUrls: DatadogTracingUrls) {
  Object.assign(globalThis, {
    DD_RUM: { getInitConfiguration: () => ({ allowedTracingUrls }) },
  });
}

afterEach(() => {
  Reflect.deleteProperty(globalThis, "__SENTRY__");

  Reflect.deleteProperty(globalThis, "DD_RUM");
});

test("finds a Sentry client whose scope and options are read through prototype methods", () => {
  installSentry({
    propagateTraceparent: true,
    tracePropagationTargets: ["api.example.com"],
  });

  expect(traceContextWriterFor(URL_UNDER_TEST)).toBe("sentry");
});

test.each<SentryOptions>([
  { propagateTraceparent: false, tracePropagationTargets: ["api.example"] },
  { propagateTraceparent: "true", tracePropagationTargets: ["api.example"] },
  { propagateTraceparent: true, tracePropagationTargets: ["other.example"] },
  { propagateTraceparent: true, tracePropagationTargets: ["api", 5] },
  { propagateTraceparent: true, tracePropagationTargets: "api.example" },
])("leaves the header to Interfere for Sentry options %j", (options) => {
  installSentry(options);

  expect(traceContextWriterFor(URL_UNDER_TEST)).toBeUndefined();
});

test.for<DatadogTracingUrls>([
  ["https://api.example.com"],
  [/api\.example/u],
  [(url) => url.endsWith("/orders")],
  [{ match: "https://api.example.com", propagatorTypes: ["tracecontext"] }],
  [{ match: /orders/u }],
])("finds Datadog RUM tracing a matching URL %#", (allowedTracingUrls) => {
  installDatadog(allowedTracingUrls);

  expect(traceContextWriterFor(URL_UNDER_TEST)).toBe("datadog");
});

test.for<DatadogTracingUrls>([
  ["api.example.com"],
  [{ match: "https://api.example.com", propagatorTypes: ["datadog"] }],
  [{ match: "https://api.example.com", propagatorTypes: "tracecontext" }],
  ["https://api.example.com", 5],
  "https://api.example.com",
  undefined,
])("ignores Datadog RUM configuration %j", (allowedTracingUrls) => {
  installDatadog(allowedTracingUrls);

  expect(traceContextWriterFor(URL_UNDER_TEST)).toBeUndefined();
});
