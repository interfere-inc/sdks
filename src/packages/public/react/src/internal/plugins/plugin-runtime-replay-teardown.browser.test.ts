import { EventType } from "@rrweb/types";
import { record } from "rrweb";

import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { startRecordingPluginRuntime } from "../../../test/fixtures/plugin-runtime.test-utils.js";
import { pluginTransport } from "../../../test/fixtures/plugin-runtime-transport.js";
import type { ReadableSpan } from "@opentelemetry/sdk-trace";
import { expect, test, vi } from "vitest";

const rageClickSpans = (spans: readonly ReadableSpan[]) =>
  spans.filter((span) => span.name === "rage_click");

test.each([
  { cause: "provider", expectedRageClickSpans: 2 },
  { cause: "dispose", expectedRageClickSpans: 1 },
] as const)(
  "replay $cause teardown preserves the recorded tail policy",
  async ({ cause, expectedRageClickSpans }) => {
    const { runtime, exporter, target } = await startRecordingPluginRuntime({
      devicePlugin: false,
    });

    await expect.poll(() => record.mirror.getId(target)).toBeGreaterThan(-1);

    target.click();

    target.click();

    target.click();

    expect(rageClickSpans(exporter.getFinishedSpans())).toHaveLength(1);

    record.addCustomEvent("baseline", cause);

    await vi.advanceTimersByTimeAsync(10_000);

    await pluginTransport.uploadsReached(1);

    expect(pluginTransport.uploads).toHaveLength(1);

    expect(pluginTransport.uploads[0]?.events).toContainEqual(
      expect.objectContaining({
        type: EventType.Custom,
        data: { tag: "baseline", payload: cause },
      }),
    );

    record.addCustomEvent("buffered-tail", cause);

    expect(pluginTransport.uploads).toHaveLength(1);

    const actions = {
      provider: () =>
        runtime.applyProviderConfig({
          ...DISABLE_ALL_PLUGINS,
          rageClick: true,
          replay: false,
        }),
      dispose: async () => runtime.dispose(),
    };

    await actions[cause]();

    expect(() => record.addCustomEvent("after-stop", cause)).toThrow(
      "please add custom event after start recording",
    );

    await vi.advanceTimersByTimeAsync(10_000);

    await pluginTransport.uploadsReached(2);

    expect(pluginTransport.uploads).toHaveLength(2);

    const events = pluginTransport.uploads.flatMap((upload) => upload.events);

    expect(events).toContainEqual(
      expect.objectContaining({
        type: EventType.Custom,
        data: {
          tag: "buffered-tail",
          payload: cause,
        },
      }),
    );

    target.click();

    target.click();

    target.click();

    expect(rageClickSpans(exporter.getFinishedSpans())).toHaveLength(
      expectedRageClickSpans,
    );
  },
);
