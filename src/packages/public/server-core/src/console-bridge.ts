/* oxlint-disable eslint/no-console */
import { omitUndefined } from "@interfere/helpers/omit-undefined";

import { SeverityNumber, context, logs, trace } from "@opentelemetry/api";
import type { LoggerProvider } from "@opentelemetry/api";

const ATTR_EXCEPTION_TYPE = "exception.type" as const;

const ATTR_EXCEPTION_MESSAGE = "exception.message" as const;

const ATTR_EXCEPTION_STACKTRACE = "exception.stacktrace" as const;

const CONSOLE_METHODS = ["debug", "log", "info", "warn", "error"] as const;

type ConsoleMethod = (typeof CONSOLE_METHODS)[number];

const SEVERITY: Record<ConsoleMethod, SeverityNumber> = {
  debug: SeverityNumber.DEBUG,
  error: SeverityNumber.ERROR,
  info: SeverityNumber.INFO,
  log: SeverityNumber.INFO,
  warn: SeverityNumber.WARN,
};

const BRIDGE_FAILURE_LOG_INTERVAL = 100;

const MAX_BODY_CHARS = 8192;

function buildBody(args: unknown[]) {
  const body = args.map((element) => stringify(element)).join(" ");

  return body.length > MAX_BODY_CHARS
    ? `${body.slice(0, MAX_BODY_CHARS)}…`
    : body;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Console arguments include arbitrary JavaScript values, including cycles and non-JSON primitives.
function stringify(value: unknown) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- String console arguments pass through without JSON quoting; other values use their serialization path.
  if (typeof value === "string") {
    return value;
  }

  if (value instanceof Error) {
    return value.stack ?? `${value.name}: ${value.message}`;
  }

  try {
    return JSON.stringify(value);
  } catch {
    return String(value);
  }
}

function extractException(args: unknown[]) {
  for (const arg of args) {
    if (arg instanceof Error) {
      return arg;
    }
  }

  return;
}

function buildExceptionAttributes(args: unknown[]) {
  const exception = extractException(args);

  if (!exception) {
    return;
  }

  return omitUndefined({
    [ATTR_EXCEPTION_TYPE]: exception.name,
    [ATTR_EXCEPTION_MESSAGE]: exception.message,
    [ATTR_EXCEPTION_STACKTRACE]: exception.stack || undefined,
  });
}

/**
 * Patches `console.{debug,log,info,warn,error}` to additionally emit OTel
 * `LogRecord`s alongside the original output. Every log record carries
 * the active span's context (so trace ↔ log correlation works in
 * dashboards) and any `Error` arg gets unpacked into semconv exception
 * attrs.
 *
 * Returns a disposer that restores the original `console.*` methods.
 *
 * Cycle protection: `emitting` guards against the bridge re-entering
 * itself if a logger backend itself logs to console (which would
 * recursively emit forever). Failures are bounded-rate logged via the
 * original `console.error` so a broken backend doesn't drown the
 * customer's logs in bridge-failure messages.
 *
 * Serialization is deferred to a microtask and bodies are capped at
 * {@link MAX_BODY_CHARS} characters, so the synchronous `console.*`
 * call only captures the active context; an arg mutated before the
 * microtask runs is serialized in its mutated state.
 */
export function bridgeConsoleToOtel(
  provider: LoggerProvider = logs,
  minimumSeverity: SeverityNumber = SeverityNumber.DEBUG,
  name = "@interfere/server/console",
) {
  const methods = CONSOLE_METHODS.filter(
    (method) => SEVERITY[method] >= minimumSeverity,
  );

  const logger = provider.getLogger(name);

  let emitting = false;
  let bridgeFailures = 0;

  const original: Record<ConsoleMethod, (...args: unknown[]) => void> = {
    debug: console.debug,
    error: console.error,
    info: console.info,
    log: console.log,
    warn: console.warn,
  };

  function emitLog(
    orig: (...args: unknown[]) => void,
    severity: SeverityNumber,
    method: ConsoleMethod,
    args: unknown[],
    activeContext: ReturnType<typeof context.active>,
    activeSpan: ReturnType<typeof trace.getSpan>,
  ) {
    if (emitting) {
      return;
    }

    emitting = true;

    try {
      const attributes = buildExceptionAttributes(args);

      logger.emit(
        omitUndefined({
          body: buildBody(args),
          severityNumber: severity,
          severityText: method.toUpperCase(),
          context: activeSpan ? activeContext : undefined,
          attributes,
        }),
      );
    } catch (error) {
      bridgeFailures += 1;

      if (bridgeFailures % BRIDGE_FAILURE_LOG_INTERVAL === 1) {
        orig.call(
          console,
          `[interfere] console→OTel bridge emission failed (${bridgeFailures} total):`,
          error,
        );
      }
    } finally {
      emitting = false;
    }
  }

  for (const method of methods) {
    const orig = original[method];
    const severity = SEVERITY[method];

    console[method] = (...args: unknown[]) => {
      orig.apply(console, args);

      const activeContext = context.active();
      const activeSpan = trace.getSpan(activeContext);

      queueMicrotask(() => {
        emitLog(orig, severity, method, args, activeContext, activeSpan);
      });
    };
  }

  return (() => {
    for (const method of methods) {
      console[method] = original[method];
    }
  }) satisfies () => void;
}
