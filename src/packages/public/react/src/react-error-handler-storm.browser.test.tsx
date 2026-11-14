import { Component } from "react";
import type { ReactNode } from "react";
import { flushSync } from "react-dom";
import { createRoot } from "react-dom/client";
import { expect, onTestFinished, test } from "vitest";

import { createTracedKernel } from "../test/fixtures/react-error-handler.js";
import { reactErrorHandler } from "./react-error-handler.js";

const RENDER_STORM = 50;

const PER_KEY_LIMIT = 5;

class Boundary extends Component<
  { readonly children: ReactNode },
  { readonly failed: boolean }
> {
  override state = { failed: false };

  static getDerivedStateFromError() {
    return { failed: true };
  }

  // oxlint-disable-next-line typescript/promise-function-async -- ReactNode includes Promise, so React class render cannot be async.
  override render() {
    return this.state.failed ? null : this.props.children;
  }
}

function Thrower({ message }: { readonly message: string }): never {
  throw new TypeError(message);
}

test("caps a storm of caught render errors sharing one message", async () => {
  const { exporter, kernel } = createTracedKernel();
  const container = document.createElement("div");

  document.body.append(container);

  const root = createRoot(container, reactErrorHandler({ kernel }));

  onTestFinished(() => {
    root.unmount();

    container.remove();
  });

  for (let i = 0; i < RENDER_STORM; i += 1) {
    flushSync(() => {
      root.render(
        <Boundary key={i}>
          <Thrower message="render storm" />
        </Boundary>,
      );
    });
  }

  flushSync(() => {
    root.render(
      <Boundary key="distinct">
        <Thrower message="distinct render" />
      </Boundary>,
    );
  });

  await kernel.flush();

  expect(
    exporter
      .getFinishedSpans()
      .map((span) => span.events[0]?.attributes?.["exception.message"]),
  ).toStrictEqual([
    ...Array.from({ length: PER_KEY_LIMIT }, () => "render storm"),
    "distinct render",
  ]);
});
