/* oxlint-disable interfere-testing/no-test-doubles -- Real passthrough wrappers model a foreign SDK that owns trace headers; Interfere's behaviour is never replaced. */
import { bytesToHex } from "@interfere/helpers/hex";

export type ForeignStyle = "b3" | "b3multi" | "vendor" | "w3c";

export interface ForeignOptions {
  allowedOrigins?: string[];
  baggage?: boolean;
  /** A vendor global the writer announces itself under, such as `DD_RUM`; it reports the writer's styles as Datadog `allowedTracingUrls` propagator types. */
  global?: string;
  injectUnsampled?: boolean;
  sampled?: boolean;
  sessionDelayMs?: number;
  styles?: ForeignStyle[];
}

export interface ForeignEvent {
  headers: string[] | null;
  kind: "fetch" | "xhr";
  status: number;
  url: string;
}

const foreignEvents: ForeignEvent[] = [];

let foreignSession: string | null = null;
let foreignStopped = true;
let foreignRestore: (() => void) | null = null;

function hex(bytes: number) {
  return bytesToHex(crypto.getRandomValues(new Uint8Array(bytes)));
}

function foreignHeaders(options: ForeignOptions) {
  if (foreignSession === null) {
    return null;
  }

  const sampled = options.sampled ?? true;

  if (!sampled && !options.injectUnsampled) {
    return null;
  }

  const traceId = hex(8);
  const spanId = hex(8);
  const flag = sampled ? "1" : "0";
  const headers: Record<string, string> = {};

  for (const style of options.styles ?? ["w3c", "vendor"]) {
    switch (style) {
      case "w3c": {
        headers["traceparent"] =
          `00-0000000000000000${traceId}-${spanId}-0${flag}`;

        headers["tracestate"] = `vendor=s:${flag}`;

        break;
      }
      case "vendor": {
        headers["x-vendor-trace-id"] = traceId;

        headers["x-vendor-parent-id"] = spanId;

        headers["x-vendor-sampling-priority"] = flag;

        break;
      }
      case "b3": {
        headers["b3"] = `${traceId}-${spanId}-${flag}`;

        break;
      }
      default: {
        headers["X-B3-TraceId"] = traceId;

        headers["X-B3-SpanId"] = spanId;

        headers["X-B3-Sampled"] = flag;
      }
    }
  }

  if (options.baggage ?? true) {
    headers["baggage"] = `session.id=${foreignSession}`;
  }

  return headers;
}

function listHeaders(raw: HeadersInit | undefined) {
  const list: [string, string][] = [];

  if (raw instanceof Headers || Array.isArray(raw)) {
    for (const [key, value] of raw) {
      list.push([key, value]);
    }
  } else if (raw) {
    for (const [key, value] of Object.entries(raw)) {
      list.push([key, value]);
    }
  }

  return list;
}

function recordForeign(event: ForeignEvent) {
  if (!foreignStopped && foreignSession !== null) {
    foreignEvents.push(event);
  }
}

const PROPAGATORS: Record<ForeignStyle, string> = {
  b3: "b3",
  b3multi: "b3multi",
  vendor: "datadog",
  w3c: "tracecontext",
};

function announce(options: ForeignOptions) {
  const propagatorTypes = (options.styles ?? ["w3c", "vendor"]).map(
    (style) => PROPAGATORS[style],
  );

  return {
    getInitConfiguration: () => ({
      allowedTracingUrls: [
        { match: globalThis.location.origin, propagatorTypes },
      ],
    }),
    version: "fixture",
  };
}

