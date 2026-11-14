const LISTENER_TAG = Symbol.for("interfere.server-core.process-listener");

type ProcessEvent =
  | "beforeExit"
  | "SIGINT"
  | "SIGTERM"
  | "uncaughtException"
  | "unhandledRejection";

type ProcessListener = (...args: never) => void;

export function isInterfereListener(listener: ProcessListener) {
  return Object.hasOwn(listener, LISTENER_TAG);
}

export function tagListener<T extends ProcessListener>(listener: T) {
  Object.defineProperty(listener, LISTENER_TAG, { value: true });

  return listener;
}

export function removeInterfereListeners(event: ProcessEvent) {
  for (const listener of process.listeners(event)) {
    if (isInterfereListener(listener)) {
      process.removeListener(event, listener);
    }
  }
}
