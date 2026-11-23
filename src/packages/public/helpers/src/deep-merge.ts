import type { DeepPartial } from "./deep-partial.js";

type IsPlainObject<T> = T extends readonly unknown[]
  ? false
  : // oxlint-disable-next-line anti-slop/no-unknown-returns -- This callable constraint excludes functions regardless of their return type.
    T extends (...args: never[]) => unknown
    ? false
    : T extends object
      ? true
      : false;

type IsArray<T> = T extends readonly unknown[] ? true : false;

type ConcatArrays<A, B> = A extends readonly (infer AItem)[]
  ? B extends readonly (infer BItem)[]
    ? (AItem | BItem)[]
    : A
  : B;

type Simplify<T> = T extends object
  ? { [K in keyof T]: T[K] } & NonNullable<unknown>
  : T;

/**
 * Merge two types following the runtime rules. Preserves keys that only exist
 * on one side, and narrows per-property based on presence in both.
 */
export type DeepMerge<A, B> = Simplify<
  A extends object ? (B extends object ? MergeObjects<A, B> : B) : B
>;

type IsOptionalKey<T, K extends keyof T> =
  object extends Pick<T, K> ? true : false;

type RequiredMergedKey<A, B, K extends keyof A | keyof B> = K extends keyof A
  ? IsOptionalKey<A, K> extends false
    ? K
    : K extends keyof B
      ? IsOptionalKey<B, K> extends false
        ? K
        : never
      : never
  : K extends keyof B
    ? IsOptionalKey<B, K> extends false
      ? K
      : never
    : never;

type OptionalMergedKey<A, B, K extends keyof A | keyof B> =
  K extends RequiredMergedKey<A, B, K> ? never : K;

type MergeObjects<A, B> = {
  [K in keyof A | keyof B as RequiredMergedKey<A, B, K>]: MergeProperty<
    A,
    B,
    K
  >;
} & {
  [K in keyof A | keyof B as OptionalMergedKey<A, B, K>]?: MergeProperty<
    A,
    B,
    K
  >;
};

type StripUndefined<T> = Exclude<T, undefined>;

type ValueAt<T, K extends PropertyKey> = K extends keyof T ? T[K] : never;

type MergeProperty<A, B, K extends keyof A | keyof B> = K extends keyof A
  ? K extends keyof B
    ? MergeBoth<A[K], B[K]>
    : StripUndefined<A[K]>
  : K extends keyof B
    ? StripUndefined<ValueAt<B, K>>
    : never;

type MergeBoth<A, B> = undefined extends B
  ? StripUndefined<A> | MergeNonUndefined<StripUndefined<A>, StripUndefined<B>>
  : MergeNonUndefined<StripUndefined<A>, StripUndefined<B>>;

type MergeNonUndefined<A, B> =
  IsPlainObject<A> extends true
    ? IsPlainObject<B> extends true
      ? DeepMerge<A, B>
      : B
    : IsArray<A> extends true
      ? IsArray<B> extends true
        ? ConcatArrays<A, B>
        : B
      : B;

/**
 * Left-fold `DeepMerge` over a tuple so the rightmost override wins (matching
 * `reduce((acc, next) => merge(acc, next))`). `undefined` members are skipped
 * because the runtime ignores `undefined` overrides.
 */
export type DeepMergeAll<
  Base,
  Overrides extends readonly unknown[],
> = Overrides extends readonly [infer Head, ...infer Tail]
  ? Head extends undefined
    ? DeepMergeAll<Base, Tail>
    : DeepMergeAll<DeepMerge<Base, Head>, Tail>
  : Base;

// oxlint-disable-next-line anti-slop/no-unsafe-dictionary-type -- Plain objects may contain heterogeneous recursive merge inputs, including functions and class instances.
function isPlainObject(value: unknown): value is Record<string, unknown> {
  return (
    typeof value === "object" &&
    value !== null &&
    !Array.isArray(value) &&
    Object.getPrototypeOf(value) === Object.prototype
  );
}

function cloneValue<T>(value: T) {
  // oxlint-disable-next-line typescript/no-unsafe-type-assertion
  return cloneUnknown(value) as T;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Recursive cloning preserves arbitrary leaf values and only traverses arrays and plain objects.
function cloneUnknown(value: unknown) {
  if (Array.isArray(value)) {
    const result: unknown = value.map((element) => cloneUnknown(element));

    return result;
  }

  if (isPlainObject(value)) {
    const returnValue2: unknown = Object.fromEntries(
      Object.entries(value).map(([key, nested]) => [key, cloneUnknown(nested)]),
    );

    return returnValue2;
  }

  const returnValue4: unknown = value;

  return returnValue4;
}

/** Merges values recursively, concatenating arrays and skipping undefined overrides. */
// oxlint-disable-next-line landmines/no-explicit-return-types
export function mergeValues<A, B>(base: A, override: B): DeepMerge<A, B> {
  // oxlint-disable-next-line typescript/no-unsafe-type-assertion
  return mergeUnknown(base, override) as DeepMerge<A, B>;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Merge inputs can have unrelated types; the public conditional return type tracks the selected branch.
function mergeUnknown(base: unknown, override: unknown) {
  if (override === undefined) {
    return cloneValue(base);
  }

  if (Array.isArray(base) || Array.isArray(override)) {
    const baseArr: unknown[] = Array.isArray(base) ? base : [];
    const overrideArr: unknown[] = Array.isArray(override) ? override : [];

    return [...baseArr, ...overrideArr].map((element) => cloneValue(element));
  }

  if (isPlainObject(base) && isPlainObject(override)) {
    const keys = new Set<string>([
      ...Object.keys(base),
      ...Object.keys(override),
    ]);

    const entries: [string, unknown][] = [];

    for (const key of keys) {
      entries.push([key, mergeUnknown(base[key], override[key])]);
    }

    return Object.fromEntries(entries);
  }

  return cloneValue(override);
}

export function deepMerge<T>(base: T): T;

/**
 * Pass `T` explicitly to treat every override as a `DeepPartial<T>`. This
 * gives you contextual typing inside the override literals — hover on a key
 * resolves back to `T`'s JSDoc — but overrides may not introduce keys that
 * don't exist on `T`.
 */
export function deepMerge<T extends object>(
  base: T,
  ...overrides: (DeepPartial<T> | undefined)[]
): T;

/**
 * Inferred, fully variadic merge. The return type is the exact merged shape
 * across every override, and overrides may introduce new keys.
 */
export function deepMerge<
  T,
  const Overrides extends readonly (object | undefined)[],
>(base: T, ...overrides: Overrides): DeepMergeAll<T, Overrides>;

export function deepMerge(
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- This overload implementation accepts every base type admitted by the public generic signatures.
  base: unknown,
  ...overrides: readonly (object | undefined)[]
) {
  if (overrides.length === 0) {
    return cloneValue(base);
  }

  const [first, ...rest] = overrides;

  if (first === undefined) {
    return deepMerge(base, ...rest);
  }

  return deepMerge(mergeValues(base, first), ...rest);
}
