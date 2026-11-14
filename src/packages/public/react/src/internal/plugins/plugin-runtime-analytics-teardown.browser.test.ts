/* oxlint-disable interfere-structure/colocated-tests */
import { EventType } from "@rrweb/types";
import { record } from "rrweb";

import { startRecordingPluginRuntime } from "../../../test/fixtures/plugin-runtime.test-utils.js";
import { pluginTransport } from "../../../test/fixtures/plugin-runtime-transport.js";
import { DeviceManager } from "../../tracking/device.js";
import { expect, test, vi } from "vitest";

test("analytics consent teardown leaves replay recording", async () => {
  const cause = "consent";

  const { kernel, runtime, exporter, target } =
    await startRecordingPluginRuntime({
      device: new DeviceManager(),
      devicePlugin: true,
    });

  await expect.poll(() => record.mirror.getId(target)).toBeGreaterThan(-1);

  target.click();

  target.click();

  target.click();

  expect(exporter.getFinishedSpans()).toHaveLength(1);

  const deviceId = kernel.device.getDeviceId();

  expect(deviceId).toStrictEqual(expect.any(String));
  expect(localStorage.getItem("interfere:device_id")).toBe(deviceId);

  expect((await cookieStore.get("interfere_did"))?.value).toBe(deviceId);

  record.addCustomEvent("before-analytics-stop", cause);

  const actions = {
    consent: () => runtime.setConsent({ analytics: false, replay: true }),
    remote: () =>
      runtime.applyRemoteConfig({ rageClick: false, device: false }),
  };

  actions[cause]();

  expect(kernel.device.getDeviceId()).toBeNull();

  expect(localStorage.getItem("interfere:device_id")).toBeNull();

  expect((await cookieStore.get("interfere_did"))?.value ?? null).toBeNull();

  target.click();

  target.click();

  target.click();

  expect(exporter.getFinishedSpans()).toHaveLength(1);

  record.addCustomEvent("after-analytics-stop", cause);

  await vi.advanceTimersByTimeAsync(10_000);

  const expectedUploads = 1;

  await pluginTransport.uploadsReached(expectedUploads);

  expect(pluginTransport.uploads).toHaveLength(expectedUploads);

  expect(
    pluginTransport.uploads.flatMap((upload) => upload.events),
  ).toStrictEqual(
    expect.arrayContaining([
      expect.objectContaining({
        type: EventType.Custom,
        data: { tag: "before-analytics-stop", payload: cause },
      }),
      expect.objectContaining({
        type: EventType.Custom,
        data: { tag: "after-analytics-stop", payload: cause },
      }),
    ]),
  );
});

test("analytics remote teardown leaves replay recording", async () => {
  const cause = "remote";

  const { kernel, runtime, exporter, target } =
    await startRecordingPluginRuntime({
      device: new DeviceManager(),
      devicePlugin: true,
    });

  await expect.poll(() => record.mirror.getId(target)).toBeGreaterThan(-1);

  target.click();

  target.click();

  target.click();

  expect(exporter.getFinishedSpans()).toHaveLength(1);

  const deviceId = kernel.device.getDeviceId();

  expect(deviceId).toStrictEqual(expect.any(String));
  expect(localStorage.getItem("interfere:device_id")).toBe(deviceId);

  expect((await cookieStore.get("interfere_did"))?.value).toBe(deviceId);

  record.addCustomEvent("before-analytics-stop", cause);

  const actions = {
    consent: () => runtime.setConsent({ analytics: false, replay: true }),
    remote: () =>
      runtime.applyRemoteConfig({ rageClick: false, device: false }),
  };

  actions[cause]();

  expect(kernel.device.getDeviceId()).toBe(deviceId);

  expect(localStorage.getItem("interfere:device_id")).toBe(deviceId);

  expect((await cookieStore.get("interfere_did"))?.value ?? null).toBe(
    deviceId,
  );

  target.click();

  target.click();

  target.click();

  expect(exporter.getFinishedSpans()).toHaveLength(1);

  record.addCustomEvent("after-analytics-stop", cause);

  await vi.advanceTimersByTimeAsync(10_000);

  const expectedUploads = 2;

  await pluginTransport.uploadsReached(expectedUploads);

  expect(pluginTransport.uploads).toHaveLength(expectedUploads);

  expect(
    pluginTransport.uploads.flatMap((upload) => upload.events),
  ).toStrictEqual(
    expect.arrayContaining([
      expect.objectContaining({
        type: EventType.Custom,
        data: { tag: "before-analytics-stop", payload: cause },
      }),
      expect.objectContaining({
        type: EventType.Custom,
        data: { tag: "after-analytics-stop", payload: cause },
      }),
    ]),
  );
});
