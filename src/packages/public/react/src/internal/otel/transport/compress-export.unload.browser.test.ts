import { compressExportBody } from "./compress-export.js";
import { expect, test, vi } from "vitest";

const body = JSON.stringify({ payload: "x".repeat(4096) });

test("sends the body raw once the page starts unloading, then compresses again after a cancelled navigation", async () => {
  vi.useFakeTimers({ toFake: ["performance"] });

  try {
    await expect(compressExportBody(body)).resolves.toMatchObject({
      encoding: "gzip",
    });

    globalThis.dispatchEvent(new Event("beforeunload"));

    await expect(compressExportBody(body)).resolves.toMatchObject({
      encoding: null,
      body,
    });

    vi.advanceTimersByTime(10_000);

    await expect(compressExportBody(body)).resolves.toMatchObject({
      encoding: "gzip",
    });
  } finally {
    vi.useRealTimers();
  }
});

test("abandons a compression already running when the page starts unloading, so the export can leave with it", async () => {
  vi.useFakeTimers({ toFake: ["performance"] });

  try {
    vi.advanceTimersByTime(60_000);

    const large = JSON.stringify({ payload: "y".repeat(2_000_000) });

    const compressing = compressExportBody(large);

    globalThis.dispatchEvent(new Event("beforeunload"));

    await expect(compressing).resolves.toMatchObject({
      encoding: null,
      body: large,
    });
  } finally {
    vi.useRealTimers();
  }
});
