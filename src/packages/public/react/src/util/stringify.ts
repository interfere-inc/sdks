/* oxlint-disable typescript/no-unnecessary-condition */
// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Console arguments can be any JavaScript value, including cyclic objects and values JSON cannot encode.
export function safeStringify(value: unknown) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Console strings must pass through unquoted while other values use JSON or string conversion.
  if (typeof value === "string") {
    return value;
  }

  try {
    return JSON.stringify(value) ?? String(value);
  } catch {
    return String(value);
  }
}
