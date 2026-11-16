import { EventType } from "@rrweb/types";
import { record } from "rrweb";
import { createReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import type { ReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import { DISABLE_ALL_PLUGINS } from "../../../../test/fixtures/disable-all-plugins.js";
import { replayUploads } from "../../../../test/fixtures/replay-uploads.js";
import replayPlugin from "../index.js";
import { beforeEach, expect, onTestFinished, test, vi } from "vitest";

let replay: ReplayHarness;

const { uploads } = replayUploads;

beforeEach(() => {
  replay = createReplayHarness();
});

test("does not subscribe or upload when native rrweb snapshot initialization fails", async () => {
  const subscribe = vi.spyOn(replay.kernel, "subscribePluginConfigState");

  const error = new Error("initial input masking failed");
  const maskInput = vi.fn<() => void>();

  replay.target.value = "initial input";

  replay.runtime.applyProviderConfig({
    ...DISABLE_ALL_PLUGINS,
    replay: {
      maskInputOptions: { text: true },
      maskInputFn: () => {
        maskInput();

        throw error;
      },
    },
  });

  replay.runtime.setConsent(false);

  onTestFinished(() => {
    const stop = record({ emit: vi.fn<() => void>() });

    stop?.();
  });

  replay.cleanup = replayPlugin.setup(replay.kernel);

  await vi.dynamicImportSettled();

  expect(maskInput).toHaveBeenCalledTimes(1);
  expect(subscribe).not.toHaveBeenCalled();

  await vi.advanceTimersByTimeAsync(10_000);

  expect(uploads).toStrictEqual([]);

  expect(() => record.addCustomEvent("unavailable", true)).toThrow(
    "please add custom event after start recording",
  );
});

test("stops after repeated native recorder faults and delivers its recorded tail", async () => {
  let faults = 0;

  replay.runtime.applyProviderConfig({
    ...DISABLE_ALL_PLUGINS,
    replay: {
      maskInputOptions: { text: true },
      maskInputFn: (text) => {
        if (text) {
          faults += 1;

          throw new TypeError("cannot read detached input");
        }

        return text;
      },
    },
  });

  replay.runtime.setConsent(false);

  replay.cleanup = replayPlugin.setup(replay.kernel);

  await expect
    .poll(() => record.mirror.getId(replay.target))
    .toBeGreaterThan(-1);

  await vi.dynamicImportSettled();

  record.addCustomEvent("before-recorder-fault", true);

  for (let i = 0; i < 25; i += 1) {
    replay.target.setAttribute("value", `fault-${i}`);

    replay.target.dispatchEvent(new Event("input", { bubbles: true }));
  }

  await replayUploads.uploadsReached(1);

  expect(uploads).toHaveLength(1);

  expect(faults).toBe(10);
  expect(uploads[0]?.text).toContain("before-recorder-fault");

  expect(uploads[0]?.events.at(-1)).toMatchObject({
    type: EventType.Custom,
    data: {
      tag: "interfere.replay-stopped",
      payload: { reason: "recorder-faults" },
    },
  });

  expect(() => record.addCustomEvent("after-recorder-fault", true)).toThrow(
    "please add custom event after start recording",
  );
});

test("keeps recording when the browser rejects the page's own stylesheet rules", async () => {
  replay.runtime.applyProviderConfig({ ...DISABLE_ALL_PLUGINS, replay: true });

  replay.runtime.setConsent(false);

  replay.cleanup = replayPlugin.setup(replay.kernel);

  await expect
    .poll(() => record.mirror.getId(replay.target))
    .toBeGreaterThan(-1);

  await vi.dynamicImportSettled();

  const style = document.createElement("style");

  document.head.append(style);

  onTestFinished(() => {
    style.remove();
  });

  const { sheet } = style;

  if (!sheet) {
    throw new Error("The fixture style element has no sheet");
  }

  for (let i = 0; i < 12; i += 1) {
    const pseudo =
      i % 2 === 0 ? "::-moz-placeholder" : ":-ms-input-placeholder";

    sheet.insertRule(`.field-${i}${pseudo}{color:red}`, sheet.cssRules.length);
  }

  expect(sheet.cssRules).toHaveLength(0);

  record.addCustomEvent("after-css-rejections", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(1);

  expect(uploads.at(-1)?.text).toContain("after-css-rejections");
});

const MUTATION_STORM_LONG_TASK_BUDGET_MS = 100;

async function startRecordingReplay() {
  replay.runtime.applyProviderConfig({ ...DISABLE_ALL_PLUGINS, replay: true });

  replay.runtime.setConsent(false);

  replay.cleanup = replayPlugin.setup(replay.kernel);

  await expect
    .poll(() => record.mirror.getId(replay.target))
    .toBeGreaterThan(-1);

  await vi.dynamicImportSettled();
}

async function mutationDelivered() {
  return new Promise<void>((resolve) => {
    const observer = new MutationObserver(() => {
      observer.disconnect();

      resolve();
    });

    observer.observe(document, {
      attributes: true,
      childList: true,
      subtree: true,
    });
  });
}

async function observerTaskMs(mutate: () => void) {
  const delivered = mutationDelivered();

  mutate();

  const mutated = performance.now();

  await delivered;

  return performance.now() - mutated;
}

function detachedNodes(count: number) {
  const host = document.createElement("div");

  host.innerHTML = '<span class="storm-node">x</span>'.repeat(count);

  onTestFinished(() => {
    host.remove();
  });

  return host;
}

test("halts replay within the long-task budget when one burst adds 100k nodes", async () => {
  await startRecordingReplay();

  record.addCustomEvent("before-node-burst", true);

  const burst = detachedNodes(100_000);

  const elapsed = await observerTaskMs(() => {
    document.body.append(burst);
  });

  expect(elapsed).toBeLessThan(MUTATION_STORM_LONG_TASK_BUDGET_MS);

  expect(() => record.addCustomEvent("after-node-burst", true)).toThrow(
    "please add custom event after start recording",
  );

  await replayUploads.uploadsReached(1);

  expect(uploads.at(-1)?.text).toContain("before-node-burst");
  expect(uploads.at(-1)?.text).not.toContain("storm-node");
});

test("halts replay when one delivery carries more than 10k mutations", async () => {
  await startRecordingReplay();

  record.addCustomEvent("before-mutation-storm", true);

  const elapsed = await observerTaskMs(() => {
    for (let i = 0; i <= 10_000; i += 1) {
      replay.target.dataset["storm"] = String(i);
    }
  });

  expect(elapsed).toBeLessThan(MUTATION_STORM_LONG_TASK_BUDGET_MS);

  expect(() => record.addCustomEvent("after-mutation-storm", true)).toThrow(
    "please add custom event after start recording",
  );

  await replayUploads.uploadsReached(1);

  expect(uploads.at(-1)?.text).toContain("before-mutation-storm");

  expect(uploads.at(-1)?.events.at(-1)).toMatchObject({
    type: EventType.Custom,
    data: {
      tag: "interfere.replay-stopped",
      payload: { reason: "mutation-storm" },
    },
  });
});

test("keeps recording an ordinary burst of a thousand mutations", async () => {
  await startRecordingReplay();

  const burst = detachedNodes(1000);

  const delivered = mutationDelivered();

  document.body.append(burst);

  await delivered;

  record.addCustomEvent("after-ordinary-burst", true);

  await vi.advanceTimersByTimeAsync(10_000);

  await replayUploads.uploadsReached(1);

  expect(uploads.at(-1)?.text).toContain("after-ordinary-burst");
});
