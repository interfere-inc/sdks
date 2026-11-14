import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";
import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace-base";

import { Component } from "react";
import type { ReactNode } from "react";
import { flushSync } from "react-dom";
import { createRoot } from "react-dom/client";
import { beforeEach, expect, onTestFinished, test } from "vitest";

import { setupErrorsPlugin } from "../../../test/fixtures/errors.test-utils.js";
import { InterfereProvider } from "../../provider.js";
import type { Kernel } from "../kernel.js";
import { captureReactError } from "./capture.js";

let kernel: Kernel;

let exporter: InMemorySpanExporter;

beforeEach(() => {
  ({ kernel, exporter } = setupErrorsPlugin());
});

function capturedMessages(message: string) {
  return exporter
    .getFinishedSpans()
    .flatMap((span) => span.events)
    .filter((event) => event.attributes?.["exception.message"] === message);
}

function sameSiteError(message: string) {
  return new Error(message);
}

class HostBoundary extends Component<
  { readonly children: ReactNode },
  { readonly failed: boolean }
> {
  override state = { failed: false };

  static getDerivedStateFromError() {
    return { failed: true };
  }

  // oxlint-disable-next-line typescript/promise-function-async -- ReactNode includes Promise, so React class render cannot be async.
  override render() {
    return this.state.failed ? <h1>Host fallback</h1> : this.props.children;
  }
}

function Thrower({ message }: { readonly message: string }): never {
  throw sameSiteError(message);
}

test("records one exception when React retries a render and the host reports the retried error", () => {
  const message = "retried render";
  const reportCaught = console.error.bind(console);
  const container = document.createElement("div");

  document.body.append(container);

  const root = createRoot(container, {
    onCaughtError: (error) => {
      reportCaught(error);
    },
  });

  onTestFinished(() => {
    root.unmount();

    container.remove();
  });

  flushSync(() => {
    root.render(
      <HostBoundary>
        <InterfereProvider kernel={kernel}>
          <Thrower message={message} />
        </InterfereProvider>
      </HostBoundary>,
    );
  });

  expect(container.textContent).toBe("Host fallback");

  expect(capturedMessages(message)).toHaveLength(1);
});

test.each([
  { first: "boundary", second: "console" },
  { first: "console", second: "boundary" },
] as const)(
  "records one exception when the $first path reports before the $second path in one turn",
  ({ first, second }) => {
    const message = `${first} then ${second}`;

    const report = {
      boundary: () => {
        captureReactError(kernel, sameSiteError(message), null, {
          handled: false,
          type: MECHANISM_TYPE.react.captureBoundary,
        });
      },
      console: () => {
        console.error(sameSiteError(message));
      },
    };

    report[first]();

    report[second]();

    expect(capturedMessages(message)).toHaveLength(1);
  },
);

test("records the same error again when it recurs after the current turn", async () => {
  const message = "recurs later";

  console.error(sameSiteError(message));

  await Promise.resolve();

  captureReactError(kernel, sameSiteError(message), null, {
    handled: false,
    type: MECHANISM_TYPE.react.captureBoundary,
  });

  expect(capturedMessages(message)).toHaveLength(2);
});
