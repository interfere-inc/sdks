/* oxlint-disable typescript/no-unsafe-assignment */
import type { MockInstance } from "vitest";
import {
  afterEach,
  assert,
  beforeEach,
  describe,
  expect,
  test,
  vi,
} from "vitest";
import { act } from "react";
import { HttpResponse, http } from "msw";
import { cleanup, render } from "vitest-browser-react";

import { ErrorBoundary } from "./error-boundary.js";
import { Kernel } from "./internal/kernel.js";
import { SessionTracker } from "./tracking/api.js";
import { PluginRuntime } from "./internal/plugins/plugin-runtime.js";
import { DISABLE_ALL_PLUGINS } from "../test/fixtures/disable-all-plugins.js";
import { InterfereProvider } from "./provider.js";
import { worker } from "../test/msw.js";

function ThrowingChild({ error }: { error: Error }) {
  throw error;
}

function GoodChild() {
  return <div>working</div>;
}

const consoleError = console.error;

describe(ErrorBoundary, () => {
  let kernel: Kernel;
  let recordExceptionSpy: MockInstance<Kernel["recordException"]>;

  let previousStorage: {
    storage: Storage;
    key: string;
    value: string | null;
  }[];

  beforeEach(() => {
    previousStorage = [
      ...[
        "interfere:session_id",
        "interfere:last_activity",
        "interfere:session_started",
      ].map((key) => ({
        storage: localStorage,
        key,
        value: localStorage.getItem(key),
      })),
      {
        storage: sessionStorage,
        key: "interfere:window_id",
        value: sessionStorage.getItem("interfere:window_id"),
      },
    ];

    vi.spyOn(console, "error");

    worker.use(
      http.post(
        "https://sdk.invalid/v3/session",
        () => new HttpResponse(null, { status: 204 }),
      ),
    );

    const tracker = new SessionTracker({
      target: {
        baseUrl: "https://sdk.invalid",
        releaseTokenHeader: { apiKey: "test-key" },
      },
    });

    kernel = new Kernel(tracker);

    const runtime = new PluginRuntime(kernel);

    kernel.attachRuntime(runtime);

    runtime.applyProviderConfig(DISABLE_ALL_PLUGINS);

    tracker.start();

    runtime.start();

    recordExceptionSpy = vi.spyOn(kernel, "recordException");
  });

  afterEach(async () => {
    try {
      try {
        await cleanup();
      } finally {
        await kernel.dispose();
      }
    } finally {
      console.error = consoleError;

      vi.restoreAllMocks();

      for (const { storage, key, value } of previousStorage) {
        if (value === null) {
          storage.removeItem(key);
        } else {
          storage.setItem(key, value);
        }
      }
    }
  });

  async function renderInProvider(node: React.ReactNode) {
    return render(
      <InterfereProvider
        captureRenderErrors={false}
        kernel={kernel}
        plugins={DISABLE_ALL_PLUGINS}
      >
        {node}
      </InterfereProvider>,
    );
  }

  test("renders fallback gracefully when ErrorBoundary mounts outside provider", async () => {
    const screen = await render(
      <ErrorBoundary fallback={<div>fallback shown</div>}>
        {/* @ts-expect-error - test */}
        <ThrowingChild error={new Error("pre-init crash")} />
      </ErrorBoundary>,
    );

    await expect
      .element(screen.getByText("fallback shown"))
      .toBeInTheDocument();
  });

  test("renders children when no error is thrown", async () => {
    const screen = await renderInProvider(
      <ErrorBoundary>
        <GoodChild />
      </ErrorBoundary>,
    );

    await expect.element(screen.getByText("working")).toBeInTheDocument();
  });

  test("captures error and renders static fallback", async () => {
    const screen = await renderInProvider(
      <ErrorBoundary fallback={<div>something went wrong</div>}>
        {/* @ts-expect-error - test */}
        <ThrowingChild error={new Error("render crash")} />
      </ErrorBoundary>,
    );

    await expect
      .element(screen.getByText("something went wrong"))
      .toBeInTheDocument();

    expect(recordExceptionSpy).toHaveBeenCalledTimes(1);

    expect(recordExceptionSpy).toHaveBeenCalledWith(
      expect.objectContaining({ message: "render crash" }),
      expect.objectContaining({
        mechanism: {
          handled: true,
          type: "auto.function.react.error_boundary",
        },
      }),
    );
  });

  test("renders null when no fallback is provided", async () => {
    const { container } = await renderInProvider(
      <ErrorBoundary>
        {/* @ts-expect-error - test */}
        <ThrowingChild error={new Error("no-fallback crash")} />
      </ErrorBoundary>,
    );

    expect(container.innerHTML).toBe("");
    expect(recordExceptionSpy).toHaveBeenCalledTimes(1);
  });

  test("calls fallback render function with error and reset", async () => {
    const screen = await renderInProvider(
      <ErrorBoundary
        fallback={(err, reset) => (
          <div>
            <span>{err instanceof Error ? err.message : String(err)}</span>
            <button onClick={reset} type="button">
              retry
            </button>
          </div>
        )}
      >
        {/* @ts-expect-error - test */}
        <ThrowingChild error={new Error("render crash")} />
      </ErrorBoundary>,
    );

    await expect.element(screen.getByText("render crash")).toBeInTheDocument();

    await expect
      .element(screen.getByRole("button", { name: "retry" }))
      .toBeInTheDocument();
  });

  test("resets error state when reset is called", async () => {
    let shouldThrow = true;

    function MaybeThrow() {
      if (shouldThrow) {
        throw new Error("transient");
      }

      return <div>recovered</div>;
    }

    const screen = await renderInProvider(
      <ErrorBoundary
        fallback={(_err, reset) => (
          <button onClick={reset} type="button">
            retry
          </button>
        )}
      >
        <MaybeThrow />
      </ErrorBoundary>,
    );

    await expect
      .element(screen.getByRole("button", { name: "retry" }))
      .toBeInTheDocument();

    const retry = screen.getByRole("button", { name: "retry" }).element();

    assert.instanceOf(retry, HTMLButtonElement);

    act(() => {
      shouldThrow = false;

      retry.click();
    });

    await expect.element(screen.getByText("recovered")).toBeInTheDocument();
  });

  test("calls onError callback with error and info", async () => {
    const error = new Error("render crash");

    const onError =
      vi.fn<
        NonNullable<
          import("react").ComponentProps<typeof ErrorBoundary>["onError"]
        >
      >();

    await renderInProvider(
      <ErrorBoundary fallback={<div>oops</div>} onError={onError}>
        {/* @ts-expect-error - test */}
        <ThrowingChild error={error} />
      </ErrorBoundary>,
    );

    expect(onError).toHaveBeenCalledTimes(1);

    expect(onError).toHaveBeenCalledWith(
      error,
      expect.objectContaining({ componentStack: expect.any(String) }),
    );
  });

  test("forwards the original error stack and appended component frames", async () => {
    const err = new Error("render crash");

    err.stack = [
      "Error: render crash",
      "    at handleClick (app.js:42:12)",
    ].join("\n");

    await renderInProvider(
      <ErrorBoundary fallback={<div>oops</div>}>
        {/* @ts-expect-error - test */}
        <ThrowingChild error={err} />
      </ErrorBoundary>,
    );

    expect(recordExceptionSpy).toHaveBeenCalledTimes(1);

    const [forwardedError, opts] = recordExceptionSpy.mock.calls[0] ?? [];

    expect(forwardedError).toHaveProperty(
      "stack",
      expect.stringContaining("handleClick"),
    );

    expect(opts?.appendFrames).toBeDefined();
  });

  test("forwards the same error and its cause to capture", async () => {
    const root = new Error("root cause");
    const wrapper = new Error("wrapper");

    wrapper.cause = root;

    await renderInProvider(
      <ErrorBoundary fallback={<div>oops</div>}>
        {/* @ts-expect-error - test */}
        <ThrowingChild error={wrapper} />
      </ErrorBoundary>,
    );

    expect(recordExceptionSpy).toHaveBeenCalledTimes(1);

    const [forwardedError, opts] = recordExceptionSpy.mock.calls[0] ?? [];

    expect(forwardedError).toBe(wrapper);
    expect(wrapper.cause).toBe(root);

    expect(opts).toMatchObject({
      mechanism: {
        handled: true,
        type: "auto.function.react.error_boundary",
      },
    });
  });
});
