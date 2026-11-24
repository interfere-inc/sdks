/* oxlint-disable typescript/no-unsafe-return */
import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace";

import { setupErrorsPlugin } from "../../test/fixtures/errors.test-utils.js";

import { beforeEach, expect, test } from "vitest";

const ANY_VIEW_ID: unknown = expect.any(String);

let exporter: InMemorySpanExporter;

let stop: () => void;

beforeEach(() => {
  ({ exporter, stop } = setupErrorsPlugin());
});

const violation = {
  blockedURI: "https://cdn.evil.example.com/blocked.png",
  columnNumber: 7,
  disposition: "enforce",
  lineNumber: 12,
  sourceFile: "https://app.example.com/page.js",
  violatedDirective: "img-src",
} as const;

test.each([
  { name: "metadata", count: 1, distinct: false, stopped: false },
  { name: "dedupe", count: 3, distinct: false, stopped: false },
  { name: "five-report cap", count: 12, distinct: true, stopped: false },
  { name: "cleanup", count: 1, distinct: false, stopped: true },
])("preserves CSP $name", ({ count, distinct, stopped }) => {
  expect.hasAssertions();

  if (stopped) {
    stop();
  }

  for (let i = 0; i < count; i += 1) {
    globalThis.dispatchEvent(
      new SecurityPolicyViolationEvent("securitypolicyviolation", {
        ...violation,
        blockedURI: distinct
          ? `https://cdn.evil.example.com/${i}.png`
          : violation.blockedURI,
      }),
    );
  }

  const uniqueViolationCount = distinct ? 5 : 1;
  const expectedCount = stopped ? 0 : uniqueViolationCount;

  const expected = Array.from({ length: expectedCount }, (_, i) => {
    const blockedURI = distinct
      ? `https://cdn.evil.example.com/${i}.png`
      : violation.blockedURI;

    return [
      expect.objectContaining({
        name: "exception",
        attributes: {
          "exception.type": "SecurityPolicyViolation",
          "exception.message": `CSP img-src blocked ${blockedURI}`,
          "interfere.exception.kind": "non-error",
          "interfere.exception.serialized": JSON.stringify({
            ...violation,
            blockedURI,
          }),
          "interfere.exception.handled": false,
          "interfere.exception.mechanism":
            "auto.browser.security_policy.violation",
          "interfere.view.id": ANY_VIEW_ID,
        },
      }),
    ];
  });

  expect(exporter.getFinishedSpans().map((span) => span.events)).toStrictEqual(
    expected,
  );
});
