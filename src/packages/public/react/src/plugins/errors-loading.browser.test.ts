import { HttpResponse, http, passthrough } from "msw";
import type { InMemorySpanExporter } from "@opentelemetry/sdk-trace-base";

import { setupErrorsPlugin } from "../../test/fixtures/errors.test-utils.js";
import { worker } from "../../test/msw.js";
import { beforeEach, expect, onTestFinished, test } from "vitest";

let exporter: InMemorySpanExporter;

beforeEach(() => {
  ({ exporter } = setupErrorsPlugin());
});

async function nextTask() {
  await new Promise<void>((resolve) => {
    setTimeout(resolve, 0);
  });
}

function messagesFor(mechanism: string) {
  return exporter
    .getFinishedSpans()
    .filter(
      (span) =>
        span.events[0]?.attributes?.["interfere.exception.mechanism"] ===
        mechanism,
    )
    .map((span) => span.events[0]?.attributes?.["exception.message"]);
}

async function failToLoad(element: HTMLElement, eventTarget: HTMLElement) {
  const failed = new Promise<void>((resolve) => {
    eventTarget.addEventListener(
      "error",
      () => {
        resolve();
      },
      { once: true },
    );
  });

  document.body.append(element);

  onTestFinished(() => {
    element.remove();
  });

  await failed;
}

test.each(["video", "audio"] as const)(
  "captures a failed %s source",
  async (tag) => {
    const url = `https://cdn.app.test/broken-${tag}.media`;

    worker.use(http.get(url, () => HttpResponse.error()));

    const element = document.createElement(tag);

    element.src = url;

    await failToLoad(element, element);

    expect(messagesFor("auto.browser.resource.error")).toStrictEqual([
      `Failed to load ${tag}: ${url}`,
    ]);
  },
);

test("captures a failed source child of a media element", async () => {
  const url = "https://cdn.app.test/broken-source.media";

  worker.use(http.get(url, () => HttpResponse.error()));

  const video = document.createElement("video");
  const source = document.createElement("source");

  source.src = url;

  video.append(source);

  await failToLoad(video, source);

  expect(messagesFor("auto.browser.resource.error")).toStrictEqual([
    `Failed to load source: ${url}`,
  ]);
});

async function crossOriginScriptUrl() {
  const candidates = ["127.0.0.1", "[::1]", "localhost"].flatMap((hostname) => {
    if (hostname === location.hostname) {
      return [];
    }

    const url = new URL("/cross-origin-throw.txt", location.href);

    url.hostname = hostname;

    return [url.href];
  });

  const probes = await Promise.allSettled(
    candidates.map(async (url) => fetch(url, { mode: "no-cors" })),
  );

  const reachable = candidates.find(
    (_url, index) => probes[index]?.status === "fulfilled",
  );

  if (reachable === undefined) {
    throw new Error(
      "the test server is not reachable on another loopback origin",
    );
  }

  return reachable;
}

async function runCrossOriginScript(crossOrigin: string | null) {
  worker.use(http.get("*/cross-origin-throw.txt", () => passthrough()));

  const url = await crossOriginScriptUrl();

  const script = document.createElement("script");
  const browserErrors: ErrorEvent[] = [];

  const observeError = (event: ErrorEvent) => {
    browserErrors.push(event);
  };

  globalThis.addEventListener("error", observeError);

  if (crossOrigin !== null) {
    script.crossOrigin = crossOrigin;
  }

  const loaded = new Promise<void>((resolve, reject) => {
    script.addEventListener(
      "load",
      () => {
        resolve();
      },
      { once: true },
    );

    script.addEventListener(
      "error",
      () => {
        reject(new Error(`could not load ${url}`));
      },
      { once: true },
    );
  });

  script.src = url;

  document.body.append(script);

  onTestFinished(() => {
    script.remove();

    globalThis.removeEventListener("error", observeError);
  });

  await loaded;

  const [browserError] = browserErrors;

  if (browserError === undefined) {
    throw new Error("the cross-origin script did not dispatch an ErrorEvent");
  }

  return browserError;
}

test("preserves the browser's error visibility for a cross-origin script loaded without CORS", async () => {
  const browserError = await runCrossOriginScript(null);

  expect(["Script error.", "TypeError: thrown by a CDN script"]).toContain(
    browserError.message,
  );

  expect(messagesFor("auto.browser.global_handlers.onerror")).toStrictEqual(
    browserError.message === "Script error." ? [] : ["thrown by a CDN script"],
  );
});

test("records the error from a cross-origin script loaded with crossorigin=anonymous", async () => {
  await runCrossOriginScript("anonymous");

  expect(messagesFor("auto.browser.global_handlers.onerror")).toStrictEqual([
    "thrown by a CDN script",
  ]);
});

test("captures a failed dynamic import once", async () => {
  const url = "https://cdn.app.test/chunk-missing.js";

  worker.use(http.get(url, () => HttpResponse.error()));

  // oxlint-disable-next-line no-inline-comments -- Vite must leave this cross-origin import to the browser.
  void import(/* @vite-ignore */ url);

  await expect
    .poll(() =>
      messagesFor("auto.browser.global_handlers.onunhandledrejection"),
    )
    .toHaveLength(1);

  await nextTask();

  expect(exporter.getFinishedSpans()).toHaveLength(1);
});
