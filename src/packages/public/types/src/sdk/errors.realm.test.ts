import { runInNewContext } from "node:vm";
import { describe, expect, test } from "vitest";
import { toError, toException, toExceptions } from "./errors.ts";
import { recoverError } from "./errors/recover.ts";

const mechanism = {
  handled: false,
  type: "auto.function.nextjs.route_handler",
};

function foreignError() {
  const value: unknown = runInNewContext(
    'new Error("qualification: edge error", { cause: new TypeError("database failed") })',
    {},
    { filename: "edge-route.js" },
  );

  return value;
}

describe("cross-realm errors", () => {
  test("preserves the Edge message, raw stack and foreign cause", () => {
    const error = foreignError();

    expect(error).not.toBeInstanceOf(Error);

    expect(toError(error)).toMatchObject({
      message: "qualification: edge error",
      name: "Error",
    });

    expect(toExceptions(toException(error), mechanism)).toStrictEqual([
      {
        kind: "error",
        mechanism,
        type: "Error",
        value: "qualification: edge error",
        stack: expect.stringContaining("edge-route.js") as unknown,
      },
      {
        kind: "error",
        mechanism: undefined,
        type: "TypeError",
        value: "database failed",
        stack: expect.stringContaining("edge-route.js") as unknown,
      },
    ]);

    expect(toError({ error })).toMatchObject({
      message: "qualification: edge error",
    });
  });

  test("recovers a native foreign stack before it is read", () => {
    const error = runInNewContext(
      'new Error("unread stack")',
      {},
      { filename: "unread-edge-route.js" },
    ) as Error;

    const recovered = toError(error);

    expect(recovered.stack).toContain("unread-edge-route.js");
    expect(recovered.stack).toBe(error.stack);
  });

  test.each(["stack", "cause"])(
    "does not invoke a foreign %s getter",
    (key) => {
      const error = foreignError();

      let getterCalls = 0;

      Object.defineProperty(error, key, {
        get() {
          getterCalls += 1;

          throw new Error("getter must not run");
        },
      });

      const recovered = toException(error);

      expect(recovered).toBeInstanceOf(Error);
      expect(recovered).toMatchObject({ message: "qualification: edge error" });
      expect(getterCalls).toBe(0);

      expect(
        Object.getOwnPropertyDescriptor(recovered, key)?.value,
      ).toBeUndefined();
    },
  );

  test("preserves an intentionally absent foreign stack", () => {
    const error = foreignError();

    Reflect.deleteProperty(error as object, "stack");

    expect(toError(error)).not.toHaveProperty("stack");
  });

  test("rejects a foreign Error whose stack is not text", () => {
    const error = foreignError();

    Object.defineProperty(error, "stack", { value: { privateField: true } });

    expect(recoverError(error)).toBeUndefined();
  });

  test("retains the message when native foreign stack formatting throws", () => {
    const error: unknown = runInNewContext(
      'Error.prepareStackTrace = () => { throw new Error("formatter failed"); }; new Error("original failure")',
    );

    const recovered = recoverError(error);

    expect(recovered).toBeInstanceOf(Error);
    expect(recovered?.message).toBe("original failure");
    expect(recovered).not.toHaveProperty("stack");
  });

  test("contains inspection failures from a revoked thrown proxy", () => {
    const { proxy, revoke } = Proxy.revocable({}, {});

    revoke();

    expect(recoverError(proxy)).toBeUndefined();
  });

  test("bounds a foreign Error cause cycle", () => {
    const error: unknown = runInNewContext(
      '(() => { const error = new Error("cyclic"); error.cause = error; return error; })()',
    );

    const exceptions = toExceptions(toException(error), mechanism);

    expect(exceptions).toHaveLength(5);

    expect(exceptions.map((exception) => exception.value)).toStrictEqual([
      "cyclic",
      "cyclic",
      "cyclic",
      "cyclic",
      "cyclic",
    ]);
  });

  test("keeps ordinary error-shaped objects structured", () => {
    const value = {
      name: "Error",
      message: "structured",
      stack: "customer field",
    };

    expect(toException(value)).toMatchObject({
      kind: "non-error",
      serialized: value,
      type: "PlainObject",
    });
  });

  test("does not invoke foreign Error field accessors", () => {
    const value = foreignError();

    let getterCalls = 0;

    function getter() {
      getterCalls += 1;

      throw new Error("getter must not run");
    }

    Reflect.deleteProperty(value as object, "cause");

    Object.defineProperty(value, "message", { get: getter });

    expect(toException(value)).toMatchObject({ kind: "non-error" });
    expect(getterCalls).toBe(0);
  });

  test("contains hostile proxy inspection failures", () => {
    const value = new Proxy(
      {},
      {
        ownKeys() {
          throw new Error("inspection failed");
        },
      },
    );

    expect(toException(value)).toMatchObject({
      kind: "non-error",
      serialized: "[inaccessible]",
    });

    expect(toError(value)).toMatchObject({
      name: "NonError",
      message: "Inaccessible thrown value",
    });
  });
});
