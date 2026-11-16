import { HttpResponse, http } from "msw";
import { record } from "rrweb";
import { worker } from "../../../../test/msw.js";
import { startReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import { replayUploadUrl } from "../../../../test/fixtures/replay-uploads.js";
import { expect, onTestFinished, test, vi } from "vitest";

function isReplayUpload([input]: Parameters<typeof fetch>) {
  const url = input instanceof Request ? input.url : String(input);

  return url.includes("/v3/replay/upload/");
}

test("revoking consent aborts a replay upload in flight", async () => {
  expect.hasAssertions();

  const arrived = Promise.withResolvers<undefined>();
  const release = Promise.withResolvers<undefined>();

  onTestFinished(() => release.resolve(undefined));

  worker.use(
    http.post(replayUploadUrl, async () => {
      arrived.resolve(undefined);

      await release.promise;

      return HttpResponse.json({});
    }),
  );

  const fetchSpy = vi.spyOn(globalThis, "fetch");

  const replay = await startReplayHarness();

  record.addCustomEvent("before-revoke", true);

  await vi.advanceTimersByTimeAsync(2000);

  await arrived.promise;

  replay.cleanup?.("consent-revoked");

  replay.cleanup = undefined;

  const [, init] = fetchSpy.mock.calls.find(isReplayUpload) ?? [];

  expect(init?.signal?.aborted).toBe(true);
});
