import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { expect, test, vi } from "vitest";

import { originPolicy, propagationTargets } from "./propagation-targets.js";
import type { PropagationPolicy } from "./header-policy.js";

const pageOrigin = "https://app.acme.test";

const ALL = ["baggage", "traceparent", "tracestate"];

function names(policy: PropagationPolicy, url: string) {
  const entry = originPolicy(policy, url);

  return entry === undefined ? undefined : [...entry.headers].toSorted();
}

function discovered(policy: PropagationPolicy, url: string) {
  return originPolicy(policy, url)?.discovered;
}

test.each([
  ["https://app.acme.test/orders", ALL],
  ["https://APP.acme.test:443/orders", ALL],
  ["https://api.acme.test/orders", undefined],
  ["http://app.acme.test/orders", undefined],
  ["https://app.acme.test:8443/orders", undefined],
  ["https://app.acme.test.evil.test/orders", undefined],
  ["https://app.acme.test@evil.test/orders", undefined],
  ["https://explicit.test:8443/orders", ALL],
  ["https://explicit.test/orders", undefined],
  ["https://backend.test/orders", ALL],
  ["http://backend.test/orders", undefined],
  ["https://backend.test:8443/orders", undefined],
])("matches parsed origin for %s: %s", (url, expected) => {
  const targets = propagationTargets({
    pageOrigin,
    propagateTo: ["https://explicit.test:8443"],
    backendHosts: ["backend.test"],
  });

  expect(names(targets, url)).toStrictEqual(expected);
});

test.each([
  "api.test",
  "https://*.api.test",
  "https://%2A.api.test",
  "https://%2a.api.test",
  "https://api.test/path",
  "https://api.test/",
  "https://api.test?x=1",
  "https://api.test#fragment",
  "https://user@api.test",
  "ftp://api.test",
  "https://api.test:bad",
  " https://api.test",
  String.raw`https://api.test\evil`,
])(
  "rejects invalid propagateTo entry %s with one warning naming it, as a string or an entry",
  (entry) => {
    const warning = vi.spyOn(console, "warn");

    const targets = propagationTargets({
      pageOrigin,
      propagateTo: [entry, { headers: ["baggage"], origin: entry }],
    });

    expect(names(targets, "https://api.test/path")).toBeUndefined();

    expect(warning).toHaveBeenCalledExactlyOnceWith(
      `[interfere] Invalid tracing.propagateTo origin: ${entry}`,
    );
  },
);

test("backendHosts normalise to exact HTTPS origins and reject wildcards, schemes and lookalikes", () => {
  const targets = propagationTargets({
    pageOrigin,
    backendHosts: [" API.partner.io. ", "*.evil.com", "https://x.com"],
  });

  expect(names(targets, "https://api.partner.io/graphql")).toStrictEqual(ALL);
  expect(names(targets, "https://api.partner.io:8443/graphql")).toBeUndefined();
  expect(names(targets, "https://www.partner.io/")).toBeUndefined();
  expect(names(targets, "https://api.partner.io.evil.com/")).toBeUndefined();
  expect(names(targets, "https://api.evil.com/")).toBeUndefined();
  expect(names(targets, "https://x.com/")).toBeUndefined();
});

test("headers: false keeps every origin tracked and writes nothing anywhere, entries included", () => {
  const targets = propagationTargets({
    backendHosts: ["backend.test"],
    headers: false,
    pageOrigin,
    propagateTo: [
      "https://explicit.test",
      { headers: ["traceparent", "tracestate"], origin: "https://api.test" },
    ],
  });

  expect(
    [
      `${pageOrigin}/orders`,
      "https://explicit.test/orders",
      "https://api.test/orders",
      "https://backend.test/orders",
    ].map((url) => names(targets, url)),
  ).toStrictEqual([[], [], [], []]);

  expect(names(targets, "https://other.test/orders")).toBeUndefined();
});

