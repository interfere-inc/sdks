import { record } from "rrweb";
import { toRecordOptions } from "../config.js";
import { startReplayRecorder } from "./recorder.js";
import { describe, expect, onTestFinished, test, vi } from "vitest";

vi.mock(import("rrweb"), { spy: true });

async function startAndGetErrorHandler(
  onFault: Parameters<typeof startReplayRecorder>[0]["onFault"],
) {
  const recorder = await startReplayRecorder({
    onEvent: vi.fn<Parameters<typeof startReplayRecorder>[0]["onEvent"]>(),
    onFault,
    onMutationStorm:
      vi.fn<Parameters<typeof startReplayRecorder>[0]["onMutationStorm"]>(),
    recordOptions: toRecordOptions({}),
  });

  onTestFinished(() => recorder?.stop());

  expect(recorder).not.toBeNull();

  const options = vi.mocked(record).mock.calls[0]?.[0];
  const handler = options?.errorHandler;

  if (!handler) {
    throw new Error(
      "test invariant: expected rrweb to receive an errorHandler",
    );
  }

  return handler;
}

describe(startReplayRecorder, () => {
  test("checks out every five minutes and samples canvas as WebP", async () => {
    await startAndGetErrorHandler(
      vi.fn<Parameters<typeof startReplayRecorder>[0]["onFault"]>(),
    );

    expect(vi.mocked(record).mock.calls[0]?.[0]).toMatchObject({
      checkoutEveryNms: 300_000,
      dataURLOptions: { type: "image/webp", quality: 0.6 },
      sampling: { canvas: 2 },
    });
  });

  test.each([null, undefined, "failed", 42, { code: "recorder-failed" }])(
    "forwards arbitrary recorder fault %s unchanged",
    async (fault) => {
      const onFault =
        vi.fn<Parameters<typeof startReplayRecorder>[0]["onFault"]>();

      const errorHandler = await startAndGetErrorHandler(onFault);

      expect(errorHandler(fault)).toBe(true);
      expect(onFault).toHaveBeenCalledExactlyOnceWith(fault);
    },
  );

  test("returns a handled result for a recorder TypeError", async () => {
    const errorHandler = await startAndGetErrorHandler(
      vi.fn<Parameters<typeof startReplayRecorder>[0]["onFault"]>(),
    );

    expect(errorHandler(new TypeError("can't access dead object"))).toBe(true);
  });

  test("reports every fault to the plugin", async () => {
    const onFault = vi.fn<() => void>();

    const errorHandler = await startAndGetErrorHandler(onFault);

    const error = new TypeError("can't access dead object");

    errorHandler(error);

    errorHandler(error);

    expect(onFault).toHaveBeenCalledTimes(2);
    expect(onFault).toHaveBeenCalledWith(error);
  });

  test("stopping releases the node mirror rrweb keeps between recordings", async () => {
    const recorder = await startReplayRecorder({
      onEvent: vi.fn<Parameters<typeof startReplayRecorder>[0]["onEvent"]>(),
      onFault: vi.fn<Parameters<typeof startReplayRecorder>[0]["onFault"]>(),
      onMutationStorm:
        vi.fn<Parameters<typeof startReplayRecorder>[0]["onMutationStorm"]>(),
      recordOptions: toRecordOptions({}),
    });

    onTestFinished(() => recorder?.stop());

    expect(record.mirror.getIds().length).toBeGreaterThan(0);

    recorder?.stop();

    expect(record.mirror.getIds()).toStrictEqual([]);
  });

  test("reads the checkout interval each time recording starts", async () => {
    let checkoutEveryNms: number | undefined = 30_000;

    const recorder = await startReplayRecorder({
      checkoutEveryNms: () => checkoutEveryNms,
      onEvent: vi.fn<Parameters<typeof startReplayRecorder>[0]["onEvent"]>(),
      onFault: vi.fn<Parameters<typeof startReplayRecorder>[0]["onFault"]>(),
      onMutationStorm:
        vi.fn<Parameters<typeof startReplayRecorder>[0]["onMutationStorm"]>(),
      recordOptions: toRecordOptions({}),
    });

    onTestFinished(() => recorder?.stop());

    recorder?.stop();

    checkoutEveryNms = undefined;

    recorder?.start();

    const intervals = vi
      .mocked(record)
      .mock.calls.map(([options]) => options?.checkoutEveryNms);

    expect(intervals.at(-2)).toBe(30_000);
    expect(intervals.at(-1)).not.toBe(30_000);
  });

  test("can be created without recording until started", async () => {
    vi.mocked(record).mockClear();

    const recorder = await startReplayRecorder({
      deferStart: true,
      onEvent: vi.fn<Parameters<typeof startReplayRecorder>[0]["onEvent"]>(),
      onFault: vi.fn<Parameters<typeof startReplayRecorder>[0]["onFault"]>(),
      onMutationStorm:
        vi.fn<Parameters<typeof startReplayRecorder>[0]["onMutationStorm"]>(),
      recordOptions: toRecordOptions({}),
    });

    onTestFinished(() => recorder?.stop());

    expect(recorder).not.toBeNull();
    expect(record).not.toHaveBeenCalled();

    expect(recorder?.start()).toBe(true);
    expect(record).toHaveBeenCalledTimes(1);
  });
});
