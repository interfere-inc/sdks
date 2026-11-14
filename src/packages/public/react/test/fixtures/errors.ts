export function createError(name = "Error", message = "test error") {
  const err = new Error(message);

  err.name = name;
  err.stack = `${name}: ${message}\n    at handleClick (app.js:42:12)\n    at onClick (app.js:10:5)`;

  return err;
}

export function createErrorChain(depth = 3) {
  let current: Error | undefined;

  for (let i = depth - 1; i >= 0; i -= 1) {
    const err = createError(`Error${i}`, `error at depth ${i}`);

    if (current) {
      err.cause = current;
    }

    current = err;
  }

  if (!current) {
    throw new RangeError("Error chain depth must be positive");
  }

  return current;
}
