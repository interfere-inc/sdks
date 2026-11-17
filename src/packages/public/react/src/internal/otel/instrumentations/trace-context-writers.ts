import { propagation } from "@opentelemetry/api";
import { isPlainRecord, isString } from "@interfere/types/sdk/guards/json";

import { getGlobal } from "../../../util/global.js";

type UrlMatcher = string | RegExp | ((url: string) => boolean);

interface TracingUrlOption {
  match: UrlMatcher;
  propagatorTypes?: string[];
}

function isGetter(value: unknown): value is () => object | undefined {
  return typeof value === "function";
}

function isUrlMatcher(value: unknown): value is UrlMatcher {
  return (
    isString(value) || value instanceof RegExp || typeof value === "function"
  );
}

function isTracingUrlOption(value: unknown): value is TracingUrlOption {
  if (!isPlainRecord(value) || !isUrlMatcher(value["match"])) {
    return false;
  }

  const types = value["propagatorTypes"];

  return (
    types === undefined ||
    (Array.isArray(types) && types.every((type) => isString(type)))
  );
}

function matches(matcher: UrlMatcher, url: string, prefix: boolean) {
  if (matcher instanceof RegExp) {
    return matcher.test(url);
  }

  if (isString(matcher)) {
    return prefix ? url.startsWith(matcher) : url.includes(matcher);
  }

  try {
    return matcher(url);
  } catch {
    return false;
  }
}

function datadogWrites(url: string) {
  const rum = getGlobal("DD_RUM");

  const config =
    isPlainRecord(rum) && isGetter(rum["getInitConfiguration"])
      ? rum["getInitConfiguration"]()
      : undefined;

  const urls = isPlainRecord(config) ? config["allowedTracingUrls"] : undefined;

  if (!Array.isArray(urls)) {
    return false;
  }

  const options: TracingUrlOption[] = [];

  for (const entry of urls) {
    if (isTracingUrlOption(entry)) {
      options.push(entry);
    } else if (isUrlMatcher(entry)) {
      options.push({ match: entry });
    } else {
      return false;
    }
  }

  return options.some(
    (option) =>
      (option.propagatorTypes?.includes("tracecontext") ?? true) &&
      matches(option.match, url, true),
  );
}

function sentryWrites(url: string) {
  const carrier = getGlobal("__SENTRY__");

  const hub =
    isPlainRecord(carrier) && isString(carrier["version"])
      ? carrier[carrier["version"]]
      : undefined;

  const scope = isPlainRecord(hub) ? hub["defaultCurrentScope"] : undefined;

  const client =
    isPlainRecord(scope) && isGetter(scope["getClient"])
      ? scope["getClient"]()
      : undefined;

  const options =
    isPlainRecord(client) && isGetter(client["getOptions"])
      ? client["getOptions"]()
      : undefined;

  if (!isPlainRecord(options) || options["propagateTraceparent"] !== true) {
    return false;
  }

  const targets = options["tracePropagationTargets"];

  if (targets === undefined) {
    return new URL(url).origin === globalThis.location.origin;
  }

  return (
    Array.isArray(targets) &&
    targets.every((target) => isUrlMatcher(target)) &&
    targets.some((target: UrlMatcher) => matches(target, url, false))
  );
}

/**
 * The tool that will write `traceparent` on a request to `url`, read from
 * that tool's own configuration when the request is sent, so a tool
 * initialised after Interfere is found from its first request. Interfere
 * never registers a global propagator, so any registered one is another
 * tool's. Datadog RUM
 * appends its header unconditionally, so writing ours first would send two.
 */
export function traceContextWriterFor(url: string) {
  try {
    if (datadogWrites(url)) {
      return "datadog";
    }

    if (sentryWrites(url)) {
      return "sentry";
    }
  } catch {
    return;
  }

  if (propagation.fields().includes("traceparent")) {
    return "opentelemetry";
  }

  return;
}
