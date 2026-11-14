export async function mapWithConcurrency<T, R>(
  items: readonly T[],
  concurrency: number,
  fn: (item: T, index: number) => Promise<R>,
) {
  if (!Number.isSafeInteger(concurrency) || concurrency < 1) {
    throw new RangeError("Concurrency must be a positive safe integer");
  }

  const results: R[] = [];
  const pending = items.entries();

  async function worker() {
    for (const [index, item] of pending) {
      // oxlint-disable-next-line no-await-in-loop -- Each worker must finish its item before taking another to enforce the concurrency limit.
      results[index] = await fn(item, index);
    }
  }

  await Promise.all(
    Array.from({ length: Math.min(concurrency, items.length) }, async () =>
      worker(),
    ),
  );

  return results;
}
