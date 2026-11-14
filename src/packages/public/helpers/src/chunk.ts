/** Splits items into ordered groups of at most size elements. */
export function chunk<T>(items: readonly T[], size: number) {
  if (!Number.isSafeInteger(size) || size < 1) {
    throw new RangeError("Chunk size must be a positive integer");
  }

  return Array.from({ length: Math.ceil(items.length / size) }, (_, index) =>
    items.slice(index * size, (index + 1) * size),
  );
}
