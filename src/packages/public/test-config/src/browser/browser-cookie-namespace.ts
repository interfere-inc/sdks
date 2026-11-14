import { z } from "zod";

import { method } from "./browser-prototype.ts";
import { NAMESPACE } from "./browser-storage-scope.ts";
import type { TestScope } from "./browser-storage-scope.ts";

interface Pair {
  readonly name: string;
  readonly value: string;
}

interface Named {
  readonly name?: string | undefined;
}

interface Scoped {
  readonly path?: string | null | undefined;
  readonly domain?: string | null | undefined;
}

type Restore = TestScope["cookies"] extends Map<string, infer R> ? R : never;

function pairs(cookie: string) {
  if (cookie === "") {
    return [];
  }

  return cookie.split("; ").map((pair) => {
    const separator = pair.indexOf("=");

    const parsed: Pair =
      separator === -1
        ? { name: "", value: pair }
        : { name: pair.slice(0, separator), value: pair.slice(separator + 1) };

    return parsed;
  });
}

function persistent(attributes: string) {
  return attributes
    .split(";")
    .map((attribute) => attribute.trim())
    .flatMap((attribute) => {
      const key = attribute.split("=")[0]?.trim().toLowerCase();

      return key === "" || key === "expires" || key === "max-age"
        ? []
        : [`; ${attribute}`];
    })
    .join("");
}

function optionAttributes(options: Scoped) {
  const domain =
    options.domain === undefined || options.domain === null
      ? ""
      : `; domain=${options.domain}`;

  return `; path=${options.path ?? "/"}${domain}`;
}

function withName<T extends Named>(input: T, name: string) {
  return { ...input, name };
}

function query(input: string | CookieStoreGetOptions | undefined) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Web IDL overload: the native method accepts a bare cookie name or an options object.
  return typeof input === "string" ? { name: input } : (input ?? {});
}

export function installCookieNamespace(
  prefix: string,
  scope: () => TestScope | undefined,
) {
  const jar = Object.getOwnPropertyDescriptor(Document.prototype, "cookie");

  if (!jar?.get || !jar.set) {
    throw new Error("document.cookie is not a configurable accessor");
  }

  // oxlint-disable-next-line typescript/unbound-method -- The native accessors are captured once and always invoked with an explicit Document receiver.
  const { get: read, set: write } = jar;

  const rawPairs = (document: Document) =>
    pairs(z.string().parse(read.call(document)));

  const visible = (name: string) =>
    name.startsWith(prefix) || !NAMESPACE.test(name);

  const strip = (name: string) =>
    name.startsWith(prefix) ? name.slice(prefix.length) : name;

  function remember(document: Document, name: string, attributes: string) {
    const active = scope();

    if (!active || active.cookies.has(name)) {
      return;
    }

    const value =
      rawPairs(document).find((pair) => pair.name === prefix + name)?.value ??
      null;

    active.cookies.set(name, { value, attributes });
  }

  Object.defineProperty(Document.prototype, "cookie", {
    configurable: true,
    enumerable: true,
    get(this: Document) {
      return rawPairs(this)
        .flatMap(({ name, value }) => {
          if (!visible(name)) {
            return [];
          }

          const shown = strip(name);

          return [shown === "" ? value : `${shown}=${value}`];
        })
        .join("; ");
    },
    set(this: Document, cookie: string) {
      const end = cookie.indexOf(";");
      const pair = end === -1 ? cookie : cookie.slice(0, end);
      const attributes = end === -1 ? "" : cookie.slice(end);
      const separator = pair.indexOf("=");
      const name = (separator === -1 ? "" : pair.slice(0, separator)).trim();
      const rest = separator === -1 ? `=${pair.trim()}` : pair.slice(separator);

      remember(this, name, attributes);

      write.call(this, `${prefix}${name}${rest}${attributes}`);
    },
  });

  const { prototype } = CookieStore;

  /* oxlint-disable typescript/unbound-method -- The native methods are captured once and always invoked with an explicit CookieStore receiver. */
  const raw = {
    get: prototype.get,
    getAll: prototype.getAll,
    set: prototype.set,
    delete: prototype.delete,
  };

  /* oxlint-enable typescript/unbound-method */

  const prefixed = <T extends Named>(input: T) =>
    input.name === undefined ? input : withName(input, prefix + input.name);

  const stripItem = (item: CookieListItem) =>
    item.name === undefined ? item : withName(item, strip(item.name));

  Object.defineProperties(prototype, {
    get: method(async function get(
      this: CookieStore,
      input: string | CookieStoreGetOptions,
    ) {
      const options = query(input);

      const own = await raw.get.call(this, prefixed(options));

      if (own) {
        return stripItem(own);
      }

      return options.name !== undefined && !NAMESPACE.test(options.name)
        ? raw.get.call(this, options)
        : null;
    }),
    getAll: method(async function getAll(
      this: CookieStore,
      input?: string | CookieStoreGetOptions,
    ) {
      const options = query(input);

      const own = await raw.getAll.call(this, prefixed(options));

      const plain =
        options.name === undefined ? [] : await raw.getAll.call(this, options);

      return [...own, ...plain].flatMap((item) =>
        item.name === undefined || visible(item.name) ? [stripItem(item)] : [],
      );
    }),
    set: method(async function set(
      this: CookieStore,
      input: string | CookieInit,
      value?: string,
    ) {
      const init: CookieInit =
        // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Web IDL overload: the native method accepts a bare cookie name or an options object.
        typeof input === "string" ? { name: input, value: value ?? "" } : input;

      remember(document, init.name, optionAttributes(init));

      await raw.set.call(this, withName(init, prefix + init.name));
    }),
    delete: method(async function remove(
      this: CookieStore,
      input: string | CookieStoreDeleteOptions,
    ) {
      const options: CookieStoreDeleteOptions =
        // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Web IDL overload: the native method accepts a bare cookie name or an options object.
        typeof input === "string" ? { name: input } : input;

      remember(document, options.name, optionAttributes(options));

      await raw.delete.call(this, withName(options, prefix + options.name));
    }),
  });

  function reset(document: Document, name: string, restore: Restore) {
    const attributes = persistent(restore.attributes);

    write.call(
      document,
      restore.value === null
        ? `${prefix}${name}=; max-age=0${attributes}`
        : `${prefix}${name}=${restore.value}${attributes}`,
    );
  }

  return {
    restore(document: Document, cookies: TestScope["cookies"]) {
      for (const [name, restore] of cookies) {
        reset(document, name, restore);
      }
    },
    purge(document: Document) {
      for (const { name } of rawPairs(document)) {
        if (name.startsWith(prefix)) {
          write.call(document, `${name}=; max-age=0; path=/`);

          write.call(document, `${name}=; max-age=0`);
        }
      }
    },
  };
}
