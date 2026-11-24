/* oxlint-disable eslint/no-console -- The foreign stand-in wraps console.error the way a vendor breadcrumb patch does. */
import {
  FORCE_ENABLE_GLOBAL,
  PUBLIC_KEY_GLOBAL,
} from "@interfere/types/sdk/injected-globals";
import { PLUGINS_GLOBAL } from "@interfere/types/sdk/remote-config/plugins";
import { propagation } from "@opentelemetry/api";
import type {
  Context,
  TextMapPropagator,
  TextMapSetter,
} from "@opentelemetry/api";
import { PAGE_VIEW_EVENT_NAME } from "@interfere/types/data/correlation-events";
import {
  InMemoryLogRecordExporter,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace";
import type {
  Kernel,
  KernelInternalOptions,
} from "../../src/internal/kernel.js";
import { createWrapperSingleton } from "../../src/internal/wrapper-singleton.js";

Object.assign(globalThis, {
  [FORCE_ENABLE_GLOBAL]: true,
  [PUBLIC_KEY_GLOBAL]: "interfere_public_us_native",
  [PLUGINS_GLOBAL]: {
    device: false,
    logs: false,
    rageClick: false,
    replay: false,
  },
});

const wrapper = createWrapperSingleton({
  initEntryName: "foreign-sdk-context.fixture",
  producerVersion: "@interfere/react-fixture@0.0.0",
});

const kernelIds = new WeakMap<Kernel, number>();
let nextKernelId = 0;
let exporter = new InMemorySpanExporter();
let logExporter = new InMemoryLogRecordExporter();

const calls = {
  consoleError: 0,
  fetch: 0,
  errorHandler: 0,
  pushState: 0,
  xhrOpen: 0,
  xhrSend: 0,
};

interface ForeignHandles {
  consoleError?: typeof console.error;
  fetch?: typeof globalThis.fetch;
  errorHandler?: OnErrorEventHandler;
  pushState?: History["pushState"];
}

const foreign: ForeignHandles = {};

type HeaderCarrier = Record<string, string>;

class ForeignPropagator implements TextMapPropagator<HeaderCarrier> {
  inject(
    _context: Context,
    carrier: HeaderCarrier,
    setter: TextMapSetter<HeaderCarrier>,
  ) {
    setter.set(carrier, "x-foreign-trace", "foreign-1");
  }

  extract(active: Context) {
    return active;
  }

  fields() {
    return ["x-foreign-trace"];
  }
}

function mergeHeaders(init: RequestInit | undefined, extra: HeadersInit) {
  const headers = new Headers(init?.headers);

  for (const [key, value] of new Headers(extra)) {
    const existing = headers.get(key);

    headers.set(
      key,
      key === "baggage" && existing ? `${existing},${value}` : value,
    );
  }

  return { ...init, headers };
}

function isApplicationRequest(input: string | URL | Request) {
  const url = new URL(
    input instanceof Request ? input.url : input,
    location.href,
  );

  return url.pathname.startsWith("/api/");
}

type ForeignKind =
  | "console"
  | "fetch"
  | "history"
  | "onerror"
  | "propagator"
  | "xhr";

function installForeignConsole() {
  const original = console.error;

  const wrapped: typeof console.error = (...args) => {
    calls.consoleError += 1;

    original.apply(console, args);
  };

  foreign.consoleError = wrapped;

  console.error = wrapped;

  return true;
}

function installForeignFetch(headers: Record<string, string> | undefined) {
  const original = globalThis.fetch;

  const wrapped: typeof globalThis.fetch = async (input, init) => {
    if (isApplicationRequest(input)) {
      calls.fetch += 1;
    }

    return original.call(
      globalThis,
      input,
      headers ? mergeHeaders(init, headers) : init,
    );
  };

  foreign.fetch = wrapped;

  // oxlint-disable-next-line landmines/no-test-doubles -- The foreign stand-in always calls the real fetch through; it observes and adds headers like a vendor fill() wrapper.
  globalThis.fetch = wrapped;

  return true;
}

function installForeignHistory() {
  // oxlint-disable-next-line typescript/unbound-method -- The foreign stand-in saves the current method to chain it, like a vendor fill().
  const original = globalThis.history.pushState;

  const wrapped: History["pushState"] = function wrapped(
    this: History,
    ...args
  ) {
    calls.pushState += 1;

    original.apply(this, args);
  };

  foreign.pushState = wrapped;

  // oxlint-disable-next-line landmines/no-test-doubles -- The foreign stand-in always calls the real pushState through; it only observes.
  globalThis.history.pushState = wrapped;

  return true;
}

function installForeignErrorHandler() {
  const previous = globalThis.onerror;

  const wrapped: OnErrorEventHandler = function wrapped(this: Window, ...args) {
    calls.errorHandler += 1;

    const handled: unknown = previous?.apply(this, args);

    return handled === true;
  };

  foreign.errorHandler = wrapped;

  // oxlint-disable-next-line unicorn/prefer-add-event-listener -- The foreign stand-in replaces window.onerror exactly like a chaining vendor wrapper and calls the previous handler through.
  globalThis.onerror = wrapped;

  return true;
}

function installForeignXhr() {
  const proto = XMLHttpRequest.prototype;

  // oxlint-disable-next-line typescript/unbound-method -- The foreign stand-in proxies the prototype methods like a vendor Proxy wrapper.
  const { open, send } = proto;

  proto.open = new Proxy(open, {
    apply(
      target,
      thisArg: XMLHttpRequest,
      args: Parameters<XMLHttpRequest["open"]>,
    ) {
      calls.xhrOpen += 1;

      target.apply(thisArg, args);
    },
  });

  proto.send = new Proxy(send, {
    apply(
      target,
      thisArg: XMLHttpRequest,
      args: Parameters<XMLHttpRequest["send"]>,
    ) {
      calls.xhrSend += 1;

      target.apply(thisArg, args);
    },
  });

  return true;
}

const fixture = {
  calls,
  installForeign(
    kind: ForeignKind,
    options: { headers?: Record<string, string> } = {},
  ) {
    switch (kind) {
      case "console": {
        return installForeignConsole();
      }
      case "fetch": {
        return installForeignFetch(options.headers);
      }
      case "history": {
        return installForeignHistory();
      }
      case "onerror": {
        return installForeignErrorHandler();
      }
      case "propagator": {
        return propagation.setGlobalPropagator(new ForeignPropagator());
      }
      case "xhr": {
        return installForeignXhr();
      }
      default: {
        return false;
      }
    }
  },
  async install(opts: { meta?: Record<string, string> } = {}) {
    for (const [name, content] of Object.entries(opts.meta ?? {})) {
      const element = document.createElement("meta");

      element.name = name;

      element.content = content;

      document.head.append(element);
    }

    exporter = new InMemorySpanExporter();

    logExporter = new InMemoryLogRecordExporter();

    const internalOptions: KernelInternalOptions = {
      enabled: true,
      apiHost: location.origin,
      _internalAdditionalLogRecordProcessors: [
        new SimpleLogRecordProcessor({ exporter: logExporter }),
      ],
      _internalAdditionalSpanProcessors: [
        new SimpleSpanProcessor({ exporter }),
      ],
    };

    const kernel = await wrapper.init(internalOptions);

    if (!kernel) {
      return null;
    }

    kernel.syncProviderConfig({});

    if (!kernelIds.has(kernel)) {
      nextKernelId += 1;

      kernelIds.set(kernel, nextKernelId);
    }

    return kernelIds.get(kernel) ?? null;
  },
  async close() {
    await wrapper.close();
  },
  async request(path: string, init?: RequestInit) {
    const response = await fetch(path, init);

    return { status: response.status, body: await response.text() };
  },
  async xhr(path: string) {
    return new Promise<{ status: number; body: string }>((resolve) => {
      const xhr = new XMLHttpRequest();

      xhr.onloadend = () =>
        resolve({ status: xhr.status, body: xhr.responseText });

      xhr.open("GET", path);

      xhr.send();
    });
  },
  navigate(path: string) {
    globalThis.history.pushState({}, "", path);
  },
  throwLater(message: string) {
    setTimeout(() => {
      throw new Error(message);
    }, 0);
  },
  spans() {
    return exporter.getFinishedSpans().map((span) => ({
      name: span.name,
      traceId: span.spanContext().traceId,
      spanId: span.spanContext().spanId,
      parentSpanId: span.parentSpanContext?.spanId ?? null,
      links: span.links.map((link) => ({
        traceId: link.context.traceId,
        traceFlags: link.context.traceFlags,
      })),
      events: span.events.map((event) => ({
        name: event.name,
        attributes: event.attributes ?? {},
      })),
    }));
  },
  pageViews() {
    return logExporter
      .getFinishedLogRecords()
      .filter((record) => record.eventName === PAGE_VIEW_EVENT_NAME)
      .map((record) => ({
        trigger: record.attributes["interfere.view.trigger"],
        traceId: record.spanContext?.traceId ?? null,
        traceFlags: record.spanContext?.traceFlags ?? null,
      }));
  },
  state() {
    return {
      calls: { ...calls },
      consoleErrorIsForeign: console.error === foreign.consoleError,
      fetchIsForeign: globalThis.fetch === foreign.fetch,
      fields: propagation.fields(),
      onerrorIsForeign: globalThis.onerror === foreign.errorHandler,
      pushStateIsForeign: globalThis.history.pushState === foreign.pushState,
    };
  },
};

declare global {
  var foreignSdkContextFixture: typeof fixture;
}

globalThis.foreignSdkContextFixture = fixture;
