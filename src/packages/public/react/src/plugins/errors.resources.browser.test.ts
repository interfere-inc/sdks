import { HttpResponse, http } from "msw";
import { worker } from "../../test/msw.js";

/* oxlint-disable typescript/no-unsafe-return */
import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace";

import { setupErrorsPlugin } from "../../test/fixtures/errors.test-utils.js";

import { beforeEach, expect, onTestFinished, test } from "vitest";

const ANY_VIEW_ID: unknown = expect.any(String);

const ANY_NUMBER: unknown = expect.any(Number);

let exporter: InMemorySpanExporter;

beforeEach(() => {
  ({ exporter } = setupErrorsPlugin());
});

const image = { tag: "img", url: "https://cdn.app.test/broken.png" };

const script = { tag: "script", url: "https://cdn.app.test/a.js" };

const css = { tag: "link", url: "https://cdn.app.test/a.css" };

test.each([
  { name: "image", resources: [image], captured: [image] },
  {
    name: "script and stylesheet",
    resources: [script, css],
    captured: [script, css],
  },
  {
    name: "ignored vendor",
    resources: [
      { tag: "script", url: "https://www.googletagmanager.com/gtm.js" },
    ],
    captured: [],
  },
  { name: "non-resource window event", resources: [], captured: [] },
  { name: "duplicate URL", resources: [image, image], captured: [image] },
])("captures only eligible $name errors", async ({ resources, captured }) => {
  expect.hasAssertions();

  for (const { tag, url } of resources) {
    const element = document.createElement(tag);

    worker.use(http.get(url, () => HttpResponse.error()));

    const failed = new Promise<void>((resolve) => {
      element.addEventListener("error", () => resolve(), { once: true });
    });

    if (tag === "link") {
      element.setAttribute("rel", "stylesheet");
    }

    element.setAttribute(tag === "link" ? "href" : "src", url);

    document.body.append(element);

    try {
      // oxlint-disable-next-line no-await-in-loop
      await failed;
    } finally {
      element.remove();
    }
  }

  if (resources.length === 0) {
    const event = new Event("error", { cancelable: true });

    event.preventDefault();

    globalThis.dispatchEvent(event);
  }

  expect(exporter.getFinishedSpans().map((span) => span.events)).toStrictEqual(
    captured.map(({ tag, url }) => {
      const serialized: unknown = expect.stringContaining(
        JSON.stringify({ url, "interfere.resource.tag": tag }).slice(0, -1),
      );

      return [
        expect.objectContaining({
          name: "exception",
          attributes: {
            "exception.type": "ResourceLoadError",
            "exception.message": `Failed to load ${tag}: ${url}`,
            "interfere.exception.kind": "non-error",
            "interfere.exception.serialized": serialized,
            "interfere.exception.handled": false,
            "interfere.exception.mechanism": "auto.browser.resource.error",
            "interfere.view.id": ANY_VIEW_ID,
          },
        }),
      ];
    }),
  );
});

test("redacts sensitive values in resource-load messages by default", async () => {
  const element = document.createElement("img");

  worker.use(
    http.get("https://cdn.app.test/receipt.png", () => HttpResponse.error()),
  );

  const failed = new Promise<void>((resolve) => {
    element.addEventListener("error", () => resolve(), { once: true });
  });

  element.src = "https://cdn.app.test/receipt.png?sig=signed-secret#frag";

  document.body.append(element);

  onTestFinished(() => {
    element.remove();
  });

  await failed;

  const attributes = exporter
    .getFinishedSpans()
    .flatMap((span) => span.events.map((event) => event.attributes));

  const serialized: unknown = expect.stringContaining(
    JSON.stringify({
      url: "https://cdn.app.test/receipt.png?sig=REDACTED#frag",
    }).slice(0, -1),
  );

  expect(attributes).toMatchObject([
    {
      "exception.message":
        "Failed to load img: https://cdn.app.test/receipt.png?sig=REDACTED#frag",
      "interfere.exception.serialized": serialized,
    },
  ]);

  expect(JSON.stringify(attributes)).not.toContain("signed-secret");
});

test.each([
  {
    name: "an HTTP error status",
    respond: () => new HttpResponse("missing", { status: 404 }),
    outcome: {
      "interfere.load.failure": "http_status",
      "http.response.status_code": 404,
    },
  },
  {
    name: "a network error",
    respond: () => HttpResponse.error(),
    outcome: { "interfere.load.failure": "no_status" },
  },
])(
  "records the browser's load outcome for $name",
  async ({ name, respond, outcome }) => {
    const url = `${location.origin}/chunks/${name.replaceAll(" ", "-")}.js`;
    const element = document.createElement("script");

    worker.use(http.get(url, respond));

    const failed = new Promise<void>((resolve) => {
      element.addEventListener("error", () => resolve(), { once: true });
    });

    element.src = url;

    document.body.append(element);

    onTestFinished(() => {
      element.remove();
    });

    await failed;

    const [serialized] = exporter
      .getFinishedSpans()
      .flatMap((span) => span.events)
      .map((event) => event.attributes?.["interfere.exception.serialized"]);

    expect(JSON.parse(String(serialized))).toStrictEqual({
      url,
      "interfere.resource.tag": "script",
      "interfere.load.duration_ms": ANY_NUMBER,
      "interfere.load.transfer_size": ANY_NUMBER,
      ...outcome,
    });
  },
);
