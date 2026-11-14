import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { SeverityNumber } from "@opentelemetry/api-logs";
import { INTERFERE_BROWSER_EVENTS_LOGGER_NAME } from "@interfere/types/data/browser-event-loggers";
import { isCorrelationId } from "@interfere/types/data/correlation-events";
import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";
import { context, propagation, trace } from "@opentelemetry/api";
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
import type { Kernel, KernelOptions } from "../kernel.js";
import { decodedBodyJson } from "../../../test/fixtures/request-body.js";

const API = "https://capture-only-events.invalid";

const VIEW_ID = "interfere.view.id";

const ACTION_ID = "interfere.action.id";

const keyValue = z.object({
  key: z.string(),
  value: z.record(z.string(), z.json()),
});

const exportBody = z.object({
  resourceLogs: z.tuple([
    z.object({
      resource: z.object({ attributes: z.array(keyValue) }),
      scopeLogs: z.tuple([
        z.object({
          scope: z.object({ name: z.string() }),
          logRecords: z.array(
            z.object({
              timeUnixNano: z.string().regex(/^\d{19}$/u),
              observedTimeUnixNano: z.string().regex(/^\d{19}$/u),
              severityNumber: z.number(),
              eventName: z.string(),
              attributes: z.array(keyValue),
            }),
          ),
        }),
      ]),
    }),
  ]),
});

interface Post {
  body: z.infer<typeof exportBody>;
  keepalive: boolean;
  producer: string | null;
  sentAt: string | null;
}

let ownedKernel: Kernel | undefined;

let posts: Post[];

let traceExports: number;

function attrs(list: z.infer<typeof keyValue>[]) {
  return Object.fromEntries(
    list.map(({ key, value }) => [key, Object.values(value)[0]]),
  );
}

function records() {
  return posts
    .flatMap((post) => post.body.resourceLogs[0].scopeLogs[0].logRecords)
    .map((record) => ({
      eventName: record.eventName,
      attributes: attrs(record.attributes),
    }));
}

beforeEach(() => {
  ownedKernel = undefined;

  posts = [];

  traceExports = 0;

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
    http.post(`${API}/drain/v1/logs`, async ({ request }) => {
      posts.push({
        body: exportBody.parse(await decodedBodyJson(request)),
        keepalive: request.keepalive,
        producer: request.headers.get("x-interfere-producer-version"),
        sentAt: request.headers.get("x-interfere-sent-at"),
      });

      return new HttpResponse(null, { status: 204 });
    }),
    http.post(`${API}/drain/v1/traces`, () => {
      traceExports += 1;

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

async function startKernel(opts: Pick<KernelOptions, "consent">) {
  const kernel = await createKernel({
    opts: { ...opts, apiHost: API, tracing: false },
  });

  ownedKernel = kernel;

  return kernel;
}

interface Seen {
  actionId: string | null;
}

function clickButton(kernel: Kernel) {
  const button = document.createElement("button");

  button.textContent = "Go";

  document.body.append(button);

  onTestFinished(() => {
    button.remove();
  });

  const seen: Seen = { actionId: null };

  button.addEventListener("click", () => {
    seen.actionId = kernel.views.actionId();
  });

  button.click();

  return seen.actionId;
}

test("tracing: false exports one page view per view and one user action per click as identity-stamped log records", async () => {
  expect.hasAssertions();

  const kernel = await startKernel({ consent: true });

  await vi.waitFor(() => {
    expect(kernel.isPageEventsCaptureEnabled()).toBe(true);
  });

  const initial = kernel.views.viewId();
  const { href, origin, pathname } = globalThis.location;

  globalThis.history.pushState(null, "", "/capture-only-events/next");

  const pushed = kernel.views.viewId();

  const actionId = clickButton(kernel);

  kernel.recordLog({
    body: "console line",
    severityNumber: SeverityNumber.INFO,
    severityText: "INFO",
  });

  await kernel.flush();

  const session = kernel.session.getId();

  expect(pushed).not.toBe(initial);

  expect(isCorrelationId(actionId)).toBe(true);

  expect(traceExports).toBe(0);

  expect(posts.map((post) => post.keepalive)).not.toContain(false);

  expect(posts[0]?.producer).toMatch(/^@interfere\/react@/u);

  expect(posts[0]?.body.resourceLogs[0].scopeLogs[0].scope.name).toBe(
    INTERFERE_BROWSER_EVENTS_LOGGER_NAME,
  );

  expect(
    attrs(posts[0]?.body.resourceLogs[0].resource.attributes ?? []),
  ).toMatchObject({
    "telemetry.sdk.language": "webjs",
    "interfere.sdk.name": "@interfere/react",
  });

  expect(records()).toStrictEqual([
    {
      eventName: "browser.navigation",
      attributes: {
        "browser.navigation.type": "pageload",
        "interfere.view.trigger": "pageload",
        "session.id": session,
        "url.full": kernel.redactUrl(href),
        "url.path": pathname,
        [VIEW_ID]: initial,
      },
    },
    {
      eventName: "browser.navigation",
      attributes: {
        "browser.navigation.type": "push",
        "interfere.view.trigger": "push",
        "session.id": session,
        "url.full": `${origin}/capture-only-events/next`,
        "url.path": "/capture-only-events/next",
        [VIEW_ID]: pushed,
      },
    },
    {
      eventName: "interfere.user_action",
      attributes: {
        "session.id": session,
        "ui.event_type": "click",
        "ui.target.tag": "button",
        "ui.target.text": "Go",
        "url.full": `${origin}/capture-only-events/next`,
        "url.path": "/capture-only-events/next",
        [ACTION_ID]: actionId,
        [VIEW_ID]: pushed,
      },
    },
  ]);

  expect(JSON.stringify(records())).not.toContain("device.id");
});

test("tracing: false stamps each log export with its dispatch time", async () => {
  expect.hasAssertions();

  const startedAt = Date.now();

  const kernel = await startKernel({ consent: true });

  await vi.waitFor(() => {
    expect(kernel.isPageEventsCaptureEnabled()).toBe(true);
  });

  await kernel.flush();

  expect(posts).toHaveLength(1);

  const firstSentAt = Number(posts[0]?.sentAt);

  expect(firstSentAt).toBeGreaterThanOrEqual(startedAt);

  expect(firstSentAt).toBeLessThanOrEqual(Date.now());

  await vi.waitFor(() => {
    expect(Date.now()).toBeGreaterThan(firstSentAt);
  });

  const nextStartedAt = Date.now();

  clickButton(kernel);

  await kernel.flush();

  expect(posts).toHaveLength(2);

  expect(Number(posts[1]?.sentAt)).toBeGreaterThanOrEqual(nextStartedAt);

  expect(Number(posts[1]?.sentAt)).toBeLessThanOrEqual(Date.now());
});

test("tracing: false with consent denied exports no page view or user action", async () => {
  expect.hasAssertions();

  const kernel = await startKernel({ consent: false });

  globalThis.history.pushState(null, "", "/capture-only-events/denied");

  clickButton(kernel);

  kernel.recordException(new Error("fence"), {
    mechanism: { handled: true, type: MECHANISM_TYPE.manual.capture },
  });

  await kernel.flush();

  expect(kernel.isPageEventsCaptureEnabled()).toBe(false);

  expect(traceExports).toBe(1);

  expect(posts).toStrictEqual([]);
});
