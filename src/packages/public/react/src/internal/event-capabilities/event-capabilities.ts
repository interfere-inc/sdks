export function hasEventListenerPair(
  add: typeof globalThis.addEventListener | undefined,
  remove: typeof globalThis.removeEventListener | undefined,
) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof
  return typeof add === "function" && typeof remove === "function";
}
