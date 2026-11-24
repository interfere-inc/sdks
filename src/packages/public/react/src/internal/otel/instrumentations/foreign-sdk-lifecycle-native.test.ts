import { describe, expect, test } from "vitest";

import {
  disposeInterfere,
  headerNames,
  installForeign,
  installInterfere,
  openRuntime,
  owner,
  probe,
} from "../../../../test/foreign/sdk.test-utils.js";
import type { Runtime } from "../../../../test/foreign/sdk.test-utils.js";

const ORDERS = [
  [
    "foreign writer installed first",
    async (runtime: Runtime) => {
      await installForeign(runtime);

      await installInterfere(runtime, {
        console: true,
        errors: true,
        navigation: true,
      });
    },
  ],
  [
    "Interfere installed first",
    async (runtime: Runtime) => {
      await installInterfere(runtime, {
        console: true,
        errors: true,
        navigation: true,
      });

      await installForeign(runtime);
    },
  ],
] as const;

describe.each(ORDERS)("%s", (_order, boot) => {
  test("disposing Interfere leaves the foreign fetch and XHR wrappers writing and emitting", async () => {
    expect.hasAssertions();

    const runtime = await openRuntime();

    await boot(runtime);

    await disposeInterfere(runtime);

    const step = await probe(runtime, "after-dispose");

    expect(step.wire).toHaveLength(2);

    for (const entry of step.wire) {
      expect(owner(entry.headers["traceparent"] ?? null)).toBe("foreign");
    }

    expect(step.foreignEvents).toHaveLength(2);

    expect(step.observations).toHaveLength(0);
  });

  test("both SDKs observe an uncaught error raised after both are installed", async () => {
    expect.hasAssertions();

    const runtime = await openRuntime();

    await boot(runtime);

    await runtime.page.evaluate(() => {
      globalThis.foreignSdkFixture.foreign.installOnError();

      globalThis.foreignSdkFixture.throwLater("shared uncaught error");
    });

    await runtime.page.waitForFunction(
      () => globalThis.foreignSdkFixture.foreign.errors().length > 0,
      undefined,
      { timeout: 1500 },
    );

    const seen = await runtime.page.evaluate(() => ({
      foreign: globalThis.foreignSdkFixture.foreign.errors(),
      interfere: globalThis.foreignSdkFixture.interfere.exceptions(),
    }));

    expect(seen.foreign).toStrictEqual(["shared uncaught error"]);

    expect(seen.interfere).toStrictEqual(["shared uncaught error"]);
  });
});

test("disposing Interfere keeps a foreign history wrapper that was installed later", async () => {
  expect.hasAssertions();

  const runtime = await openRuntime();

  await installInterfere(runtime, { navigation: true });

  await runtime.page.evaluate(() => {
    globalThis.foreignSdkFixture.foreign.installHistory();
  });

  await disposeInterfere(runtime);

  const views = await runtime.page.evaluate(() => {
    globalThis.foreignSdkFixture.pushState("/after-dispose");

    return globalThis.foreignSdkFixture.foreign.views();
  });

  expect(views).toStrictEqual(["/after-dispose"]);
});

test("disposing Interfere keeps a foreign console wrapper that was installed later", async () => {
  expect.hasAssertions();

  const runtime = await openRuntime();

  await installInterfere(runtime, { console: true });

  const during = await runtime.page.evaluate(() => {
    globalThis.foreignSdkFixture.foreign.installConsole();

    console.error("shared console error");

    return {
      foreign: [...globalThis.foreignSdkFixture.foreign.console()],
      interfere: [...globalThis.foreignSdkFixture.interfere.console()],
    };
  });

  expect(during.foreign).toStrictEqual(["shared console error"]);

  expect(during.interfere).toStrictEqual(["error:shared console error"]);

  await disposeInterfere(runtime);

  const after = await runtime.page.evaluate(() => {
    console.error("after dispose");

    return globalThis.foreignSdkFixture.foreign.console();
  });

  expect(after).toStrictEqual(["shared console error", "after dispose"]);
});

test("disposing Interfere keeps a foreign onerror handler that was installed later", async () => {
  expect.hasAssertions();

  const runtime = await openRuntime();

  await installInterfere(runtime, { errors: true });

  await runtime.page.evaluate(() => {
    globalThis.foreignSdkFixture.foreign.installOnError();
  });

  await disposeInterfere(runtime);

  await expect(
    runtime.page.evaluate(() =>
      globalThis.foreignSdkFixture.foreign.onErrorIsCurrent(),
    ),
  ).resolves.toBe(true);

  await runtime.page.evaluate(() => {
    globalThis.foreignSdkFixture.throwLater("after dispose");
  });

  await runtime.page.waitForFunction(
    () => globalThis.foreignSdkFixture.foreign.errors().length > 0,
    undefined,
    { timeout: 1500 },
  );

  await expect(
    runtime.page.evaluate(() => globalThis.foreignSdkFixture.foreign.errors()),
  ).resolves.toStrictEqual(["after dispose"]);
});

