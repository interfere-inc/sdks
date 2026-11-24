import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { ExportResultCode } from "@opentelemetry/core";
import { browserStorage } from "@interfere/test-config/browser-storage";
import { InterfereError } from "@interfere/javascript";
import { toError } from "@interfere/types/sdk/errors";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, onTestFinished, test } from "vitest";
import { z } from "zod";
import { worker } from "../../../../test/msw.js";
import {
  hidePage,
  resultCallback,
  showPage,
} from "../../../../test/fixtures/export-collector.js";
import { isPageHidden } from "../../page-lifecycle/page-lifecycle.js";
import { clearExportCarryover, withExportCarryover } from "./carryover.js";
import type { SignalSender } from "../exporter.js";

const prefix = "interfere:export:v1:";
const endpoint = "https://carryover.example.com/deliver";
const encoder = new TextEncoder();

const transport: SignalSender = (payload, callback, signal) => {
  async function send() {
    try {
      const response = await fetch(
        endpoint,
        omitUndefined({
          method: "POST",
          body: new Uint8Array(payload ?? []),
          signal,
        }),
      );

      await response.arrayBuffer();

      callback(
        response.ok
          ? { code: ExportResultCode.SUCCESS }
          : {
              code: ExportResultCode.FAILED,
              error: new InterfereError({ statusCode: response.status }),
            },
      );

      return;
    } catch (error) {
      callback({ code: ExportResultCode.FAILED, error: toError(error) });

      return;
    }
  }

  void send();
};

function queue(scope = "key-one", allowed = () => true) {
  const carryover = withExportCarryover(scope, transport, allowed);

  onTestFinished(() => carryover.dispose());

  return carryover;
}

function retained() {
  return Array.from({ length: localStorage.length }, (_, index) =>
    localStorage.key(index),
  ).filter((key): key is string => key?.startsWith(prefix) ?? false);
}

beforeEach(() => {
  isPageHidden();

  clearExportCarryover();
});

afterEach(() => {
  clearExportCarryover();

  showPage();
});

test("deliver sends at once while another holder keeps the drain lock and its acknowledgement removes custody", async () => {
  const acquired = Promise.withResolvers<null>();
  const release = Promise.withResolvers<null>();
  const bodies: string[] = [];

  worker.use(
    http.post(endpoint, async ({ request }) => {
      bodies.push(await request.text());

      return new HttpResponse(null);
    }),
  );

  const lock = navigator.locks.request(`${prefix}key-one`, async () => {
    acquired.resolve(null);

    await release.promise;
  });

  onTestFinished(() => release.resolve(null));

  await acquired.promise;

  const carryover = queue();
  const result = resultCallback();

  carryover.deliver(encoder.encode("unloading"), result.callback);

  expect(retained()).toHaveLength(1);

  await expect(result.called).resolves.toStrictEqual({ code: 0 });

  expect(bodies).toStrictEqual(["unloading"]);
  expect(retained()).toHaveLength(0);

  release.resolve(null);

  await lock;
});

test("a drain skips an in-flight delivery, and a transient failure keeps custody for the next drain", async () => {
  const arrived = Promise.withResolvers<null>();
  const release = Promise.withResolvers<null>();
  const bodies: string[] = [];

  worker.use(
    http.post(endpoint, async ({ request }) => {
      bodies.push(await request.text());

      if (bodies.length === 1) {
        arrived.resolve(null);

        await release.promise;

        return new HttpResponse(null, { status: 503 });
      }

      return new HttpResponse(null);
    }),
  );

  onTestFinished(() => release.resolve(null));

  const carryover = queue();
  const result = resultCallback();

  carryover.deliver(encoder.encode("in flight"), result.callback);

  await arrived.promise;

  hidePage();

  showPage();

  await navigator.locks.request(`${prefix}key-one`, () => null);

  expect(bodies).toStrictEqual(["in flight"]);

  release.resolve(null);

  await expect(result.called).resolves.toStrictEqual({ code: 0 });

  expect(retained()).toHaveLength(1);

  hidePage();

  showPage();

  await expect.poll(retained).toHaveLength(0);

  expect(bodies).toStrictEqual(["in flight", "in flight"]);
  expect(result.results).toHaveLength(1);
});

test("deliver falls back to a direct send when consent forbids retention", async () => {
  const bodies: string[] = [];

  worker.use(
    http.post(endpoint, async ({ request }) => {
      bodies.push(await request.text());

      return new HttpResponse(null);
    }),
  );

  const carryover = queue("key-one", () => false);
  const result = resultCallback();

  carryover.deliver(encoder.encode("direct"), result.callback);

  await expect(result.called).resolves.toStrictEqual({ code: 0 });

  expect(bodies).toStrictEqual(["direct"]);
  expect(retained()).toHaveLength(0);
});

