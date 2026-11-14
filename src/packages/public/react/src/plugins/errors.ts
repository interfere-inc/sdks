/* oxlint-disable import/no-default-export */
import type { IngestedFrame } from "@interfere/types/data/frame";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import {
  MECHANISM_TYPE,
  isNonErrorException,
} from "@interfere/types/sdk/errors";
import type { NonErrorException } from "@interfere/types/sdk/errors";
import type { ErrorMechanism } from "@interfere/types/sdk/plugins/payload/errors";

import { hasEventListenerPair } from "../internal/event-capabilities/event-capabilities.js";
import { onConsoleCall } from "../internal/console-patch/console-patch.js";
import { hasServerDigest } from "../internal/server-digest/server-digest.js";
import { THIRD_PARTY_IGNORE } from "../internal/third-party-hosts.js";
import { createRepeatGate, exceptionKey } from "./lib/repeat-gate.js";
import type { Plugin, PluginContext } from "./lib/types.js";
import {
  normalizeThrown,
  siteFallbackFrame,
  siteKey,
  uncaughtException,
} from "./lib/uncaught.js";
import type { UncaughtSite } from "./lib/uncaught.js";

const STACK_TRACE_LIMIT = 50;

const MAX_CSP_REPORTS = 5;

interface CaptureOpts {
  readonly error: Error | NonErrorException;
  readonly fallbackFrame?: IngestedFrame;
  readonly mechanism: ErrorMechanism;
}

function capture(ctx: PluginContext, opts: CaptureOpts) {
  ctx.recordException(
    opts.error,
    omitUndefined({
      mechanism: opts.mechanism,
      fallbackFrames:
        opts.fallbackFrame && !isNonErrorException(opts.error)
          ? [opts.fallbackFrame]
          : undefined,
    }),
  );
}

interface UncaughtMark {
  readonly error: unknown;
  readonly key: string;
}

function isSameMark(previous: UncaughtMark | null, next: UncaughtMark) {
  return (
    previous !== null &&
    Object.is(previous.error, next.error) &&
    previous.key === next.key
  );
}

function findErrorArg(args: readonly unknown[]) {
  return args.find((arg): arg is Error => arg instanceof Error) ?? null;
}

const RESOURCE_DEDUP_CAP = 100;

const RESOURCE_RATE_MAX = 20;

const RESOURCE_RATE_WINDOW_MS = 60_000;

interface ResourceError {
  readonly tag: string;
  readonly url: string;
}

function resourceError(target: EventTarget | null) {
  if (
    target instanceof HTMLImageElement ||
    target instanceof HTMLScriptElement ||
    target instanceof HTMLSourceElement
  ) {
    return (
      target.src ? { tag: target.localName, url: target.src } : null
    ) satisfies ResourceError | null;
  }

  if (target instanceof HTMLMediaElement) {
    const url = target.currentSrc || target.src;

    return (
      url ? { tag: target.localName, url } : null
    ) satisfies ResourceError | null;
  }

  if (target instanceof HTMLLinkElement) {
    return (
      target.href ? { tag: target.localName, url: target.href } : null
    ) satisfies ResourceError | null;
  }

  return null;
}

function isIgnoredResourceUrl(url: string) {
  return THIRD_PARTY_IGNORE.some((pattern) => pattern.test(url));
}

function createResourceGate() {
  const seen = new Set<string>();

  let windowStart = 0;
  let windowCount = 0;

  return ((url: string) => {
    if (seen.has(url)) {
      return false;
    }

    const now = Date.now();

    if (now - windowStart > RESOURCE_RATE_WINDOW_MS) {
      windowStart = now;

      windowCount = 0;
    }

    if (windowCount >= RESOURCE_RATE_MAX) {
      return false;
    }

    if (seen.size >= RESOURCE_DEDUP_CAP) {
      seen.clear();
    }

    seen.add(url);

    windowCount += 1;

    return true;
  }) satisfies (url: string) => boolean;
}

