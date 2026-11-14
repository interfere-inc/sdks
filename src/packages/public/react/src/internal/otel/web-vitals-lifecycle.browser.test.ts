import { onCLS } from "web-vitals";
import { expect, onTestFinished, test, vi } from "vitest";
import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { buildKernel } from "../../../test/fixtures/kernel.js";
import { resolveTargets } from "../config/config.js";
import { wireOtel } from "./wire.js";

vi.mock(import("web-vitals"), { spy: true });

test("page observers survive reinitialization without retaining a disposed kernel", async () => {
  const first = buildKernel({ pluginOverrides: DISABLE_ALL_PLUGINS }).kernel;

  wireOtel({
    kernel: first,
    releaseSlug: null,
    sdkStack: [],
    targets: resolveTargets("https://test.local"),
    tracing: { mode: "application" },
  });

  const report = vi.mocked(onCLS).mock.lastCall?.[0];

  expect(report).toBeDefined();

  const firstIdentity = vi.spyOn(first.session, "getId");

  await first.dispose();

  await first.dispose();

  const next = buildKernel({ pluginOverrides: DISABLE_ALL_PLUGINS }).kernel;

  onTestFinished(async () => next.dispose());

  wireOtel({
    kernel: next,
    releaseSlug: null,
    sdkStack: [],
    targets: resolveTargets("https://test.local"),
    tracing: { mode: "application" },
  });

  const nextIdentity = vi.spyOn(next.session, "getId");

  firstIdentity.mockClear();

  nextIdentity.mockClear();

  function record() {
    report?.({
      name: "CLS",
      value: 0.1,
      delta: 0.1,
      rating: "good",
      id: "bfcache",
      navigationType: "back-forward-cache",
      navigationId: 2,
      entries: [],
    });
  }

  record();

  expect(onCLS).toHaveBeenCalledTimes(1);
  expect(firstIdentity).not.toHaveBeenCalled();
  expect(nextIdentity).toHaveBeenCalledTimes(1);

  await next.dispose();

  nextIdentity.mockClear();

  record();

  expect(firstIdentity).not.toHaveBeenCalled();
  expect(nextIdentity).not.toHaveBeenCalled();
});
