import { createErrorsKernel } from "../../../test/fixtures/errors.test-utils.js";
import {
  isPageHidden,
  onPageHidden,
  onPageShown,
  onPageUnloading,
  resetPageLifecycleForTests,
} from "./page-lifecycle.js";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

function pageTransition(type: "pagehide" | "pageshow", persisted = true) {
  return new PageTransitionEvent(type, { persisted });
}

function captureReportedErrors() {
  const reported: unknown[] = [];

  const onError = (event: ErrorEvent) => {
    reported.push(event.error);

    event.preventDefault();
  };

  globalThis.addEventListener("error", onError);

  return {
    reported,
    stop: () => {
      globalThis.removeEventListener("error", onError);
    },
  };
}

beforeEach(() => {
  resetPageLifecycleForTests();
});

afterEach(() => {
  resetPageLifecycleForTests();

  vi.restoreAllMocks();
});

test.each([
  {
    event: "hidden",
    subscribe: onPageHidden,
    trigger: () => {
      globalThis.dispatchEvent(pageTransition("pagehide"));
    },
  },
  {
    event: "shown",
    subscribe: onPageShown,
    trigger: () => {
      globalThis.dispatchEvent(pageTransition("pagehide"));

      globalThis.dispatchEvent(pageTransition("pageshow"));
    },
  },
  {
    event: "unloading",
    subscribe: onPageUnloading,
    trigger: () => {
      globalThis.dispatchEvent(pageTransition("pagehide", false));
    },
  },
])(
  "a throwing $event subscriber reports nothing to the host and still reaches later subscribers",
  ({ subscribe, trigger }) => {
    const later = vi.fn<() => void>();

    subscribe(() => {
      throw new Error("subscriber failure");
    });

    subscribe(later);

    const errors = captureReportedErrors();

    try {
      trigger();
    } finally {
      errors.stop();
    }

    expect(errors.reported).toStrictEqual([]);

    expect(later).toHaveBeenCalledTimes(1);
  },
);

test("never registers an unload listener", () => {
  const added = vi.spyOn(globalThis, "addEventListener");

  onPageHidden(vi.fn<() => void>());

  onPageShown(vi.fn<() => void>());

  onPageUnloading(vi.fn<() => void>());

  const types = added.mock.calls.map(([type]) => type);

  expect(types).not.toContain("unload");
});

test("fires unloading handlers early on beforeunload without marking the page hidden, and on every pagehide", () => {
  const unloading = vi.fn<() => void>();
  const hidden = vi.fn<() => void>();

  onPageUnloading(unloading);

  onPageHidden(hidden);

  globalThis.dispatchEvent(new Event("beforeunload"));

  expect(unloading).toHaveBeenCalledTimes(1);

  expect(isPageHidden()).toBe(false);

  expect(hidden).not.toHaveBeenCalled();

  globalThis.dispatchEvent(pageTransition("pagehide"));

  globalThis.dispatchEvent(pageTransition("pageshow"));

  globalThis.dispatchEvent(pageTransition("pagehide"));

  expect(unloading).toHaveBeenCalledTimes(3);
});

test("unloading handlers run before the page is marked hidden, as they did at beforeunload", () => {
  const order: string[] = [];

  onPageHidden(() => {
    order.push("hidden");
  });

  onPageUnloading(() => {
    order.push(isPageHidden() ? "unloading-while-hidden" : "unloading");
  });

  globalThis.dispatchEvent(pageTransition("pagehide"));

  expect(order).toStrictEqual(["unloading", "hidden"]);
});

test("a persisted pageshow keeps the same session, window and view", () => {
  const { kernel } = createErrorsKernel();

  const before = {
    session: kernel.session.getId(),
    view: kernel.views.viewId(),
    window: kernel.session.getWindowId(),
  };

  globalThis.dispatchEvent(pageTransition("pagehide"));

  globalThis.dispatchEvent(pageTransition("pageshow"));

  expect({
    session: kernel.session.getId(),
    view: kernel.views.viewId(),
    window: kernel.session.getWindowId(),
  }).toStrictEqual(before);
});
