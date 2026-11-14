export function getGlobal(key: string) {
  // oxlint-disable-next-line anti-slop/no-reflect-get -- The caller supplies an arbitrary host-global key, including accessor-backed properties.
  const decodedValue: unknown = Reflect.get(globalThis, key);

  return decodedValue;
}

export function getStringGlobal(key: string) {
  const value = getGlobal(key);

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Injected strings must reject non-string host values while preserving the empty string.
  return (typeof value === "string" ? value : undefined) satisfies
    | string
    | undefined;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Host globals store customer and SDK values without a common schema or serializable-only restriction.
export function setGlobal(key: string, value: unknown) {
  Reflect.set(globalThis, key, value);
}
