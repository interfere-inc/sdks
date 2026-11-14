import { expect, test } from "vitest";

import { DEFAULT_TRACING_MODE, tracingModeSchema } from "./tracing-mode.ts";
import { tracingOwnershipSchema } from "./tracing-ownership.ts";

test("adds auto, the default, to the explicit owners", () => {
  expect(tracingModeSchema.options).toStrictEqual([
    DEFAULT_TRACING_MODE,
    ...tracingOwnershipSchema.options,
  ]);

  expect(tracingOwnershipSchema.safeParse("auto").success).toBe(false);
});
