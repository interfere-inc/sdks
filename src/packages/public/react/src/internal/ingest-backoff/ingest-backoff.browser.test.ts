import { answerIngestQueueQuery } from "../sw-queue-protocol.js";
import type {
  IngestQueueController,
  IngestQueueQuery,
} from "../sw-queue-protocol.js";
import { IngestBackoffError, fetchIngest } from "./ingest-backoff.js";
import { HttpResponse, http } from "msw";

import { worker } from "../../../test/msw.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

function ingestQueue(prefix: string) {
  return {
    postMessage(query: IngestQueueQuery, transfer: Transferable[]) {
      const [port] = transfer;

      if (port instanceof MessagePort) {
        answerIngestQueueQuery(query, port, (url) =>
          url.pathname.startsWith(prefix),
        );
      }
    },
  } satisfies IngestQueueController;
}

function trackedSend() {
  const sent: string[] = [];

  worker.use(
    http.post("*/sent/:label", ({ params }) => {
      sent.push(String(params["label"]));

      return new HttpResponse(null, { status: 202 });
    }),
  );

  return {
    sent,
    send: (label: string) => async () =>
      fetch(`/sent/${label}`, { method: "POST" }),
  };
}

async function backOff(controller: IngestQueueController | null) {
  await expect(
    fetchIngest(
      "/unqueued",
      async () => {
        await Promise.resolve();

        throw new TypeError("Failed to fetch");
      },
      controller,
    ),
  ).rejects.toThrow(TypeError);
}

test("a backoff still sends requests the controlling ingest queue accepts", async () => {
  const controller = ingestQueue("/api/interfere/");

  const { sent, send } = trackedSend();

  await fetchIngest("/api/interfere/traces", send("warm"), controller);

  await backOff(controller);

  await expect(
    fetchIngest("/api/interfere/traces", send("queued"), controller),
  ).resolves.toHaveProperty("status", 202);

  await expect(
    fetchIngest("/v3/session", send("unmatched"), controller),
  ).rejects.toThrow(IngestBackoffError);

  expect(sent).toStrictEqual(["warm", "queued"]);
});

test("a backoff gates every request when no worker or a silent worker controls the page", async () => {
  const silent: IngestQueueController = {
    postMessage: () => null,
  };

  const { sent, send } = trackedSend();

  await fetchIngest("/api/interfere/traces", send("warm"), silent);

  await backOff(silent);

  await expect(
    fetchIngest("/api/interfere/traces", send("silent"), silent),
  ).rejects.toThrow(IngestBackoffError);

  await expect(
    fetchIngest("/api/interfere/traces", send("uncontrolled"), null),
  ).rejects.toThrow(IngestBackoffError);

  expect(sent).toStrictEqual(["warm"]);
});

test("a backoff gates requests when posting to the controller fails", async () => {
  const detached: IngestQueueController = {
    postMessage() {
      throw new DOMException("detached", "DataCloneError");
    },
  };

  const { sent, send } = trackedSend();

  await fetchIngest("/api/interfere/traces", send("warm"), detached);

  await backOff(detached);

  await expect(
    fetchIngest("/api/interfere/traces", send("detached"), detached),
  ).rejects.toThrow(IngestBackoffError);

  expect(sent).toStrictEqual(["warm"]);
});

async function succeed() {
  await Promise.resolve();

  return new Response(null, { status: 200 });
}

async function serverError() {
  await Promise.resolve();

  return new Response(null, { status: 500 });
}

const failNetwork: () => Promise<Response> = async () => {
  await Promise.resolve();

  throw new TypeError("Failed to fetch");
};

async function attempt(send: () => Promise<Response>) {
  try {
    await fetchIngest("/api/interfere/traces", send, null);

    return "sent";
  } catch (error) {
    return error instanceof IngestBackoffError ? "skipped" : "failed";
  }
}

