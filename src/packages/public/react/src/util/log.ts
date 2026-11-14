import { runUnobserved } from "../internal/console-patch/unobserved.js";

export type LogLevel = "debug" | "info" | "warn" | "error" | "none";

const PRIORITY: Record<LogLevel, number> = {
  debug: 0,
  error: 3,
  info: 1,
  none: 4,
  warn: 2,
};

const CONSOLE_FN: Record<
  Exclude<LogLevel, "none">,
  "debug" | "info" | "warn" | "error"
> = {
  debug: "debug",
  error: "error",
  info: "info",
  warn: "warn",
};

let currentLevel: LogLevel = "warn";

export function setLogLevel(level: LogLevel) {
  currentLevel = level;
}

export function getLogLevel() {
  return currentLevel;
}

export interface Logger {
  debug: (...args: unknown[]) => void;
  error: (...args: unknown[]) => void;
  info: (...args: unknown[]) => void;
  warn: (...args: unknown[]) => void;
}

function emit(
  level: Exclude<LogLevel, "none">,
  prefix: string,
  args: unknown[],
) {
  if (PRIORITY[level] < PRIORITY[currentLevel]) {
    return;
  }

  const fn = CONSOLE_FN[level];

  runUnobserved(() => {
    globalThis.console[fn](prefix, ...args);
  });
}

export function createLogger(scope: string) {
  const prefix = `[Interfere:${scope}]`;

  return {
    debug: (...args) => {
      emit("debug", prefix, args);
    },
    error: (...args) => {
      emit("error", prefix, args);
    },
    info: (...args) => {
      emit("info", prefix, args);
    },
    warn: (...args) => {
      emit("warn", prefix, args);
    },
  } satisfies Logger;
}
