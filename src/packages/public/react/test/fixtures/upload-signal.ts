export function createCountSignal(recorded: readonly unknown[]) {
  const waiters = new Set<{
    count: number;
    resolve: (reached: true) => void;
  }>();

  return {
    notify: () => {
      for (const waiter of waiters) {
        if (recorded.length >= waiter.count) {
          waiters.delete(waiter);

          waiter.resolve(true);
        }
      }
    },
    reached: async (count: number) => {
      if (recorded.length >= count) {
        return true;
      }

      const deferred = Promise.withResolvers<true>();

      waiters.add({ count, resolve: deferred.resolve });

      return deferred.promise;
    },
  };
}
