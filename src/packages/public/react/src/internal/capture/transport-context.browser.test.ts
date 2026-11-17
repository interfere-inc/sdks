import { afterEach, expect, test } from "vitest";

import { setPageLeaving } from "../page-lifecycle/unloading.js";
import {
  transportContextAttrs,
  transportFailureAttrs,
} from "./transport-context.js";

afterEach(() => {
  setPageLeaving(false);
});

test("records that the page is leaving when a transport failure lands during unload", () => {
  setPageLeaving(true);

  expect(transportContextAttrs()).toMatchObject({
    "error.type": "transport_failure",
    "interfere.page.leaving": true,
  });
});

test("reads the browser's own visibility and connectivity", () => {
  expect(transportContextAttrs()).toStrictEqual({
    "error.type": "transport_failure",
    "interfere.page.leaving": false,
    "interfere.page.visibility": document.visibilityState,
  });
});

test("records an offline browser as an unavailable network connection", () => {
  expect(transportContextAttrs(false)).toMatchObject({
    "network.connection.type": "unavailable",
  });
});

test("follows the cause chain to a transport failure", () => {
  const error = new Error("sync stopped", {
    cause: new Error("wrapped", { cause: new TypeError("Load failed") }),
  });

  expect(transportFailureAttrs(error)).not.toBeNull();
  expect(transportFailureAttrs(new Error("sync stopped"))).toBeNull();
});
