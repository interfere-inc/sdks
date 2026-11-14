/**
 * Returns a new array with `null` and `undefined` removed, narrowing the
 * element type to `NonNullable<T>`. Use instead of `.filter(Boolean)`, which
 * leaves the element type nullable (and also drops other falsy values like
 * `0` / `""`).
 *
 * @example
 * ```ts
 * const ids = compact([1, null, 2, undefined]); // number[] → [1, 2]
 *
 * // Merge an optional value with a default, no leading separator:
 * const selector = compact([customSelector, ".default"]).join(", ");
 * ```
 */
export function compact<T>(items: readonly T[]) {
  return items.filter(
    (item): item is NonNullable<T> => item !== null && item !== undefined,
  );
}
