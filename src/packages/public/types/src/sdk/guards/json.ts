/** Whether a value is a JSON object rather than null, an array or a primitive, matching what zod's object schemas accept. */
export function isPlainRecord(
  value: unknown,
  // oxlint-disable-next-line anti-slop/no-unsafe-dictionary-type -- This guard narrows only the container; callers validate each field they read.
): value is Readonly<Partial<Record<string, unknown>>> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

/** Whether a value is a string. */
export function isString(value: unknown): value is string {
  return typeof value === "string";
}
