/* oxlint-disable typescript/unbound-method -- Identity assertions compare history methods without invoking them unbound. */
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { isCorrelationId } from "@interfere/types/data/correlation-events";
import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";
import { context, propagation, trace } from "@opentelemetry/api";
import {
  InMemoryLogRecordExporter,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import { HttpResponse, http } from "msw";
import {
  afterEach,
  beforeEach,
  expect,
  onTestFinished,
  test,
  vi,
} from "vitest";
import { z } from "zod";

import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { worker } from "../../../test/msw.js";
import { createKernel } from "../kernel.js";
import type { Kernel } from "../kernel.js";
import { decodedBodyJson } from "../../../test/fixtures/request-body.js";

const API = "https://capture-only-identity.invalid";

const VIEW_ID = "interfere.view.id";

const ACTION_ID = "interfere.action.id";

const HANDLED = {
  mechanism: { handled: true, type: MECHANISM_TYPE.manual.capture },
} as const;

const keyValue = z.object({
  key: z.string(),
  value: z.record(z.string(), z.json()),
});

const span = z.object({
  attributes: z.array(keyValue),
  events: z.tuple([z.object({ attributes: z.array(keyValue) })]),
});

const exportBody = z.object({
  resourceSpans: z.tuple([
    z.object({
      scopeSpans: z.tuple([z.object({ spans: z.array(span) })]),
    }),
  ]),
});

let ownedKernel: Kernel | undefined;

let spans: z.infer<typeof span>[];

function identityOf(list: z.infer<typeof keyValue>[]) {
  const all = Object.fromEntries(
    list.map(({ key, value }) => [key, Object.values(value)[0]]),
  );

  return { action: all[ACTION_ID], view: all[VIEW_ID] };
}

function identities() {
  return spans.map((item) => ({
    event: identityOf(item.events[0].attributes),
    span: identityOf(item.attributes),
  }));
}

beforeEach(() => {
  ownedKernel = undefined;

  spans = [];

  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

  trace.disable();

  context.disable();

  propagation.disable();

  const { href } = globalThis.location;
  const state: unknown = globalThis.history.state;

  onTestFinished(() => {
    globalThis.history.replaceState(state, "", href);
  });

  worker.use(
    http.get(`${API}/v3/config`, () =>
      HttpResponse.json({
        plugins: { ...DISABLE_ALL_PLUGINS, errors: true, pageEvents: true },
      }),
    ),
    http.post(`${API}/drain/v1/traces`, async ({ request }) => {
      spans.push(
        ...exportBody.parse(await decodedBodyJson(request)).resourceSpans[0]
          .scopeSpans[0].spans,
      );

      return new HttpResponse(null, { status: 204 });
    }),
    http.all(`${API}/*`, () => new HttpResponse(null, { status: 204 })),
  );
});

afterEach(async () => {
  try {
    await ownedKernel?.dispose();
  } finally {
    trace.disable();

    context.disable();

    propagation.disable();
  }
});

test.each([false, {}] as const)(
  "records navigation during config loading with tracing=%s",
  async (tracing) => {
    expect.hasAssertions();

    const configRequested = Promise.withResolvers<undefined>();
    const configResponse = Promise.withResolvers<undefined>();
    const exporter = new InMemoryLogRecordExporter();

    worker.use(
      http.get(`${API}/v3/config`, async () => {
        configRequested.resolve(undefined);

        await configResponse.promise;

        return HttpResponse.json({
          plugins: { ...DISABLE_ALL_PLUGINS, pageEvents: true },
        });
      }),
      http.post(
        `${API}/drain/v1/traces`,
        () => new HttpResponse(null, { status: 204 }),
      ),
    );

    const initializing = createKernel({
      opts: {
        apiHost: API,
        tracing,
        _internalAdditionalLogRecordProcessors: [
          new SimpleLogRecordProcessor({ exporter }),
        ],
      },
    });

    await configRequested.promise;

    globalThis.history.pushState(null, "", "/config-pending");

    configResponse.resolve(undefined);

    const kernel = await initializing;

    ownedKernel = kernel;

    kernel.syncProviderConfig({});

    await vi.waitFor(() => {
      expect(kernel.isPageEventsCaptureEnabled()).toBe(true);
    });

    expect(kernel.views.current().trigger).toBe("push");

    const expected =
      tracing === false
        ? []
        : [
            {
              viewId: kernel.views.viewId(),
              trigger: "push",
            },
          ];

    expect(
      exporter.getFinishedLogRecords().map((record) => ({
        viewId: record.attributes[VIEW_ID],
        trigger: record.attributes["interfere.view.trigger"],
      })),
    ).toStrictEqual(expected);
  },
);

test("tracing: false starts a view per real URL change and restores history on dispose", async () => {
  const originalPush = globalThis.history.pushState;
  const originalReplace = globalThis.history.replaceState;

  const kernel = await createKernel({ opts: { apiHost: API, tracing: false } });

  ownedKernel = kernel;

  const initial = kernel.views.viewId();

  globalThis.history.pushState(null, "", "/capture-only/next");

  const pushed = kernel.views.viewId();

  globalThis.history.pushState(null, "", "/capture-only/next#tab");

  expect(kernel.views.viewId()).toBe(pushed);

  globalThis.history.replaceState(null, "", "/capture-only/other");

  const replaced = kernel.views.viewId();

  originalReplace.call(globalThis.history, null, "", "/capture-only/back");

  globalThis.dispatchEvent(new PopStateEvent("popstate"));

  const traversed = kernel.views.viewId();

  expect(new Set([initial, pushed, replaced, traversed]).size).toBe(4);

  await kernel.dispose();

  ownedKernel = undefined;

  expect(globalThis.history.pushState).toBe(originalPush);

  expect(globalThis.history.replaceState).toBe(originalReplace);

  globalThis.history.pushState(null, "", "/capture-only/after-dispose");

  expect(kernel.views.viewId()).toBe(traversed);
});

test("tracing: false stamps the capturing view and action on the exception span and its event", async () => {
  expect.hasAssertions();

  const kernel = await createKernel({ opts: { apiHost: API, tracing: false } });

  ownedKernel = kernel;

  kernel.syncProviderConfig({});

  await vi.waitFor(() => {
    expect(kernel.isPageEventsCaptureEnabled()).toBe(true);
  });

  const first = kernel.views.viewId();

  kernel.recordException(new Error("first view"), HANDLED);

  globalThis.history.pushState(null, "", "/capture-only/second");

  const second = kernel.views.viewId();

  const button = document.createElement("button");

  button.textContent = "Boom";

  document.body.append(button);

  onTestFinished(() => {
    button.remove();
  });

  let actionId: string | null = null;

  button.addEventListener("click", () => {
    actionId = kernel.views.actionId();

    kernel.recordException(new Error("inside click"), HANDLED);
  });

  button.click();

  kernel.recordException(new Error("after click"), HANDLED);

  await kernel.flush();

  expect(second).not.toBe(first);

  expect(isCorrelationId(actionId)).toBe(true);

  expect(identities()).toStrictEqual([
    {
      event: { action: undefined, view: first },
      span: { action: undefined, view: first },
    },
    {
      event: { action: actionId, view: second },
      span: { action: actionId, view: second },
    },
    {
      event: { action: undefined, view: second },
      span: { action: undefined, view: second },
    },
  ]);
});
