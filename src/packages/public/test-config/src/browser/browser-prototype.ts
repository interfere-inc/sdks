export function method<T>(value: T) {
  return { configurable: true, writable: true, value };
}
