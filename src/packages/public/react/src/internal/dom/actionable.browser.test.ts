import { describeActionable } from "./actionable.js";
import { DEFAULT_PRIVACY_POLICY } from "./privacy.js";
import { describe, expect, test } from "vitest";

describe(describeActionable, () => {
  test("omits missing and empty optional fields", () => {
    const element = document.createElement("button");

    element.setAttribute("name", "");

    element.setAttribute("aria-label", "");

    element.setAttribute("role", "");

    element.textContent = "   ";

    expect(describeActionable(element, DEFAULT_PRIVACY_POLICY)).toStrictEqual({
      tag: "button",
    });
  });

  test("extracts the supported DOM attributes without the target URL", () => {
    const element = document.createElement("a");

    element.id = "details";
    element.href = "/details?token=secret";

    element.setAttribute("name", "details-link");

    element.setAttribute("aria-label", "Read details");

    element.setAttribute("role", "link");

    element.dataset["custom"] = "not captured";

    element.textContent = "  Details  ";

    expect(describeActionable(element, DEFAULT_PRIVACY_POLICY)).toStrictEqual({
      tag: "a",
      id: "details",
      name: "details-link",
      ariaLabel: "Read details",
      role: "link",
      text: "Details",
    });
  });

  test("limits captured text to 120 characters", () => {
    const element = document.createElement("button");

    element.textContent = `  ${"x".repeat(121)}  `;

    expect(describeActionable(element, DEFAULT_PRIVACY_POLICY)).toStrictEqual({
      tag: "button",
      text: "x".repeat(120),
    });
  });
});
