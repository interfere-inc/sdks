import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { context, propagation, trace } from "@opentelemetry/api";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";
import { z } from "zod";

import { DISABLE_ALL_PLUGINS } from "../../test/fixtures/disable-all-plugins.js";
import { createError } from "../../test/fixtures/errors.js";
import { createCountSignal } from "../../test/fixtures/upload-signal.js";
import { worker } from "../../test/msw.js";
import { createWrapperSingleton } from "./wrapper-singleton.js";
import {
  decodedBodyJson,
  decodedBodyText,
} from "../../test/fixtures/request-body.js";

const API = "https://pending-consent.invalid";

const logBody = z.object({
  resourceLogs: z.array(
    z.object({
      scopeLogs: z.array(
        z.object({
          logRecords: z.array(z.object({ eventName: z.string() })),
        }),
      ),
    }),
  ),
});

const events: string[] = [];
const exceptions: string[] = [];
const exceptionReceived = createCountSignal(exceptions);

beforeEach(() => {
  events.length = 0;

  exceptions.length = 0;

  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", "interfere_pk_test");

  setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", true);

  worker.use(
    http.post(`${API}/v3/session`, () => HttpResponse.json({ ok: true })),
    http.post(`${API}/drain/v1/logs`, async ({ request }) => {
      const body = logBody.parse(await decodedBodyJson(request));

      events.push(
        ...body.resourceLogs.flatMap((resource) =>
          resource.scopeLogs.flatMap((scope) =>
            scope.logRecords.map((record) => record.eventName),
          ),
        ),
      );

      return new HttpResponse(null, { status: 204 });
    }),
    http.post(`${API}/drain/v1/traces`, async ({ request }) => {
      exceptions.push(await decodedBodyText(request));

      exceptionReceived.notify();

      return new HttpResponse(null, { status: 204 });
    }),
    http.post(
      `${API}/drain/v1/metrics`,
      () => new HttpResponse(null, { status: 204 }),
    ),
  );
});

afterEach(() => {
  trace.disable();

  context.disable();

  propagation.disable();

  setInjectedConfiguration("__INTERFERE_RELEASE_TOKEN__", undefined);

  setInjectedConfiguration("__INTERFERE_FORCE_ENABLE__", undefined);
});

describe.each([false, { webVitals: false }] as const)(
  "pending consent with tracing %j",
  (tracing) => {
    test.each([
      { timing: "before init", enabled: false, expected: [] },
      { timing: "during init", enabled: false, expected: [] },
      { timing: "after init", enabled: true, expected: ["browser.navigation"] },
    ])(
      "revocation $timing preserves necessary errors and gates page views",
      async ({ timing, enabled, expected }) => {
        expect.hasAssertions();

        const requested = Promise.withResolvers<undefined>();
        const response = Promise.withResolvers<undefined>();

        const wrapper = createWrapperSingleton({
          initEntryName: "test",
          producerVersion: "@interfere/test@0.0.0",
        });

        worker.use(
          http.get(`${API}/v3/config`, async () => {
            requested.resolve(undefined);

            await response.promise;

            return HttpResponse.json({
              plugins: {
                ...DISABLE_ALL_PLUGINS,
                errors: true,
                pageEvents: true,
              },
            });
          }),
        );

        wrapper.consent.set(
          timing === "before init" ? false : { replay: false },
        );

        const initializing = wrapper.init({
          apiHost: API,
          enabled: true,
          tracing,
        });

        try {
          await requested.promise;

          if (timing === "during init") {
            wrapper.consent.set(false);
          }

          globalThis.onerror?.(
            "startup-consent-error",
            "https://app.invalid/main.js",
            4,
            2,
            createError("TypeError", "startup-consent-error"),
          );

          response.resolve(undefined);

          const kernel = await initializing;

          expect(kernel).not.toBeNull();

          await vi.waitFor(() => {
            expect(kernel?.isPageEventsCaptureEnabled()).toBe(enabled);
          });

          if (timing === "after init") {
            wrapper.consent.set(false);
          }

          expect(wrapper.consent.get()).toStrictEqual({
            analytics: false,
            replay: false,
          });

          await wrapper.close();

          await exceptionReceived.reached(1);

          expect(events).toStrictEqual(expected);
          expect(exceptions.join("\n")).toContain("startup-consent-error");
        } finally {
          response.resolve(undefined);

          await initializing;

          await wrapper.close();
        }
      },
    );
  },
);
