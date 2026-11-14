import { describe, expect, test } from "vitest";

import { createErrorChain } from "./errors";

describe(createErrorChain, () => {
  test("links errors from the outermost error to the deepest cause", () => {
    const error = createErrorChain(3);

    expect(error).toMatchObject({
      name: "Error0",
      message: "error at depth 0",
      cause: {
        name: "Error1",
        cause: { name: "Error2", message: "error at depth 2" },
      },
    });
  });

  test.each([0, -1, Number.NaN])(
    "rejects an empty chain at depth %s",
    (depth) => {
      expect(() => createErrorChain(depth)).toThrow(RangeError);
    },
  );
});
