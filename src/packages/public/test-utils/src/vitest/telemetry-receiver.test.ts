import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { gzipSync } from "node:zlib";
import { request as requestHttp } from "node:http";
import type { IncomingMessage } from "node:http";
import { buffer } from "node:stream/consumers";
import { afterEach, beforeEach, expect, test } from "vitest";
import { receiver } from "./telemetry-receiver";

let collector: Awaited<ReturnType<typeof receiver>>;

beforeEach(async () => {
  collector = await receiver();
});

afterEach(async () => {
  await collector.close();
});

test("accepts preflight without recording it and decodes compressed telemetry", async () => {
  const preflight = await fetch(`${collector.url}/drain/v1/traces`, {
    method: "OPTIONS",
    headers: {
      "access-control-request-headers": "authorization, content-encoding",
    },
  });

  expect(preflight.status).toBe(204);

  expect(preflight.headers.get("access-control-allow-headers")).toBe(
    "authorization, content-encoding",
  );

  expect(collector.requests).toStrictEqual([]);

  const payload = JSON.stringify({ resourceSpans: [] });

  const result = await fetch(`${collector.url}/drain/v1/traces?batch=1`, {
    method: "POST",
    headers: { "content-encoding": "gzip", authorization: "test-key" },
    body: gzipSync(payload),
  });

  await expect(result.json()).resolves.toStrictEqual({});

  expect(collector.requests).toHaveLength(1);

  expect(collector.requests[0]).toMatchObject({
    method: "POST",
    url: "/drain/v1/traces?batch=1",
    body: payload,
    status: 200,
  });

  expect(collector.requests[0]?.headers.get("authorization")).toBe("test-key");
});

test("exhausts transient failures and resets pending failures and history", async () => {
  expect.hasAssertions();

  collector.fail("/v3/config", 503, 2, '{"retry":true}');

  await Promise.all(
    [0, 1].map(async () => {
      const result = await fetch(`${collector.url}/v3/config`);

      expect(result.status).toBe(503);

      await expect(result.json()).resolves.toStrictEqual({ retry: true });
    }),
  );

  const recovered = await fetch(`${collector.url}/v3/config`);

  await expect(recovered.json()).resolves.toStrictEqual({
    plugins: {
      errors: true,
      logs: true,
      device: true,
      pageEvents: true,
      rageClick: true,
      replay: true,
    },
  });

  expect(collector.requests.map((request) => request.status)).toStrictEqual([
    503, 503, 200,
  ]);

  collector.fail("/drain", 401);

  collector.reset();

  expect(collector.requests).toStrictEqual([]);

  expect((await fetch(`${collector.url}/drain`)).status).toBe(200);
});

test("forwards original compressed bytes and upstream response status and headers", async () => {
  expect.hasAssertions();

  const payload = gzipSync("telemetry");

  collector.forward(async (request) => {
    expect(request.method).toBe("POST");
    expect(request.headers.get("content-encoding")).toBe("gzip");

    expect(Buffer.from(await request.arrayBuffer())).toStrictEqual(payload);

    return new Response("accepted", {
      status: 202,
      headers: { "x-upstream": "real" },
    });
  });

  const result = await fetch(`${collector.url}/drain/v1/traces`, {
    method: "POST",
    headers: { "content-encoding": "gzip" },
    body: payload,
  });

  expect(result.status).toBe(202);
  expect(result.headers.get("x-upstream")).toBe("real");

  await expect(result.text()).resolves.toBe("accepted");

  expect(collector.requests[0]).toMatchObject({
    status: 202,
    body: "telemetry",
  });
});

