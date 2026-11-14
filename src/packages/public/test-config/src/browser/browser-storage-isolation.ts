import { afterAll, beforeEach } from "vitest";
import { z } from "zod";

import { installCookieNamespace } from "./browser-cookie-namespace.ts";
import { installStorageNamespace } from "./browser-storage-namespace.ts";
import { createTestScope, publishIsolation } from "./browser-storage-scope.ts";
import type { TestScope } from "./browser-storage-scope.ts";

const runner = z
  .object({
    __vitest_browser_runner__: z.object({ testerId: z.string() }).optional(),
  })
  .parse(globalThis);

// oxlint-disable-next-line eslint/no-underscore-dangle -- Vitest names the runner state it injects into every tester iframe with dangling underscores.
const testerId = runner.__vitest_browser_runner__?.testerId;

const id =
  testerId === undefined || testerId === "none"
    ? crypto.randomUUID()
    : testerId;

const prefix = `vt${id.replaceAll("-", "").slice(0, 8)}_`;

let scope: TestScope | undefined;

// WARNING: every browser test file runs in its own iframe on ONE origin, so localStorage, sessionStorage and cookies are shared across parallel files and `storage` events fire into every sibling; the patches below namespace each key with a per-iframe prefix so app and test code keep using plain keys without seeing each other's state.
const storages = installStorageNamespace(prefix, () => scope);

const cookies = installCookieNamespace(prefix, () => scope);

publishIsolation({
  prefix,
  exhaust: storages.exhaust,
  relieve: storages.relieve,
});

// WARNING: the restore is registered through onTestFinished rather than afterEach because the runner calls onTestFinished hooks after every afterEach, in reverse registration order; a file that releases its own storage in afterEach or onTestFinished therefore runs before this restore instead of after it.
beforeEach(({ onTestFinished }) => {
  const active = createTestScope();

  scope = active;

  onTestFinished(() => {
    scope = undefined;

    for (const [storage, snapshot] of active.storages) {
      storages.restore(storage, snapshot);
    }

    cookies.restore(document, active.cookies);
  });
});

afterAll(() => {
  storages.purge(localStorage);

  storages.purge(sessionStorage);

  cookies.purge(document);
});
