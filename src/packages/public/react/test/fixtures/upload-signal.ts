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

export function createMatchSignal<Item>(recorded: readonly Item[]) {
  const waiters = new Set<{
    matches: (item: Item) => boolean;
    resolve: (item: Item) => void;
  }>();

  return {
    notify: () => {
      for (const waiter of waiters) {
        const found = recorded.find((item) => waiter.matches(item));

        if (found !== undefined) {
          waiters.delete(waiter);

          waiter.resolve(found);
        }
      }
    },
    reached: async (matches: (item: Item) => boolean) => {
      const found = recorded.find((item) => matches(item));

      if (found !== undefined) {
        return found;
      }

      const deferred = Promise.withResolvers<Item>();

      waiters.add({ matches, resolve: deferred.resolve });

      return deferred.promise;
    },
  };
}
