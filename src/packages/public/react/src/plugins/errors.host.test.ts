import { expect, onTestFinished, test } from "vitest";

import { Kernel } from "../internal/kernel.js";
import { SessionTracker } from "../tracking/api.js";
import errorsPlugin from "./errors.js";

test("sets up and cleans up in native Node without browser event registration", () => {
  const kernel = new Kernel(
    new SessionTracker({
      target: {
        baseUrl: "https://errors.invalid",
        releaseTokenHeader: { apiKey: "test" },
      },
    }),
  );

  onTestFinished(async () => {
    await kernel.dispose();
  });

  const original = globalThis.onerror;

  expect(globalThis.addEventListener).toBeUndefined();
  expect(globalThis.removeEventListener).toBeUndefined();

  let cleanup = errorsPlugin.setup(kernel);

  onTestFinished(() => cleanup?.());

  expect(globalThis.onerror).toBeTypeOf("function");

  cleanup?.();

  cleanup = undefined;

  expect(globalThis.onerror).toBe(original);
});
