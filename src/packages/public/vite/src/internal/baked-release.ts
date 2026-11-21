// oxlint-disable-next-line typescript/triple-slash-reference
/// <reference path="../vite-env.d.ts" />

export const BAKED_RELEASE_SLUG_DEFINE = "__INTERFERE_BAKED_RELEASE_SLUG__";

export const BAKED_RELEASE_TOKEN_DEFINE = "__INTERFERE_BAKED_RELEASE_TOKEN__";

// oxlint-disable-next-line eslint/no-underscore-dangle -- Vite replaces this build-time identifier; the dunder name keeps it clear of application identifiers.
declare const __INTERFERE_BAKED_RELEASE_SLUG__: string | null | undefined;

// oxlint-disable-next-line eslint/no-underscore-dangle -- Vite replaces this build-time identifier; the dunder name keeps it clear of application identifiers.
declare const __INTERFERE_BAKED_RELEASE_TOKEN__: string | null | undefined;

// WARNING: production builds bake per-release values through bare identifiers, not import.meta.env, because Vite copies every import.meta.env define into each chunk that reads import.meta.env whole, renaming those chunks and all their importers on every commit.
export function bakedReleaseSlug() {
  // oxlint-disable-next-line unicorn/no-typeof-undefined -- Outside a Vite build the identifier is undeclared, so comparing it directly throws a ReferenceError.
  return typeof __INTERFERE_BAKED_RELEASE_SLUG__ === "undefined"
    ? // oxlint-disable-next-line typescript/no-unnecessary-condition, typescript/dot-notation -- Native Node omits env; downstream source consumers see only Vite's index signature.
      import.meta.env?.["VITE_INTERFERE_RELEASE_SLUG"]
    : __INTERFERE_BAKED_RELEASE_SLUG__;
}

export function bakedReleaseToken() {
  // oxlint-disable-next-line unicorn/no-typeof-undefined -- Outside a Vite build the identifier is undeclared, so comparing it directly throws a ReferenceError.
  return typeof __INTERFERE_BAKED_RELEASE_TOKEN__ === "undefined"
    ? // oxlint-disable-next-line typescript/no-unnecessary-condition, typescript/dot-notation -- Native Node omits env; downstream source consumers see only Vite's index signature.
      import.meta.env?.["VITE_INTERFERE_RELEASE_TOKEN"]
    : __INTERFERE_BAKED_RELEASE_TOKEN__;
}
