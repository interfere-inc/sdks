import { expect, test } from "vitest";

import {
  BROWSER_EVENT_LOGGER_NAMES,
  BROWSER_NAVIGATION_LOGGER_NAME,
  INTERFERE_BROWSER_EVENTS_LOGGER_NAME,
} from "./browser-event-loggers";
import { isCorrelationId } from "./correlation-events";

test("accepts UUIDv7 ids and rejects other shapes", () => {
  expect(isCorrelationId("019f4cac-2178-768b-ac0a-5b98b34d4c58")).toBe(true);
  expect(isCorrelationId("019F4CAC-2178-768B-AC0A-5B98B34D4C58")).toBe(true);
  expect(isCorrelationId("9d1e3f2a-8c4b-4f6e-9a7d-1b2c3d4e5f60")).toBe(false);
  expect(isCorrelationId("")).toBe(false);
  expect(isCorrelationId(42)).toBe(false);
  expect(isCorrelationId(null)).toBe(false);
});

test("scans the v12 navigation scope and the v13 Interfere scope", () => {
  expect(BROWSER_EVENT_LOGGER_NAMES).toStrictEqual([
    BROWSER_NAVIGATION_LOGGER_NAME,
    INTERFERE_BROWSER_EVENTS_LOGGER_NAME,
  ]);
});
