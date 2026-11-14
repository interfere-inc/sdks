const KEY_PREFIX = "interfere:replay_seq:";

function sessionStore() {
  try {
    const store = globalThis.sessionStorage;
    const probe = "__interfere_replay_seq_probe__";

    store.setItem(probe, "1");

    store.removeItem(probe);

    return store;
  } catch {
    return null;
  }
}

export interface SegmentSequence {
  take: () => number;
}

export function createSegmentSequence(windowId: string) {
  const key = `${KEY_PREFIX}${windowId}`;
  const store = sessionStore();
  const stored = store?.getItem(key);

  let next = stored ? Number(stored) : 0;

  if (!Number.isInteger(next) || next < 0) {
    next = 0;
  }

  return {
    take() {
      const persisted = Number(store?.getItem(key));

      if (Number.isSafeInteger(persisted) && persisted > next) {
        next = persisted;
      }

      const seq = next;

      next += 1;

      store?.setItem(key, String(next));

      return seq;
    },
  } satisfies SegmentSequence;
}
