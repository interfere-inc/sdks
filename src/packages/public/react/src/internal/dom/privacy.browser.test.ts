import {
  DEFAULT_PRIVACY_POLICY,
  fieldLabel,
  privacyLevel,
  resolvePrivacyPolicy,
} from "./privacy.js";
import { setLogLevel } from "../../util/log.js";
import { expect, onTestFinished, test, vi } from "vitest";

function render(html: string) {
  const host = document.createElement("div");

  host.innerHTML = html;

  document.body.append(host);

  onTestFinished(() => {
    host.remove();
  });

  return host;
}

test("drops an invalid privacy selector and keeps the valid ones", () => {
  setLogLevel("warn");

  const warn = vi.spyOn(console, "warn");

  const host = render(`
    <button>Pay</button>
    <div data-sensitive>4242</div>
    <div data-mask>Ada</div>
  `);

  const policy = resolvePrivacyPolicy({
    blockSelector: "[data-sensitive], [",
    maskTextSelector: "[data-mask]",
  });

  const [button, sensitive, masked] = host.children;

  expect(
    [host, button, sensitive, masked].map(
      (element) => element && privacyLevel(element, policy),
    ),
  ).toStrictEqual(["public", "public", "blocked", "masked"]);

  expect(warn).toHaveBeenCalledTimes(1);
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
