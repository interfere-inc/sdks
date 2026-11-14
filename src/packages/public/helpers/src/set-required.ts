/**
 * Make the keys `K` of `T` required, leaving every other key (and each key's
 * type) untouched. Handy for options types where a function guarantees certain
 * optional fields are always present in its output.
 *
 * @example
 * ```ts
 * type Options = { a?: string; b?: number };
 * type WithA = SetRequired<Options, "a">;      // { a: string; b?: number }
 * type WithBoth = SetRequired<Options, "a" | "b">; // { a: string; b: number }
 * ```
 */
export type SetRequired<T, K extends keyof T> = Omit<T, K> &
  Required<Pick<T, K>>;
