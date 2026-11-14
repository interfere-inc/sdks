import { sessionIdSchema } from "@interfere/types/data/session";
import { identifyParamsSchema } from "@interfere/types/sdk/identify";
import { HttpResponse, http } from "msw";
import { vi } from "vitest";
import { z } from "zod";

import { worker } from "../../test/msw.js";
import { resetPageLifecycleForTests } from "../internal/page-lifecycle/page-lifecycle.js";
import { SessionTracker } from "./api.js";
import { DeviceManager } from "./device.js";

const BASE_URL = "https://lifecycle.invalid";

export const FP_HASH = "fp-hash-lifecycle";

export const SOURCE = { name: "Audit", type: "custom" } as const;

export const createSessionBody = z.strictObject({
  deviceId: z.uuid().nullable(),
  fpHash: z.string().nullable(),
  sessionId: sessionIdSchema,
});

export const identifySessionBody = z.strictObject({
  ...createSessionBody.shape,
  ...identifyParamsSchema.shape,
});

const recordedBody = z.looseObject({
  deviceId: z.string().nullable().optional(),
  fpHash: z.string().nullable().optional(),
  identifier: z.string().optional(),
  sessionId: z.string().optional(),
});

type RecordedBody = z.infer<typeof recordedBody>;

const recorded: { path: string; body: RecordedBody }[] = [];

const waiters: { path: string; count: number; resolve: () => void }[] = [];

const trackers: SessionTracker[] = [];

const requestInits: RequestInit[] = [];

const identifyResponse = { status: 200 };

function bodies(path: string) {
  return recorded.flatMap((r) => (r.path === path ? [r.body] : []));
}

export function sessions() {
  return bodies("/v3/session");
}

export function identifies() {
  return bodies("/v3/session/identify");
}

function release() {
  for (const waiter of waiters.splice(0)) {
    if (bodies(waiter.path).length >= waiter.count) {
      waiter.resolve();
    } else {
      waiters.push(waiter);
    }
  }
}

async function arrived(path: string, count: number) {
  const { promise, resolve } = Promise.withResolvers<undefined>();

  waiters.push({
    count,
    path,
    resolve: () => {
      resolve(undefined);
    },
  });

  release();

  await promise;
}

/** Resolves once the ingest has received `count` session requests. */
export async function sessionsArrived(count: number) {
  await arrived("/v3/session", count);
}

/** Resolves once the ingest has received `count` identify requests. */
export async function identifiesArrived(count: number) {
  await arrived("/v3/session/identify", count);
}

export function inits() {
  return requestInits;
}

export function failIdentify(status: number) {
  identifyResponse.status = status;
}

export function readyDevice() {
  // oxlint-disable-next-line typescript/require-await -- The fingerprint must resolve on the first microtask so device start sends one backfill.
  return new DeviceManager(async () => FP_HASH);
}

export function pendingDevice() {
  const fingerprint = Promise.withResolvers<string | null>();

  return {
    device: new DeviceManager(async () => fingerprint.promise),
    release: (hash: string | null) => {
      fingerprint.resolve(hash);
    },
  };
}

export function makeTracker(device: DeviceManager = readyDevice()) {
  const tracker = new SessionTracker({
    device,
    fetcher: async (input, init) => {
      if (init) {
        requestInits.push(init);
      }

      return globalThis.fetch(input, init);
    },
    target: { baseUrl: BASE_URL, publicKeyHeader: { apiKey: "test-key" } },
  });

  trackers.push(tracker);

  return tracker;
}

export async function started(device?: DeviceManager) {
  const tracker = makeTracker(device);

  tracker.start();

  await sessionsArrived(1);

  return tracker;
}

/** Waits for `arrival`, giving up after one second so a known gap fails fast. */
export async function withinASecond(arrival: Promise<void>) {
  const { promise, resolve } = Promise.withResolvers<undefined>();

  const timer = setTimeout(() => {
    resolve(undefined);
  }, 1000);

  await Promise.race([arrival, promise]);

  clearTimeout(timer);
}

export async function settle() {
  const { promise, resolve } = Promise.withResolvers<undefined>();

  setTimeout(() => {
    resolve(undefined);
  }, 50);

  await promise;
}

export function setupLifecycle() {
  recordIngest(BASE_URL);
}

/** Records session and identify requests sent to `baseUrl` and resets the recorder. */
export function recordIngest(baseUrl: string) {
  recorded.length = 0;

  waiters.length = 0;

  requestInits.length = 0;

  identifyResponse.status = 200;

  worker.use(
    ...["/v3/session", "/v3/session/identify"].map((path) =>
      http.post(`${baseUrl}${path}`, async ({ request }) => {
        recorded.push({ path, body: recordedBody.parse(await request.json()) });

        release();

        const status = path === "/v3/session" ? 200 : identifyResponse.status;

        return HttpResponse.json({ ok: status === 200 }, { status });
      }),
    ),
  );
}

export async function teardownLifecycle() {
  for (const tracker of trackers.splice(0)) {
    tracker.dispose();
  }

  await settle();

  resetPageLifecycleForTests();

  vi.useRealTimers();

  await cookieStore.delete("interfere_did");
}
