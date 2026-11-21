import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace";

import { createError } from "../../../test/fixtures/errors.js";
import { createTracedKernel } from "../../../test/fixtures/react-error-handler.js";
import type { Kernel } from "../kernel.js";
import { reactErrorHandler } from "../../react-error-handler.js";
import { captureReactError } from "./capture.js";
import { beforeEach, expect, test } from "vitest";

let kernel: Kernel;

let exporter: InMemorySpanExporter;

beforeEach(() => {
  ({ exporter, kernel } = createTracedKernel());
});

async function flushedException() {
  await kernel.flush();

  const spans = exporter.getFinishedSpans();
  const attributes = spans[0]?.events[0]?.attributes;

  const chain: unknown = JSON.parse(
    String(attributes?.["interfere.exception.chain"]),
  );

  expect(spans).toHaveLength(1);
  expect(spans[0]?.events).toHaveLength(1);

  return { attributes, chain };
}

test.each([
  {
    message:
      "Hydration failed because the initial UI does not match what was rendered on the server.",
    subtype: "hydration",
    synthetic: false,
  },
  {
    message: "Some other recoverable error",
    subtype: undefined,
    synthetic: true,
  },
])(
  "records recoverable metadata for $message",
  async ({ message, subtype, synthetic }) => {
    const error = createError("Error", message);

    const mechanism = {
      handled: true,
      synthetic: true,
      type: "auto.function.react.recoverable_error",
    };

    captureReactError(kernel, error, null, mechanism);

    const { attributes, chain } = await flushedException();

    expect(attributes?.["interfere.exception.subtype"]).toBe(subtype);

    expect(chain).toStrictEqual([
      {
        kind: "error",
        type: "Error",
        value: message,
        stack: error.stack,
        mechanism: { ...mechanism, synthetic },
      },
    ]);

    expect(attributes).not.toHaveProperty(
      "interfere.exception.appended_frames",
    );

    expect(attributes).not.toHaveProperty("interfere.error.digest");
  },
);

test("preserves component frames, digest and both errors in a cause chain", async () => {
  const cause = createError("TypeError", "cause");

  const error = Object.assign(createError("Error", "with stack"), {
    cause,
    digest: "render-digest",
  });

  const mechanism = { handled: true, type: "auto.function.react.caught_error" };

  reactErrorHandler({ kernel }).onCaughtError(error, {
    componentStack: "    in BuyPageComponent\n    in App",
  });

  const { attributes, chain } = await flushedException();

  expect(attributes?.["interfere.exception.appended_frames"]).toBe(
    JSON.stringify([
      { fn: "BuyPageComponent", id: "react-0" },
      { fn: "App", id: "react-1" },
    ]),
  );

  expect(attributes?.["interfere.error.digest"]).toBe("render-digest");

  expect(chain).toStrictEqual([
    {
      kind: "error",
      type: "Error",
      value: "with stack",
      stack: error.stack,
      mechanism,
    },
    { kind: "error", type: "TypeError", value: "cause", stack: cause.stack },
  ]);
});
