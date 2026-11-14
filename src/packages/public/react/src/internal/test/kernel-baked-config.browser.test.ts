import { setInjectedConfiguration } from "@interfere/test-config/injected-configuration";
import { INGEST_URL_GLOBAL } from "@interfere/types/sdk/injected-globals";
import { PLUGIN_MANIFEST } from "@interfere/types/sdk/plugins/manifest";

import { DISABLE_ALL_PLUGINS } from "../../../test/fixtures/disable-all-plugins.js";
import { worker } from "../../../test/msw.js";
import { createKernel } from "../kernel.js";
import type { KernelOptions } from "../kernel.js";
import { HttpResponse, http } from "msw";
import { beforeEach, describe, expect, onTestFinished, test } from "vitest";

const API = "https://config-audit.invalid";

const INGEST = "https://ingest-audit.invalid";

function serveConfig(body: Parameters<typeof HttpResponse.json>[0]) {
  const requests: Request[] = [];

  worker.use(
    http.get(`${API}/v3/config`, ({ request }) => {
      requests.push(request);

      return HttpResponse.json(body);
    }),
  );

  return requests;
}

function serveSessions() {
  const hosts: string[] = [];
  const first = Promise.withResolvers<string>();

  worker.use(
    http.post("*/v3/session", ({ request }) => {
      hosts.push(new URL(request.url).origin);

      first.resolve(new URL(request.url).origin);

      return new HttpResponse(null, { status: 204 });
    }),
  );

  return Object.assign(hosts, { first: first.promise });
}

async function startKernel(opts: KernelOptions = {}) {
  const kernel = await createKernel({
    opts: { apiHost: API, tracing: false, ...opts },
  });

  onTestFinished(async () => {
    await kernel.dispose();
  });

  return kernel;
}

beforeEach(() => {
  setInjectedConfiguration("__INTERFERE_PUBLIC_KEY__", "test-key");

  setInjectedConfiguration("__INTERFERE_PLUGINS__", undefined);

  setInjectedConfiguration(INGEST_URL_GLOBAL, undefined);
});

describe("D-RC-05 baked plugins are authoritative over the live endpoint", () => {
  test("baked replay true wins over a live replay false until the next deploy", async () => {
    setInjectedConfiguration(
      "__INTERFERE_PLUGINS__",
      Object.fromEntries(PLUGIN_MANIFEST.map((p) => [p.name, true])),
    );

    const requests = serveConfig({ plugins: { replay: false } });

    serveSessions();

    const kernel = await startKernel();

    expect(requests).toHaveLength(0);
    expect(kernel.getPluginConfigState("replay")).toBe("enabled");
  });

  test("a baked errors false never installs the errors plugin during startup", async () => {
    setInjectedConfiguration("__INTERFERE_PLUGINS__", DISABLE_ALL_PLUGINS);

    serveSessions();

    const host = globalThis.onerror;
    const creating = startKernel();

    expect(globalThis.onerror).toBe(host);

    await creating;
  });
});

describe("D-RC-06 baked plugins missing a newer manifest key still count as baked", () => {
  test("a baked config without the logs key skips the runtime fetch and keeps replay false", async () => {
    const partial = Object.fromEntries(
      PLUGIN_MANIFEST.filter((p) => p.name !== "logs").map((p) => [
        p.name,
        p.name !== "replay",
      ]),
    );

    setInjectedConfiguration("__INTERFERE_PLUGINS__", partial);

    const requests = serveConfig({ plugins: {} });

    serveSessions();

    const kernel = await startKernel();

    expect(requests).toHaveLength(0);
    expect(kernel.getPluginConfigState("replay")).toBe("disabled");
    expect(kernel.getPluginConfigState("logs")).toBe("enabled");
  });
});

describe("D-RC-07 malformed baked plugins fall back to the runtime fetch", () => {
  test("a non-boolean baked value is ignored and the live config applies", async () => {
    setInjectedConfiguration("__INTERFERE_PLUGINS__", {
      ...Object.fromEntries(PLUGIN_MANIFEST.map((p) => [p.name, true])),
      replay: "no",
    });

    const requests = serveConfig({ plugins: { replay: false } });

    serveSessions();

    const kernel = await startKernel();

    expect(requests).toHaveLength(1);
    expect(kernel.getPluginConfigState("replay")).toBe("disabled");
    expect(kernel.remoteProtocolVersion).toBeNull();
  });
});