test("deliver at the custody bound reports capacity without sending", async () => {
  const bodies: string[] = [];

  worker.use(
    http.post(endpoint, async ({ request }) => {
      bodies.push(await request.text());

      return new HttpResponse(null);
    }),
  );

  browserStorage.local.set(
    `${prefix}full`,
    JSON.stringify({
      body: "x".repeat(1024 * 1024),
      scope: "other-key",
      created: Date.now(),
    }),
  );

  const carryover = queue();
  const result = resultCallback();

  carryover.deliver(encoder.encode("over"), result.callback);

  await expect(result.called).resolves.toMatchObject({
    code: 1,
    error: { message: "Telemetry carryover capacity exceeded" },
  });

  expect(bodies).toStrictEqual([]);
  expect(retained()).toStrictEqual([`${prefix}full`]);
});

function recording() {
  const sends: { body: string; carryoverId: string | undefined }[] = [];

  const sender: SignalSender = (payload, callback, _signal, retainedSend) => {
    sends.push({
      body: new TextDecoder().decode(payload),
      carryoverId: retainedSend?.id,
    });

    callback({ code: ExportResultCode.SUCCESS });
  };

  return { sends, sender };
}

function seedDispatched(key: string, body: string, dispatched: number) {
  browserStorage.local.set(
    prefix + key,
    JSON.stringify({ body, scope: "key-one", created: Date.now(), dispatched }),
  );
}

test("a keepalive send marks custody dispatched and carries the entry's stable carryover id, while a plain send leaves it unmarked", async () => {
  const sends: { id: string; dispatched: number | null }[] = [];
  const stored = z.object({ dispatched: z.number().optional() });
  let keepalive = true;

  const carryover = withExportCarryover(
    "key-one",
    (_payload, callback, _signal, retainedSend) => {
      if (keepalive) {
        retainedSend?.onKeepalive();
      }

      const id = retainedSend?.id ?? "";

      const entry = stored.parse(
        JSON.parse(localStorage.getItem(prefix + id) ?? "{}"),
      );

      sends.push({ id, dispatched: entry.dispatched ?? null });

      callback({ code: ExportResultCode.SUCCESS });
    },
    () => true,
  );

  onTestFinished(() => carryover.dispose());

  const first = resultCallback();

  carryover.deliver(encoder.encode("keepalive"), first.callback);

  await expect(first.called).resolves.toStrictEqual({ code: 0 });

  keepalive = false;

  const second = resultCallback();

  carryover.deliver(encoder.encode("too big for keepalive"), second.callback);

  await expect(second.called).resolves.toStrictEqual({ code: 0 });

  expect(sends.map(({ id }) => z.uuid().safeParse(id).success)).toStrictEqual([
    true,
    true,
  ]);

  expect(sends.map(({ dispatched }) => dispatched !== null)).toStrictEqual([
    true,
    false,
  ]);

  expect(retained()).toHaveLength(0);
});

test("a later page holds an entry another page dispatched moments ago instead of sending it twice", async () => {
  seedDispatched("recent", "maybe delivered", Date.now());

  const { sends, sender } = recording();
  const carryover = withExportCarryover("key-one", sender, () => true);

  onTestFinished(() => carryover.dispose());

  await new Promise((resolve) => {
    queueMicrotask(() => {
      resolve(null);
    });
  });

  await navigator.locks.request(`${prefix}key-one`, () => null);

  expect(sends).toStrictEqual([]);
  expect(retained()).toStrictEqual([`${prefix}recent`]);
});

test("a later page re-sends a stale dispatched entry with the same carryover id so the gateway can drop it", async () => {
  seedDispatched("stale", "maybe delivered", Date.now() - 10 * 60 * 1000);

  seed("fresh", "never sent");

  const { sends, sender } = recording();
  const carryover = withExportCarryover("key-one", sender, () => true);

  onTestFinished(() => carryover.dispose());

  await expect.poll(retained).toHaveLength(0);

  expect(sends).toStrictEqual([
    { body: "maybe delivered", carryoverId: "stale" },
    { body: "never sent", carryoverId: "fresh" },
  ]);
});

function seed(key: string, body: string) {
  browserStorage.local.set(
    prefix + key,
    JSON.stringify({ body, scope: "key-one", created: Date.now() + 1 }),
  );
}
