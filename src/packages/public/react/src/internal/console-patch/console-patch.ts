import { isUnobserved, runUnobserved } from "./unobserved.js";

/**
 * Single point of `console.{debug,log,info,warn,error}` patching. The
 * errors plugin and the logs plugin both want to observe console calls;
 * before this consolidation, each patched independently and the
 * resulting wrap-order coupling made teardown leaky.
 *
 * Now: a single set of patched methods is installed on first
 * subscription, fans out to every subscriber, and is uninstalled on the
 * last unsubscription. Handlers never wrap each other; they observe.
 * Uninstalling restores a method only while the installed function is
 * still ours; a wrapper another SDK installed on top stays in place and
 * our method beneath it becomes a plain pass-through.
 */
export const CONSOLE_LEVELS = [
  "debug",
  "log",
  "info",
  "warn",
  "error",
] as const;

export type ConsoleLevel = (typeof CONSOLE_LEVELS)[number];

export type ConsoleHandler = (level: ConsoleLevel, args: unknown[]) => void;

type ConsoleMethod = (...args: unknown[]) => void;

interface Installation {
  active: boolean;
  pinned: boolean;
  patched: Map<
    ConsoleLevel,
    { original: ConsoleMethod; wrapper: ConsoleMethod }
  >;
}

const handlers = new Set<ConsoleHandler>();

function dispatch(level: ConsoleLevel, args: unknown[]) {
  if (isUnobserved()) {
    return;
  }

  runUnobserved(() => {
    for (const handler of handlers) {
      try {
        handler(level, args);
      } catch {
        continue;
      }
    }
  });
}

let installation: Installation | null = null;

function install() {
  if (installation) {
    return;
  }

  const current: Installation = {
    active: true,
    patched: new Map(),
    pinned: false,
  };

  installation = current;

  const target = globalThis.console;

  for (const level of CONSOLE_LEVELS) {
    const original = target[level];

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Embedded browser hosts and customer overrides may omit or replace individual console methods.
    if (typeof original !== "function") {
      continue;
    }

    const wrapper: ConsoleMethod = (...args) => {
      original.apply(globalThis.console, args);

      if (current.active) {
        dispatch(level, args);
      }
    };

    current.patched.set(level, { original, wrapper });

    target[level] = wrapper;
  }
}

function uninstall(force = false) {
  if (!installation || (installation.pinned && !force)) {
    return;
  }

  const current = installation;

  installation = null;

  current.active = false;

  const target = globalThis.console;

  for (const [level, { original, wrapper }] of current.patched) {
    if (target[level] === wrapper) {
      target[level] = original;
    }
  }
}

/** Installs the console wrappers now and keeps them installed, so code that saves a console method before the SDK starts still reaches later subscribers. */
export function pinConsolePatch() {
  install();

  if (installation) {
    installation.pinned = true;
  }
}

export function onConsoleCall(handler: ConsoleHandler) {
  install();

  handlers.add(handler);

  return (() => {
    handlers.delete(handler);

    if (handlers.size === 0) {
      uninstall();
    }
  }) satisfies () => void;
}

/** Test hook — drops every handler and restores originals. */
export function resetConsolePatchForTests() {
  handlers.clear();

  uninstall(true);
}
