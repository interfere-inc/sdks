/* oxlint-disable unicorn/no-useless-promise-resolve-reject -- LogRecordProcessor requires promise-returning flush and shutdown. */
import { INTERFERE_BROWSER_EVENTS_LOGGER_NAME } from "@interfere/types/data/browser-event-loggers";
import {
  USER_ACTION_EVENT_NAME,
  isCorrelationId,
} from "@interfere/types/data/correlation-events";

import {
  InMemoryLogRecordExporter,
  LoggerProvider,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";

import { ViewIds, identityAttributes } from "../../tracking/view.js";
import { DEFAULT_PRIVACY_POLICY } from "../dom/privacy.js";
import { installUserActions } from "./user-actions.js";
import { afterEach, beforeEach, expect, test } from "vitest";
import { userEvent } from "vitest/browser";

const ACTION_ID = "interfere.action.id";

let exporter: InMemoryLogRecordExporter;
let views: ViewIds;
let enabled: boolean;
let dispose: (() => void) | null = null;
const nodes: Element[] = [];

function install() {
  const loggerProvider = new LoggerProvider({
    processors: [
      {
        forceFlush: async () => Promise.resolve(),
        onEmit: (record) => {
          record.setAttributes(
            identityAttributes({
              getActionId: () => views.actionId(),
              getViewId: () => views.viewId(),
            }),
          );
        },
        shutdown: async () => Promise.resolve(),
      },
      new SimpleLogRecordProcessor({ exporter }),
    ],
  });

  dispose = installUserActions({
    isEnabled: () => enabled,
    loggerProvider,
    privacyPolicy: DEFAULT_PRIVACY_POLICY,
    resolveRoute: () => "/test-route",
    views,
  });
}

function mount<T extends Element>(element: T) {
  document.body.append(element);

  nodes.push(element);

  return element;
}

function actions() {
  return exporter
    .getFinishedLogRecords()
    .filter((record) => record.eventName === USER_ACTION_EVENT_NAME);
}

beforeEach(() => {
  exporter = new InMemoryLogRecordExporter();

  views = new ViewIds();

  enabled = true;
});

afterEach(() => {
  dispose?.();

  dispose = null;

  for (const node of nodes.splice(0)) {
    node.remove();
  }
});

test("an admitted click emits one user action with ui attributes and its own action id", () => {
  install();

  const button = mount(document.createElement("button"));

  button.id = "save-btn";

  button.setAttribute("name", "save");

  button.setAttribute("aria-label", "Save changes");

  const icon = document.createElement("span");

  icon.textContent = "Save";

  button.append(icon);

  icon.click();

  const [action] = actions();

  expect(actions()).toHaveLength(1);

  expect(action?.instrumentationScope.name).toBe(
    INTERFERE_BROWSER_EVENTS_LOGGER_NAME,
  );

  expect(action?.attributes).toMatchObject({
    "http.route": "/test-route",
    "ui.event_type": "click",
    "ui.target.aria_label": "Save changes",
    "ui.target.id": "save-btn",
    "ui.target.name": "save",
    "ui.target.tag": "button",
    "ui.target.text": "Save",
    "interfere.view.id": views.viewId(),
  });

  expect(isCorrelationId(action?.attributes[ACTION_ID])).toBe(true);

  expect(action?.spanContext).toBeUndefined();
});

test("clicks on non-actionable elements, text fields and while disabled emit nothing", () => {
  install();

  mount(document.createElement("div")).dispatchEvent(
    new MouseEvent("click", { bubbles: true }),
  );

  mount(document.createElement("input")).click();

  enabled = false;

  mount(document.createElement("button")).click();

  expect(actions()).toStrictEqual([]);

  expect(views.actionId()).toBeNull();
});

test("the action id is current inside the page's handlers and cleared after the dispatch", () => {
  install();

  const seen: (string | null)[] = [];

  const button = mount(document.createElement("button"));

  button.addEventListener("click", () => {
    seen.push(views.actionId());
  });

  button.click();

  expect(seen).toStrictEqual([actions()[0]?.attributes[ACTION_ID]]);

  expect(views.actionId()).toBeNull();
});

test("a stopped async handler keeps the id only before its await, and code after the dispatch never sees it", async () => {
  install();

  const seen: (string | null)[] = [];
  const settled = Promise.withResolvers<undefined>();
  const button = mount(document.createElement("button"));

  const afterAwait = async () => {
    await Promise.resolve();

    seen.push(views.actionId());

    settled.resolve(undefined);
  };

  button.addEventListener("click", (event) => {
    event.stopPropagation();

    seen.push(views.actionId());

    void afterAwait();
  });

  button.click();

  seen.push(views.actionId());

  await settled.promise;

  expect(seen).toStrictEqual([actions()[0]?.attributes[ACTION_ID], null, null]);
});

test("a nested click restores the outer action for the rest of the outer handler", () => {
  install();

  const seen: (string | null)[] = [];
  const outer = mount(document.createElement("button"));
  const inner = mount(document.createElement("button"));

  inner.addEventListener("click", () => {
    seen.push(views.actionId());
  });

  outer.addEventListener("click", () => {
    seen.push(views.actionId());

    inner.click();

    seen.push(views.actionId());
  });

  outer.click();

  const [outerId, innerId] = actions().map((row) => row.attributes[ACTION_ID]);

  expect(seen).toStrictEqual([outerId, innerId, outerId]);
  expect(innerId).not.toBe(outerId);
});

test("a real pointer click scopes the handler and nothing after it", async () => {
  install();

  const seen: (string | null)[] = [];
  const button = mount(document.createElement("button"));

  button.textContent = "Native";

  button.addEventListener("click", () => {
    seen.push(views.actionId());

    queueMicrotask(() => seen.push(views.actionId()));
  });

  await userEvent.click(button);

  expect(seen).toStrictEqual([actions()[0]?.attributes[ACTION_ID], null]);
});

test("a form submit is an action and two clicks get distinct ids", () => {
  install();

  const form = mount(document.createElement("form"));

  form.addEventListener("submit", (event) => {
    event.preventDefault();
  });

  form.dispatchEvent(new Event("submit", { bubbles: true, cancelable: true }));

  const first = mount(document.createElement("button"));
  const second = mount(document.createElement("a"));

  second.setAttribute("href", "#");

  second.addEventListener("click", (event) => {
    event.preventDefault();
  });

  first.click();

  second.click();

  const rows = actions();

  expect(rows.map((row) => row.attributes["ui.event_type"])).toStrictEqual([
    "submit",
    "click",
    "click",
  ]);

  expect(new Set(rows.map((row) => row.attributes[ACTION_ID])).size).toBe(3);
});

test("dispose removes the listeners", () => {
  install();

  dispose?.();

  dispose = null;

  mount(document.createElement("button")).click();

  expect(actions()).toStrictEqual([]);
});
