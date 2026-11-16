import { describe, expect, test } from "vitest";

import type { ForeignStyle } from "../../../../test/foreign-writer.js";
import {
  baggageKeys,
  headerNames,
  installForeign,
  installInterfere,
  openRuntime,
  owner,
  probe,
} from "../../../../test/foreign-sdk.test-utils.js";

const KINDS = ["fetch", "fetch-request", "xhr"] as const;

const MODES = ["default"] as const;

const STYLES: { expected: string[]; name: string; styles: ForeignStyle[] }[] = [
  {
    expected: [
      "baggage",
      "traceparent",
      "tracestate",
      "x-vendor-parent-id",
      "x-vendor-sampling-priority",
      "x-vendor-trace-id",
    ],
    name: "w3c+vendor",
    styles: ["w3c", "vendor"],
  },
  {
    expected: ["baggage", "traceparent", "tracestate"],
    name: "w3c",
    styles: ["w3c"],
  },
  {
    expected: [
      "baggage",
      "x-vendor-parent-id",
      "x-vendor-sampling-priority",
      "x-vendor-trace-id",
    ],
    name: "vendor",
    styles: ["vendor"],
  },
  { expected: ["b3", "baggage"], name: "b3", styles: ["b3"] },
  {
    expected: ["baggage", "x-b3-sampled", "x-b3-spanid", "x-b3-traceid"],
    name: "b3multi",
    styles: ["b3multi"],
  },
];

const MERGED_BAGGAGE = [
  "interfere.device.id",
  "interfere.session.id",
  "session.id",
];

function withTraceparent(names: string[]) {
  return names.includes("traceparent")
    ? names
    : [...names, "traceparent"].toSorted();
}

describe.each(MODES)("%s mode beneath a foreign writer installed later", () => {
  test.each(STYLES)(
    "keeps a foreign W3C context, writes its own only when the writer carries none, and merges its session into the foreign baggage ($name)",
    async ({ expected, styles }) => {
      expect.hasAssertions();

      const runtime = await openRuntime();

      await installInterfere(runtime, {});

      await installForeign(runtime, { styles });

      const step = await probe(runtime, "outer-foreign", [...KINDS]);

      const foreignW3c = styles.includes("w3c");

      expect(step.results.map((result) => result.status)).toStrictEqual([
        200, 200, 200,
      ]);

      expect(step.wire).toHaveLength(KINDS.length);

      for (const entry of step.wire) {
        expect(headerNames(entry)).toStrictEqual(withTraceparent(expected));

        expect(owner(entry.headers["traceparent"] ?? null)).toBe(
          foreignW3c ? "foreign" : "interfere",
        );

        expect(baggageKeys(entry.headers["baggage"] ?? null)).toStrictEqual(
          MERGED_BAGGAGE,
        );
      }

      expect(step.observations).toHaveLength(KINDS.length);
    },
  );
});

const INNER = [["default", {}]] as const;

describe.each(INNER)(
  "%s with explicit propagation ownership above a foreign writer installed first",
  (_setup, options) => {
    test.each(STYLES)(
      "leaves W3C to an external owner and writes its own W3C context only when selected ($name)",
      async ({ expected, styles }) => {
        expect.hasAssertions();

        const runtime = await openRuntime();

        await installForeign(runtime, { styles });

        await installInterfere(runtime, {
          ...options,
          propagation: styles.includes("w3c") ? "external" : "interfere",
        });

        const step = await probe(runtime, "inner-foreign", [...KINDS]);

        const foreignW3c = styles.includes("w3c");

        expect(step.results.map((result) => result.status)).toStrictEqual([
          200, 200, 200,
        ]);

        expect(step.wire).toHaveLength(KINDS.length);

        for (const entry of step.wire) {
          expect(headerNames(entry)).toStrictEqual(withTraceparent(expected));

          expect(owner(entry.headers["traceparent"] ?? null)).toBe(
            foreignW3c ? "foreign" : "interfere",
          );

          expect(baggageKeys(entry.headers["baggage"] ?? null)).toStrictEqual(
            MERGED_BAGGAGE,
          );
        }

        expect(step.observations).toHaveLength(KINDS.length);
      },
    );
  },
);

test.each([undefined, "DD_RUM", "__SENTRY__"])(
  "auto ownership and headers are unchanged by vendor global %s",
  async (global) => {
    expect.hasAssertions();

    const runtime = await openRuntime();

    const diagnostics: string[] = [];

    runtime.page.on("console", (message) => {
      if (message.text().includes("owns tracing on this page")) {
        diagnostics.push(message.type());
      }
    });

    await installForeign(runtime, { global: global ?? "", styles: ["vendor"] });

    await installInterfere(runtime);

    const step = await probe(runtime, "global-neutral", [...KINDS]);

    expect(step.results.map((result) => result.status)).toStrictEqual([
      200, 200, 200,
    ]);

    expect(step.wire).toHaveLength(KINDS.length);

    for (const entry of step.wire) {
      expect(owner(entry.headers["traceparent"] ?? null)).toBe("interfere");

      expect(entry.headers["traceparent"]).toMatch(
        /^00-[a-f0-9]{32}-[a-f0-9]{16}-01$/u,
      );

      expect(baggageKeys(entry.headers["baggage"] ?? null)).toStrictEqual(
        MERGED_BAGGAGE,
      );
    }

    expect(step.observations).toHaveLength(KINDS.length);

    expect(
      step.observations.filter((entry) => entry.kind === "span"),
    ).toHaveLength(KINDS.length);

    expect(diagnostics).toStrictEqual([]);
  },
);

