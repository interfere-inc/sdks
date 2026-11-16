import { StackContextManager } from "@opentelemetry/sdk-trace-web";
import { USER_ACTION_EVENT_NAME } from "@interfere/types/data/correlation-events";

import { context, propagation, trace } from "@opentelemetry/api";
import {
  InMemoryLogRecordExporter,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { HttpResponse, http } from "msw";
import { expect, onTestFinished, test, vi } from "vitest";

import { worker } from "../../../test/msw.js";
import { DEFAULT_PRIVACY_POLICY } from "../dom/privacy.js";
import { ViewIds } from "../../tracking/view.js";
import { registerBundledInstrumentations } from "./instrumentations/instrumentations.js";
import { buildOtelProvider } from "./provider.js";
import { installUserActions } from "./user-actions.js";

const VIEW_ID = "interfere.view.id";
const ACTION_ID = "interfere.action.id";

test("a native click emits a user action, stamps synchronous work with its action id, and leaves long tasks as roots", async () => {
  expect.hasAssertions();

  worker.use(
    ...["drain/v1/traces", "drain/v1/logs", "drain/v1/metrics"].map((path) =>
      http.post(
        `https://sdk.invalid/${path}`,
        () => new HttpResponse(null, { status: 204 }),
      ),
    ),
  );

  context.disable();

  const spans = new InMemorySpanExporter();
  const logs = new InMemoryLogRecordExporter();
  const views = new ViewIds();

  const handle = buildOtelProvider({
    additionalLogRecordProcessors: [
      new SimpleLogRecordProcessor({ exporter: logs }),
    ],
    additionalSpanProcessors: [new SimpleSpanProcessor(spans)],
    clientOptions: {
      baseUrl: "https://sdk.invalid",
      publicKeyHeader: { apiKey: "test-key" },
    },
    getActionId: () => views.actionId(),
    getDeviceId: () => null,
    getSessionId: () => null,
    getViewId: () => views.viewId(),
    sdkStack: [],
  });

  context.setGlobalContextManager(new StackContextManager().enable());

  const instrumentations = registerBundledInstrumentations({
    ignoreUrls: [/sdk\.invalid/u],
    tracerProvider: handle.tracerProvider,
  });

  const disposeActions = installUserActions({
    isEnabled: () => true,
    loggerProvider: handle.loggerProvider,
    privacyPolicy: DEFAULT_PRIVACY_POLICY,
    views,
  });

  const button = document.createElement("button");

  button.textContent = "Run work";

  document.body.append(button);

  button.addEventListener("click", function performWork() {
    handle.tracerProvider.getTracer("app").startSpan("handler-work").end();

    const deadline = performance.now() + 80;

    do {
      document.body.getBoundingClientRect();
    } while (performance.now() < deadline);
  });

  onTestFinished(async () => {
    button.remove();

    disposeActions();

    instrumentations.dispose();

    context.disable();

    await handle.shutdown();

    context.disable();

    propagation.disable();

    trace.disable();
  });

  await new Promise<void>((resolve) => {
    setTimeout(() => {
      button.click();

      resolve();
    }, 0);
  });

  handle.tracerProvider.getTracer("app").startSpan("after-click").end();

  await vi.waitFor(() => {
    const finished = spans.getFinishedSpans();
    const work = finished.find((span) => span.name === "handler-work");
    const after = finished.find((span) => span.name === "after-click");
    const longTask = finished.find((span) => span.name === "longtask");

    const action = logs
      .getFinishedLogRecords()
      .find((record) => record.eventName === USER_ACTION_EVENT_NAME);

    expect(action?.attributes).toMatchObject({
      "ui.event_type": "click",
      "ui.target.tag": "button",
      [VIEW_ID]: views.viewId(),
    });

    expect(action?.attributes[ACTION_ID]).toBeTypeOf("string");

    expect(work?.attributes[ACTION_ID]).toBe(action?.attributes[ACTION_ID]);
    expect(after?.attributes[ACTION_ID]).toBeUndefined();

    expect(longTask).toBeDefined();
    expect(longTask?.parentSpanContext).toBeUndefined();
    expect(longTask?.attributes[VIEW_ID]).toBe(views.viewId());
    expect(work?.parentSpanContext).toBeUndefined();
  });
});