describe("D-RC-08 remote ingestUrl applies before the first ingest request", () => {
  test("the first session POST goes to the remote ingest host, not the config host", async () => {
    serveConfig({ ingestUrl: `${INGEST}/`, plugins: DISABLE_ALL_PLUGINS });

    const hosts = serveSessions();

    await startKernel();

    await expect(hosts.first).resolves.toBe(INGEST);

    expect(new Set(hosts)).toStrictEqual(new Set([INGEST]));
  });
});

describe("D-RC-09 the runtime config request omits cookies", () => {
  test("credentials are omit and the public key is sent", async () => {
    const requests = serveConfig({ plugins: DISABLE_ALL_PLUGINS });

    serveSessions();

    await startKernel();

    expect(requests).toHaveLength(1);
    expect(requests[0]?.credentials).toBe("omit");
    expect([...(requests[0]?.headers.values() ?? [])]).toContain("test-key");
  });
});

describe("D-RC-10 a newer protocol version and unknown fields are tolerated", () => {
  test("known settings still apply and a malformed version costs nothing", async () => {
    serveConfig({
      consent: { analytics: true, replay: true },
      futureSetting: { nested: ["value"] },
      plugins: { ...DISABLE_ALL_PLUGINS, errors: true },
      protocolVersion: 99,
    });

    serveSessions();

    const kernel = await startKernel();

    expect(kernel.getPluginConfigState("replay")).toBe("disabled");
    expect(kernel.getPluginConfigState("errors")).toBe("enabled");
    expect(kernel.consent.get()).toBeNull();
    expect(kernel.remoteProtocolVersion).toBe(99);

    worker.resetHandlers();

    serveConfig({ plugins: { replay: false }, protocolVersion: "one" });

    serveSessions();

    const malformed = await startKernel();

    expect(malformed.getPluginConfigState("replay")).toBe("disabled");
    expect(malformed.remoteProtocolVersion).toBeNull();
  });
});

describe("D-RC-11 remote config cannot widen local capture or consent", () => {
  const ALL_ON = Object.fromEntries(PLUGIN_MANIFEST.map((p) => [p.name, true]));

  test("a remote true does not enable a plugin the host disabled", async () => {
    serveConfig({ plugins: ALL_ON });

    serveSessions();

    const kernel = await startKernel();

    kernel.syncProviderConfig({
      consent: { analytics: true, replay: true },
      plugins: { ...DISABLE_ALL_PLUGINS, errors: true },
    });

    expect(kernel.getPluginConfigState("device")).toBe("enabled");
    expect(kernel.isDeviceInfoCaptureEnabled()).toBe(false);
    expect(kernel.device.getDeviceId()).toBeNull();
  });

  test("a remote consent object does not broaden consent the host withheld", async () => {
    serveConfig({
      consent: { analytics: true, replay: true },
      plugins: ALL_ON,
    });

    serveSessions();

    const kernel = await startKernel({
      consent: { analytics: false, replay: false },
    });

    expect(kernel.consent.get()).toStrictEqual({
      analytics: false,
      replay: false,
    });

    expect(kernel.isDeviceInfoCaptureEnabled()).toBe(false);
  });
});

describe("D-RC-03 / D-RC-13 an unusable config response leaves capture on local defaults", () => {
  test.each([
    { name: "503", reply: () => new HttpResponse("down", { status: 503 }) },
    {
      name: "malformed JSON",
      reply: () =>
        new HttpResponse("{", {
          headers: { "content-type": "application/json" },
        }),
    },
    { name: "non-object", reply: () => HttpResponse.json([1, 2]) },
    { name: "network error", reply: () => HttpResponse.error() },
  ])("$name", async ({ reply }) => {
    worker.use(http.get(`${API}/v3/config`, reply));

    const sessions = serveSessions();

    const kernel = await startKernel();

    expect(kernel.remoteProtocolVersion).toBeNull();
    expect(kernel.getPluginConfigState("errors")).toBe("enabled");
    expect(kernel.getPluginConfigState("logs")).toBe("enabled");
    expect(kernel.getPluginConfigState("replay")).toBe("enabled");
    expect(globalThis.onerror).not.toBeNull();

    await expect(sessions.first).resolves.toBe(API);
  });
});
