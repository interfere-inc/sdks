import { SERVICE_NAME_GLOBAL } from "./service-name.ts";
import { expect, test } from "vitest";

test("pins the stamped service-name global to its wire string", () => {
  expect(SERVICE_NAME_GLOBAL).toBe("__INTERFERE_SERVICE_NAME__");
});
