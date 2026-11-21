import { describeDomEvent } from "./errors/dom-event.ts";
import { recoverError } from "./errors/recover.ts";
import {
  classifyNonError,
  describeNonError,
  serializeNonError,
} from "./errors/serialize.ts";
import {
  isStackFrameLine,
  truncateExceptionMessage,
  truncateExceptionStack,
} from "./errors/truncate.ts";
import type { IngestedFrame } from "#data/frame";
import type {
  ErrorMechanism,
  ExceptionValue,
} from "#sdk/plugins/payload/errors";

/**
 * Structured taxonomy of capture sources, tagged onto each
 * `ErrorMechanism.type`. Follows the `auto.<surface>.<library>.<method>`
 * convention Sentry also uses, so downstream analytics / classification can
 * group on common prefixes:
 *
 *   `auto.browser.*`   — captured by SDK global handlers in the browser
 *   `auto.function.*`  — captured by SDK functions (boundaries, callbacks,
 *                        server wrappers)
 *
 * Leaves (`.onerror`, `.error_boundary`, `.on_request_error`, …) identify
 * the specific capture site, which lets Interfere's skill classifier reason
 * about what "kind" of event this is without string-matching loose names.
 */
export const MECHANISM_TYPE = {
  browser: {
    consoleError: "auto.browser.console.error",
    fetchFailedResponse: "auto.browser.fetch.failed_response",
    fetchNetworkError: "auto.browser.fetch.network_error",
    onerror: "auto.browser.global_handlers.onerror",
    onunhandledrejection: "auto.browser.global_handlers.onunhandledrejection",
    resourceError: "auto.browser.resource.error",
    securityPolicyViolation: "auto.browser.security_policy.violation",
    xhrFailedResponse: "auto.browser.xhr.failed_response",
    xhrNetworkError: "auto.browser.xhr.network_error",
  },
  manual: {
    /** Customer called `capture(error)` from `@interfere/react/api`. */
    capture: "manual.api.capture",
  },
  nextjs: {
    after: "auto.function.nextjs.after",
    captureError: "auto.function.nextjs.capture_error",
    onRequestError: "auto.function.nextjs.on_request_error",
    proxy: "auto.function.nextjs.proxy",
    routeHandler: "auto.function.nextjs.route_handler",
    serverAction: "auto.function.nextjs.server_action",
  },
  node: {
    captureError: "auto.function.node.capture_error",
    uncaughtException: "auto.node.global_handlers.uncaught_exception",
    unhandledRejection: "auto.node.global_handlers.unhandled_rejection",
  },
  react: {
    captureBoundary: "auto.function.react.capture_boundary",
    caughtError: "auto.function.react.caught_error",
    errorBoundary: "auto.function.react.error_boundary",
    recoverableError: "auto.function.react.recoverable_error",
    uncaughtError: "auto.function.react.uncaught_error",
  },
} as const;

const EXTENSION_URL_RE =
  /\b(?:chrome|moz|safari-web|safari|ms-browser|webkit-masked)-(?:extension|url):\/\//u;

const FRAME_LOCATION_RE = /:\d+(?::\d+)?\)?\s*$/u;

function topLocatedFrame(stack: string) {
  return stack
    .split("\n")
    .find((line) => isStackFrameLine(line) && FRAME_LOCATION_RE.test(line));
}

/**
 * True when the top located frame of any supplied stack (head plus cause
 * chain) is a browser-extension or Safari masked URL. Frames without a
 * source location (`<anonymous>`, `native`) are skipped; extension frames
 * below an app frame keep the error, matching Sentry's top-frame semantics.
 */
export function shouldDropBrowserExtensionNoise(stacks: readonly string[]) {
  return stacks.some((stack) => {
    const frame = topLocatedFrame(stack);

    return frame !== undefined && EXTENSION_URL_RE.test(frame);
  });
}

const HAS_SOURCE_LOCATION_RE =
  /(?:https?|file|blob|webpack-internal):\/\/|\sat\s+[\w$.<>]+\s+\([^)]+:\d/u;

/**
 * True when **none** of the supplied stacks (head + cause chain entries)
 * carries any source-location anchor. Non-Error rejections (no stack at
 * all) are exempt at the call site — they ship structured payloads the
 * agent can reason over without frames.
 */
