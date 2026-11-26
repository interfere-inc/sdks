import {
  TRACEPARENT,
  driver,
  foreignSdkPages,
} from "../../../test/foreign/sdk.driver.js";
import type { SeenRequest } from "../../../test/foreign/sdk.driver.js";
import { beforeEach, expect, test } from "vitest";

const seen: SeenRequest[] = [];

const createPage = foreignSdkPages(seen);

beforeEach(() => {
  seen.length = 0;
});

const FOREIGN_HEADERS = {
  headers: { baggage: "foreign-key=1", "x-foreign-trace": "foreign-1" },
};

test("a foreign fetch wrapper installed first is called once per request and same-origin requests still carry traceparent (B1a)", async () => {
  const runtime = driver((await createPage()).page);

  await expect(runtime.foreign("fetch")).resolves.toBe(true);

  await expect(runtime.install()).resolves.toBe(1);

  await expect(runtime.request("/api/one")).resolves.toStrictEqual({
    status: 200,
    body: "ok",
  });

  expect(seen).toHaveLength(1);

  expect(seen[0]?.headers["traceparent"]).toMatch(TRACEPARENT);

  const state = await runtime.state();

  expect(state.calls.fetch).toBe(1);
});

test("a foreign fetch wrapper installed after Interfere survives close() (B1b)", async () => {
  const runtime = driver((await createPage()).page);

  await expect(runtime.install()).resolves.toBe(1);

  await expect(runtime.foreign("fetch")).resolves.toBe(true);

  await runtime.close();

  const closed = await runtime.state();

  expect(closed.fetchIsForeign).toBe(true);

  await expect(runtime.request("/api/after")).resolves.toStrictEqual({
    status: 200,
    body: "ok",
  });

  const after = await runtime.state();

  expect(after.calls.fetch).toBe(1);
});

test("the Interfere fetch wrapper left beneath a later foreign wrapper injects nothing after close() (B1c)", async () => {
  const runtime = driver((await createPage()).page);

  await expect(runtime.install()).resolves.toBe(1);

  await expect(runtime.foreign("fetch")).resolves.toBe(true);

  await runtime.close();

  await runtime.request("/api/after-close");

  expect(seen[0]?.headers["traceparent"]).toBeNull();

  expect(seen[0]?.headers["baggage"]).toBeNull();
});

test("foreign XHR open/send proxies installed first keep same-origin XHR tracing with one send (B2)", async () => {
  const runtime = driver((await createPage()).page);

  await expect(runtime.foreign("xhr")).resolves.toBe(true);

  await expect(runtime.install()).resolves.toBe(1);

  await expect(runtime.xhr("/api/xhr")).resolves.toStrictEqual({
    status: 200,
    body: "ok",
  });

  expect(seen[0]?.headers["traceparent"]).toMatch(TRACEPARENT);

  const { calls } = await runtime.state();

  expect(calls.xhrOpen).toBe(1);

  expect(calls.xhrSend).toBe(1);
});

test("a foreign wrapper beneath Interfere keeps its own headers and merges baggage (B3a)", async () => {
  const runtime = driver((await createPage()).page);

  await expect(runtime.foreign("fetch", FOREIGN_HEADERS)).resolves.toBe(true);

  await expect(runtime.install()).resolves.toBe(1);

  await runtime.request("/api/baggage");

  expect(seen[0]?.headers["x-foreign-trace"]).toBe("foreign-1");

  expect(seen[0]?.headers["traceparent"]).toMatch(TRACEPARENT);

  expect(seen[0]?.headers["baggage"]).toContain("foreign-key=1");

  expect(seen[0]?.headers["baggage"]).toContain("session.id=");
});

test("a foreign wrapper above Interfere keeps its baggage entries when Interfere injects (B3b)", async () => {
  const runtime = driver((await createPage()).page);

  await expect(runtime.install()).resolves.toBe(1);

  await expect(runtime.foreign("fetch", FOREIGN_HEADERS)).resolves.toBe(true);

  await runtime.request("/api/baggage");

  expect(seen[0]?.headers["x-foreign-trace"]).toBe("foreign-1");

  expect(seen[0]?.headers["baggage"]).toContain("session.id=");

  expect(seen[0]?.headers["baggage"]).toContain("foreign-key=1");
});

test("a foreign console.error wrapper installed after Interfere survives close() (B5)", async () => {
  const runtime = driver((await createPage()).page);

  await expect(runtime.install()).resolves.toBe(1);

  await expect(runtime.foreign("console")).resolves.toBe(true);

  await runtime.close();

  const state = await runtime.state();

  expect(state.consoleErrorIsForeign).toBe(true);
});

test("a foreign pushState wrapper installed after Interfere survives close() (B6a)", async () => {
  const runtime = driver((await createPage()).page);

  await expect(runtime.install()).resolves.toBe(1);

  await expect(runtime.foreign("history")).resolves.toBe(true);

  await runtime.close();

  const state = await runtime.state();

  expect(state.pushStateIsForeign).toBe(true);
});

test("a foreign pushState wrapper installed first is called once per soft navigation (B6b)", async () => {
  const { page } = await createPage();

  const runtime = driver(page);

  await expect(runtime.foreign("history")).resolves.toBe(true);

  await expect(runtime.install()).resolves.toBe(1);

  await runtime.navigate("/next");

  expect(new URL(page.url()).pathname).toBe("/next");

  const state = await runtime.state();

  expect(state.calls.pushState).toBe(1);
});

test("a foreign onerror wrapper installed after Interfere survives close() (B7a)", async () => {
  const runtime = driver((await createPage()).page);

  await expect(runtime.install()).resolves.toBe(1);

  await expect(runtime.foreign("onerror")).resolves.toBe(true);

  await runtime.close();

  const state = await runtime.state();

  expect(state.onerrorIsForeign).toBe(true);
});

test("a foreign onerror wrapper installed first sees an uncaught error once and Interfere records it once (B7b)", async () => {
  const runtime = driver((await createPage()).page);

  await expect(runtime.foreign("onerror")).resolves.toBe(true);

  await expect(runtime.install()).resolves.toBe(1);

  await runtime.throwLater("boom-foreign-first");

  await expect
    .poll(async () =>
      (await runtime.spans()).filter((span) =>
        span.events.some(
          (event) =>
            event.name === "exception" &&
            event.attributes["exception.message"] === "boom-foreign-first",
        ),
      ),
    )
    .toHaveLength(1);

  const state = await runtime.state();

  expect(state.calls.errorHandler).toBe(1);
});