test("serves browser assets while keeping configuration requests on the collector", async () => {
  const directory = await mkdtemp(join(tmpdir(), "receiver-assets-"));

  try {
    await writeFile(join(directory, "index.html"), "<main>fixture</main>");

    await writeFile(join(directory, "app.js"), "globalThis.ready = true;");

    collector.serve(directory);

    const page = await fetch(collector.url);

    expect(page.headers.get("content-type")).toBe("text/html");

    await expect(page.text()).resolves.toBe("<main>fixture</main>");

    const script = await fetch(`${collector.url}/app.js`);

    expect(script.headers.get("content-type")).toBe("text/javascript");

    await expect(script.text()).resolves.toBe("globalThis.ready = true;");

    expect((await fetch(`${collector.url}/missing.js`)).status).toBe(500);

    expect((await fetch(`${collector.url}/v3/config`)).status).toBe(200);

    expect(collector.requests).toHaveLength(1);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});

test("records native GET request bodies without reconstructing their bytes", async () => {
  const body = "wire bytes";

  const response = await new Promise<IncomingMessage>((resolve, reject) => {
    const request = requestHttp(
      `${collector.url}/drain/v1/traces`,
      { method: "GET", headers: { "content-length": Buffer.byteLength(body) } },
      resolve,
    );

    request.once("error", reject);

    request.end(body);
  });

  expect(response.statusCode).toBe(200);

  expect((await buffer(response)).toString()).toBe("{}");

  expect(collector.requests).toMatchObject([
    { method: "GET", body, status: 200 },
  ]);
});

test.each([204, 205, 304])(
  "preserves a configured bodyless HTTP %s response",
  async (status) => {
    collector.fail("/drain", status, 1, "discarded body");

    const response = await fetch(`${collector.url}/drain`);

    expect(response.status).toBe(status);

    await expect(response.text()).resolves.toBe("");

    expect(collector.requests[0]?.status).toBe(status);

    expect((await fetch(`${collector.url}/drain`)).status).toBe(200);
  },
);

test("serves release bindings separately from runtime plugin configuration", async () => {
  const release = await fetch(`${collector.url}/v3/releases/config`);

  await expect(release.json()).resolves.toStrictEqual({
    org: { id: "org_acceptance", slug: "acceptance" },
    surface: {
      id: "surface_acceptance",
      slug: "web",
      sourceProvider: "github",
      destinationProvider: "cli",
      environmentFiltering: true,
      sdkConfig: null,
    },
  });

  const runtime = await fetch(`${collector.url}/v3/config`);

  await expect(runtime.json()).resolves.toMatchObject({
    plugins: { errors: true, replay: true },
  });
});

test.each([false, true])(
  "custom publication handlers receive decoded bytes and retain HTTP evidence (gzip=%s)",
  async (compressed) => {
    await collector.close();

    const bytes = Buffer.from([0, 127, 128, 255]);
    const received: Uint8Array[] = [];

    collector = await receiver({
      respond: (entry, body) => {
        if (entry.url !== "/v3/releases/upload?path=chunk.map") {
          return;
        }

        received.push(body);

        return Response.json(
          { objectKey: "accepted/chunk.map" },
          { status: 201, headers: { "x-publication": "retained" } },
        );
      },
    });

    const result = await fetch(
      `${collector.url}/v3/releases/upload?path=chunk.map`,
      {
        method: "PUT",
        headers: compressed ? { "content-encoding": "gzip" } : {},
        body: compressed ? gzipSync(bytes) : bytes,
      },
    );

    expect(result.status).toBe(201);
    expect(result.headers.get("x-publication")).toBe("retained");
    expect(result.headers.get("access-control-allow-origin")).toBe("*");

    await expect(result.json()).resolves.toStrictEqual({
      objectKey: "accepted/chunk.map",
    });

    expect(received.map((body) => Buffer.from(body))).toStrictEqual([bytes]);

    expect(collector.requests).toMatchObject([
      {
        method: "PUT",
        url: "/v3/releases/upload?path=chunk.map",
        body: bytes.toString(),
        status: 201,
      },
    ]);

    const config = await fetch(`${collector.url}/v3/config`);

    await expect(config.json()).resolves.toMatchObject({
      plugins: { errors: true },
    });
  },
);

test("injected failures prevent custom publication until the retry is accepted", async () => {
  await collector.close();

  const accepted: string[] = [];

  collector = await receiver({
    respond: (entry) => {
      accepted.push(entry.body);

      return Response.json({ accepted: true }, { status: 202 });
    },
  });

  collector.fail("/v3/releases", 503, 1);

  const upload = async () =>
    fetch(`${collector.url}/v3/releases/upload`, {
      method: "PUT",
      body: "same map bytes",
    });

  const failed = await upload();

  expect(failed.status).toBe(503);

  await failed.arrayBuffer();

  expect(accepted).toStrictEqual([]);

  const retry = await upload();

  expect(retry.status).toBe(202);

  await expect(retry.json()).resolves.toStrictEqual({ accepted: true });

  expect(accepted).toStrictEqual(["same map bytes"]);

  expect(collector.requests.map(({ status }) => status)).toStrictEqual([
    503, 202,
  ]);
});
