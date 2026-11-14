import { createClickSpanGate } from "./click-span-gate.js";
import { describe, expect, test } from "vitest";

function element<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  setup?: (el: HTMLElementTagNameMap[K]) => void,
) {
  const el = document.createElement(tag);

  setup?.(el);

  document.body.append(el);

  return el;
}

describe(createClickSpanGate, () => {
  test("admits clicks on distinct actionable elements", () => {
    const admit = createClickSpanGate();

    expect(admit(element("button"))).toBe(true);
    expect(admit(element("a"))).toBe(true);
  });

  test("skips a repeat click on the same element within a second", () => {
    const admit = createClickSpanGate();

    const button = element("button");

    expect(admit(button)).toBe(true);
    expect(admit(button)).toBe(false);
  });

  test("skips focusing a text field but keeps checkboxes", () => {
    const admit = createClickSpanGate();

    expect(admit(element("input", (el) => (el.type = "email")))).toBe(false);
    expect(admit(element("textarea"))).toBe(false);
    expect(admit(element("input", (el) => (el.type = "checkbox")))).toBe(true);
  });

  test("skips the control click a label forwards", () => {
    const admit = createClickSpanGate();

    const checkbox = element("input", (el) => {
      el.type = "checkbox";
      el.id = "terms";
    });

    const label = element("label", (el) => {
      el.htmlFor = "terms";
    });

    expect(admit(label)).toBe(true);
    expect(admit(checkbox)).toBe(false);
  });
});
