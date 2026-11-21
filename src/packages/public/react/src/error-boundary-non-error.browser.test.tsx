import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace";
import { page } from "vitest/browser";
import { cleanup, render } from "vitest-browser-react";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

import { DISABLE_ALL_PLUGINS } from "../test/fixtures/disable-all-plugins.js";
import { createTracedKernel } from "../test/fixtures/react-error-handler.js";
import { ErrorBoundary } from "./error-boundary.js";
import type { ErrorBoundaryProps } from "./error-boundary.js";
import type { Kernel } from "./internal/kernel.js";
import { PluginRuntime } from "./internal/plugins/plugin-runtime.js";
import { InterfereProvider } from "./provider.js";

type Thrown = string | null | { readonly code: string };

const THROWN = [
  {
    label: "string",
    value: "render string",
    expected: {
      "exception.message": "render string",
      "interfere.exception.kind": "error",
    },
  },
  {
    label: "null",
    value: null,
    expected: {
      "exception.type": "null",
      "interfere.exception.kind": "non-error",
    },
  },
  {
    label: "plain object",
    value: { code: "E_RENDER" },
    expected: {
      "exception.type": "PlainObject",
      "interfere.exception.kind": "non-error",
    },
  },
] as const;

function Thrower({ value }: { value: Thrown }): never {
  // oxlint-disable-next-line typescript/only-throw-error -- The test reproduces host code throwing non-Error values during render.
  throw value;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Boundary callbacks receive arbitrary thrown values, including the non-Error fixtures here.
function describeThrown(value: unknown) {
  return `fallback:${JSON.stringify(value)}`;
}

let kernel: Kernel;
let exporter: InMemorySpanExporter;

beforeEach(() => {
  vi.spyOn(console, "error");

  ({ exporter, kernel } = createTracedKernel());

  const runtime = new PluginRuntime(kernel);

  kernel.attachRuntime(runtime);

  runtime.applyProviderConfig(DISABLE_ALL_PLUGINS);
});

afterEach(async () => {
  await cleanup();

  vi.restoreAllMocks();
});

async function exceptionEvents() {
  await kernel.flush();

  return exporter
    .getFinishedSpans()
    .flatMap((span) => span.events)
    .filter((event) => event.name === "exception")
    .map((event) => event.attributes ?? {});
}

describe.each(THROWN)("thrown $label", ({ value, expected }) => {
  test("error boundary renders the fallback and calls onError with the thrown value", async () => {
    const onError = vi.fn<NonNullable<ErrorBoundaryProps["onError"]>>();
    const fallbackErrors: unknown[] = [];

    // oxlint-disable-next-line anti-slop/no-unknown-parameters -- This fallback records the original value forwarded by React without narrowing it.
    function fallback(error: unknown) {
      fallbackErrors.push(error);

      return <div>{describeThrown(error)}</div>;
    }

    await render(
      <InterfereProvider
        captureRenderErrors={false}
        kernel={kernel}
        plugins={DISABLE_ALL_PLUGINS}
      >
        <ErrorBoundary fallback={fallback} onError={onError}>
          <Thrower value={value} />
        </ErrorBoundary>
      </InterfereProvider>,
    );

    await expect
      .element(page.getByText(describeThrown(value)))
      .toBeInTheDocument();

    expect(fallbackErrors.at(-1)).toBe(value);
    expect(onError).toHaveBeenCalledTimes(1);
    expect(onError.mock.calls[0]?.[0]).toBe(value);

    const events = await exceptionEvents();

    expect(events).toHaveLength(1);

    expect(events[0]).toMatchObject({
      ...expected,
      "interfere.exception.handled": true,
      "interfere.exception.mechanism": "auto.function.react.error_boundary",
    });
  });

  test("provider capture rethrows the thrown value to the host boundary", async () => {
    const onError = vi.fn<NonNullable<ErrorBoundaryProps["onError"]>>();

    await render(
      <ErrorBoundary
        fallback={(error) => <div>{describeThrown(error)}</div>}
        onError={onError}
      >
        <InterfereProvider kernel={kernel} plugins={DISABLE_ALL_PLUGINS}>
          <Thrower value={value} />
        </InterfereProvider>
      </ErrorBoundary>,
    );

    await expect
      .element(page.getByText(describeThrown(value)))
      .toBeInTheDocument();

    expect(onError).toHaveBeenCalledTimes(1);
    expect(onError.mock.calls[0]?.[0]).toBe(value);

    const events = await exceptionEvents();

    expect(events).toHaveLength(1);

    expect(events[0]).toMatchObject({
      ...expected,
      "interfere.exception.handled": false,
      "interfere.exception.mechanism": "auto.function.react.capture_boundary",
    });
  });
});
