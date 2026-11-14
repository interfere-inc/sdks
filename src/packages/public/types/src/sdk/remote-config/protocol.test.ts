import { expect, test } from "vitest";

import { CAPTURE_PROTOCOL_VERSION, protocolVersionSchema } from "./protocol.ts";

test.each([
  [CAPTURE_PROTOCOL_VERSION, true],
  [99, true],
  [0, false],
  [-1, false],
  [1.5, false],
  ["1", false],
  [null, false],
  [Number.MAX_SAFE_INTEGER + 2, false],
])("protocolVersion %j accepted: %s", (value, accepted) => {
  expect(protocolVersionSchema.safeParse(value).success).toBe(accepted);
});
