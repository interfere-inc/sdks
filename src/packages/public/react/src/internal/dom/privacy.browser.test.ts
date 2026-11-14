import {
  DEFAULT_PRIVACY_POLICY,
  fieldLabel,
  privacyLevel,
  resolvePrivacyPolicy,
} from "./privacy.js";
import { expect, onTestFinished, test } from "vitest";

function render(html: string) {
  const host = document.createElement("div");

  host.innerHTML = html;

  document.body.append(host);

  onTestFinished(() => {
    host.remove();
  });

  return host;
}

test("blocks every target when a privacy selector is invalid", () => {
  const host = render(`<button>Pay</button>`);

  const policy = resolvePrivacyPolicy({ blockSelector: "[" });

  expect(privacyLevel(host, policy)).toBe("blocked");
});

test("labels a field from public aria-labelledby targets only", () => {
  const host = render(`
    <span id="card-label">Card</span>
    <span id="card-hint" class="interfere-mask">ends 4242</span>
    <div data-interfere-label=" ">
      <input aria-labelledby="card-label card-hint missing-label">
    </div>
  `);

  const input = host.querySelector("input");

  expect(input && fieldLabel(input, DEFAULT_PRIVACY_POLICY)).toBe("Card");
});
