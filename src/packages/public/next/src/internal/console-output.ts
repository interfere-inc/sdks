function isConsoleMethod(method: unknown): method is typeof console.log {
  return typeof method === "function";
}

export function writeConsoleLines(
  // oxlint-disable-next-line anti-slop/no-unknown-parameters
  method: unknown,
  lines: readonly string[],
) {
  if (!isConsoleMethod(method)) {
    return;
  }

  for (const line of lines) {
    method.call(globalThis.console, line);
  }
}
