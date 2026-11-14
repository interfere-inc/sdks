import {
  onPageHidden,
  onPageShown,
  onPageUnloading,
  resetPageLifecycleForTests,
} from "./page-lifecycle.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

function pageTransition(type: "pagehide" | "pageshow", persisted = true) {
  return new PageTransitionEvent(type, { persisted });
}

describe("internal/page-lifecycle", () => {
  beforeEach(() => {
    resetPageLifecycleForTests();
  });

  afterEach(() => {
    resetPageLifecycleForTests();

    vi.restoreAllMocks();
  });

  test("fires hidden once for repeated pagehide events", () => {
    const hidden = vi.fn<() => void>();

    onPageHidden(hidden);

    globalThis.dispatchEvent(pageTransition("pagehide"));

    globalThis.dispatchEvent(pageTransition("pagehide"));

    expect(hidden).toHaveBeenCalledTimes(1);
  });

  test("fires shown once when a hidden page is restored from the back-forward cache", () => {
    const shown = vi.fn<() => void>();

    onPageShown(shown);

    globalThis.dispatchEvent(pageTransition("pageshow"));

    expect(shown).not.toHaveBeenCalled();

    globalThis.dispatchEvent(pageTransition("pagehide"));

    globalThis.dispatchEvent(pageTransition("pageshow"));

    document.dispatchEvent(new Event("visibilitychange", { bubbles: true }));

    expect(shown).toHaveBeenCalledTimes(1);
  });

  test("fires unloading handlers on pagehide alongside the hidden transition", () => {
    const hidden = vi.fn<() => void>();
    const shown = vi.fn<() => void>();
    const unloading = vi.fn<() => void>();

    onPageHidden(hidden);

    onPageShown(shown);

    onPageUnloading(unloading);

    globalThis.dispatchEvent(pageTransition("pagehide"));

    expect(unloading).toHaveBeenCalledTimes(1);

    expect(hidden).toHaveBeenCalledTimes(1);

    expect(shown).not.toHaveBeenCalled();
  });

  test("stops delivering to an unsubscribed unloading handler", () => {
    const unloading = vi.fn<() => void>();

    onPageUnloading(unloading)();

    globalThis.dispatchEvent(pageTransition("pagehide"));

    expect(unloading).not.toHaveBeenCalled();
  });
});
