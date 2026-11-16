import { execFile } from "node:child_process";
import { randomBytes } from "node:crypto";
import { mkdtemp, readFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import { chromium } from "@playwright/test";
import type { Browser, Page } from "@playwright/test";
import { afterAll, beforeAll, onTestFinished } from "vitest";
import { z } from "zod";
import { application } from "@interfere/test-utils/vitest/packed-application";
import { receiver } from "@interfere/test-utils/vitest/telemetry-receiver";
import {
  attribute,
  exportedLogRecords,
  exportedSpans,
} from "@interfere/test-utils/vitest/otlp";

const execute = promisify(execFile);

const HOST_KEY = "interfere_secret_us_acceptance";

export const FOREIGN_TRACEPARENT =
  /^00-0000000000000000(?<trace>[\da-f]{16})-(?<span>[\da-f]{16})-0(?<sampled>[01])$/u;

export interface ForeignOptions {
  injectUnsampled?: boolean;
  sampled?: boolean;
  sessionDelayMs?: number;
  styles?: ("b3" | "b3multi" | "vendor" | "w3c")[];
}

const echo = z.object({
  case: z.string().nullable(),
  headers: z.record(z.string(), z.string().nullable()),
  server: z.object({ traceFlags: z.number(), traceId: z.string() }).nullable(),
});

export type Echo = z.infer<typeof echo>;

export interface JourneyOptions {
  foreign?: ForeignOptions | false;
  interfere?: boolean;
  order?: "foreign-first" | "interfere-first";
}

async function bundleForeignWriter(directory: string) {
  const entry = new URL(
    "../../react/test/foreign-writer.fixture.ts",
    import.meta.url,
  );

  await execute("bun", [
    "build",
    fileURLToPath(entry),
    "--target=browser",
    "--format=iife",
    "--conditions=monorepo",
    "--outdir",
    directory,
  ]);

  return readFile(join(directory, "foreign-writer.fixture.js"), "utf8");
}

/**
 * Builds the application-owned tracing packed Next app once per file: a host
 * `@vercel/otel` provider exporting OTLP to the receiver, Interfere's
 * `register({ tracing: "application" })`, the browser initialised with its
 * defaults, and a Datadog-shaped foreign writer double.
 */
export function foreignTracingOwnerHarness() {
  let collector: Awaited<ReturnType<typeof receiver>>;
  let host: { url: string };
  let browser: Browser;
  let writer = "";
  let sequence = 0;
  const cleanups: (() => Promise<void>)[] = [];

  beforeAll(async () => {
    collector = await receiver();

    cleanups.push(async () => collector.close());

    const app = await application(`${import.meta.dirname}/..`, [
      "@interfere/next",
      "next",
      "react",
      "react-dom",
      "typescript",
      "@types/node",
      "@types/react",
      "@types/react-dom",
      "@vercel/otel",
      "@opentelemetry/api",
    ]);

    cleanups.push(async () => app.close());

    await app.copyFixture("acceptance");

    await app.copyFixture("application-otlp");

    await app.copyFixture("foreign-tracing-owner");

    const env = {
      INTERFERE_PUBLIC_KEY: "interfere_public_us_acceptance",
      INTERFERE_API_KEY: HOST_KEY,
      INTERFERE_API_URL: collector.url,
      INTERFERE_SOURCE_ID: "0123456789abcdef0123456789abcdef01234567",
      INTERFERE_ENVIRONMENT: "preview",
      OTEL_TRACES_SAMPLER: "parentbased_always_on",
    };

    const directory = await mkdtemp(join(tmpdir(), "foreign-writer-"));

    cleanups.push(async () => rm(directory, { force: true, recursive: true }));

    [writer] = await Promise.all([
      bundleForeignWriter(directory),
      app.buildNext(env, "turbopack"),
    ]);

    const started = await app.start("serve.ts", {
      ...env,
      OTEL_BSP_SCHEDULE_DELAY: "250",
      OTEL_EXPORTER_OTLP_ENDPOINT: `${collector.url}/drain`,
      OTEL_EXPORTER_OTLP_PROTOCOL: "http/json",
      OTEL_EXPORTER_OTLP_HEADERS: `Authorization=Bearer%20${HOST_KEY}`,
    });

    host = started;

    cleanups.push(async () => started.close());

    browser = await chromium.launch({ headless: true });

    cleanups.push(async () => browser.close());
  }, 300_000);

  afterAll(async () => {
    for (const cleanup of cleanups.toReversed()) {
      // oxlint-disable-next-line eslint/no-await-in-loop -- Resources close in reverse order of creation.
      await cleanup();
    }
  });

  function hostSpans() {
    return exportedSpans(
      collector.requests.filter(
        (request) =>
          request.headers.get("authorization") === `Bearer ${HOST_KEY}`,
      ),
    );
  }

  function interfereRequests() {
    return collector.requests.filter(
      (request) =>
        request.url.startsWith("/drain/") &&
        request.headers.get("authorization") === null,
    );
  }

  function observationsFor(label: string) {
    return exportedSpans(interfereRequests()).filter(
      (span) =>
        attribute(span.attributes, "http.request.method") !== undefined &&
        String(attribute(span.attributes, "url.full") ?? "").includes(
          `case=${label}&`,
        ),
    );
  }

  function navigations(fragment: string) {
    return exportedLogRecords(interfereRequests()).flatMap((record) => {
      const type = attribute(record.attributes, "browser.navigation.type");
      const url = String(attribute(record.attributes, "url.full"));

      return type !== undefined && url.endsWith(fragment) ? [String(type)] : [];
    });
  }

  async function sampledMarker() {
    sequence += 1;

    const traceId = randomBytes(16).toString("hex");

    const response = await fetch(
      `${host.url}/api/journey?case=marker-${sequence}&kind=node&n=0`,
      {
        headers: {
          traceparent: `00-${traceId}-${randomBytes(8).toString("hex")}-01`,
        },
      },
    );

    await response.text();

    return traceId;
  }

  function pageLoads(session: string | null) {
    return exportedLogRecords(interfereRequests())
      .filter(
        (record) =>
          attribute(record.attributes, "browser.navigation.type") ===
            "pageload" &&
          attribute(record.attributes, "session.id") === session,
      )
      .map(
        (record) =>
          new URL(String(attribute(record.attributes, "url.full"))).pathname,
      );
  }

  function installScript(options: ForeignOptions) {
    return `${writer}\nglobalThis.foreignWriter.install(${JSON.stringify({ ...options, allowedOrigins: [host.url] })});`;
  }

  async function installWriter(page: Page, options: ForeignOptions) {
    await page.addScriptTag({ content: installScript(options) });
  }

  async function openJourney(options: JourneyOptions = {}) {
    const {
      foreign = {},
      interfere = true,
      order = "interfere-first",
    } = options;

    const context = await browser.newContext();

    onTestFinished(async () => context.close());

    if (interfere) {
      await context.addInitScript(() => {
        Object.assign(globalThis, { __INTERFERE_FORCE_ENABLE__: true });
      });
    }

    if (foreign !== false && order === "foreign-first") {
      await context.addInitScript({
        content: installScript(foreign),
      });
    }

    const page = await context.newPage();

    const consoleLines: string[] = [];

    page.on("console", (message) => {
      consoleLines.push(message.text());
    });

    await page.goto(`${host.url}/journey`);

    if (interfere) {
      await page.waitForFunction(
        () =>
          /^[\da-f-]{36}$/u.test(
            document.querySelector('output[aria-label="Session"]')
              ?.textContent ?? "",
          ),
        undefined,
        { timeout: 4000 },
      );
    }

    const session = interfere
      ? await page.getByLabel("Session").textContent()
      : null;

    if (foreign !== false && order === "interfere-first") {
      await installWriter(page, foreign);
    }

    if (foreign !== false && foreign.sessionDelayMs === undefined) {
      await page.waitForFunction(
        "globalThis.foreignWriter.hasSession()",
        undefined,
        { timeout: 2000 },
      );
    }

    return { consoleLines, page, session };
  }

  async function journey(page: Page, name: string) {
    sequence += 1;

    const label = `${name}-${sequence}`;

    const bodies = await page.evaluate(async (needle) => {
      const url = (kind: string, index: number) =>
        `/api/journey?case=${needle}&kind=${kind}&n=${index}`;

      const viaFetch = async (index: number) =>
        (await fetch(url("fetch", index))).text();

      const viaXhr = async (index: number) =>
        new Promise<string>((resolve, reject) => {
          const xhr = new XMLHttpRequest();

          xhr.addEventListener("load", () => {
            resolve(xhr.responseText);
          });

          xhr.addEventListener("error", () => {
            reject(new Error("xhr failed"));
          });

          xhr.open("GET", url("xhr", index));

          xhr.send();
        });

      return Promise.all([viaFetch(0), viaXhr(0), viaFetch(1), viaXhr(1)]);
    }, label);

    return {
      echoes: bodies.map((body) => echo.parse(JSON.parse(body))),
      label,
    };
  }

  return {
    hostSpans,
    interfereRequests,
    journey,
    navigations,
    observationsFor,
    openJourney,
    pageLoads,
    sampledMarker,
  };
}

/** Navigating away fires pagehide, which flushes Interfere's batched exports at once. */
export async function flushByLeaving(page: Page) {
  await page.goto("about:blank");
}

export function traceparentsOf(echoes: Echo[]) {
  return echoes.map((entry) => entry.headers["traceparent"] ?? null);
}

export function interfereBaggageKeys(echoes: Echo[]) {
  return echoes.flatMap((entry) =>
    (entry.headers["baggage"] ?? "")
      .split(",")
      .map((member) => member.split("=")[0]?.trim() ?? "")
      .filter((key) => key.startsWith("interfere.")),
  );
}