test("a header list restricts inherited origins while an entry keeps its own list", () => {
  const targets = propagationTargets({
    backendHosts: ["backend.test"],
    headers: ["baggage"],
    pageOrigin,
    propagateTo: [
      "https://explicit.test",
      { headers: ["traceparent", "tracestate"], origin: "https://api.test" },
    ],
  });

  expect(
    [
      `${pageOrigin}/orders`,
      "https://explicit.test/orders",
      "https://api.test/orders",
      "https://backend.test/orders",
    ].map((url) => names(targets, url)),
  ).toStrictEqual([
    ["baggage"],
    ["baggage"],
    ["traceparent", "tracestate"],
    ["baggage"],
  ]);
});

test("a vendor that propagates W3C itself leaves only baggage in every set", () => {
  const targets = propagationTargets({
    backendHosts: ["backend.test"],
    pageOrigin,
    propagateTo: [
      { headers: ["traceparent", "tracestate"], origin: "https://api.test" },
    ],
    writesTraceContext: false,
  });

  expect(
    [
      `${pageOrigin}/orders`,
      "https://api.test/orders",
      "https://backend.test/orders",
    ].map((url) => names(targets, url)),
  ).toStrictEqual([["baggage"], [], ["baggage"]]);
});

test("backend hosts are discovered while the page origin and propagateTo are explicit", () => {
  const targets = propagationTargets({
    backendHosts: ["backend.test"],
    pageOrigin,
    propagateTo: ["https://explicit.test"],
  });

  expect(
    [
      `${pageOrigin}/orders`,
      "https://explicit.test/orders",
      "https://backend.test/orders",
      "https://other.test/orders",
    ].map((url) => discovered(targets, url)),
  ).toStrictEqual([false, false, true, undefined]);
});

test("an explicit entry wins over a configured backend host with the same origin", () => {
  const targets = propagationTargets({
    backendHosts: ["backend.test"],
    pageOrigin,
    propagateTo: [{ headers: ["baggage"], origin: "https://backend.test" }],
  });

  expect(names(targets, "https://backend.test/orders")).toStrictEqual([
    "baggage",
  ]);

  expect(discovered(targets, "https://backend.test/orders")).toBe(false);
});

test.each([
  {
    propagateTraceBaggage: undefined,
    inherited: ALL,
    explicit: ["baggage", "traceparent"],
  },
  {
    propagateTraceBaggage: true,
    inherited: ALL,
    explicit: ["baggage", "traceparent"],
  },
  {
    propagateTraceBaggage: false,
    inherited: ["traceparent", "tracestate"],
    explicit: ["traceparent"],
  },
])(
  "propagateTraceBaggage=$propagateTraceBaggage applies to inherited and explicit header policies",
  ({ propagateTraceBaggage, inherited, explicit }) => {
    const targets = propagationTargets(
      omitUndefined({
        pageOrigin,
        backendHosts: ["backend.test"],
        propagateTraceBaggage,
        propagateTo: [
          "https://explicit.test",
          {
            origin: "https://api.test",
            headers: ["baggage" as const, "traceparent" as const],
          },
        ],
      }),
    );

    expect(
      [pageOrigin, "https://backend.test", "https://explicit.test"].map((url) =>
        names(targets, url),
      ),
    ).toStrictEqual([inherited, inherited, inherited]);

    expect(names(targets, "https://api.test")).toStrictEqual(explicit);
  },
);

test("baggage opt-out does not enable headers forbidden by the existing policy or vendor ownership", () => {
  expect(
    names(
      propagationTargets({
        pageOrigin,
        propagateTraceBaggage: false,
        headers: ["baggage"],
      }),
      pageOrigin,
    ),
  ).toStrictEqual([]);

  expect(
    names(
      propagationTargets({
        pageOrigin,
        propagateTraceBaggage: false,
        writesTraceContext: false,
      }),
      pageOrigin,
    ),
  ).toStrictEqual([]);

  expect(
    names(
      propagationTargets({
        pageOrigin,
        propagateTraceBaggage: true,
        headers: false,
        propagateTo: [{ origin: pageOrigin, headers: ["baggage"] }],
      }),
      pageOrigin,
    ),
  ).toStrictEqual([]);
});