export function shouldDropUnresolvableStack(stacks: readonly string[]) {
  if (stacks.length === 0) {
    return false;
  }

  for (const stack of stacks) {
    if (HAS_SOURCE_LOCATION_RE.test(stack)) {
      return false;
    }
  }

  return true;
}

const MAX_CAUSE_DEPTH = 5;

const V8_COMPONENT_FRAME_RE =
  /^(?:in|at)\s+(?:(?<name>[^\s(]+)\s+\((?<location>[^()]*)\)|(?<bare>\S+))/u;

const GECKO_COMPONENT_FRAME_RE = /^(?<name>[^@\s]*)@(?<location>\S+)$/u;

const FRAME_LOCATION_PARTS_RE =
  /^(?<file>.+?):(?<line>\d+)(?::(?<column>\d+))?$/u;

function componentFrame(line: string) {
  const groups = (
    V8_COMPONENT_FRAME_RE.exec(line) ?? GECKO_COMPONENT_FRAME_RE.exec(line)
  )?.groups;

  if (!groups) {
    return null;
  }

  const { bare, location, name } = groups;
  const parts = FRAME_LOCATION_PARTS_RE.exec(location ?? bare ?? "")?.groups;
  const located = parts?.["file"] && Number(parts["line"]) > 0 ? parts : null;
  const fn = name || (located ? undefined : bare);

  if (!fn && !located) {
    return null;
  }

  const frame: IngestedFrame = {};

  if (located) {
    frame.file = located["file"];
    frame.line = Number(located["line"]);

    if (located["column"]) {
      frame.column = Number(located["column"]);
    }
  }

  if (fn) {
    frame.fn = fn;
  }

  return frame;
}

/**
 * Parses a React `errorInfo.componentStack` string into synthetic frames.
 * Server-side stack parsing handles JS frames; component-stack frames are
 * supplementary context React emits separately and have a different shape,
 * so they're parsed at capture time and shipped via `appendFrames`.
 *
 * React produces different formats across versions:
 *   - React 16–18 legacy:    `    in ComponentName`
 *                            `    in ComponentName (created by Parent)`
 *   - React 19, V8:          `    at ComponentName (https://host/app.js:10:5)`
 *                            `    at https://host/app.js:10:5`
 *                            `    at Suspense (<anonymous>)`
 *   - React 19, Gecko/JSC:   `ComponentName@https://host/app.js:10:5`
 *                            `@https://host/app.js:10:5`
 *                            `Suspense@unknown:0:0`
 */
export function parseReactComponentStack(componentStack: string) {
  const frames: IngestedFrame[] = [];

  for (const rawLine of componentStack.split("\n")) {
    const frame = componentFrame(rawLine.trim());

    if (frame) {
      frames.push({ ...frame, id: `react-${frames.length}` });
    }
  }

  return frames;
}

/** Returns an Error, preferring an existing or nested Error over a stringified fallback. */
// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Public error capture accepts every JavaScript thrown value before normalization.
export function toError(error: unknown) {
  try {
    const recovered = recoverError(error);

    if (recovered) {
      return recovered;
    }

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Thrown strings retain their original message instead of JSON quoting.
    if (typeof error === "string") {
      return new Error(error) satisfies Error;
    }

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Only thrown objects can contain a recoverable nested Error.
    if (error !== null && typeof error === "object") {
      const nested = tryUnwrapNestedError(error);

      if (nested) {
        return nested;
      }

      return tagAsNonError(
        safeJson(error) ?? Object.prototype.toString.call(error),
      );
    }

    return tagAsNonError(String(error));
  } catch {
    return tagAsNonError("Inaccessible thrown value");
  }
}

// oxlint-disable-next-line anti-slop/no-object-parameters -- Any thrown object can carry a nested Error under arbitrary keys.
function tryUnwrapNestedError(record: object) {
  const descriptors = Object.getOwnPropertyDescriptors(record);
  const cause = recoverError(descriptors["cause"]?.value);

  if (cause) {
    return cause;
  }

  for (const descriptor of Object.values(descriptors)) {
    const recovered = recoverError(descriptor.value);

    if (recovered) {
      return recovered;
    }
  }

  return;
}

function tagAsNonError(message: string) {
  const e = new Error(message);

  e.name = "NonError";

  return e;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- This serializer must attempt JSON encoding of arbitrary thrown values.
function safeJson(value: unknown) {
  try {
    return JSON.stringify(value);
  } catch {
    return null;
  }
}

/**
 * Capture-side variant for thrown values that aren't Errors and don't
 * carry a nested Error (e.g. `Promise.reject({ code: 401, body: "..." })`,
 * `Promise.reject(42)`). The structured payload rides on `serialized`
 * so downstream consumers can render the original shape instead of
 * parsing JSON back out of an Error's message string.
 *
 * Strings are *not* represented here — `Promise.reject("boom")` maps
 * cleanly to `new Error("boom")` without lying about the shape, so it
 * goes through `toException`'s Error branch.
 */
export interface NonErrorException {
  readonly kind: "non-error";
  /**
   * Original payload, bounded by depth, breadth and length and circular-safe.
   * Values past the depth limit become `"[depth-limit]"`, circular back-edges
   * `"[circular]"`, and entries past the breadth or length budget
   * `"[truncated]"`.
   */
  readonly serialized: unknown;
  /**
   * Stable label for grouping. `"PlainObject"` / `"Array"` / `"number"`
   * etc. — derived from the value's structural class, not its content,
   * so two `Promise.reject({ code: 401 })` calls land in the same group
   * regardless of which keys the object carries.
   */
  readonly type: string;
  /**
   * Best-effort one-liner. For objects: a sorted list of keys. For
   * primitives: `String(value)`. The dashboard shows this; the
   * `serialized` field carries the actual payload.
   */
  readonly value: string;
}

/** Preserves recoverable Errors and serializes other thrown values as structured exceptions. */
// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Public exception capture normalizes every JavaScript thrown value.
export function toException(value: unknown) {
  try {
    const recovered = recoverError(value);

    if (recovered) {
      return recovered;
    }

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Thrown strings become Error messages without losing their text.
    if (typeof value === "string") {
      return new Error(value) satisfies Error | NonErrorException;
    }

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Only thrown objects can carry a nested recoverable Error.
    if (value !== null && typeof value === "object") {
      const nested = tryUnwrapNestedError(value);

      if (nested) {
        return nested;
      }

      const event = describeDomEvent(value);

      if (event) {
        return event;
      }
    }

    return {
      kind: "non-error",
      serialized: serializeNonError(value),
      type: classifyNonError(value),
      value: describeNonError(value),
    } satisfies Error | NonErrorException;
  } catch {
    return {
      kind: "non-error",
      serialized: "[inaccessible]",
      type: "unknown",
      value: "Non-Error captured: inaccessible value",
    } satisfies NonErrorException;
  }
}

export function isNonErrorException(
  value: unknown,
): value is NonErrorException {
  return (
    typeof value === "object" &&
    value !== null &&
    "kind" in value &&
    value.kind === "non-error"
  );
}

/** Flattens a bounded Error graph (each AggregateError.errors entry, then cause) with bounded messages and stacks. */
export function toExceptions(
  value: Error | NonErrorException,
  mechanism: ErrorMechanism,
) {
  // oxlint-disable-next-line anti-slop/no-known-value-widening -- The public exception guard inspects this union without widening the stored value.
  if (isNonErrorException(value)) {
    return [
      {
        kind: "non-error",
        mechanism,
        serialized: value.serialized,
        type: value.type,
        value: value.value,
      },
    ] satisfies ExceptionValue[];
  }

  const exceptions: ExceptionValue[] = [];
  const seen = new Set<Error>();

  const visit = (error: Error | undefined) => {
    if (!error || seen.has(error) || exceptions.length >= MAX_CAUSE_DEPTH) {
      return;
    }

    seen.add(error);

    exceptions.push({
      kind: "error",
      mechanism: exceptions.length === 0 ? mechanism : undefined,
      stack: truncateExceptionStack(error.stack ?? "", error.message),
      type: error.name,
      value: truncateExceptionMessage(error.message),
    });

    const aggregated: unknown = Object.getOwnPropertyDescriptor(
      error,
      "errors",
    )?.value;

    if (Array.isArray(aggregated)) {
      for (const child of aggregated) {
        if (exceptions.length >= MAX_CAUSE_DEPTH) {
          return;
        }

        visit(recoverError(child));
      }
    }

    visit(recoverError(Object.getOwnPropertyDescriptor(error, "cause")?.value));
  };

  visit(value);

  return exceptions;
}