const errorsPlugin: Plugin = {
  name: "errors",

  setup(ctx) {
    const originalOnError = globalThis.onerror;
    const originalStackTraceLimit = Error.stackTraceLimit;

    if (Error.stackTraceLimit < STACK_TRACE_LIMIT) {
      Error.stackTraceLimit = STACK_TRACE_LIMIT;
    }

    const repeatGate = createRepeatGate();
    const rejectionGate = createRepeatGate();
    const consoleGate = createRepeatGate();

    let active = true;

    let listenerMark: UncaughtMark | null = null;
    let onErrorMark: UncaughtMark | null = null;

    // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Uncaught handlers receive any thrown JavaScript value.
    const captureUncaught = (site: UncaughtSite, error: unknown) => {
      try {
        if (!active || hasServerDigest(error)) {
          return;
        }

        const uncaught = uncaughtException(site, error);

        if (!uncaught || !repeatGate(siteKey(site))) {
          return;
        }

        capture(
          ctx,
          omitUndefined({
            error: uncaught.exception,
            fallbackFrame: siteFallbackFrame(site),
            mechanism: omitUndefined({
              handled: false,
              type: MECHANISM_TYPE.browser.onerror,
              synthetic: uncaught.synthetic ? true : undefined,
            }),
          }),
        );
      } catch {
        return;
      }
    };

    const onError: OnErrorEventHandlerNonNull = (
      msg,
      source,
      line,
      col,
      error,
    ) => {
      const site = { message: msg, source, line, column: col };
      const mark = { error, key: siteKey(site) };

      if (isSameMark(listenerMark, mark)) {
        listenerMark = null;
      } else {
        onErrorMark = mark;

        captureUncaught(site, error);
      }

      // oxlint-disable-next-line anti-slop/no-runtime-typeof -- The preexisting host error handler may be null or a non-callable JavaScript assignment.
      if (typeof originalOnError === "function") {
        const result: unknown = originalOnError.call(
          globalThis,
          msg,
          source,
          line,
          col,
          error,
        );

        return result;
      }

      return false;
    };

    // oxlint-disable-next-line unicorn/prefer-add-event-listener -- deliberately wraps and chains the previous onerror; addEventListener cannot suppress default handling via return value
    globalThis.onerror = onError;

    const onUnhandledRejection = (event: PromiseRejectionEvent) => {
      if (hasServerDigest(event.reason)) {
        return;
      }

      const rejection = normalizeThrown(event.reason);

      if (!rejectionGate(exceptionKey(rejection.exception))) {
        return;
      }

      capture(ctx, {
        error: rejection.exception,
        mechanism: omitUndefined({
          handled: false,
          type: MECHANISM_TYPE.browser.onunhandledrejection,
          synthetic: rejection.synthetic ? true : undefined,
        }),
      });
    };

    const cspReportsByDisposition = {
      enforce: new Set<string>(),
      report: new Set<string>(),
    };

    const onSecurityPolicyViolation = (event: SecurityPolicyViolationEvent) => {
      const seenCspReports = cspReportsByDisposition[event.disposition];
      const key = `${event.violatedDirective}|${event.blockedURI}`;

      if (seenCspReports.has(key) || seenCspReports.size >= MAX_CSP_REPORTS) {
        return;
      }

      seenCspReports.add(key);

      capture(ctx, {
        error: {
          kind: "non-error",
          serialized: {
            blockedURI: event.blockedURI,
            columnNumber: event.columnNumber,
            disposition: event.disposition,
            lineNumber: event.lineNumber,
            sourceFile: event.sourceFile,
            violatedDirective: event.violatedDirective,
          },
          type: "SecurityPolicyViolation",
          value:
            event.disposition === "report"
              ? `CSP ${event.violatedDirective} reported ${event.blockedURI || "inline"} (report-only)`
              : `CSP ${event.violatedDirective} blocked ${event.blockedURI || "inline"}`,
        },
        mechanism: {
          handled: event.disposition === "report",
          type: MECHANISM_TYPE.browser.securityPolicyViolation,
        },
      });
    };

    const resourceGate = createResourceGate();

    const onWindowError = (event: Event) => {
      if (event instanceof ErrorEvent) {
        const site = {
          message: event.message,
          source: event.filename,
          line: event.lineno,
          column: event.colno,
        };

        const thrown: unknown = event.error;
        const mark = { error: thrown, key: siteKey(site) };

        if (isSameMark(onErrorMark, mark)) {
          onErrorMark = null;
        } else {
          listenerMark = mark;

          captureUncaught(site, thrown);
        }

        return;
      }

      const resource = resourceError(event.target);

      if (
        !resource ||
        isIgnoredResourceUrl(resource.url) ||
        !resourceGate(resource.url)
      ) {
        return;
      }

      capture(ctx, {
        error: {
          kind: "non-error",
          serialized: { url: resource.url },
          type: "ResourceLoadError",
          value: `Failed to load ${resource.tag}: ${resource.url}`,
        },
        mechanism: {
          handled: false,
          type: MECHANISM_TYPE.browser.resourceError,
        },
      });
    };

    const supportsEventTarget = hasEventListenerPair(
      globalThis.addEventListener,
      globalThis.removeEventListener,
    );

    if (supportsEventTarget) {
      globalThis.addEventListener("unhandledrejection", onUnhandledRejection);

      globalThis.addEventListener(
        "securitypolicyviolation",
        onSecurityPolicyViolation,
      );

      globalThis.addEventListener("error", onWindowError, true);
    }

    const unsubscribeConsole = onConsoleCall((level, args) => {
      if (level !== "error") {
        return;
      }

      const error = findErrorArg(args);

      if (
        error &&
        error.name !== "AbortError" &&
        !hasServerDigest(error) &&
        consoleGate(exceptionKey(error))
      ) {
        capture(ctx, {
          error,
          mechanism: {
            handled: true,
            type: MECHANISM_TYPE.browser.consoleError,
          },
        });
      }
    });

    return () => {
      active = false;

      Error.stackTraceLimit = originalStackTraceLimit;

      if (globalThis.onerror === onError) {
        // oxlint-disable-next-line unicorn/prefer-add-event-listener -- restores the saved onerror handler only while ours is still installed; a later handler keeps chaining through the inert one
        globalThis.onerror = originalOnError;
      }

      if (supportsEventTarget) {
        globalThis.removeEventListener(
          "unhandledrejection",
          onUnhandledRejection,
        );

        globalThis.removeEventListener(
          "securitypolicyviolation",
          onSecurityPolicyViolation,
        );

        globalThis.removeEventListener("error", onWindowError, true);
      }

      unsubscribeConsole();
    };
  },
};

export default errorsPlugin;
