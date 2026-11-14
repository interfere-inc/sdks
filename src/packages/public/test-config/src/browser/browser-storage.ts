import { onTestFinished } from "vitest";

import { requireIsolation } from "./browser-storage-scope.ts";

function writer(storage: () => Storage) {
  return {
    set(key: string, value: string) {
      requireIsolation("browserStorage");

      storage().setItem(key, value);
    },
    remove(key: string) {
      requireIsolation("browserStorage");

      storage().removeItem(key);
    },
    clear() {
      requireIsolation("browserStorage");

      storage().clear();
    },
  };
}

export const browserStorage = {
  local: writer(() => localStorage),
  session: writer(() => sessionStorage),
  cookies: {
    set(name: string, value: string, attributes = "path=/") {
      requireIsolation("browserStorage");

      // oxlint-disable-next-line unicorn/no-document-cookie -- Synchronous write on the isolated test origin; the Cookie Store API is asynchronous.
      document.cookie = `${name}=${value}; ${attributes}`;
    },
    remove(name: string, attributes = "path=/") {
      requireIsolation("browserStorage");

      // oxlint-disable-next-line unicorn/no-document-cookie -- Synchronous write on the isolated test origin; the Cookie Store API is asynchronous.
      document.cookie = `${name}=; max-age=0; ${attributes}`;
    },
  },
};

// WARNING: the real quota is shared by every parallel iframe on the origin, so padding it starved sibling files' writes while a quota test ran (and a sibling's clear() freed the padding mid-test); the exhaustion is simulated inside this file's realm instead and lifted when the test finishes.
export function fillStorageQuota(storage: Storage) {
  const isolation = requireIsolation("fillStorageQuota");

  isolation.exhaust(storage);

  onTestFinished(() => {
    isolation.relieve(storage);
  });
}

export function fillLocalStorageQuota() {
  fillStorageQuota(localStorage);
}
