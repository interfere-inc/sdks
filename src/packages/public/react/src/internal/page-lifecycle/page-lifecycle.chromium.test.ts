import { nativeBrowserFixture } from "@interfere/test-config/native-browser";
import { expect, test } from "vitest";

const createPage = nativeBrowserFixture(
  new URL("../../../test/page-lifecycle.fixture.ts", import.meta.url),
);

test("delivers native hidden and shown notifications to their separate subscribers", async () => {
  const runtime = await createPage();

  await runtime.page.evaluate(() => {
    const fixture = globalThis.lifecycleFixture;

    fixture.onPageHidden(() => fixture.record("hidden"));

    fixture.onPageShown(() => fixture.record("shown"));
  });

  await runtime.hide();

  await runtime.show();

  const result = await runtime.page.evaluate(() => ({
    calls: globalThis.lifecycleFixture.calls,
    lifecycle: globalThis.lifecycleFixture.lifecycle,
  }));

  expect(result.calls).toStrictEqual([
    { name: "hidden", state: "visible" },
    { name: "shown", state: "visible" },
  ]);

  expect(result.lifecycle).toStrictEqual([
    { type: "pageshow", persisted: false },
    { type: "pagehide", persisted: true },
    { type: "pageshow", persisted: true },
  ]);
});

test.each([
  {
    scenario: "all subscribers",
    expected: ["first", "second"],
  },
  { scenario: "unsubscribe hidden", expected: ["kept"] },
  { scenario: "unsubscribe shown", expected: ["hidden"] },
  { scenario: "reset all", expected: [] },
  { scenario: "reset stale hidden", expected: ["fresh"] },
  { scenario: "reset stale shown", expected: ["fresh"] },
  {
    scenario: "reinstall once",
    expected: ["fresh", "other"],
  },
])(
  "preserves subscriptions through native navigation: $scenario",
  async ({ scenario, expected }) => {
    const runtime = await createPage();

    await runtime.page.evaluate((selected) => {
      const fixture = globalThis.lifecycleFixture;

      switch (selected) {
        case "all subscribers": {
          fixture.onPageHidden(() => fixture.record("first"));

          fixture.onPageHidden(() => fixture.record("second"));

          break;
        }
        case "unsubscribe hidden": {
          const unsubscribe = fixture.onPageHidden(() =>
            fixture.record("removed"),
          );

          fixture.onPageHidden(() => fixture.record("kept"));

          unsubscribe();

          break;
        }
        case "unsubscribe shown": {
          fixture.onPageHidden(() => fixture.record("hidden"));

          const unsubscribe = fixture.onPageShown(() =>
            fixture.record("shown"),
          );

          unsubscribe();

          break;
        }
        case "reset all": {
          fixture.onPageHidden(() => fixture.record("hidden"));

          fixture.onPageShown(() => fixture.record("shown"));

          fixture.reset();

          break;
        }
        case "reset stale hidden":
        case "reinstall once": {
          fixture.onPageHidden(() => fixture.record("stale"));

          fixture.reset();

          fixture.onPageHidden(() => fixture.record("fresh"));

          if (selected === "reinstall once") {
            fixture.onPageHidden(() => fixture.record("other"));
          }

          break;
        }
        case "reset stale shown": {
          fixture.onPageShown(() => fixture.record("stale"));

          fixture.reset();

          fixture.onPageShown(() => fixture.record("fresh"));

          break;
        }
        default: {
          throw new Error(`Unknown lifecycle scenario: ${selected}`);
        }
      }
    }, scenario);

    await runtime.hide();

    await runtime.show();

    await expect(
      runtime.page.evaluate(() =>
        globalThis.lifecycleFixture.calls.map((call) => call.name),
      ),
    ).resolves.toStrictEqual(expected);

    await expect(
      runtime.page.evaluate(() => globalThis.lifecycleFixture.lifecycle),
    ).resolves.toContainEqual({ type: "pageshow", persisted: true });
  },
);
