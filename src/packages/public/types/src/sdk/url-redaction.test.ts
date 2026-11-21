import { describe, expect, test, vi } from "vitest";

import {
  createUrlRedactionSpanProcessor,
  redactUrl,
  resolveUrlRedaction,
} from "./url-redaction.ts";
import type { UrlRedactionOption } from "./url-redaction.ts";

describe(redactUrl, () => {
  test.each([
    [
      "redacts built-in sensitive parameters and keeps the rest",
      undefined,
      "https://shop.test/a?page=2&token=t&X-Amz-Signature=s&sig=s&AWSAccessKeyId=k&client_secret=c&oauth_state=o&code=c#tab",
      "https://shop.test/a?page=2&token=REDACTED&X-Amz-Signature=REDACTED&sig=REDACTED&AWSAccessKeyId=REDACTED&client_secret=REDACTED&oauth_state=REDACTED&code=REDACTED#tab",
    ],
    [
      "matches encoded and mixed-case names",
      undefined,
      "/a?Access%5FToken=t&SIG=s",
      "/a?Access%5FToken=REDACTED&SIG=REDACTED",
    ],
    [
      "keeps query values but redacts credentials when disabled",
      false,
      "https://user:pass@shop.test/a?token=t#x",
      "https://REDACTED:REDACTED@shop.test/a?token=t#x",
    ],
    [
      "redacts every value and removes fragments when enabled",
      true,
      "https://user@shop.test/a?page=2&eyJhbGciOiJIUzI1NiJ9&debug=#tab",
      "https://REDACTED@shop.test/a?page=REDACTED&REDACTED&debug=REDACTED",
    ],
    [
      "keeps only allowed values and keeps fragments",
      { allow: ["page", "Code"] },
      "/a?page=2&code=promo&q=shoes&token=t#tab",
      "/a?page=2&code=promo&q=REDACTED&token=REDACTED#tab",
    ],
    [
      "redacts denied values with the built-in names",
      { deny: ["email"] },
      "/a?page=2&email=a%40b.test&token=t#tab",
      "/a?page=2&email=REDACTED&token=REDACTED#tab",
    ],
    [
      "removes fragments only when asked",
      { fragment: true },
      "/a?page=2#/account?token=x",
      "/a?page=2",
    ],
    [
      "lets deny override allow",
      { allow: ["q"], deny: ["q"] },
      "/a?q=shoes",
      "/a?q=REDACTED",
    ],
    [
      "keeps hash-route queries inside the kept fragment",
      { allow: [] },
      "https://shop.test/?token=t#/account?token=x",
      "https://shop.test/?token=REDACTED#/account?token=x",
    ],
    [
      "leaves malformed values parseable",
      undefined,
      "not a url?token=%E0%A4%A",
      "not a url?token=REDACTED",
    ],
    [
      "returns URLs without query, fragment or credentials",
      true,
      "/a/b",
      "/a/b",
    ],
    ["keeps an empty query", true, "/a?", "/a?"],
    [
      "matches malformed names by their raw text",
      { deny: ["%E0%A4%A"] },
      "/a?%E0%A4%A=1&q=2",
      "/a?%E0%A4%A=REDACTED&q=2",
    ],
  ] satisfies [string, UrlRedactionOption | undefined, string, string][])(
    "%s",
    (_name, option, url, expected) => {
      const policy = resolveUrlRedaction(option);

      expect(redactUrl(url, policy)).toBe(expected);
      expect(redactUrl(expected, policy)).toBe(expected);
    },
  );
});

