import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { context, propagation, trace } from "@opentelemetry/api";
import {
  InMemoryLogRecordExporter,
  SimpleLogRecordProcessor,
} from "@opentelemetry/sdk-logs";
import { HttpResponse, http } from "msw";
import { afterEach, beforeEach, expect, test } from "vitest";

import { DISABLE_ALL_PLUGINS } from "../../test/fixtures/disable-all-plugins.js";
import { worker } from "../../test/msw.js";
import { createKernel } from "./kernel.js";
import type { Kernel } from "./kernel.js";
import type { KernelInternalOptions } from "./kernel/options.js";

const API = "https://kernel.invalid";

const OTHER = "https://other.invalid";

let ownedKernel: Kernel | undefined;

beforeEach(() => {
  ownedKernel = undefined;

  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "interfere_pk_test");

  setInjectedConfiguration("__INTERFERE_INGEST_URL__", undefined);

  trace.disable();

  context.disable();

  propagation.disable();
});

afterEach(async () => {
  try {
    await ownedKernel?.dispose();
  } finally {
    trace.disable();

    context.disable();

    propagation.disable();
  }
});

async function sendXhr(url: string) {
  return new Promise<void>((resolve, reject) => {
    const xhr = new XMLHttpRequest();

    xhr.open("POST", url);

    xhr.setRequestHeader("content-type", "application/json");

    xhr.addEventListener("load", () => resolve(), { once: true });

    xhr.addEventListener("error", () => reject(new Error("XHR failed")), {
      once: true,
    });

    xhr.send("{}");
  });
}

function sessionMembers(value: string | null) {
  return (value ?? "")
    .split(",")
    .map((member) => member.trim())
    .filter((member) => member.startsWith("interfere.session.id="));
}

function startingKernel(
  tracing: NonNullable<KernelInternalOptions["tracing"]>,
  opts: Omit<KernelInternalOptions, "apiHost" | "tracing"> = {},
) {
  let created: Kernel | undefined;
  const config = Promise.withResolvers<undefined>();
  const ingest: string[] = [];
  const seen = new Map<string, (string | null)[]>();

  worker.use(
    http.get(`${API}/v3/config`, async () => {
      await config.promise;

      return HttpResponse.json({ plugins: DISABLE_ALL_PLUGINS });
    }),
    http.all(`${API}/*`, ({ request }) => {
      ingest.push(new URL(request.url).pathname);

      return new HttpResponse(null, { status: 204 });
    }),
    http.all(`${OTHER}/*`, ({ request }) => {
      if (request.method === "OPTIONS") {
        return new HttpResponse(null, {
          headers: {
            "access-control-allow-headers": "*",
            "access-control-allow-origin": "*",
          },
          status: 204,
        });
      }

      seen.set(request.url, [
        ...(seen.get(request.url) ?? []),
        request.headers.get("baggage"),
      ]);

      return new HttpResponse(null, {
        headers: {
          "access-control-allow-headers": "*",
          "access-control-allow-origin": "*",
        },
        status: 204,
      });
    }),
    http.all(`${location.origin}/startup-*`, ({ request }) => {
      seen.set(request.url, [
        ...(seen.get(request.url) ?? []),
        request.headers.get("baggage"),
      ]);

      return new HttpResponse(null, { status: 204 });
    }),
  );

  const pending = createKernel({
    onCreated: (kernel) => {
      created = kernel;

      if (opts.consent === undefined) {
        kernel.consent.set(true);
      }
    },
    opts: { ...opts, apiHost: API, tracing },
  });

  return {
    created: () => created,
    ingest,
    ready: async () => {
      config.resolve(undefined);

      ownedKernel = await pending;

      return ownedKernel;
    },
    seen: (url: string) => seen.get(url) ?? [],
  };
}

test("requests sent while the kernel starts carry session baggage once, and the session registers only after remote config", async () => {
  expect.hasAssertions();

  const starting = startingKernel({ propagateTo: [OTHER] });
  const page = `${location.origin}/startup-page`;
  const other = `${OTHER}/startup-other`;

  await fetch(page, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: "{}",
  });

  await fetch(new Request(other, { headers: { baggage: "vendor=kept" } }));

  await sendXhr(page);

  expect(starting.ingest).toStrictEqual([]);

  const kernel = await starting.ready();

  const session = `interfere.session.id=${kernel.session.getId()}`;

  await fetch(page, { method: "POST", body: "{}" });

  await sendXhr(other);

  expect(
    starting.seen(page).map((value) => sessionMembers(value)),
  ).toStrictEqual([[session], [session], [session]]);

  expect(
    starting.seen(other).map((value) => sessionMembers(value)),
  ).toStrictEqual([[session], [session]]);

  expect(starting.seen(other)[0]).toMatch(/^vendor=kept, /u);
});

test("requests sent while the kernel starts carry no baggage to unlisted origins or ignored URLs", async () => {
  expect.hasAssertions();

  const ignored = `${location.origin}/startup-ignored`;
  const starting = startingKernel({ ignoreUrls: [ignored] });
  const page = `${location.origin}/startup-listed`;
  const other = `${OTHER}/startup-unlisted`;

  await fetch(page);

  await fetch(ignored);

  await fetch(other);

  const kernel = await starting.ready();

  expect(
    starting.seen(page).map((value) => sessionMembers(value)),
  ).toStrictEqual([[`interfere.session.id=${kernel.session.getId()}`]]);

  expect(starting.seen(ignored)).toStrictEqual([null]);

  expect(starting.seen(other)).toStrictEqual([null]);
});

test("requests sent while the kernel starts carry no baggage when baggage is off", async () => {
  expect.hasAssertions();

  const starting = startingKernel({ propagateTraceBaggage: false });
  const page = `${location.origin}/startup-off`;

  await fetch(page);

  await sendXhr(page);

  await starting.ready();

  expect(starting.seen(page)).toStrictEqual([null, null]);
});

test("analytics granted while the kernel starts links the stamped page session to the persisted one", async () => {
  expect.hasAssertions();

  const logs = new InMemoryLogRecordExporter();
  const page = `${location.origin}/startup-consent`;

  const starting = startingKernel(
    {},
    {
      _internalAdditionalLogRecordProcessors: [
        new SimpleLogRecordProcessor({ exporter: logs }),
      ],
      consent: "required",
    },
  );

  await fetch(page);

  const [stamped] = starting.seen(page).map((value) => sessionMembers(value));

  starting.created()?.consent.set(true);

  const kernel = await starting.ready();

  await kernel.flush();

  const starts = logs
    .getFinishedLogRecords()
    .filter((record) => record.eventName === "session.start")
    .map((record) => [
      `interfere.session.id=${String(record.attributes["session.previous_id"])}`,
      record.attributes["session.id"],
    ]);

  expect(stamped?.[0]).toMatch(/^interfere\.session\.id=page_/u);

  expect(starts).toStrictEqual([[stamped?.[0], kernel.session.getId()]]);
});
