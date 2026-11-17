import { commands } from "@interfere/test-config/browser-platform";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";

import { createPluginContext } from "../../test/fixtures/plugin-context.js";
import rageClicksPlugin from "./rage-clicks.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

function simulateClick(el: Element, x: number, y: number) {
  el.dispatchEvent(
    new MouseEvent("click", {
      bubbles: true,
      clientX: x,
      clientY: y,
    }),
  );
}

describe("rage-clicks plugin", () => {
  let ctx: ReturnType<typeof createPluginContext>;
  let cleanup: (() => void) | undefined;
  let target: HTMLDivElement;
  let exporter: InMemorySpanExporter;

  beforeEach(() => {
    vi.useFakeTimers();

    target = document.createElement("div");

    target.id = "rage-target";

    document.body.append(target);

    exporter = new InMemorySpanExporter();

    ctx = createPluginContext([new SimpleSpanProcessor(exporter)]);
  });

  afterEach(() => {
    cleanup?.();

    vi.useRealTimers();

    target.remove();
  });

  function rageClickSpans() {
    return exporter.getFinishedSpans().filter((s) => s.name === "rage_click");
  }

  test("does not fire below click threshold (3)", () => {
    cleanup = rageClicksPlugin.setup(ctx);

    simulateClick(target, 100, 100);

    simulateClick(target, 100, 100);

    expect(rageClickSpans()).toHaveLength(0);
  });

  test("fires after 3 clustered clicks within the time window", () => {
    cleanup = rageClicksPlugin.setup(ctx);

    simulateClick(target, 100, 100);

    vi.advanceTimersByTime(100);

    simulateClick(target, 105, 105);

    vi.advanceTimersByTime(100);

    simulateClick(target, 110, 110);

    const spans = rageClickSpans();

    expect(spans).toHaveLength(1);

    expect(spans[0]?.attributes).toMatchObject({
      "ui.event_type": "rage_click",
      "ui.rage_click.count": 3,
      "ui.rage_click.selector": "#rage-target",
    });
  });

  test("does not fire if clicks exceed the time window (800ms)", () => {
    cleanup = rageClicksPlugin.setup(ctx);

    simulateClick(target, 100, 100);

    vi.advanceTimersByTime(500);

    simulateClick(target, 100, 100);

    vi.advanceTimersByTime(500);

    simulateClick(target, 100, 100);

    expect(rageClickSpans()).toHaveLength(0);
  });

  test("does not fire if clicks are too far apart spatially", () => {
    cleanup = rageClicksPlugin.setup(ctx);

    simulateClick(target, 0, 0);

    vi.advanceTimersByTime(50);

    simulateClick(target, 0, 0);

    vi.advanceTimersByTime(50);

    simulateClick(target, 100, 100);

    expect(rageClickSpans()).toHaveLength(0);
  });

  test("clears the click buffer after detection", () => {
    cleanup = rageClicksPlugin.setup(ctx);

    simulateClick(target, 100, 100);

    vi.advanceTimersByTime(50);

    simulateClick(target, 100, 100);

    vi.advanceTimersByTime(50);

    simulateClick(target, 100, 100);

    expect(rageClickSpans()).toHaveLength(1);

    simulateClick(target, 100, 100);

    expect(rageClickSpans()).toHaveLength(1);
  });

  test("uses CSS selector with element id", () => {
    cleanup = rageClicksPlugin.setup(ctx);

    simulateClick(target, 100, 100);

    vi.advanceTimersByTime(50);

    simulateClick(target, 100, 100);

    vi.advanceTimersByTime(50);

    simulateClick(target, 100, 100);

    expect(rageClickSpans()[0]?.attributes).toMatchObject({
      "ui.rage_click.selector": "#rage-target",
    });
  });

  test("falls back to tag name when no id", () => {
    const plain = document.createElement("span");

    document.body.append(plain);

    cleanup = rageClicksPlugin.setup(ctx);

    simulateClick(plain, 100, 100);

    vi.advanceTimersByTime(50);

    simulateClick(plain, 100, 100);

    vi.advanceTimersByTime(50);

    simulateClick(plain, 100, 100);

    expect(rageClickSpans()[0]?.attributes).toMatchObject({
      "ui.rage_click.selector": "span",
    });

    plain.remove();
  });

  test("lets a clicked element be collected once it leaves the document", async () => {
    cleanup = rageClicksPlugin.setup(ctx);

    const clickThenDetach = () => {
      const button = document.createElement("button");

      document.body.append(button);

      simulateClick(button, 100, 100);

      simulateClick(button, 100, 100);

      button.remove();

      return new WeakRef(button);
    };

    const collected = clickThenDetach();

    await commands.collectGarbage();

    expect(collected.deref()).toBeUndefined();
  });

  test("reports an unknown selector when the clicks have no element target", () => {
    cleanup = rageClicksPlugin.setup(ctx);

    for (let click = 0; click < 3; click += 1) {
      document.dispatchEvent(
        new MouseEvent("click", { clientX: 100, clientY: 100 }),
      );
    }

    const spans = rageClickSpans();

    expect(spans).toHaveLength(1);

    expect(spans[0]?.attributes).toMatchObject({
      "ui.rage_click.selector": "unknown",
    });
  });

  describe("cleanup", () => {
    test("stops detecting rage clicks after cleanup", () => {
      cleanup = rageClicksPlugin.setup(ctx);

      cleanup?.();

      cleanup = undefined;

      simulateClick(target, 100, 100);

      vi.advanceTimersByTime(50);

      simulateClick(target, 100, 100);

      vi.advanceTimersByTime(50);

      simulateClick(target, 100, 100);

      expect(rageClickSpans()).toHaveLength(0);
    });
  });
});