async function blockedFor() {
  const start = Date.now();

  for (let elapsed = 0; elapsed <= 700_000; elapsed += 1000) {
    vi.setSystemTime(start + elapsed);

    // oxlint-disable-next-line no-await-in-loop -- Each probe must observe the clock it just set before the next advance.
    if ((await attempt(failNetwork)) !== "skipped") {
      return elapsed;
    }
  }

  return Number.POSITIVE_INFINITY;
}

describe("B25-B26 backoff growth", () => {
  beforeEach(() => {
    vi.useFakeTimers({ toFake: ["Date"] });
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  test("backoff doubles from 5 s per consecutive failure and caps at 10 minutes (B25, B26)", async () => {
    await expect(attempt(failNetwork)).resolves.toBe("failed");

    const windows: number[] = [];

    for (let index = 0; index < 8; index += 1) {
      // oxlint-disable-next-line no-await-in-loop -- Each window starts from the failure that ended the previous one.
      windows.push(await blockedFor());
    }

    expect(windows).toStrictEqual([
      5000, 10_000, 20_000, 40_000, 80_000, 160_000, 320_000, 600_000,
    ]);
  });
});

describe("B27-B31 backoff reset and gating", () => {
  beforeEach(() => {
    vi.useFakeTimers({ toFake: ["Date"] });
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  test("any HTTP response, even a 500, clears the backoff (B27)", async () => {
    await expect(attempt(failNetwork)).resolves.toBe("failed");

    vi.setSystemTime(Date.now() + 5000);

    await expect(attempt(failNetwork)).resolves.toBe("failed");

    vi.setSystemTime(Date.now() + 10_000);

    await expect(attempt(serverError)).resolves.toBe("sent");

    await expect(attempt(failNetwork)).resolves.toBe("failed");

    await expect(blockedFor()).resolves.toBe(5000);
  });

  test("concurrent failures inside one window count as one (B28)", async () => {
    const results = await Promise.all([
      attempt(failNetwork),
      attempt(failNetwork),
      attempt(failNetwork),
    ]);

    expect(results).toStrictEqual(["failed", "failed", "failed"]);

    await expect(blockedFor()).resolves.toBe(5000);

    await expect(blockedFor()).resolves.toBe(10_000);
  });

  test("an abort or timeout does not trip the backoff (B30)", async () => {
    await expect(
      attempt(async () => {
        await Promise.resolve();

        throw new DOMException("The operation was aborted.", "AbortError");
      }),
    ).resolves.toBe("failed");

    await expect(attempt(succeed)).resolves.toBe("sent");
  });

  test("a request skipped by the backoff never calls send (B31)", async () => {
    await expect(attempt(failNetwork)).resolves.toBe("failed");

    let calls = 0;

    await expect(
      attempt(async () => {
        calls += 1;

        return succeed();
      }),
    ).resolves.toBe("skipped");

    expect(calls).toBe(0);
  });
});

describe("B35 unresponsive ingest queue", () => {
  afterEach(() => {
    vi.useRealTimers();
  });

  test("a worker that stops answering gates the request after 1 s (B35)", async () => {
    let answered = 0;

    const answersOnce = {
      postMessage(query: IngestQueueQuery, transfer: Transferable[]) {
        const [port] = transfer;

        if (answered === 0 && port instanceof MessagePort) {
          answered += 1;

          answerIngestQueueQuery(query, port, () => true);
        }
      },
    } satisfies IngestQueueController;

    const { sent, send } = trackedSend();

    await fetchIngest("/api/interfere/traces", send("warm"), answersOnce);

    await backOff(answersOnce);

    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });

    let settled = false;

    const gated = (async () => {
      try {
        return await fetchIngest(
          "/api/interfere/logs",
          send("silent"),
          answersOnce,
        );
      } finally {
        settled = true;
      }
    })();

    await vi.advanceTimersByTimeAsync(999);

    expect(settled).toBe(false);

    await vi.advanceTimersByTimeAsync(1);

    await expect(gated).rejects.toThrow(IngestBackoffError);

    expect(sent).toStrictEqual(["warm"]);
  });
});
