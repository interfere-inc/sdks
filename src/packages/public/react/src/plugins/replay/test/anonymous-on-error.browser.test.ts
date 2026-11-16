import { EventType } from "@rrweb/types";
import { HttpResponse, http } from "msw";
import { record } from "rrweb";
import { worker } from "../../../../test/msw.js";
import { startReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import { replayUploads } from "../../../../test/fixtures/replay-uploads.js";
import { beforeEach, expect, test, vi } from "vitest";
import { z } from "zod";

const { uploads, requests } = replayUploads;

const mechanism = { handled: false, type: "browser.onerror" } as const;

const SOURCE = { name: "Clerk", type: "clerk" } as const;

const sessionBodySchema = z.looseObject({
  replayTrigger: z.string().optional(),
  sessionId: z.string().nullish(),
});

const chunkMetaSchema = z.looseObject({ trigger: z.string().optional() });

const sessionCreates: z.infer<typeof sessionBodySchema>[] = [];

const identifies: z.infer<typeof sessionBodySchema>[] = [];

beforeEach(() => {
  sessionCreates.length = 0;

  identifies.length = 0;

  worker.use(
    http.post("https://replay.invalid/v3/session", async ({ request }) => {
      sessionCreates.push(sessionBodySchema.parse(await request.json()));

      return HttpResponse.json({});
    }),
    http.post(
      "https://replay.invalid/v3/session/identify",
      async ({ request }) => {
        identifies.push(sessionBodySchema.parse(await request.json()));

        return HttpResponse.json({});
      },
    ),
  );
});

function failure(message: string) {
  const error = new Error(message);

  error.stack = `Error: ${message}\n    at checkout (https://app.example/assets/checkout-a1b2.js:1:4821)`;

  return error;
}

function uploadedText() {
  return uploads.map((upload) => upload.text).join("\n");
}

test("an anonymous session on an on_error surface uploads nothing until an error", async () => {
  expect.hasAssertions();

  await startReplayHarness({ anonymousReplayPolicy: "on_error" });

  record.addCustomEvent("browsing", true);

  await vi.advanceTimersByTimeAsync(45_000);

  expect(requests).toHaveLength(0);

  expect(sessionCreates.some((body) => body.replayTrigger !== undefined)).toBe(
    false,
  );
});

test("an error flushes the buffered replay with the error trigger and admits the session", async () => {
  expect.hasAssertions();

  const harness = await startReplayHarness({
    anonymousReplayPolicy: "on_error",
  });

  record.addCustomEvent("before-error", true);

  await vi.advanceTimersByTimeAsync(15_000);

  expect(requests).toHaveLength(0);

  harness.kernel.recordException(failure("checkout failed"), { mechanism });

  await replayUploads.uploadsReached(1);

  const [first] = uploads;

  expect(first?.metadata).toMatchObject({ trigger: "error" });

  expect(first?.events.slice(0, 2)).toMatchObject([
    { type: EventType.Meta },
    { type: EventType.FullSnapshot },
  ]);

  expect(first?.text).toContain("before-error");

  await vi.waitFor(() => {
    expect(sessionCreates).toContainEqual(
      expect.objectContaining({
        replayTrigger: "error",
        sessionId: harness.kernel.getSessionId(),
      }),
    );
  });
});

test("after the error the session records continuously and every chunk carries the trigger", async () => {
  expect.hasAssertions();

  const harness = await startReplayHarness({
    anonymousReplayPolicy: "on_error",
  });

  harness.kernel.recordException(failure("checkout failed"), { mechanism });

  await replayUploads.uploadsReached(1);

  record.addCustomEvent("after-error", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(2);

  expect(uploads[1]?.text).toContain("after-error");

  expect(
    uploads.map((upload) => chunkMetaSchema.parse(upload.metadata).trigger),
  ).toStrictEqual(uploads.map(() => "error"));
});

test("the buffer keeps about the last minute, starting at a full snapshot", async () => {
  expect.hasAssertions();

  const harness = await startReplayHarness({
    anonymousReplayPolicy: "on_error",
    fakeDate: true,
  });

  record.addCustomEvent("stale", true);

  await vi.advanceTimersByTimeAsync(40_000);

  record.takeFullSnapshot(true);

  await vi.advanceTimersByTimeAsync(80_000);

  record.addCustomEvent("recent", true);

  const errorAt = Date.now();

  harness.kernel.recordException(failure("checkout failed"), { mechanism });

  await replayUploads.uploadsReached(1);

  const text = uploadedText();

  expect(text).toContain("recent");

  expect(text).not.toContain("stale");

  expect(uploads[0]?.events.slice(0, 2)).toMatchObject([
    { type: EventType.Meta },
    { type: EventType.FullSnapshot },
  ]);

  expect(uploads[0]?.metadata).toSatisfy(
    (meta: { firstTs: number }) =>
      meta.firstTs >= errorAt - 90_000 && meta.firstTs <= errorAt - 30_000,
  );
});

test("a failed request reported as an exception span triggers the flush", async () => {
  expect.hasAssertions();

  const harness = await startReplayHarness({
    anonymousReplayPolicy: "on_error",
  });

  record.addCustomEvent("before-500", true);

  harness.kernel.noteException();

  await replayUploads.uploadsReached(1);

  expect(uploads[0]?.metadata).toMatchObject({ trigger: "error" });

  expect(uploads[0]?.text).toContain("before-500");
});

test("closing the page before an error uploads nothing", async () => {
  expect.hasAssertions();

  await startReplayHarness({ anonymousReplayPolicy: "on_error" });

  record.addCustomEvent("bounced", true);

  globalThis.dispatchEvent(
    new PageTransitionEvent("pagehide", { persisted: false }),
  );

  await vi.advanceTimersByTimeAsync(20_000);

  expect(requests).toHaveLength(0);
});

test("identify switches an on_error session to normal recording without the trigger", async () => {
  expect.hasAssertions();

  const harness = await startReplayHarness({
    anonymousReplayPolicy: "on_error",
  });

  record.addCustomEvent("before-identify", true);

  await vi.advanceTimersByTimeAsync(15_000);

  expect(requests).toHaveLength(0);

  await harness.kernel.identity.set({ identifier: "user-1", source: SOURCE });

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(1);

  expect(identifies).toHaveLength(1);

  expect(uploads[0]?.events.slice(0, 2)).toMatchObject([
    { type: EventType.Meta },
    { type: EventType.FullSnapshot },
  ]);

  expect(uploads[0]?.text).toContain("before-identify");

  expect(uploads[0]?.metadata).not.toHaveProperty("trigger");
});

test("an anonymous session on a never surface uploads nothing, even after an error", async () => {
  expect.hasAssertions();

  const harness = await startReplayHarness({ anonymousReplayPolicy: "never" });

  harness.kernel.recordException(failure("checkout failed"), { mechanism });

  await vi.advanceTimersByTimeAsync(30_000);

  expect(requests).toHaveLength(0);

  expect(sessionCreates.some((body) => body.replayTrigger !== undefined)).toBe(
    false,
  );
});

test("identify on a never surface starts recording from a fresh full snapshot", async () => {
  expect.hasAssertions();

  const harness = await startReplayHarness({ anonymousReplayPolicy: "never" });

  await vi.advanceTimersByTimeAsync(15_000);

  await harness.kernel.identity.set({ identifier: "user-2", source: SOURCE });

  record.addCustomEvent("identified", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(2);

  expect(uploads[0]?.events.slice(0, 2)).toMatchObject([
    { type: EventType.Meta },
    { type: EventType.FullSnapshot },
  ]);

  expect(uploadedText()).toContain("identified");
});

test("a never surface does not start rrweb until the visitor is identified", async () => {
  expect.hasAssertions();

  const harness = await startReplayHarness({ anonymousReplayPolicy: "never" });

  expect(() => record.addCustomEvent("anonymous", true)).toThrow(
    "please add custom event after start recording",
  );

  await harness.kernel.identity.set({ identifier: "user-3", source: SOURCE });

  expect(() => record.addCustomEvent("identified", true)).not.toThrow();
});

test("a never policy that arrives with the remote config stops the pending recording", async () => {
  expect.hasAssertions();

  const harness = await startReplayHarness({ remoteConfig: "pending" });

  expect(() => record.addCustomEvent("pending", true)).not.toThrow();

  harness.kernel.anonymousReplayPolicy = "never";

  harness.runtime.applyRemoteConfigDefaults();

  expect(() => record.addCustomEvent("anonymous", true)).toThrow(
    "please add custom event after start recording",
  );

  await vi.advanceTimersByTimeAsync(10_000);

  expect(requests).toHaveLength(0);
});

test("an always surface records anonymous sessions without the trigger", async () => {
  expect.hasAssertions();

  await startReplayHarness({ anonymousReplayPolicy: "always" });

  record.addCustomEvent("always", true);

  await vi.advanceTimersByTimeAsync(2000);

  await replayUploads.uploadsReached(1);

  expect(uploads[0]?.metadata).not.toHaveProperty("trigger");
});