function installForeign(options: ForeignOptions = {}) {
  foreignRestore?.();

  if (options.global) {
    Object.assign(globalThis, { [options.global]: announce(options) });
  }

  foreignEvents.length = 0;

  foreignSession = null;

  foreignStopped = false;

  const allowed = (url: string) => {
    const { origin } = new URL(url, globalThis.location.href);

    return options.allowedOrigins?.includes(origin) ?? true;
  };

  const originalFetch = globalThis.fetch;

  const wrappedFetch: typeof fetch = async function wrappedFetch(input, init) {
    const url = input instanceof Request ? input.url : String(input);

    const injected =
      !foreignStopped && allowed(url) ? foreignHeaders(options) : null;

    const headers = injected ? Object.keys(injected).toSorted() : null;

    let nextInput = input;
    let nextInit = init;

    if (injected) {
      if (input instanceof Request && !init?.headers) {
        nextInput = new Request(input);

        for (const [key, value] of Object.entries(injected)) {
          nextInput.headers.append(key, value);
        }
      } else {
        nextInit = {
          ...init,
          headers: [...listHeaders(init?.headers), ...Object.entries(injected)],
        };
      }
    }

    try {
      const response =
        nextInit === undefined
          ? await originalFetch.call(globalThis, nextInput)
          : await originalFetch.call(globalThis, nextInput, nextInit);

      recordForeign({ headers, kind: "fetch", status: response.status, url });

      return response;
    } catch (error) {
      recordForeign({ headers, kind: "fetch", status: 0, url });

      throw error;
    }
  };

  globalThis.fetch = wrappedFetch;

  const proto = XMLHttpRequest.prototype;
  // oxlint-disable-next-line typescript/unbound-method
  const originalOpen = proto.open;
  // oxlint-disable-next-line typescript/unbound-method
  const originalSend = proto.send;
  const urls = new WeakMap<XMLHttpRequest, string>();

  const wrappedOpen: typeof originalOpen = function wrappedOpen(
    this: XMLHttpRequest,
    ...args: [
      string,
      string | URL,
      boolean?,
      (string | null)?,
      (string | null)?,
    ]
  ) {
    const [method, url, async = true, username, password] = args;

    urls.set(this, String(url));

    originalOpen.call(this, method, url, async, username, password);
  };

  const wrappedSend: typeof originalSend = function wrappedSend(
    this: XMLHttpRequest,
    body?: Parameters<typeof originalSend>[0],
  ) {
    const url = urls.get(this);

    const injected =
      url !== undefined && !foreignStopped && allowed(url)
        ? foreignHeaders(options)
        : null;

    if (injected) {
      for (const [key, value] of Object.entries(injected)) {
        this.setRequestHeader(key, value);
      }
    }

    if (url !== undefined) {
      const headers = injected ? Object.keys(injected).toSorted() : null;

      this.addEventListener("loadend", () => {
        recordForeign({ headers, kind: "xhr", status: this.status, url });
      });
    }

    originalSend.call(this, body);
  };

  proto.open = wrappedOpen;

  proto.send = wrappedSend;

  setTimeout(() => {
    if (!foreignStopped) {
      foreignSession = `foreign-${hex(4)}`;
    }
  }, options.sessionDelayMs ?? 0);

  foreignRestore = () => {
    foreignStopped = true;

    foreignRestore = null;

    if (options.global) {
      Reflect.deleteProperty(globalThis, options.global);
    }

    if (globalThis.fetch === wrappedFetch) {
      globalThis.fetch = originalFetch;
    }

    if (proto.open === wrappedOpen) {
      proto.open = originalOpen;
    }

    if (proto.send === wrappedSend) {
      proto.send = originalSend;
    }
  };
}

/**
 * A Datadog-shaped foreign tracer double: it wraps fetch and XMLHttpRequest,
 * writes its own W3C/vendor/B3 headers once its asynchronous session starts,
 * and records the requests it saw. No vendor package is involved.
 */
export const foreignWriter = {
  importUninitialized: () => {
    Object.assign(globalThis, {
      DD_RUM: { getInitConfiguration: () => undefined },
    });
  },
  events: () => foreignEvents,
  hasSession: () => foreignSession !== null,
  install: installForeign,
  stop: () => {
    foreignRestore?.();
  },
  stopSession: () => {
    foreignSession = null;
  },
};
