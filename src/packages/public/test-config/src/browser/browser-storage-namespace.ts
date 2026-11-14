import { z } from "zod";

import { method } from "./browser-prototype.ts";
import type { TestScope } from "./browser-storage-scope.ts";

export function installStorageNamespace(
  prefix: string,
  scope: () => TestScope | undefined,
) {
  const { prototype } = Storage;

  /* oxlint-disable typescript/unbound-method -- The native methods are captured once and always invoked with an explicit Storage receiver. */
  const raw = {
    getItem: prototype.getItem,
    setItem: prototype.setItem,
    removeItem: prototype.removeItem,
    key: prototype.key,
    lengthOf: Object.getOwnPropertyDescriptor(prototype, "length")?.get,
  };

  /* oxlint-enable typescript/unbound-method */

  if (!raw.lengthOf) {
    throw new Error("Storage.prototype.length is not an accessor");
  }

  const { lengthOf } = raw;

  const budgets = new WeakMap<Storage, number>();

  function ownKeys(storage: Storage) {
    const keys: string[] = [];
    const count = z.number().parse(lengthOf.call(storage));

    for (let index = 0; index < count; index += 1) {
      const key = raw.key.call(storage, index);

      if (key?.startsWith(prefix)) {
        keys.push(key.slice(prefix.length));
      }
    }

    return keys;
  }

  function remember(storage: Storage, key: string) {
    const active = scope();

    if (!active) {
      return;
    }

    let snapshot = active.storages.get(storage);

    if (!snapshot) {
      snapshot = new Map();

      active.storages.set(storage, snapshot);
    }

    if (!snapshot.has(key)) {
      snapshot.set(key, raw.getItem.call(storage, prefix + key));
    }
  }

  // WARNING: a quota test reserves room by writing a key before the fill and freeing it afterwards, so the simulated budget must credit removals and charge only the growth of a replaced value, exactly as the real full origin did.
  function charge(storage: Storage, key: string, value: string | null) {
    const budget = budgets.get(storage);

    if (budget === undefined) {
      return;
    }

    const previous = raw.getItem.call(storage, prefix + key);
    const freed = previous === null ? 0 : key.length + previous.length;
    const needed = value === null ? 0 : key.length + value.length;
    const delta = needed - freed;

    if (delta > budget) {
      throw new DOMException(
        `Failed to execute 'setItem' on 'Storage': Setting the value of '${key}' exceeded the quota.`,
        "QuotaExceededError",
      );
    }

    budgets.set(storage, budget - delta);
  }

  Object.defineProperties(prototype, {
    getItem: method(function getItem(this: Storage, key: string) {
      return raw.getItem.call(this, prefix + key);
    }),
    setItem: method(function setItem(
      this: Storage,
      key: string,
      value: string,
    ) {
      charge(this, key, value);

      remember(this, key);

      raw.setItem.call(this, prefix + key, value);
    }),
    removeItem: method(function removeItem(this: Storage, key: string) {
      charge(this, key, null);

      remember(this, key);

      raw.removeItem.call(this, prefix + key);
    }),
    clear: method(function clear(this: Storage) {
      for (const key of ownKeys(this)) {
        charge(this, key, null);

        remember(this, key);

        raw.removeItem.call(this, prefix + key);
      }
    }),
    key: method(function key(this: Storage, index: number) {
      return ownKeys(this)[index] ?? null;
    }),
    length: {
      configurable: true,
      get(this: Storage) {
        return ownKeys(this).length;
      },
    },
  });

  return {
    exhaust: (storage: Storage) => {
      budgets.set(storage, 0);
    },
    relieve: (storage: Storage) => {
      budgets.delete(storage);
    },
    restore(storage: Storage, snapshot: Map<string, string | null>) {
      for (const [key, value] of snapshot) {
        if (value === null) {
          raw.removeItem.call(storage, prefix + key);
        } else {
          raw.setItem.call(storage, prefix + key, value);
        }
      }
    },
    purge(storage: Storage) {
      for (const key of ownKeys(storage)) {
        raw.removeItem.call(storage, prefix + key);
      }
    },
  };
}
