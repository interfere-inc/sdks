import { browserStorage } from "@interfere/test-config/browser-storage";
import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { context, propagation, trace } from "@opentelemetry/api";
import type {
  LogRecordProcessor,
  ReadableLogRecord,
} from "@opentelemetry/sdk-logs";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, test } from "vitest";

import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { worker } from "../../../test/msw.js";
import { createKernel } from "../kernel.js";
import type { Kernel } from "../kernel.js";

const API = "https://kernel-consent.invalid";

const BACKEND = `${location.origin}/consent-required/create`;

const PAGE_ID = /^page_[0-9a-f-]{36}$/u;

let ownedKernel: Kernel | undefined;

const baggageHeaders: (string | null)[] = [];

function recordLogs() {
  const records: ReadableLogRecord[] = [];

  const processor = {
    async forceFlush() {
      await Promise.resolve();
    },
    onEmit: (record) => {
      records.push(record);
    },
    async shutdown() {
      await Promise.resolve();
    },
  } satisfies LogRecordProcessor;

  return {
    processor,
    sessionStarts: () =>
      records.filter((record) => record.eventName === "session.start"),
  };
}

function sessionBaggage(header: string | null) {
  return (
    /interfere\.session\.id=(?<id>[^,;]+)/u.exec(header ?? "")?.groups?.[
      "id"
    ] ?? null
  );
}

function interfereStorageKeys() {
  return [...Object.keys(localStorage), ...Object.keys(sessionStorage)].filter(
    (key) => key.startsWith("interfere"),
  );
}

function consentManager(onChange: (granted: boolean) => void) {
  let measurement = false;
  const listeners = new Set<() => void>();

  const store = {
    getState: () => ({
      has: (category: string) => category === "measurement" && measurement,
    }),
    subscribe: (listener: () => void) => {
      listeners.add(listener);

      return () => listeners.delete(listener);
    },
  };

  const apply = () => {
    onChange(store.getState().has("measurement"));
  };

  apply();

  store.subscribe(apply);

  return {
    accept: () => {
      measurement = true;

      for (const listener of listeners) {
        listener();
      }
    },
  };
}

beforeEach(() => {
  ownedKernel = undefined;

  baggageHeaders.length = 0;

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

  setInjectedConfiguration("__INTERFERE_PLUGINS__", {
    ...DISABLE_ALL_PLUGINS,
    device: true,
    errors: true,
  });

  worker.use(
    http.get(`${API}/v3/config`, () => HttpResponse.json({ plugins: {} })),
    http.all(`${API}/*`, () => new HttpResponse(null, { status: 204 })),
    http.post(BACKEND, ({ request }) => {
      baggageHeaders.push(request.headers.get("baggage"));

      return HttpResponse.json({ ok: true });
    }),
  );
});

afterEach(async () => {
  try {
    await ownedKernel?.dispose();
  } finally {
    trace.disable();

    context.disable();

    propagation.disable();

    setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);
  }
});

test("a required-consent app correlates its first request before consent and links the session on grant", async () => {
  browserStorage.local.clear();

  browserStorage.session.clear();

  const logs = recordLogs();

  const kernel = await createKernel({
    opts: {
      apiHost: API,
      consent: "required",
      _internalAdditionalLogRecordProcessors: [logs.processor],
    },
  });

  ownedKernel = kernel;

  kernel.syncProviderConfig({});

  const manager = consentManager((granted) => {
    kernel.consent.set(granted);
  });

  await fetch(BACKEND, { method: "POST" });

  const pageId = kernel.session.getId();

  expect(pageId).toMatch(PAGE_ID);
  expect(sessionBaggage(baggageHeaders[0] ?? null)).toBe(pageId);
  expect(baggageHeaders[0]).not.toContain("interfere.device.id");
  expect(kernel.device.getDeviceId()).toBeNull();
  expect(interfereStorageKeys()).toStrictEqual([]);

  manager.accept();

  const sessionId = kernel.session.getId();

  expect(sessionId).not.toMatch(PAGE_ID);
  expect(localStorage.getItem("interfere:session_id")).toBe(sessionId);

  await fetch(BACKEND, { method: "POST" });

  expect(sessionBaggage(baggageHeaders[1] ?? null)).toBe(sessionId);

  expect(logs.sessionStarts().map((record) => record.attributes)).toStrictEqual(
    [
      expect.objectContaining({
        "session.id": sessionId,
        "session.previous_id": pageId,
      }),
    ],
  );
});

test("without a consent option the session is persisted from the start and no link is emitted", async () => {
  const logs = recordLogs();

  const kernel = await createKernel({
    opts: {
      apiHost: API,
      _internalAdditionalLogRecordProcessors: [logs.processor],
    },
  });

  ownedKernel = kernel;

  kernel.syncProviderConfig({});

  await fetch(BACKEND, { method: "POST" });

  const sessionId = kernel.session.getId();

  expect(sessionId).not.toMatch(PAGE_ID);
  expect(sessionBaggage(baggageHeaders[0] ?? null)).toBe(sessionId);
  expect(localStorage.getItem("interfere:session_id")).toBe(sessionId);

  expect(logs.sessionStarts()).toStrictEqual([]);
});