test("the default keeps an unsampled foreign context injected with flags 00", async () => {
  expect.hasAssertions();

  const runtime = await openRuntime();

  await installInterfere(runtime, {});

  await installForeign(runtime, {
    injectUnsampled: true,
    sampled: false,
    styles: ["w3c"],
  });

  const step = await probe(runtime, "unsampled");

  expect(step.wire).toHaveLength(2);

  for (const entry of step.wire) {
    expect(entry.headers["traceparent"]).toMatch(
      /^00-0000000000000000[\da-f]{16}-[\da-f]{16}-00$/u,
    );

    expect(entry.headers["tracestate"]).toBe("vendor=s:0");
  }

  expect(step.observations).toHaveLength(2);
});

test("the default alone writes its own trace context and baggage", async () => {
  expect.hasAssertions();

  const runtime = await openRuntime();

  await installInterfere(runtime, {});

  const step = await probe(runtime, "application-alone");

  expect(step.wire).toHaveLength(2);

  for (const entry of step.wire) {
    expect(headerNames(entry)).toStrictEqual(["baggage", "traceparent"]);

    expect(owner(entry.headers["traceparent"] ?? null)).toBe("interfere");

    expect(baggageKeys(entry.headers["baggage"] ?? null)).toStrictEqual([
      "interfere.device.id",
      "interfere.session.id",
    ]);
  }

  expect(step.observations).toHaveLength(2);
});

test("the default propagates W3C context and baggage to the page origin and nothing to a cross-site origin", async () => {
  expect.hasAssertions();

  const runtime = await openRuntime();

  await installInterfere(runtime);

  const same = await probe(runtime, "same-origin");

  expect(same.wire).toHaveLength(2);

  for (const entry of same.wire) {
    expect(owner(entry.headers["traceparent"] ?? null)).toBe("interfere");

    expect(entry.headers["baggage"]).not.toBeNull();
  }

  const cross = await probe(runtime, "cross-site", ["fetch", "xhr"], {
    target: await runtime.crossSite(),
  });

  expect(cross.wire).toHaveLength(2);

  for (const entry of cross.wire) {
    expect(headerNames(entry)).toStrictEqual([]);
  }

  expect(same.observations).toHaveLength(2);

  expect(cross.observations).toHaveLength(2);
});

test("the default never strips or rewrites a foreign writer's vendor headers", async () => {
  expect.hasAssertions();

  const runtime = await openRuntime();

  await installForeign(runtime, { styles: ["vendor"] });

  await installInterfere(runtime);

  const step = await probe(runtime, "vendor-intact");

  expect(step.wire).toHaveLength(2);

  for (const entry of step.wire) {
    expect(entry.headers["x-vendor-trace-id"]).toMatch(/^[\da-f]{16}$/u);

    expect(entry.headers["x-vendor-parent-id"]).toMatch(/^[\da-f]{16}$/u);

    expect(entry.headers["x-vendor-sampling-priority"]).toBe("1");

    expect(owner(entry.headers["traceparent"] ?? null)).toBe("interfere");
  }

  expect(step.foreignEvents).toHaveLength(2);

  expect(step.observations).toHaveLength(2);
});

test("the default carries interfere.session.id and interfere.device.id in baggage instead of the bare semconv keys", async () => {
  expect.hasAssertions();

  const runtime = await openRuntime();

  await installInterfere(runtime);

  const step = await probe(runtime, "baggage-keys");

  expect(step.wire).toHaveLength(2);

  for (const entry of step.wire) {
    expect(baggageKeys(entry.headers["baggage"] ?? null)).toStrictEqual([
      "interfere.device.id",
      "interfere.session.id",
    ]);
  }
});

test("the default preserves a foreign session.id baggage member next to its own namespaced keys", async () => {
  expect.hasAssertions();

  const runtime = await openRuntime();

  await installForeign(runtime, { styles: ["vendor"] });

  await installInterfere(runtime);

  const step = await probe(runtime, "baggage-merge");

  expect(step.wire).toHaveLength(2);

  for (const entry of step.wire) {
    expect(baggageKeys(entry.headers["baggage"] ?? null)).toStrictEqual([
      "interfere.device.id",
      "interfere.session.id",
      "session.id",
    ]);
  }
});
