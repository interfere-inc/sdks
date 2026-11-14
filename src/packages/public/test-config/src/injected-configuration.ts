import { onTestFinished } from "vitest";

export function setInjectedConfiguration<Value>(
  name: `__INTERFERE_${string}__`,
  value: Value,
) {
  const original = Object.getOwnPropertyDescriptor(globalThis, name);

  Object.defineProperty(globalThis, name, {
    configurable: true,
    enumerable: true,
    writable: true,
    value,
  });

  onTestFinished(() => {
    if (original) {
      Object.defineProperty(globalThis, name, original);
    } else {
      Reflect.deleteProperty(globalThis, name);
    }
  });

  return value;
}