describe(createUrlRedactionSpanProcessor, () => {
  const failure = "failed at https://shop.test/a?token=t";
  const redactedFailure = "failed at https://shop.test/a?token=REDACTED";

  test("redacts the span name, status, URL attributes and exception events before export", () => {
    const processor = createUrlRedactionSpanProcessor(
      resolveUrlRedaction({ fragment: true }),
    );

    const updateName = vi.fn<(name: string) => void>();

    const setStatus =
      vi.fn<(status: { code: number; message: string }) => void>();

    const attributes = {
      "http.request.header.referer": ["https://shop.test/r?sig=s", null],
      "http.url": { path: "/?token=secret" },
      "url.original": [["/?token=secret"]],
      "http.response.status_code": 500,
      "url.fragment": "frag",
      "url.full": "https://user:pw@shop.test/a?token=t&page=2#frag",
      "url.query": "token=t&page=2",
    };

    const exception = {
      "exception.message": failure,
      "exception.stacktrace": `Error: ${failure}\n    at https://shop.test/app.js?token=t:1:2`,
      "interfere.exception.chain": JSON.stringify([
        { stack: `Error: ${failure}\n    at f`, type: "Error", value: failure },
        { type: "Error", value: "cause https://api.test/b?secret=s" },
        { type: "Error" },
      ]),
      "interfere.exception.serialized": JSON.stringify({
        url: "https://cdn.test/x.png?sig=s",
      }),
    };

    processor.onEnding({
      attributes,
      events: [{ attributes: exception, name: "exception" }, { name: "mark" }],
      name: "GET /orders?token=t",
      setStatus,
      status: { code: 2, message: failure },
      updateName,
    });

    expect(updateName).toHaveBeenCalledWith("GET /orders?token=REDACTED");

    expect(setStatus).toHaveBeenCalledWith({
      code: 2,
      message: redactedFailure,
    });

    expect(attributes).toStrictEqual({
      "http.url": "REDACTED",
      "url.original": ["REDACTED"],
      "http.request.header.referer": ["https://shop.test/r?sig=REDACTED", null],
      "http.response.status_code": 500,
      "url.full": "https://REDACTED:REDACTED@shop.test/a?token=REDACTED&page=2",
      "url.query": "token=REDACTED&page=2",
    });

    expect(exception).toStrictEqual({
      "exception.message": redactedFailure,
      "exception.stacktrace": `Error: ${redactedFailure}\n    at https://shop.test/app.js?token=t:1:2`,
      "interfere.exception.chain": JSON.stringify([
        {
          stack: `Error: ${redactedFailure}\n    at f`,
          type: "Error",
          value: redactedFailure,
        },
        { type: "Error", value: "cause https://api.test/b?secret=REDACTED" },
        { type: "Error" },
      ]),
      "interfere.exception.serialized": JSON.stringify({
        url: "https://cdn.test/x.png?sig=REDACTED",
      }),
    });
  });

  test("redacts query tokens in fallback and appended frame file URLs", () => {
    const processor = createUrlRedactionSpanProcessor(resolveUrlRedaction());

    const frames = (file: string) =>
      JSON.stringify([
        { column: 2, file, fn: "render", id: "f-0", line: 1 },
        { fn: "Anonymous", id: "f-1" },
      ]);

    const exception = {
      "interfere.exception.appended_frames": frames(
        "https://shop.test/app.js?token=t&v=3",
      ),
      "interfere.exception.fallback_frames": frames(
        "https://REDACTED@shop.test/app.js?sig=s",
      ),
    };

    processor.onEnd({
      attributes: {},
      events: [{ attributes: exception, name: "exception" }],
    });

    expect(exception).toStrictEqual({
      "interfere.exception.appended_frames": frames(
        "https://shop.test/app.js?token=REDACTED&v=3",
      ),
      "interfere.exception.fallback_frames": frames(
        "https://REDACTED@shop.test/app.js?sig=REDACTED",
      ),
    });
  });

  test("keeps unparseable frame attributes unchanged", () => {
    const processor = createUrlRedactionSpanProcessor(resolveUrlRedaction());

    const exception = {
      "interfere.exception.fallback_frames": "not json ?token=t",
    };

    processor.onEnd({
      attributes: {},
      events: [{ attributes: exception, name: "exception" }],
    });

    expect(exception).toStrictEqual({
      "interfere.exception.fallback_frames": "not json ?token=t",
    });
  });

  test("redacts attributes in onEnd for SDKs without onEnding and keeps unparseable chains", () => {
    const processor = createUrlRedactionSpanProcessor(resolveUrlRedaction());

    const attributes = {
      "interfere.exception.chain": `not json ${failure}`,
      "url.full": "https://shop.test/a?token=t#frag",
    };

    processor.onEnd({ attributes, events: [] });

    expect(attributes).toStrictEqual({
      "interfere.exception.chain": `not json ${failure}`,
      "url.full": "https://shop.test/a?token=REDACTED#frag",
    });
  });
});
