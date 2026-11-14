import { nativeBrowserFixture } from "@interfere/test-config/native-browser";
import type { Page } from "playwright";

import { handlers, wireFor } from "./foreign-sdk.network.js";
import type { WireRequest } from "./foreign-sdk.network.js";
import type { ForeignEvent, ForeignOptions } from "./foreign-writer.js";
import type {
  InterfereOptions,
  Observation,
  RequestKind,
  RequestResult,
  RequestSpec,
} from "./foreign-sdk.fixture.js";

export type HeaderOwner = "duplicate" | "foreign" | "interfere" | "none";

export interface Runtime {
  crossSite: () => Promise<string>;
  origin: string;
  page: Page;
  warnings: string[];
}

export interface Step {
  foreignEvents: ForeignEvent[];
  label: string;
  observations: Observation[];
  results: RequestResult[];
  wire: WireRequest[];
}

const createPage = nativeBrowserFixture(
  new URL("foreign-sdk.fixture.ts", import.meta.url),
  handlers,
);

let sequence = 0;

export async function openRuntime() {
  const created = await createPage();

  const warnings: string[] = [];

  created.page.on("console", (message) => {
    if (message.type() === "warning" || message.type() === "error") {
      warnings.push(message.text());
    }
  });

  created.page.on("pageerror", (error) => {
    warnings.push(`pageerror: ${error.message}`);
  });

  const { origin } = new URL(created.page.url());

  let sibling: string | null = null;

  return {
    crossSite: async () => {
      sibling ??= new URL((await createPage()).page.url()).origin;

      const url = new URL(sibling);

      url.hostname = "localhost";

      return url.origin;
    },
    origin,
    page: created.page,
    warnings,
  } satisfies Runtime;
}

export async function installForeign(
  runtime: Runtime,
  options: ForeignOptions = {},
  waitForSession = true,
) {
  await runtime.page.evaluate((input) => {
    globalThis.foreignSdkFixture.foreign.install(input);
  }, options);

  if (waitForSession) {
    await runtime.page.waitForFunction(
      () => globalThis.foreignSdkFixture.foreign.hasSession(),
      undefined,
      { timeout: 2000 },
    );
  }
}

export async function installInterfere(
  runtime: Runtime,
  options: InterfereOptions = {},
) {
  await runtime.page.evaluate((input) => {
    globalThis.foreignSdkFixture.interfere.install(input);
  }, options);
}

export async function disposeInterfere(runtime: Runtime) {
  await runtime.page.evaluate(async () =>
    globalThis.foreignSdkFixture.interfere.dispose(),
  );
}

function matches(url: string, label: string) {
  return decodeURIComponent(url).includes(`case=${label}&`);
}

export async function probe(
  runtime: Runtime,
  name: string,
  kinds: RequestKind[] = ["fetch", "xhr"],
  options: { path?: string; target?: string } = {},
) {
  sequence += 1;

  const label = `${name}-${sequence}`;

  const specs = kinds.map((kind) => {
    const url = new URL(
      options.path ?? "/echo",
      options.target ?? runtime.origin,
    );

    url.searchParams.set("case", label);

    url.searchParams.set("kind", kind);

    return { kind, url: url.href } satisfies RequestSpec;
  });

  const results = await runtime.page.evaluate(
    async (list) => globalThis.foreignSdkFixture.requestMany(list),
    specs,
  );

  await runtime.page
    .waitForFunction(
      ([needle, count]) =>
        globalThis.foreignSdkFixture.interfere
          .observations()
          .filter((entry) =>
            decodeURIComponent(entry.url).includes(`case=${needle}&`),
          ).length >= count,
      [label, kinds.length] as const,
      { timeout: 1500 },
    )
    .catch(() => undefined);

  const snapshot = await runtime.page.evaluate(() => ({
    foreignEvents: globalThis.foreignSdkFixture.foreign.events(),
    observations: globalThis.foreignSdkFixture.interfere.observations(),
  }));

  return {
    foreignEvents: snapshot.foreignEvents.filter((event) =>
      matches(event.url, label),
    ),
    label,
    observations: snapshot.observations.filter((entry) =>
      matches(entry.url, label),
    ),
    results,
    wire: wireFor(label),
  } satisfies Step;
}

export function owner(value: string | null) {
  if (value === null) {
    return "none" satisfies HeaderOwner;
  }

  if (value.split(",").length > 1) {
    return "duplicate" satisfies HeaderOwner;
  }

  return (
    value.startsWith("00-0000000000000000") ? "foreign" : "interfere"
  ) satisfies HeaderOwner;
}

export function headerNames(entry: WireRequest) {
  return Object.entries(entry.headers)
    .filter(([, value]) => value !== null)
    .map(([name]) => name)
    .toSorted();
}

export function baggageKeys(value: string | null) {
  return (value ?? "")
    .split(",")
    .map((member) => member.split("=")[0]?.trim() ?? "")
    .filter((key) => key !== "")
    .toSorted();
}
