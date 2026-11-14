// oxlint-disable-next-line import/no-unassigned-import
import "next/dist/server/node-environment-baseline.js";
import { omitUndefined } from "@interfere/helpers/omit-undefined";

import {
  ROOT_CONTEXT,
  TraceFlags,
  defaultTextMapSetter,
  trace,
} from "@opentelemetry/api";
import type { Context, TextMapPropagator } from "@opentelemetry/api";
import {
  CompositePropagator,
  TraceState,
  W3CBaggagePropagator,
  W3CTraceContextPropagator,
} from "@opentelemetry/core";
import { workUnitAsyncStorage } from "next/dist/server/app-render/work-unit-async-storage.external.js";
import { workAsyncStorage } from "next/dist/server/app-render/work-async-storage.external.js";
import { createWorkStore } from "next/dist/server/async-storage/work-store.js";
import { CloseController } from "next/dist/server/web/web-on-close.js";
import { describe, expect, onTestFinished, test } from "vitest";

import { CacheSafeTraceContextPropagator } from "./cache-safe-trace-context.js";
import { FreshnessStampingPropagator } from "./freshness-propagator.js";
import { managedPropagators } from "./managed-propagators.js";
import {
  RENDER_OPTIONS,
  createRequestStore,
} from "../../../../test/render-options.fixture.js";
import { isStaticGenerationRender } from "../static-generation.js";

const active = trace.setSpanContext(ROOT_CONTEXT, {
  spanId: "b7ad6b7169203331",
  traceId: "0af7651916cd43dd8448eb211c80319c",
  traceFlags: TraceFlags.SAMPLED,
  isRemote: true,
});

const traceparent = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01";

const clientTraceDataSetter = {
  set: (
    carrier: { key: string; value: string }[],
    key: string,
    value: string,
  ) => {
    carrier.push({ key, value });
  },
};

test.each([
  { scenario: "static", dynamic: false, expected: [] },
  {
    scenario: "dynamic",
    dynamic: true,
    expected: [{ key: "traceparent", value: traceparent }],
  },
])(
  "injects only cache-safe client metadata during $scenario renders",
  ({ dynamic, expected }) => {
    const close = new CloseController();

    const store = createWorkStore({
      page: "/account/page",
      buildId: "cache-safe-build",
      deploymentId: "cache-safe-deployment",
      previouslyRevalidatedTags: [],
      renderOpts: {
        ...RENDER_OPTIONS,
        isBuildTimePrerendering: !dynamic,
        onClose: (callback) => close.onClose(callback),
      },
    });

    const carrier: { key: string; value: string }[] = [];

    onTestFinished(() => close.dispatchClose());

    expect(store.isBuildTimePrerendering).toBe(!dynamic);

    workUnitAsyncStorage.run(createRequestStore(), () =>
      workAsyncStorage.run(store, () =>
        new CacheSafeTraceContextPropagator().inject(
          active,
          carrier,
          clientTraceDataSetter,
        ),
      ),
    );

    expect(carrier).toStrictEqual(expected);
    expect(workAsyncStorage.getStore()).toBeUndefined();
  },
);

test("injects outbound headers even while static HTML is generated", () => {
  const close = new CloseController();

  const store = createWorkStore({
    page: "/account/page",
    buildId: "cache-safe-build",
    deploymentId: "cache-safe-deployment",
    previouslyRevalidatedTags: [],
    renderOpts: {
      ...RENDER_OPTIONS,
      isBuildTimePrerendering: true,
      onClose: (callback) => close.onClose(callback),
    },
  });

  const carrier: Record<string, string> = {};

  onTestFinished(() => close.dispatchClose());

  expect(store.isBuildTimePrerendering).toBe(true);

  workAsyncStorage.run(store, () =>
    new CacheSafeTraceContextPropagator().inject(
      active,
      carrier,
      defaultTextMapSetter,
    ),
  );

  expect(carrier).toStrictEqual({ traceparent });
});

test("reports no static generation outside a Next render", () => {
  expect(workAsyncStorage.getStore()).toBeUndefined();
  expect(isStaticGenerationRender()).toBe(false);
});

test("omits visitor metadata before a render scope exists and during runtime ISR", () => {
  expect.hasAssertions();

  const close = new CloseController();

  const store = createWorkStore({
    page: "/account/page",
    buildId: "isr-build",
    deploymentId: "isr-deployment",
    previouslyRevalidatedTags: [],
    renderOpts: {
      ...RENDER_OPTIONS,
      onClose: (callback) => close.onClose(callback),
    },
  });

  onTestFinished(() => close.dispatchClose());

  const carrier: { key: string; value: string }[] = [];
  const propagator = new CacheSafeTraceContextPropagator();

  workAsyncStorage.run(store, () => {
    propagator.inject(active, carrier, clientTraceDataSetter);

    expect(carrier).toStrictEqual([]);

    workUnitAsyncStorage.run(
      {
        type: "prerender-legacy",
        phase: "render",
        implicitTags: { tags: [], expirationsByCacheKind: new Map() },
        rootParams: {},
        revalidate: 60,
        expire: 3600,
        stale: 300,
        tags: null,
      },
      () => propagator.inject(active, carrier, clientTraceDataSetter),
    );

    expect(carrier).toStrictEqual([]);

    workUnitAsyncStorage.run(createRequestStore(), () =>
      propagator.inject(active, carrier, clientTraceDataSetter),
    );

    expect(carrier).toStrictEqual([{ key: "traceparent", value: traceparent }]);
  });
});

