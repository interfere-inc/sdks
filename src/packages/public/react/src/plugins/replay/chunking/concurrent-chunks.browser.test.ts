import { http } from "msw";
import { record } from "rrweb";
import { startReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import {
  replayUploadUrl,
  replayUploads,
} from "../../../../test/fixtures/replay-uploads.js";
import { worker } from "../../../../test/msw.js";
import { beforeEach, expect, onTestFinished, test } from "vitest";

const { uploads, requests } = replayUploads;

beforeEach(async () => {
  await startReplayHarness();
});

test("starts both split uploads before either collector response resolves", async () => {
  const first = Promise.withResolvers<Response>();
  const second = Promise.withResolvers<Response>();

  onTestFinished(() => {
    first.resolve(Response.json({}));

    second.resolve(Response.json({}));
  });

  worker.use(
    http.post(replayUploadUrl, async ({ request }) => {
      await replayUploads.record(request);

      return uploads.length === 1 ? first.promise : second.promise;
    }),
  );

  record.addCustomEvent("held-first", "x".repeat(3 * 1024 * 1024));

  record.addCustomEvent("held-second", "y".repeat(3 * 1024 * 1024));

  await replayUploads.uploadsReached(2);

  expect(uploads).toHaveLength(2);

  expect(requests).toHaveLength(2);

  expect(uploads.map(({ metadata }) => metadata)).toStrictEqual(
    expect.arrayContaining([
      expect.objectContaining({ segmentId: 0 }),
      expect.objectContaining({ segmentId: 1 }),
    ]),
  );

  expect(
    uploads.find(({ text }) => text.includes("held-first"))?.metadata,
  ).toStrictEqual(expect.objectContaining({ segmentId: 0 }));

  expect(
    uploads.find(({ text }) => text.includes("held-second"))?.metadata,
  ).toStrictEqual(expect.objectContaining({ segmentId: 1 }));

  first.resolve(Response.json({ objectKey: "replays/first.json" }));

  second.resolve(Response.json({ objectKey: "replays/second.json" }));
});
