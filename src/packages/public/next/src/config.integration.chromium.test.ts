import { reportChromiumFailures } from "../test/chromium-diagnostics";
import { chromium } from "@playwright/test";
import type { Browser } from "@playwright/test";
import { readFile, readdir } from "node:fs/promises";
import { join } from "node:path";
import { afterAll, assert, beforeAll, describe, expect, test } from "vitest";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import {
  attribute,
  exceptions,
  exportedSpans,
} from "@interfere/test-utils/vitest/otlp";

function sessionAttribute(attributes: Parameters<typeof attribute>[0]) {
  return (
    attribute(attributes, "interfere.session.id") ??
    attribute(attributes, "session.id")
  );
}

function environment(url: string) {
  return {
    INTERFERE_API_KEY: "interfere_secret_us_acceptance",
    INTERFERE_API_URL: url,
    INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
    INTERFERE_ENVIRONMENT: "preview",
    OTEL_TRACES_SAMPLER: "always_on",
    OTEL_EXPORTER_OTLP_TRACES_ENDPOINT: `${url}/drain/v1/traces/platform`,
    OTEL_EXPORTER_OTLP_TRACES_PROTOCOL: "http/json",
  };
}

describe.each([
  { bundler: "webpack", serverTracing: "full", platform: "vercel" },
  { bundler: "turbopack", serverTracing: "full", platform: null },
  { bundler: "webpack", serverTracing: "drain", platform: null },
  { bundler: "turbopack", serverTracing: "drain", platform: "vercel" },
] as const)(
  "packed Next SDK in a production $bundler app with $serverTracing tracing on platform $platform",
  ({ bundler, serverTracing, platform }) => {
    let collector: Awaited<ReturnType<typeof receiver>>;
    let app: Awaited<ReturnType<typeof application>>;
    let env: ReturnType<typeof environment>;
    let host: Awaited<ReturnType<typeof app.start>>;
    let browser: Browser;
    const cleanups: (() => Promise<void>)[] = [];

    beforeAll(async () => {
      collector = await receiver({
        sdkConfig: omitUndefined({
          platform: platform ?? undefined,
          serverTracing: serverTracing === "drain" ? serverTracing : undefined,
        }),
      });

      cleanups.push(async () => collector.close());

      app = await application(`${import.meta.dirname}/..`, [
        "@interfere/next",
        "next",
        "react",
        "react-dom",
        "typescript",
        "@types/node",
        "@types/react",
        "@types/react-dom",
      ]);

      cleanups.push(async () => app.close());

      await app.copyFixture("acceptance");

      env = environment(collector.url);

      await app.buildNext(env, bundler);

      host = await app.start("serve.ts", env);

      cleanups.push(async () => host.close());

      browser = await chromium.launch({ headless: true });

      cleanups.push(async () => browser.close());
    }, 180_000);

    afterAll(async () => {
      for (const cleanup of cleanups.toReversed()) {
        // oxlint-disable-next-line eslint/no-await-in-loop -- Resources close in reverse order of creation.
        await cleanup();
      }
    });

    test("correlates browser, route and action telemetry", async () => {
      expect.hasAssertions();

      const page = await browser.newPage({
        userAgent:
          "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36",
      });

      reportChromiumFailures(page);

      await page.addInitScript(() => {
        Object.assign(globalThis, { __INTERFERE_FORCE_ENABLE__: true });
      });

      expect((await page.goto(host.url))?.status()).toBe(200);

      await expect
        .poll(async () =>
          page.getByLabel("Session", { exact: true }).textContent(),
        )
        .toMatch(/\S/u);

      const session = await page
        .getByLabel("Session", { exact: true })
        .textContent();

      expect(session).toMatch(/\S/u);

      await page.getByRole("button", { name: "Capture client error" }).click();

      await page.getByRole("button", { name: "Successful action" }).click();

      await expect
        .poll(async () => page.getByLabel("Result").textContent())
        .toBe("action-result");

      await page.getByRole("button", { name: "Failing action" }).click();

      await expect
        .poll(async () => page.getByLabel("Result").textContent())
        .toBe("action-rejected");

      await page.getByRole("button", { name: "Failing route" }).click();

      await expect
        .poll(async () => page.getByLabel("Result").textContent())
        .toBe("route-500");

      await expect
        .poll(
          () =>
            ["next-client-error", "next-action-error", "next-route-error"].map(
              (message) => exceptions(collector.requests, message).length,
            ),
          { timeout: 15_000 },
        )
        .toEqual([1, 1, 1]);

      const captured = [
        "next-client-error",
        "next-action-error",
        "next-route-error",
      ].map((message) => exceptions(collector.requests, message)[0]);

      const [routeError] = exceptions(collector.requests, "next-route-error");

      assert(routeError);

      const routeRequests = collector.requests.filter((request) =>
        exportedSpans([request]).some(
          (item) => item.spanId === routeError.spanId,
        ),
      );

      expect(routeRequests).toHaveLength(1);

      expect(
        routeRequests[0]?.url.startsWith("/drain/v1/traces/platform"),
      ).toBe(serverTracing === "drain");

      await expect
        .poll(
          () =>
            exportedSpans(collector.requests)
              .filter(
                (item) =>
                  attribute(item.resource, "interfere.sdk.name") ===
                  "@interfere/react",
              )
              .map((item) => item.traceId),
          { timeout: 15_000 },
        )
        .toContain(routeError.traceId);

      const release = attribute(routeError.resource, "release.slug");

      expect(release).toMatch(/^rel_/u);

      for (const item of captured) {
        assert(item);

        expect(attribute(item.resource, "release.slug")).toBe(release);
        expect(sessionAttribute(item.attributes)).toBe(session);
      }

      const successfulActions = exportedSpans(collector.requests).filter(
        (item) => item.name === "serverAction acceptance-success",
      );

      expect(successfulActions).toHaveLength(1);

      const [successfulAction] = successfulActions;

      assert(successfulAction);

      expect(
        attribute(successfulAction.attributes, "interfere.session.id"),
      ).toBe(session);

      expect(successfulAction.parentSpanId).toMatch(/\S/u);

      expect(
        exportedSpans(collector.requests).some(
          (item) =>
            item.spanId === successfulAction.parentSpanId &&
            item.traceId === successfulAction.traceId &&
            sessionAttribute(item.attributes) === session,
        ),
      ).toBe(true);

      const assets = await readdir(join(app.root, ".next/static"), {
        recursive: true,
      });

      const scripts = await Promise.all(
        assets
          .filter((file) => file.endsWith(".js"))
          .map(async (file) =>
            readFile(join(app.root, ".next/static", file), "utf8"),
          ),
      );

      expect(scripts.length).toBeGreaterThan(0);
      expect(scripts.join("\n")).not.toContain(env.INTERFERE_API_KEY);

      const serverFiles = await readdir(join(app.root, ".next/server"), {
        recursive: true,
      });

      const serverScripts = await Promise.all(
        serverFiles
          .filter((file) => file.endsWith(".js"))
          .map(async (file) =>
            readFile(join(app.root, ".next/server", file), "utf8"),
          ),
      );

      expect(serverScripts.join("\n")).toMatch(
        new RegExp(
          String.raw`__INTERFERE_PLATFORM__\W{1,6}${JSON.stringify(platform)}`,
          "u",
        ),
      );

      expect(
        collector.requests
          .filter((request) => request.url.startsWith("/drain/"))
          .every((request) => !request.body.includes(env.INTERFERE_API_KEY)),
      ).toBe(true);
    });
  },
);