test("the default writes its own trace context until the foreign writer's asynchronous session starts, then keeps the foreign one", async () => {
  expect.hasAssertions();

  const runtime = await openRuntime();

  await installInterfere(runtime, {});

  await installForeign(runtime, { sessionDelayMs: 600 }, false);

  const early = await probe(runtime, "pre-session", ["fetch"]);

  expect(early.wire).toHaveLength(1);

  expect(early.wire.map((entry) => headerNames(entry))).toStrictEqual([
    ["baggage", "traceparent"],
  ]);

  expect(owner(early.wire[0]?.headers["traceparent"] ?? null)).toBe(
    "interfere",
  );

  const inflight = runtime.page.evaluate(async (base) => {
    const url = new URL("/slow", base);

    url.searchParams.set("case", "in-flight");

    url.searchParams.set("kind", "fetch");

    return globalThis.foreignSdkFixture.request({
      kind: "fetch",
      url: url.href,
    });
  }, runtime.origin);

  await runtime.page.waitForFunction(
    () => globalThis.foreignSdkFixture.foreign.hasSession(),
    undefined,
    { timeout: 2000 },
  );

  await expect(inflight).resolves.toMatchObject({ status: 200 });

  const late = await probe(runtime, "post-session", ["fetch"]);

  expect(owner(late.wire[0]?.headers["traceparent"] ?? null)).toBe("foreign");

  expect(early.observations).toHaveLength(1);

  expect(late.observations).toHaveLength(1);
});

const STATES = [
  {
    arrange: async (runtime: Runtime) => {
      await installForeign(runtime, { sessionDelayMs: 60_000 }, false);
    },
    name: "untracked session",
  },
  {
    arrange: async (runtime: Runtime) => {
      await installForeign(runtime);

      await runtime.page.evaluate(() => {
        globalThis.foreignSdkFixture.foreign.stopSession();
      });
    },
    name: "stopped session",
  },
  {
    arrange: async (runtime: Runtime) => {
      await installForeign(runtime);

      await runtime.page.evaluate(() => {
        globalThis.foreignSdkFixture.foreign.stop();
      });
    },
    name: "foreign writer stopped",
  },
] as const;

describe.each(STATES)("foreign writer in state: $name", ({ arrange }) => {
  test("records an Interfere request observation per request", async () => {
    expect.hasAssertions();

    const runtime = await openRuntime();

    await arrange(runtime);

    await installInterfere(runtime);

    const step = await probe(runtime, "observed");

    expect(step.results.map((result) => result.status)).toStrictEqual([
      200, 200,
    ]);

    expect(step.observations).toHaveLength(2);

    expect(step.foreignEvents).toHaveLength(0);
  });

  test("the default writes baggage and its own trace context while the foreign writer is silent", async () => {
    expect.hasAssertions();

    const runtime = await openRuntime();

    await arrange(runtime);

    await installInterfere(runtime, {});

    const step = await probe(runtime, "silent");

    expect(step.wire).toHaveLength(2);

    for (const entry of step.wire) {
      expect(headerNames(entry)).toStrictEqual(["baggage", "traceparent"]);

      expect(owner(entry.headers["traceparent"] ?? null)).toBe("interfere");
    }
  });
});

test("leaves a foreign SDK global untouched and keeps Interfere storage under its own prefix", async () => {
  expect.hasAssertions();

  const runtime = await openRuntime();

  await runtime.page.evaluate(() => {
    Object.assign(globalThis, { FOREIGN_SDK: { q: [] } });
  });

  await installInterfere(runtime, { errors: true });

  await probe(runtime, "storage", ["fetch"]);

  const snapshot = await runtime.page.evaluate(() => ({
    foreignGlobal: JSON.stringify(
      (globalThis as { FOREIGN_SDK?: unknown }).FOREIGN_SDK,
    ),
    storage: globalThis.foreignSdkFixture.storageKeys(),
  }));

  expect(snapshot.foreignGlobal).toBe('{"q":[]}');

  for (const key of [
    ...snapshot.storage.cookies,
    ...snapshot.storage.local,
    ...snapshot.storage.session,
  ]) {
    expect(key).toMatch(/^interfere[:_]/u);
  }
});
