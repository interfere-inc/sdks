import { HttpResponse, http } from "msw";
import { EventType } from "@rrweb/types";
import { nativeBrowserFixture } from "@interfere/test-config/native-browser";
import { replayUploads } from "../../../../test/fixtures/replay-uploads.js";
import { beforeEach, expect, test } from "vitest";

const createPage = nativeBrowserFixture(
  new URL("../../../../test/replay-visibility.fixture.ts", import.meta.url),
  [
    http.post("*/v3/session", () => HttpResponse.json({})),
    http.post("*/v3/replay/upload/:id", async ({ request }) => {
      await replayUploads.record(request);

      return HttpResponse.json({ objectKey: "replays/recorded.json" });
    }),
  ],
);

beforeEach(() => {
  replayUploads.reset();
});

test("native hidden events deliver the recorded tail uncompressed with keepalive after the snapshot left while the page was alive", async () => {
  const runtime = await createPage();

  await runtime.page.waitForFunction(() =>
    globalThis.replayVisibilityFixture.isRecording(),
  );

  await replayUploads.uploadsReached(1);

  expect(
    replayUploads.uploads[0]?.request.headers.get("content-encoding"),
  ).toBe("gzip");

  expect(replayUploads.uploads[0]?.events).toContainEqual(
    expect.objectContaining({ type: EventType.FullSnapshot }),
  );

  await runtime.page.evaluate(() =>
    globalThis.replayVisibilityFixture.addMarker("native-hide"),
  );

  await runtime.hide();

  await replayUploads.uploadsReached(2);

  expect(replayUploads.uploads).toHaveLength(2);

  const [, upload] = replayUploads.uploads;

  expect(upload?.request.headers.get("content-encoding")).toBeNull();

  expect(upload?.events).toContainEqual({
    type: EventType.Custom,
    data: { tag: "native-hide", payload: true },
    // oxlint-disable-next-line typescript/no-unsafe-assignment
    timestamp: expect.any(Number),
  });
});

test("native BFCache resume records a fresh snapshot after stopping while hidden", async () => {
  const runtime = await createPage();

  await runtime.page.waitForFunction(() =>
    globalThis.replayVisibilityFixture.isRecording(),
  );

  await runtime.page.evaluate(() =>
    globalThis.replayVisibilityFixture.firePageHideAndShow(),
  );

  await replayUploads.uploadsReached(1);

  await runtime.page.evaluate(() => {
    globalThis.replayVisibilityFixture.addMarker("before-pause");

    globalThis.replayVisibilityFixture.observeHidden();
  });

  await runtime.hide();

  await replayUploads.uploadsReached(2);

  expect(replayUploads.uploads).toHaveLength(2);

  await runtime.show();

  await runtime.page.waitForFunction(
    () =>
      globalThis.replayVisibilityFixture.resumes.length > 0 &&
      globalThis.replayVisibilityFixture.lifecycle.some(
        (entry) => entry.type === "pageshow" && entry.persisted,
      ),
  );

  await runtime.page.evaluate(() =>
    globalThis.replayVisibilityFixture.firePageHideAndShow(),
  );

  await replayUploads.uploadsReached(3);

  const restored = await runtime.page.evaluate(() => ({
    hiddenFaults: globalThis.replayVisibilityFixture.hiddenFaults,
    lifecycle: globalThis.replayVisibilityFixture.lifecycle,
  }));

  expect(restored.hiddenFaults).toStrictEqual([
    "please add custom event after start recording",
  ]);

  expect(restored.lifecycle).toContainEqual({
    type: "pageshow",
    persisted: true,
  });

  await runtime.page.evaluate(() =>
    globalThis.replayVisibilityFixture.addMarker("after-resume"),
  );

  await runtime.hide();

  await replayUploads.uploadsReached(4);

  expect(replayUploads.uploads).toHaveLength(4);

  const resumed = replayUploads.uploads.slice(2);

  expect(resumed[0]?.events).toContainEqual(
    expect.objectContaining({ type: EventType.FullSnapshot }),
  );

  const resumedText = resumed.map((upload) => upload.text).join("\n");

  expect(resumedText).toContain("changed-while-hidden");
  expect(resumedText).toContain("after-resume");
  expect(resumedText).not.toContain("hidden-event");
});

test("native visibility does not restart a recorder cancelled during initialization", async () => {
  const runtime = await createPage();

  await runtime.page.waitForFunction(() =>
    globalThis.replayVisibilityFixture.isRecording(),
  );

  await replayUploads.uploadsReached(1);

  await runtime.page.evaluate(() =>
    globalThis.replayVisibilityFixture.setupAndCancel(),
  );

  await runtime.hide();

  await runtime.show();

  const result = await runtime.page.evaluate(() => {
    try {
      globalThis.replayVisibilityFixture.addMarker("after-cancellation");

      return "recording";
    } catch (error) {
      return error instanceof Error ? error.message : String(error);
    }
  });

  expect(result).toBe("please add custom event after start recording");
  expect(replayUploads.requests).toHaveLength(1);
});

test("native pagehide flushes the recorded tail without stopping the recorder and never resends it", async () => {
  const runtime = await createPage();

  await runtime.page.waitForFunction(() =>
    globalThis.replayVisibilityFixture.isRecording(),
  );

  await replayUploads.uploadsReached(1);

  await runtime.page.evaluate(() => {
    globalThis.replayVisibilityFixture.addMarker("before-unload");

    globalThis.replayVisibilityFixture.firePageHideAndShow();
  });

  await replayUploads.uploadsReached(2);

  expect(
    replayUploads.uploads[1]?.request.headers.get("content-encoding"),
  ).toBeNull();

  expect(replayUploads.uploads[1]?.text).toContain("before-unload");

  await runtime.page.evaluate(() => {
    globalThis.replayVisibilityFixture.addMarker("navigation-cancelled");

    globalThis.replayVisibilityFixture.firePageHideAndShow();
  });

  await replayUploads.uploadsReached(3);

  expect(replayUploads.uploads[2]?.text).toContain("navigation-cancelled");

  expect(replayUploads.uploads[2]?.text).not.toContain("before-unload");
});
