import { z } from "zod";
import { once } from "node:events";
import { createServer } from "node:http";
import { afterAll, beforeAll, expect, test } from "vitest";
import { setupServer } from "msw/node";
import { fetchNetwork } from "../fetch-network.ts";
import { checkUnhandledBrowserFrame } from "./browser-network-policy.ts";
import { HttpResponse, http } from "msw";

const origin = createServer((_request, response) => response.end("local"));
let page: URL;

beforeAll(async () => {
  origin.listen(0, "127.0.0.1");

  await once(origin, "listening");

  const { port } = z.object({ port: z.number() }).parse(origin.address());

  page = new URL(`http://127.0.0.1:${port}`);
});

afterAll(async () => origin[Symbol.asyncDispose]());

test("allows Vite assets, records unexpected HTTP requests, and keeps fetch handlers active", async () => {
  const unexpected: string[] = [];

  const network = fetchNetwork(
    http.get(new URL("/fixture", page).href, () => HttpResponse.text("mocked")),
  );

  network.configure({
    onUnhandledFrame: ({ frame }) =>
      checkUnhandledBrowserFrame(frame, page, unexpected),
  });

  network.enable();

  try {
    await expect((await fetch(new URL("/fixture", page))).text()).resolves.toBe(
      "mocked",
    );

    await expect(
      (await fetch(new URL("/@vite/client", page))).text(),
    ).resolves.toBe("local");

    const forbidden = new URL("/undeclared", page);

    const rejected = await fetch(forbidden);

    expect(rejected.status).toBe(500);

    await expect(rejected.json()).resolves.toMatchObject({
      message: `Unexpected request: GET ${forbidden.href}`,
    });

    expect(unexpected).toStrictEqual([`GET ${forbidden.href}`]);
  } finally {
    network.disable();
  }
});

test("only allows an unhandled WebSocket on the page host and port", async () => {
  const network = setupServer();
  const unexpected: string[] = [];
  const captured = Promise.withResolvers<{ protocol: string }>();

  network.listen({
    onUnhandledFrame({ frame }) {
      captured.resolve(frame);
    },
  });

  const socketUrl = new URL("/socket", page);

  socketUrl.protocol = "ws:";

  try {
    const socket = new WebSocket(socketUrl);

    const frame = await captured.promise;

    checkUnhandledBrowserFrame(frame, page, unexpected);

    expect(unexpected).toStrictEqual([]);

    expect(() =>
      checkUnhandledBrowserFrame(
        frame,
        new URL("http://localhost"),
        unexpected,
      ),
    ).toThrow(`Unexpected WebSocket: ${socketUrl.href}`);

    expect(unexpected).toStrictEqual([`WS ${socketUrl.href}`]);

    socket.close();
  } finally {
    network.close();
  }
});

test("rejects unsupported protocols", () => {
  expect(() =>
    checkUnhandledBrowserFrame({ protocol: "custom" }, page, []),
  ).toThrow("Unexpected network protocol: custom");
});