test.each([true, false])(
  "supports older Next work stores with isStaticGeneration=%s",
  (isStaticGeneration) => {
    const close = new CloseController();

    const store = createWorkStore({
      page: "/account/page",
      buildId: "legacy-build",
      deploymentId: "legacy-deployment",
      previouslyRevalidatedTags: [],
      renderOpts: {
        ...RENDER_OPTIONS,
        onClose: (callback) => close.onClose(callback),
      },
    });

    onTestFinished(() => close.dispatchClose());

    const legacyStore = { ...store, isStaticGeneration };

    expect(workAsyncStorage.run(legacyStore, isStaticGenerationRender)).toBe(
      isStaticGeneration,
    );
  },
);

test("reflects the native work store and restores the surrounding render context", () => {
  const close = new CloseController();

  const input = {
    page: "/account/page",
    buildId: "cache-safe-build",
    deploymentId: "cache-safe-deployment",
    previouslyRevalidatedTags: [],
    renderOpts: {
      ...RENDER_OPTIONS,
      onClose: (callback: () => void) => close.onClose(callback),
    },
  };

  const dynamic = createWorkStore(input);

  const staticStore = createWorkStore({
    ...input,
    renderOpts: { ...input.renderOpts, isBuildTimePrerendering: true },
  });

  onTestFinished(() => close.dispatchClose());

  expect(workAsyncStorage.run(staticStore, isStaticGenerationRender)).toBe(
    true,
  );

  expect(
    workUnitAsyncStorage.run(createRequestStore(), () =>
      workAsyncStorage.run(dynamic, isStaticGenerationRender),
    ),
  ).toBe(false);

  expect(isStaticGenerationRender()).toBe(false);
});

test("removes trace metadata written by an earlier composed propagator during static renders", () => {
  const carrier = [
    { key: "traceparent", value: traceparent },
    { key: "tracestate", value: "vendor=value" },
    { key: "baggage", value: "session.id=test" },
  ];

  new CacheSafeTraceContextPropagator({ isStaticRender: () => true }).inject(
    active,
    carrier,
    clientTraceDataSetter,
  );

  expect(carrier).toStrictEqual([{ key: "baggage", value: "session.id=test" }]);
});

function renderMetadata(
  dynamic: boolean,
  propagator: TextMapPropagator,
  ctx: Context,
) {
  const close = new CloseController();

  const store = createWorkStore({
    page: "/account/page",
    buildId: "chain-build",
    deploymentId: "chain-deployment",
    previouslyRevalidatedTags: [],
    renderOpts: {
      ...RENDER_OPTIONS,
      isBuildTimePrerendering: !dynamic,
      onClose: (callback) => close.onClose(callback),
    },
  });

  onTestFinished(() => close.dispatchClose());

  const carrier: { key: string; value: string }[] = [];

  workUnitAsyncStorage.run(createRequestStore(), () =>
    workAsyncStorage.run(store, () =>
      propagator.inject(ctx, carrier, clientTraceDataSetter),
    ),
  );

  return carrier;
}

describe("managed propagator chain", () => {
  test("is complete and ordered inside the SDK: cache-safe W3C, freshness, baggage", () => {
    expect(managedPropagators().map((p) => p.constructor)).toStrictEqual([
      CacheSafeTraceContextPropagator,
      FreshnessStampingPropagator,
      W3CBaggagePropagator,
    ]);
  });

  test.each([
    {
      scenario: "sampled dynamic render",
      dynamic: true,
      flags: TraceFlags.SAMPLED,
      inbound: `interfere=1000000000`,
      traceparents: [traceparent],
      lastTracestate: /^interfere=(?!1000000000)\d+$/u,
    },
    {
      scenario: "unsampled dynamic render",
      dynamic: true,
      flags: TraceFlags.NONE,
      inbound: "vendor=abc123",
      traceparents: [traceparent.replace(/01$/u, "00")],
      lastTracestate: /^interfere=\d+,vendor=abc123$/u,
    },
  ])(
    "emits stamped, cache-safe client metadata for a $scenario behind a host W3C propagator",
    ({ dynamic, flags, inbound, traceparents, lastTracestate }) => {
      const chain = new CompositePropagator({
        propagators: [new W3CTraceContextPropagator(), ...managedPropagators()],
      });

      const ctx = trace.setSpanContext(
        ROOT_CONTEXT,
        omitUndefined({
          spanId: "b7ad6b7169203331",
          traceId: "0af7651916cd43dd8448eb211c80319c",
          traceFlags: flags,
          isRemote: true,
          traceState: inbound ? new TraceState(inbound) : undefined,
        }),
      );

      const carrier = renderMetadata(dynamic, chain, ctx);

      expect(
        carrier.flatMap((entry) =>
          entry.key === "traceparent" ? [entry.value] : [],
        ),
      ).toStrictEqual(traceparents);

      expect(
        carrier.findLast((entry) => entry.key === "tracestate")?.value,
      ).toMatch(lastTracestate);
    },
  );
});
