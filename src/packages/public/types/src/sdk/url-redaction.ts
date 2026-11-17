import { isPlainRecord } from "./guards/json.ts";

/** Query parameters and fragments to redact in SDK telemetry. Parameter names match case-insensitively. */
export interface UrlRedactionOptions {
  /**
   * Parameters whose values are kept. Every other value becomes `REDACTED`,
   * including the built-in sensitive names unless they are listed here.
   */
  allow?: string[];
  /** Parameters whose values become `REDACTED` in addition to the built-in sensitive names. */
  deny?: string[];
  /** Remove fragments, including hash-router paths. Defaults to `false`. */
  fragment?: boolean;
}

/**
 * Omit to redact the values of built-in sensitive query parameters such as
 * `token`, `password` and `sig`. `true` redacts every value and removes
 * fragments. `false` keeps query values. URL credentials are always redacted.
 */
export type UrlRedactionOption = boolean | UrlRedactionOptions;

export interface UrlRedactionPolicy {
  readonly fragment: boolean;
  readonly redactsValue: (name: string) => boolean;
}

type AttributeScalar = boolean | number | string;

type SpanAttributes = Record<
  string,
  AttributeScalar | readonly (AttributeScalar | null | undefined)[] | undefined
>;

interface RedactableSpan {
  attributes: SpanAttributes;
  events: readonly { attributes?: SpanAttributes | undefined; name: string }[];
}

interface EndingSpan extends RedactableSpan {
  name: string;
  setStatus: (status: { code: number; message: string }) => void;
  status: { code: number; message?: string };
  updateName: (name: string) => void;
}

const URL_ATTRIBUTE_KEYS = [
  "http.referrer",
  "http.request.header.referer",
  "http.target",
  "http.url",
  "url.full",
  "url.original",
  "url.path",
];

const FRAME_ATTRIBUTE_KEYS = [
  "interfere.exception.appended_frames",
  "interfere.exception.fallback_frames",
];

