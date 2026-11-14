export const NAMESPACE = /^vt[0-9a-f]{8}_/u;

export interface TestScope {
  readonly storages: Map<Storage, Map<string, string | null>>;
  readonly cookies: Map<
    string,
    { readonly value: string | null; readonly attributes: string }
  >;
}

export interface Isolation {
  readonly prefix: string;
  readonly exhaust: (storage: Storage) => void;
  readonly relieve: (storage: Storage) => void;
}

let published: Isolation | undefined;

export function createTestScope() {
  const scope: TestScope = { storages: new Map(), cookies: new Map() };

  return scope;
}

export function publishIsolation(isolation: Isolation) {
  published = isolation;
}

export function requireIsolation(caller: string) {
  if (!published) {
    throw new Error(
      `${caller} needs the @interfere/test-config browser() setup, which namespaces storage per test file; this page has no isolation installed`,
    );
  }

  return published;
}
