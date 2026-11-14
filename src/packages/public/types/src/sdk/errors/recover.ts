/* oxlint-disable anti-slop/no-runtime-typeof, anti-slop/no-object-parameters -- Recovering foreign Error data requires inspecting untrusted object descriptors without invoking their getters. */
function ownOrInheritedDescriptor(value: object, key: string) {
  let current: object | null = value;

  for (let depth = 0; current && depth < 8; depth += 1) {
    const descriptor = Object.getOwnPropertyDescriptor(current, key);

    if (descriptor) {
      return descriptor;
    }

    current = Reflect.getPrototypeOf(current);
  }

  return;
}

const nativeStack = ownOrInheritedDescriptor(
  new Error("Interfere stack accessor"),
  "stack",
);

function dataProperty(value: object, key: string) {
  let current: object | null = value;

  for (let depth = 0; current && depth < 8; depth += 1) {
    const descriptor = Object.getOwnPropertyDescriptor(current, key);

    if (descriptor) {
      if ("value" in descriptor) {
        return descriptor.value as unknown;
      }

      if (
        key === "stack" &&
        nativeStack?.get &&
        descriptor.get &&
        // oxlint-disable-next-line typescript/unbound-method -- Compare accessor source without invoking the foreign getter; only the local native getter is called below.
        Function.prototype.toString.call(descriptor.get) ===
          // oxlint-disable-next-line typescript/unbound-method -- Inspect the local getter source without invoking it on the descriptor.
          Function.prototype.toString.call(nativeStack.get)
      ) {
        try {
          return nativeStack.get.call(value) as unknown;
        } catch {
          return;
        }
      }

      return;
    }

    current = Reflect.getPrototypeOf(current);
  }

  return;
}

/** Preserves native Errors and copies foreign-realm Error data without invoking its custom getters. */
// oxlint-disable-next-line anti-slop/no-unknown-parameters -- SDK capture accepts arbitrary thrown values across browser and server realms.
export function recoverError(value: unknown) {
  try {
    if (value instanceof Error) {
      return value;
    }

    if (
      value === null ||
      typeof value !== "object" ||
      Symbol.toStringTag in value ||
      Object.prototype.toString.call(value) !== "[object Error]"
    ) {
      return;
    }

    const message = dataProperty(value, "message");
    const name = dataProperty(value, "name");

    if (typeof message !== "string" || typeof name !== "string") {
      return;
    }

    const stack = dataProperty(value, "stack");

    if (stack !== undefined && typeof stack !== "string") {
      return;
    }

    const error = new Error(message, { cause: dataProperty(value, "cause") });

    error.name = name;

    if (stack === undefined) {
      delete error.stack;
    } else {
      error.stack = stack;
    }

    return error;
  } catch {
    return;
  }
}
