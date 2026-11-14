import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { ExportResultCode } from "@opentelemetry/core";
import { browserStorage } from "@interfere/test-config/browser-storage";
import { InterfereError } from "@interfere/javascript";
import { toError } from "@interfere/types/sdk/errors";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, onTestFinished, test } from "vitest";
import { worker } from "../../../test/msw.js";
import {
  hidePage,
  resultCallback,
  showPage,
} from "../../../test/fixtures/export-collector.js";
import { isPageHidden } from "../page-lifecycle/page-lifecycle.js";
import {
  clearExportCarryover,
  withExportCarryover,
} from "./export-carryover.js";
import type { SignalSender } from "./exporter.js";

const prefix = "interfere:export:v1:";
const endpoint = "https://carryover.example.com/export";
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

function seed(
  key: string,
  body: string,
  scope = "key-one",
  created = Date.now(),
) {
  browserStorage.local.set(
    prefix + key,
    JSON.stringify({ body, scope, created }),
  );
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

test("prunes invalid and expired records while delivering only the matching key", async () => {
  browserStorage.local.set(`${prefix}broken`, "{");

  browserStorage.local.set(`${prefix}invalid`, JSON.stringify({ body: 4 }));

  seed("expired", "expired", "key-one", Date.now() - 25 * 60 * 60 * 1000);

  seed("other", "other key", "key-two");

  seed("ready", "ready");

  browserStorage.local.set("customer-preference", "kept");

  const received = Promise.withResolvers<string>();

  worker.use(
    http.post(endpoint, async ({ request }) => {
      received.resolve(await request.text());

      return new HttpResponse(null);
    }),
  );

  queue();

  await expect(received.promise).resolves.toBe("ready");

  await expect.poll(retained).toStrictEqual([`${prefix}other`]);

  expect(localStorage.getItem("customer-preference")).toBe("kept");
});

test.each(["count", "bytes"] as const)(
  "rejects new custody at the %s bound without evicting older records",
  async (bound) => {
    if (bound === "count") {
      for (let index = 0; index < 64; index += 1) {
        seed(String(index), "older", "other-key");
      }
    } else {
      seed("full", "x".repeat(1024 * 1024), "other-key");
    }

    const before = retained();
    const carryover = queue();

    hidePage();

    const result = resultCallback();

    carryover.send(encoder.encode("new"), result.callback);

    await expect(result.called).resolves.toMatchObject({
      code: 1,
      error: { message: "Telemetry carryover capacity exceeded" },
    });

    expect(retained()).toStrictEqual(before);
  },
);

test("retains transient failure custody and drains once when the page is shown", async () => {
  expect.hasAssertions();

  const bodies: string[] = [];

  worker.use(
    http.post(endpoint, async ({ request }) => {
      bodies.push(await request.text());

      return new HttpResponse(null, {
        status: bodies.length === 1 ? 503 : 200,
      });
    }),
  );

  const carryover = queue();

  hidePage();

  const result = resultCallback();

  carryover.send(encoder.encode("survives"), result.callback);

  await expect(result.called).resolves.toStrictEqual({ code: 0 });

  await navigator.locks.request(`${prefix}key-one`, () => {
    expect(retained()).toHaveLength(1);
  });

  showPage();

  await navigator.locks.request(`${prefix}key-one`, () => {
    expect(retained()).toHaveLength(0);
  });

  expect(bodies).toStrictEqual(["survives", "survives"]);
  expect(result.results).toHaveLength(1);
});

test("permanent rejection removes custody and reports the original status", async () => {
  worker.use(
    http.post(endpoint, () => new HttpResponse(null, { status: 400 })),
  );

  const carryover = queue();

  hidePage();

  const result = resultCallback();

  carryover.send(encoder.encode("invalid"), result.callback);

  await expect(result.called).resolves.toMatchObject({
    code: 1,
    error: { statusCode: 400 },
  });

  expect(retained()).toHaveLength(0);
});

test("revoking consent aborts an in-flight retained export", async () => {
  let allowed = true;
  const arrived = Promise.withResolvers<null>();
  const release = Promise.withResolvers<null>();

  worker.use(
    http.post(endpoint, async () => {
      arrived.resolve(null);

      await release.promise;

      return new HttpResponse(null);
    }),
  );

  onTestFinished(() => release.resolve(null));

  const carryover = queue("key-one", () => allowed);

  hidePage();

  const result = resultCallback();

  carryover.send(encoder.encode("private"), result.callback);

  await arrived.promise;

  allowed = false;

  clearExportCarryover();

  release.resolve(null);

  await expect(result.called).resolves.toMatchObject({
    code: 1,
    error: { message: "Telemetry carryover cleared" },
  });

  expect(retained()).toHaveLength(0);
});

test("disposal prevents a later page-show from draining retained records", async () => {
  expect.hasAssertions();

  const carryover = queue("key-one", () => false);

  carryover.dispose();

  seed("ready", "private");

  hidePage();

  showPage();

  await navigator.locks.request(`${prefix}key-one`, () => {
    expect(retained()).toStrictEqual([`${prefix}ready`]);
  });
});

test("reconciles custody acknowledged by another holder of the drain lock", async () => {
  const acquired = Promise.withResolvers<null>();
  const release = Promise.withResolvers<null>();

  const lock = navigator.locks.request(`${prefix}key-one`, async () => {
    acquired.resolve(null);

    await release.promise;
  });

  onTestFinished(() => release.resolve(null));

  await acquired.promise;

  const carryover = queue();

  hidePage();

  const result = resultCallback();

  carryover.send(encoder.encode("already delivered"), result.callback);

  const [key] = retained();

  expect(key).toBeDefined();

  browserStorage.local.remove(key ?? "");

  release.resolve(null);

  await lock;

  await expect(result.called).resolves.toStrictEqual({ code: 0 });

  expect(retained()).toHaveLength(0);
});

test("a payload queued during an active drain gets a later wakeup", async () => {
  const arrived = Promise.withResolvers<null>();
  const release = Promise.withResolvers<null>();
  const bodies: string[] = [];

  worker.use(
    http.post(endpoint, async ({ request }) => {
      bodies.push(await request.text());

      if (bodies.length === 1) {
        arrived.resolve(null);

        await release.promise;
      }

      return new HttpResponse(null);
    }),
  );

  onTestFinished(() => release.resolve(null));

  const carryover = queue();

  hidePage();

  const first = resultCallback();
  const second = resultCallback();

  carryover.send(encoder.encode("first"), first.callback);

  await arrived.promise;

  carryover.send(encoder.encode("second"), second.callback);

  release.resolve(null);

  await expect(first.called).resolves.toStrictEqual({ code: 0 });

  await expect(second.called).resolves.toStrictEqual({ code: 0 });

  expect(bodies).toStrictEqual(["first", "second"]);
  expect(retained()).toHaveLength(0);
});
