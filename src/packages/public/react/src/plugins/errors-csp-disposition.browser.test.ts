import { setupErrorsPlugin } from "../../test/fixtures/errors.test-utils.js";

import { expect, test } from "vitest";

const violation = {
  blockedURI: "https://assets.example.com/video.mp4",
  violatedDirective: "media-src",
};

test.each(["report", "enforce"] as const)(
  "retains report-only and enforced evidence for the same URI when %s arrives first",
  (first) => {
    expect.hasAssertions();

    const { exporter } = setupErrorsPlugin();

    const dispositions =
      first === "report"
        ? (["report", "enforce"] as const)
        : (["enforce", "report"] as const);

    for (const disposition of [...dispositions, ...dispositions]) {
      globalThis.dispatchEvent(
        new SecurityPolicyViolationEvent("securitypolicyviolation", {
          ...violation,
          disposition,
        }),
      );
    }

    const events = exporter.getFinishedSpans().flatMap((span) => span.events);

    expect(events).toHaveLength(2);

    for (const [index, disposition] of dispositions.entries()) {
      expect(events[index]?.attributes).toMatchObject({
        "exception.type": "SecurityPolicyViolation",
        "exception.message":
          disposition === "report"
            ? `CSP media-src reported ${violation.blockedURI} (report-only)`
            : `CSP media-src blocked ${violation.blockedURI}`,
        "interfere.exception.handled": disposition === "report",
        "interfere.exception.mechanism":
          "auto.browser.security_policy.violation",
      });

      expect(
        events[index]?.attributes?.["interfere.exception.serialized"],
      ).toContain(`"disposition":"${disposition}"`);
    }
  },
);

test("a full report-only budget cannot hide a subsequent enforced failure", () => {
  const { exporter } = setupErrorsPlugin();

  for (let index = 0; index < 6; index += 1) {
    globalThis.dispatchEvent(
      new SecurityPolicyViolationEvent("securitypolicyviolation", {
        ...violation,
        blockedURI: `https://assets.example.com/${index}.mp4`,
        disposition: "report",
      }),
    );
  }

  globalThis.dispatchEvent(
    new SecurityPolicyViolationEvent("securitypolicyviolation", {
      ...violation,
      blockedURI: "https://assets.example.com/0.mp4",
      disposition: "enforce",
    }),
  );

  const events = exporter.getFinishedSpans().flatMap((span) => span.events);

  expect(events).toHaveLength(6);

  expect(events.at(-1)?.attributes).toMatchObject({
    "exception.message":
      "CSP media-src blocked https://assets.example.com/0.mp4",
    "interfere.exception.handled": false,
  });
});

test.each([
  {
    disposition: "report",
    expectedMessage: "CSP script-src reported inline (report-only)",
    expectedHandled: true,
  },
  {
    disposition: "enforce",
    expectedMessage: "CSP script-src blocked inline",
    expectedHandled: false,
  },
] as const)(
  "labels inline violations accurately for $disposition policies",
  ({ disposition, expectedMessage, expectedHandled }) => {
    const { exporter } = setupErrorsPlugin();

    globalThis.dispatchEvent(
      new SecurityPolicyViolationEvent("securitypolicyviolation", {
        violatedDirective: "script-src",
        disposition,
      }),
    );

    expect(exporter.getFinishedSpans()[0]?.events[0]?.attributes).toMatchObject(
      {
        "exception.message": expectedMessage,
        "interfere.exception.handled": expectedHandled,
      },
    );
  },
);
