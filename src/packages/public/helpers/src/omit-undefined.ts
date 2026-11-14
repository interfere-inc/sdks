export type OmitUndefined<T extends object> = {
  [K in keyof T as undefined extends T[K] ? never : K]: T[K];
} & {
  [K in keyof T as undefined extends T[K] ? K : never]?: Exclude<
    T[K],
    undefined
  >;
};

export function omitUndefined<T extends object>(value: T) {
  const entries: [string, unknown][] = Object.entries(value);

  // oxlint-disable-next-line typescript/no-unsafe-type-assertion
  return Object.fromEntries(
    entries.filter(([, entry]) => entry !== undefined),
  ) as OmitUndefined<T>;
}