const URL_PARTS_RE =
  /^(?<base>[^?#]*)(?:\?(?<query>[^#]*))?(?<fragment>#.*)?$/su;

const ABSOLUTE_URL_RE = /https?:\/\/[^\s"'<>]+/giu;

const REDACTED = "REDACTED";

const SENSITIVE_NAMES: ReadonlySet<string> = new Set([
  "awsaccesskeyid",
  "code",
  "key",
  "sig",
]);

const SENSITIVE_TERMS = [
  "api_key",
  "apikey",
  "auth",
  "credential",
  "passwd",
  "password",
  "secret",
  "session",
  "signature",
  "token",
];

function isSensitiveName(name: string) {
  return (
    SENSITIVE_NAMES.has(name) ||
    SENSITIVE_TERMS.some((term) => name.includes(term))
  );
}

export function resolveUrlRedaction(option: UrlRedactionOption = {}) {
  if (option === true || option === false) {
    return {
      fragment: option,
      redactsValue: () => option,
    } satisfies UrlRedactionPolicy;
  }

  const allow =
    option.allow && new Set(option.allow.map((name) => name.toLowerCase()));

  const deny = new Set(option.deny?.map((name) => name.toLowerCase()));

  return {
    fragment: option.fragment ?? false,
    redactsValue: (name) =>
      deny.has(name) || (allow ? !allow.has(name) : isSensitiveName(name)),
  } satisfies UrlRedactionPolicy;
}

function decodeName(name: string) {
  try {
    return decodeURIComponent(name).toLowerCase();
  } catch {
    return name.toLowerCase();
  }
}

function redactQuery(query: string, policy: UrlRedactionPolicy) {
  return query
    .split("&")
    .map((parameter) => {
      const separator = parameter.indexOf("=");
      const name = separator === -1 ? parameter : parameter.slice(0, separator);

      if (!(parameter && policy.redactsValue(decodeName(name)))) {
        return parameter;
      }

      return separator === -1 ? REDACTED : `${name}=${REDACTED}`;
    })
    .join("&");
}

function redactCredentials(base: string) {
  return base
    .replace(
      /^(?<scheme>[a-z][\d+.a-z-]*:\/\/)[^/@:]*:[^/@]*@/iu,
      `$<scheme>${REDACTED}:${REDACTED}@`,
    )
    .replace(
      /^(?<scheme>[a-z][\d+.a-z-]*:\/\/)[^/@:]*@/iu,
      `$<scheme>${REDACTED}@`,
    );
}

export function redactUrl(url: string, policy: UrlRedactionPolicy) {
  const {
    base = "",
    fragment = "",
    query,
  } = URL_PARTS_RE.exec(url)?.groups ?? {};

  return [
    redactCredentials(base),
    query === undefined ? "" : `?${redactQuery(query, policy)}`,
    policy.fragment ? "" : fragment,
  ].join("");
}

function redactMessage(message: string, policy: UrlRedactionPolicy) {
  return message.replaceAll(ABSOLUTE_URL_RE, (url) => redactUrl(url, policy));
}

function redactSpanName(name: string, policy: UrlRedactionPolicy) {
  return name
    .split(" ")
    .map((token) =>
      token.startsWith("/") || token.includes("://")
        ? redactUrl(token, policy)
        : token,
    )
    .join(" ");
}

interface ExceptionChainEntry {
  stack?: string | undefined;
  value?: string | undefined;
}

function isOptionalString(value: unknown): value is string | undefined {
  return value === undefined || typeof value === "string";
}

function isExceptionChain(value: unknown): value is ExceptionChainEntry[] {
  return (
    Array.isArray(value) &&
    value.every(
      (entry) =>
        isPlainRecord(entry) &&
        isOptionalString(entry["stack"]) &&
        isOptionalString(entry["value"]),
    )
  );
}

function redactExceptionChain(chain: string, policy: UrlRedactionPolicy) {
  try {
    const entries: unknown = JSON.parse(chain);

    if (!isExceptionChain(entries)) {
      return chain;
    }

    for (const entry of entries) {
      if (entry.value !== undefined) {
        const value = redactMessage(entry.value, policy);

        entry.stack = entry.stack?.replace(entry.value, value);
        entry.value = value;
      }
    }

    return JSON.stringify(entries);
  } catch {
    return chain;
  }
}

function redactFrames(frames: string, policy: UrlRedactionPolicy) {
  try {
    return JSON.stringify(
      // oxlint-disable-next-line anti-slop/no-unknown-parameters -- A JSON reviver receives every parsed value before any schema can run.
      JSON.parse(frames, (key, value: unknown) =>
        // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Frame JSON is untrusted; only string `file` values hold URLs.
        key === "file" && typeof value === "string"
          ? redactUrl(value, policy)
          : value,
      ),
    );
  } catch {
    return frames;
  }
}

function rewrite(
  attributes: SpanAttributes,
  key: string,
  redact: (value: string) => string,
) {
  const value = attributes[key];

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- OTel attribute values may be scalars or scalar arrays; only strings hold URLs.
  if (typeof value === "string") {
    attributes[key] = redact(value);
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Arrays are the only object attribute values.
  } else if (typeof value === "object") {
    attributes[key] = value.map((item) =>
      // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Array members may be any scalar; only strings hold URLs.
      typeof item === "string" ? redact(item) : item,
    );
  }
}

function redactAttributes(
  attributes: SpanAttributes,
  policy: UrlRedactionPolicy,
) {
  for (const key of URL_ATTRIBUTE_KEYS) {
    rewrite(attributes, key, (url) => redactUrl(url, policy));
  }

  rewrite(attributes, "url.query", (query) => redactQuery(query, policy));

  if (policy.fragment) {
    delete attributes["url.fragment"];
  }

  rewrite(attributes, "exception.message", (message) => {
    const redacted = redactMessage(message, policy);

    rewrite(attributes, "exception.stacktrace", (stack) =>
      stack.replace(message, redacted),
    );

    return redacted;
  });

  rewrite(attributes, "interfere.exception.chain", (chain) =>
    redactExceptionChain(chain, policy),
  );

  rewrite(attributes, "interfere.exception.serialized", (serialized) =>
    redactMessage(serialized, policy),
  );

  for (const key of FRAME_ATTRIBUTE_KEYS) {
    rewrite(attributes, key, (frames) => redactFrames(frames, policy));
  }
}

function redactSpan(span: RedactableSpan, policy: UrlRedactionPolicy) {
  redactAttributes(span.attributes, policy);

  for (const event of span.events) {
    if (event.attributes) {
      redactAttributes(event.attributes, policy);
    }
  }
}

/** Redacts URL attributes, URL-bearing span names and absolute URLs in exception messages in `onEnding`, before any processor exports the span. */
export function createUrlRedactionSpanProcessor(policy: UrlRedactionPolicy) {
  return {
    async forceFlush() {
      await Promise.resolve();
    },
    onEnd(span: RedactableSpan) {
      redactSpan(span, policy);
    },
    onEnding(span: EndingSpan) {
      span.updateName(redactSpanName(span.name, policy));

      if (span.status.message !== undefined) {
        span.setStatus({
          ...span.status,
          message: redactMessage(span.status.message, policy),
        });
      }

      redactSpan(span, policy);
    },
    onStart: () => undefined,
    async shutdown() {
      await Promise.resolve();
    },
  };
}
