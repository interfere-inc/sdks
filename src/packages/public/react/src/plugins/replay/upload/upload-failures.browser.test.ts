import { HttpResponse, http } from "msw";
import { record } from "rrweb";
import { startReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import {
  replayUploadUrl,
  replayUploads,
} from "../../../../test/fixtures/replay-uploads.js";
import { worker } from "../../../../test/msw.js";
import { beforeEach, expect, test, vi } from "vitest";

const { uploads, requests } = replayUploads;

beforeEach(async () => {
  await startReplayHarness();
});

test.each([401, 403, 413])(
  "halts recording on fatal rejection %s",
  async (status) => {
    expect.hasAssertions();

    worker.use(
      http.post(replayUploadUrl, async ({ request }) => {
        await replayUploads.record(request);

        return HttpResponse.json({}, { status });
      }),
    );

    await vi.advanceTimersByTimeAsync(2000);

    await replayUploads.uploadsReached(1);

    await vi.waitFor(() =>
      expect(() => record.addCustomEvent("halted", true)).toThrow(
        "please add custom event after start recording",
      ),
    );

    await vi.advanceTimersByTimeAsync(30_000);

    expect(requests).toHaveLength(1);
  },
);

test("keeps recording after a segment conflict and uploads the following chunks", async () => {
  worker.use(
    http.post(
      replayUploadUrl,
      async ({ request }) => {
        await replayUploads.record(request);

        return HttpResponse.json(
          { code: "REPLAY_UPLOAD_CONFLICT" },
          { status: 409 },
        );
      },
      { once: true },
    ),
  );

  record.addCustomEvent("conflicting-chunk", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(1);

  record.addCustomEvent("after-conflict", 1);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(2);

  record.addCustomEvent("after-conflict", 2);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(3);

  expect(uploads.map(({ text }) => text)).toStrictEqual([
    expect.stringContaining("conflicting-chunk"),
    expect.stringContaining("after-conflict"),
    expect.stringContaining("after-conflict"),
  ]);
});

test("retries a failed chunk without changing its segment metadata", async () => {
  worker.use(
    http.post(
      replayUploadUrl,
      async ({ request }) => {
        await replayUploads.record(request);

        return HttpResponse.error();
      },
      { once: true },
    ),
  );

  record.addCustomEvent("failed-chunk", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(1);

  expect(uploads).toHaveLength(1);

  record.addCustomEvent("following-chunk", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(2);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(3);

  expect(uploads).toHaveLength(3);

  const retried = uploads
    .slice(1)
    .find(({ text }) => text.includes("failed-chunk"));

  expect(retried?.metadata).toStrictEqual(uploads[0]?.metadata);

  expect(
    uploads
      .slice(1)
      .map(({ text }) => text)
      .join(""),
  ).toContain("following-chunk");
});
