/* oxlint-disable typescript/no-deprecated -- The browser SDK restores native request functions on disposal. browser-api 0.2 keeps its patches installed after disable(). */
import type { Span } from "@opentelemetry/api";
import type { FetchInstrumentation } from "@opentelemetry/instrumentation-fetch";
import type { XMLHttpRequestInstrumentation } from "@opentelemetry/instrumentation-xml-http-request";

type FetchFunction = typeof globalThis.fetch;

export type XhrOpen = (
  this: XMLHttpRequest,
  ...args: Parameters<XMLHttpRequest["open"]>
) => void;

export type XhrSend = (
  this: XMLHttpRequest,
  ...args: Parameters<XMLHttpRequest["send"]>
) => void;

type Instrumentation = FetchInstrumentation | XMLHttpRequestInstrumentation;

type PatchMethod<T> = (this: Instrumentation) => (original: T) => T;

function isPatch<T>(value: unknown): value is PatchMethod<T> {
  return typeof value === "function";
}

function isXhrMemory(
  value: unknown,
): value is WeakMap<XMLHttpRequest, { span?: Span }> {
  return value instanceof WeakMap;
}

// WARNING: These are private members of @opentelemetry/instrumentation-fetch and -xml-http-request which offer no fetch response hook and no XHR request hook; upstream-request-contract.browser.test.ts fails when they change.
function readMember<T>(
  instrumentation: Instrumentation,
  name: string,
  guard: (value: unknown) => value is T,
) {
  // oxlint-disable-next-line anti-slop/no-reflect-get -- The member is private in the upstream typings and composed at runtime.
  const member: unknown = Reflect.get(instrumentation, name);

  if (!guard(member)) {
    throw new TypeError(
      `Upstream instrumentation member ${name} changed shape`,
    );
  }

  return member;
}

function composePatch<T>(
  instrumentation: Instrumentation,
  name: string,
  factory: PatchMethod<T>,
  wrap: (patched: T) => T,
) {
  const composed: PatchMethod<T> = function composed(this: Instrumentation) {
    const patch = factory.call(this);

    return (original: T) => wrap(patch(original));
  };

  Reflect.set(instrumentation, name, composed);
}

/** Wraps the function upstream installs on `fetch`, so Interfere runs in the application's frame inside the single upstream patch. */
export function composeFetchPatch(
  instrumentation: FetchInstrumentation,
  wrap: (patched: FetchFunction) => FetchFunction,
) {
  composePatch(
    instrumentation,
    "_patchConstructor",
    readMember(instrumentation, "_patchConstructor", isPatch<FetchFunction>),
    wrap,
  );
}

/** Wraps the functions upstream installs on `XMLHttpRequest.prototype.open` and `.send`. */
export function composeXhrPatches(
  instrumentation: XMLHttpRequestInstrumentation,
  wrapOpen: (patched: XhrOpen) => XhrOpen,
  wrapSend: (patched: XhrSend) => XhrSend,
) {
  composePatch(
    instrumentation,
    "_patchOpen",
    readMember(instrumentation, "_patchOpen", isPatch<XhrOpen>),
    wrapOpen,
  );

  composePatch(
    instrumentation,
    "_patchSend",
    readMember(instrumentation, "_patchSend", isPatch<XhrSend>),
    wrapSend,
  );
}

/** Stops upstream fetch injection through the global propagator; Interfere's request hook writes its headers instead. */
export function skipFetchHeaderInjection(
  instrumentation: FetchInstrumentation,
) {
  Reflect.set(instrumentation, "_addHeaders", () => undefined);
}

/** Replaces upstream XHR injection through the global propagator with Interfere's writer, given the request's span. */
export function replaceXhrHeaderInjection(
  instrumentation: XMLHttpRequestInstrumentation,
  write: (xhr: XMLHttpRequest, url: string, span: Span) => void,
) {
  Reflect.set(
    instrumentation,
    "_addHeaders",
    (xhr: XMLHttpRequest, url: string) => {
      const span = readMember(instrumentation, "_xhrMem", isXhrMemory).get(
        xhr,
      )?.span;

      if (span) {
        write(xhr, url, span);
      }
    },
  );
}
