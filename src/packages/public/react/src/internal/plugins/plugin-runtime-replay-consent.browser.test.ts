import { EventType } from "@rrweb/types";
import { record } from "rrweb";

import { startRecordingPluginRuntime } from "../../../test/fixtures/plugin-runtime.test-utils.js";
import { pluginTransport } from "../../../test/fixtures/plugin-runtime-transport.js";
import { expect, test, vi } from "vitest";

test.each([
  { consent: false, expectedSpans: 1 },
  { consent: { analytics: true, replay: false }, expectedSpans: 2 },
] as const)(
  "replay revocation $consent discards its tail",
  async ({ consent, expectedSpans }) => {
    const { runtime, exporter, target } = await startRecordingPluginRuntime({
      devicePlugin: false,
    });

    await expect.poll(() => record.mirror.getId(target)).toBeGreaterThan(-1);

    target.click();

    target.click();

    target.click();

    expect(exporter.getFinishedSpans()).toHaveLength(1);

    record.addCustomEvent("baseline", "consent");

    await vi.advanceTimersByTimeAsync(10_000);

    await pluginTransport.uploadsReached(1);

    expect(pluginTransport.uploads).toHaveLength(1);

    expect(pluginTransport.uploads[0]?.events).toContainEqual(
      expect.objectContaining({
        type: EventType.Custom,
        data: { tag: "baseline", payload: "consent" },
      }),
    );

    record.addCustomEvent("buffered-tail", "consent");

    expect(pluginTransport.uploads).toHaveLength(1);

    runtime.setConsent(consent);

    expect(() => record.addCustomEvent("after-stop", "consent")).toThrow(
      "please add custom event after start recording",
    );

    target.click();

    target.click();

    target.click();

    expect(exporter.getFinishedSpans()).toHaveLength(expectedSpans);

    await vi.advanceTimersByTimeAsync(10_000);

    runtime.setConsent(true);

    await vi.waitFor(() => record.addCustomEvent("after-reenable", "consent"));

    await vi.advanceTimersByTimeAsync(10_000);

    await pluginTransport.uploadsReached(2);

    expect(pluginTransport.uploads).toHaveLength(2);

    const events = pluginTransport.uploads.flatMap((upload) => upload.events);

    expect(events).toContainEqual(
      expect.objectContaining({
        type: EventType.Custom,
        data: { tag: "after-reenable", payload: "consent" },
      }),
    );

    expect(events).not.toContainEqual(
      expect.objectContaining({
        type: EventType.Custom,
        data: { tag: "buffered-tail", payload: "consent" },
      }),
    );
  },
);
