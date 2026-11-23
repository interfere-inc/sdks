import { execFileSync } from "node:child_process";
import { once } from "node:events";
import { mkdtemp, readFile, rm } from "node:fs/promises";
import type { IncomingMessage, ServerResponse } from "node:http";
import type { Http2ServerRequest, Http2ServerResponse } from "node:http2";
import { createSecureServer } from "node:http2";
import { createServer } from "node:https";
import type { AddressInfo } from "node:net";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { text } from "node:stream/consumers";
import { W3CTraceContextPropagator } from "@opentelemetry/core";
import { beforeAll, expect, test, vi } from "vitest";
import { startNodeRuntime } from "../node.js";

interface CollectorTls {
  key: Buffer;
  cert: Buffer;
}

let tls: CollectorTls;
let certificatePath: string;

beforeAll(async () => {
  const directory = await mkdtemp(join(tmpdir(), "interfere-collector-"));

  const keyPath = join(directory, "key.pem");

  certificatePath = join(directory, "cert.pem");

  execFileSync(
    "openssl",
    [
      "req",
      "-x509",
      "-newkey",
      "rsa:2048",
      "-nodes",
      "-days",
      "1",
      "-subj",
      "/CN=localhost",
      "-addext",
      "subjectAltName=IP:127.0.0.1",
      "-keyout",
      keyPath,
      "-out",
      certificatePath,
    ],
    { stdio: "ignore" },
  );

  tls = { key: await readFile(keyPath), cert: await readFile(certificatePath) };

  return async () => rm(directory, { recursive: true, force: true });
});

test.each(["2.0", "1.1"])(
  "the Node trace exporter delivers OTLP over negotiated HTTP/%s",
  async (version) => {
    expect.hasAssertions();

    let connections = 0;
    let inFlight = 0;
    let peak = 0;

    const received: {
      version: string;
      payload: Promise<string>;
      auth: string | undefined;
      path: string | undefined;
    }[] = [];

    function receive(
      req: IncomingMessage | Http2ServerRequest,
      res: ServerResponse | Http2ServerResponse,
    ) {
      inFlight += 1;

      peak = Math.max(peak, inFlight);

      received.push({
        version: req.httpVersion,
        payload: text(req),
        auth: req.headers.authorization,
        path: req.url,
      });

      setTimeout(() => {
        inFlight -= 1;

        res.setHeader("content-type", "application/json");

        res.end("{}");
      }, 40);
    }

    const server =
      version === "2.0"
        ? createSecureServer(tls, receive)
        : createServer(tls, receive);

    server.on("secureConnection", () => {
      connections += 1;
    });

    server.listen(0, "127.0.0.1");

    await once(server, "listening");

    vi.stubEnv("OTEL_EXPORTER_OTLP_TRACES_CERTIFICATE", certificatePath);

    vi.stubEnv("OTEL_BSP_MAX_EXPORT_BATCH_SIZE", "1");

    try {
      const { port } = server.address() as AddressInfo;

      const pipeline = await startNodeRuntime({
        name: "sdk-transport",
        version: "1",
        apiUrl: `https://127.0.0.1:${port}`,
        credential: { apiKey: "test-key" },
        resourceAttributes: {},
        options: { consoleBridge: false, tracing: "managed" },
        drain: false,
        logDrain: true,
        vercel: false,
        propagator: new W3CTraceContextPropagator(),
      });

      try {
        const tracer = pipeline.provider.getTracer("tls-test");
        const count = 8;

        for (let index = 0; index < count; index += 1) {
          tracer.startSpan("collector-protocol").end();
        }

        await pipeline.flush();

        expect(received).toHaveLength(count);

        const payloads = await Promise.all(
          received.map(async ({ payload }) => payload),
        );

        for (const payload of payloads) {
          expect(payload).toContain("collector-protocol");
        }

        for (const request of received) {
          expect(request.version).toBe(version);

          expect(request.auth).toBe("Bearer test-key");
          expect(request.path).toBe("/drain/v1/traces");
        }

        expect(peak).toBeGreaterThan(1);
        expect(connections === 1).toBe(version === "2.0");
      } finally {
        await pipeline.shutdown();
      }
    } finally {
      vi.unstubAllEnvs();

      await server[Symbol.asyncDispose]();
    }
  },
);
