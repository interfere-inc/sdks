import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { context, propagation, trace } from "@opentelemetry/api";
import {
  InMemoryLogRecordExporter,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace";

import { DISABLE_ALL_PLUGINS } from "../../test/fixtures/disable-all-plugins.js";
import { createKernel } from "./kernel.js";
import type { Kernel, KernelOptions } from "./kernel.js";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

import { worker } from "../../test/msw.js";

const API_HOST = "https://kernel.invalid";

const SECRET = "fixture-secret";

const WAIT_OPTS = { timeout: 5000 };

const PRIVACY = {
  blockSelector: "[data-sensitive], .sensitive",
  maskTextSelector: "[data-mask-text]",
};

let ownedKernel: Kernel | undefined;
let host: HTMLDivElement;
let priorUrl: string;
let priorState: unknown;

beforeEach(() => {
  priorUrl = location.href;

  priorState = history.state;

  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", "interfere_pk_test");

  trace.disable();

  context.disable();

  propagation.disable();

  host = document.createElement("div");

  document.body.append(host);
});

afterEach(async () => {
  try {
    await ownedKernel?.dispose();
  } finally {
    ownedKernel = undefined;

    host.remove();

    history.replaceState(priorState, "", priorUrl);

    trace.disable();

    context.disable();

    propagation.disable();
  }
});

async function startKernel(opts: KernelOptions) {
  const spans = new InMemorySpanExporter();
  const logs = new InMemoryLogRecordExporter();

  worker.use(
    http.get(`${API_HOST}/v3/config`, () =>
      HttpResponse.json({
        plugins: { ...DISABLE_ALL_PLUGINS, pageEvents: true, rageClick: true },
      }),
    ),
    http.all(`${API_HOST}/*`, () => new HttpResponse(null, { status: 204 })),
  );

  const kernel = await createKernel({
    onCreated: (created) => {
      created.consent.set(true);
    },
    opts: {
      ...opts,
      apiHost: API_HOST,
      _internalAdditionalSpanProcessors: [
        new SimpleSpanProcessor({ exporter: spans }),
      ],
      _internalAdditionalLogRecordProcessors: [
        new SimpleLogRecordProcessor({ exporter: logs }),
      ],
    },
  });

  ownedKernel = kernel;

  await vi.waitFor(() => {
    expect(navigationLogs(logs)).not.toHaveLength(0);
  }, WAIT_OPTS);

  return { kernel, logs, spans };
}

function navigationLogs(logs: InMemoryLogRecordExporter) {
  return logs
    .getFinishedLogRecords()
    .filter((record) => "browser.navigation.type" in record.attributes);
}

function interactions(logs: InMemoryLogRecordExporter) {
  return logs
    .getFinishedLogRecords()
    .filter((record) => record.eventName === "interfere.user_action");
}

function lastInteraction(logs: InMemoryLogRecordExporter) {
  return interactions(logs).at(-1);
}

function serialized(
  spans: InMemorySpanExporter,
  logs: InMemoryLogRecordExporter,
) {
  return JSON.stringify([
    ...spans
      .getFinishedSpans()
      .map(({ attributes, events, name }) => ({ attributes, events, name })),
    ...logs.getFinishedLogRecords().map(({ attributes }) => ({ attributes })),
  ]);
}

function render(html: string) {
  const fragment = document.createRange().createContextualFragment(html);

  for (const node of fragment.querySelectorAll(
    "button, a, input, label, select, form",
  )) {
    node.addEventListener(
      node instanceof HTMLFormElement ? "submit" : "click",
      (event) => {
        event.preventDefault();
      },
    );
  }

  host.append(fragment);
}

function fixture(selector: string) {
  const found = host.querySelector<HTMLElement>(selector);

  if (!found) {
    throw new Error(`Missing fixture element ${selector}`);
  }

  return found;
}

function rageClick(target: Element) {
  for (let index = 0; index < 3; index += 1) {
    target.dispatchEvent(
      new MouseEvent("click", { bubbles: true, clientX: 10, clientY: 10 }),
    );
  }
}

describe("interaction privacy", () => {
  test("applies init privacy selectors to clicks, submits and rage clicks from the first capture", async () => {
    expect.hasAssertions();

    const { kernel, logs, spans } = await startKernel({
      privacy: PRIVACY,
    });

    render(`
      <button id="public-pay" aria-label="Pay now">
        Pay <span data-sensitive>${SECRET}-descendant</span>
        <span data-mask-text>${SECRET}-masked-descendant</span>
      </button>
      <div data-sensitive id="${SECRET}-ancestor">
        <button id="${SECRET}-id" name="${SECRET}-name" aria-label="${SECRET}-label">${SECRET}-text</button>
      </div>
      <a class="sensitive" href="/account?q=${SECRET}" id="${SECRET}-link">${SECRET}-link-text</a>
      <button data-mask-text role="tab" id="${SECRET}-masked-id" aria-label="${SECRET}-masked-label">${SECRET}-masked-text</button>
      <label id="plan-label">Plan <select name="plan"><option>${SECRET}-option</option></select></label>
      <form data-sensitive id="${SECRET}-form"><button type="submit">${SECRET}-submit</button></form>
      <label for="account-number">Account number</label>
      <input type="checkbox" id="account-number" name="accountNumber" data-sensitive aria-label="${SECRET}-field-aria" placeholder="${SECRET}-placeholder">
      <div data-sensitive data-interfere-label="Security code">
        <input type="checkbox" id="code-1" aria-label="${SECRET}-digit">
      </div>
    `);

    fixture("#public-pay").click();

    await kernel.flush();

    expect(lastInteraction(logs)?.attributes).toMatchObject({
      "ui.event_type": "click",
      "ui.target.aria_label": "Pay now",
      "ui.target.id": "public-pay",
      "ui.target.tag": "button",
      "ui.target.text": "Pay",
    });

    expect(lastInteraction(logs)?.attributes).not.toHaveProperty(
      "ui.target.privacy",
    );

    fixture(`#${SECRET}-id`).click();

    expect(lastInteraction(logs)?.attributes).toMatchObject({
      "ui.target.privacy": "blocked",
      "ui.target.tag": "button",
    });

    fixture(".sensitive").click();

    fixture("[data-mask-text][role=tab]").click();

    expect(lastInteraction(logs)?.attributes).toMatchObject({
      "ui.target.privacy": "masked",
      "ui.target.role": "tab",
    });

    fixture("select").click();

    expect(lastInteraction(logs)?.attributes).toMatchObject({
      "ui.target.label": "Plan",
      "ui.target.name": "plan",
      "ui.target.tag": "select",
    });

    expect(lastInteraction(logs)?.attributes).not.toHaveProperty(
      "ui.target.text",
    );

    for (const [selector, label] of [
      ["#account-number", "Account number"],
      ["#code-1", "Security code"],
    ] as const) {
      const input = fixture(selector);

      if (input instanceof HTMLInputElement) {
        input.value = `${SECRET}-typed`;
      }

      input.click();

      expect(lastInteraction(logs)?.attributes).toMatchObject({
        "ui.event_type": "click",
        "ui.target.label": label,
        "ui.target.privacy": "blocked",
        "ui.target.tag": "input",
      });
    }

    fixture("#plan-label").click();

    expect(
      interactions(logs).find(
        (record) => record.attributes["ui.target.tag"] === "label",
      )?.attributes,
    ).toMatchObject({ "ui.target.text": "Plan" });

    host.querySelector("form")?.requestSubmit();

    expect(lastInteraction(logs)?.attributes).toMatchObject({
      "ui.event_type": "submit",
      "ui.target.privacy": "blocked",
      "ui.target.tag": "form",
    });

    await vi.waitFor(() => {
      rageClick(fixture(`#${SECRET}-id`));

      expect(
        spans.getFinishedSpans().findLast((span) => span.name === "rage_click")
          ?.attributes,
      ).toMatchObject({
        "ui.rage_click.selector": "button",
        "ui.rage_click.text": "",
        "ui.target.privacy": "blocked",
      });
    }, WAIT_OPTS);

    expect(serialized(spans, logs)).not.toContain(SECRET);
  });

  test("interactions: false drops user actions but keeps page views and document load", async () => {
    expect.hasAssertions();

    const { kernel, logs, spans } = await startKernel({});

    render(`<button id="go">Go</button>`);

    kernel.syncProviderConfig({
      plugins: { pageEvents: { interactions: false } },
    });

    fixture("#go").click();

    history.pushState({}, "", "/interactions-off");

    expect(interactions(logs)).toHaveLength(0);

    expect(navigationLogs(logs).at(-1)?.attributes["url.full"]).toBe(
      new URL("/interactions-off", location.origin).href,
    );

    expect(
      spans.getFinishedSpans().some((span) => span.name === "documentLoad"),
    ).toBe(true);

    kernel.syncProviderConfig({ plugins: { pageEvents: true } });

    fixture("#go").click();

    expect(interactions(logs)).not.toHaveLength(0);
  });
});
