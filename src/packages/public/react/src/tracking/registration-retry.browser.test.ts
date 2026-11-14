import { HttpResponse, http } from "msw";
import { worker } from "../../test/msw.js";
import { expect, test, vi } from "vitest";

import { registration, sessionEndpoint } from "./registration.fixture.js";

test("retries a transient device registration without another session getter", async () => {
  const { registrations, requests, settle, tracker } = await registration();

  worker.use(
    http.post(sessionEndpoint, () => new HttpResponse(null, { status: 503 }), {
      once: true,
    }),
  );

  tracker.startDeviceIdentity();

  tracker.start();

  await settle();

  await settle();

  expect(requests).toHaveLength(1);

  expect(registrations).toHaveBeenCalledTimes(1);
  expect(tracker.getDeviceId()).not.toBeNull();

  await vi.advanceTimersByTimeAsync(5000);

  await settle();

  expect(requests).toHaveLength(2);

  expect(registrations).toHaveBeenCalledTimes(2);

  await expect(requests[1]?.text()).resolves.toContain(tracker.getDeviceId());

  await vi.advanceTimersByTimeAsync(60_000);

  expect(requests).toHaveLength(2);

  expect(registrations).toHaveBeenCalledTimes(2);
});

test("bounds autonomous retries and preserves later on-demand registration", async () => {
  const { registrations, requests, settle, tracker } = await registration();

  worker.use(
    http.post(sessionEndpoint, () => new HttpResponse(null, { status: 503 })),
  );

  tracker.start();

  await settle();

  await vi.advanceTimersByTimeAsync(5000);

  await settle();

  expect(requests).toHaveLength(2);

  expect(registrations).toHaveBeenCalledTimes(2);

  await vi.advanceTimersByTimeAsync(15_000);

  await settle();

  expect(requests).toHaveLength(3);

  expect(registrations).toHaveBeenCalledTimes(3);

  await vi.advanceTimersByTimeAsync(60_000);

  expect(requests).toHaveLength(3);

  expect(registrations).toHaveBeenCalledTimes(3);

  tracker.sessionId();

  await settle();

  expect(requests).toHaveLength(4);

  expect(registrations).toHaveBeenCalledTimes(4);
});

test("does not autonomously retry a network failure and backs off on-demand registration", async () => {
  const { requests, settle, tracker } = await registration();

  worker.use(http.post(sessionEndpoint, () => HttpResponse.error()));

  tracker.start();

  await settle();

  await vi.advanceTimersByTimeAsync(60_000);

  expect(requests).toHaveLength(1);

  tracker.sessionId();

  await settle();

  expect(requests).toHaveLength(2);

  await vi.advanceTimersByTimeAsync(6000);

  tracker.sessionId();

  await settle();

  expect(requests).toHaveLength(2);

  await vi.advanceTimersByTimeAsync(5000);

  tracker.sessionId();

  await settle();

  expect(requests).toHaveLength(3);
});

test.each([400, 401, 403, 422])(
  "does not autonomously retry status %i",
  async (status) => {
    const { registrations, requests, settle, tracker } = await registration();

    worker.use(
      http.post(sessionEndpoint, () => new HttpResponse(null, { status })),
    );

    tracker.start();

    await settle();

    await vi.advanceTimersByTimeAsync(60_000);

    expect(requests).toHaveLength(1);

    expect(registrations).toHaveBeenCalledTimes(1);
  },
);

test("coalesces a timer with a simultaneous getter request", async () => {
  const { arrived, registrations, requests, settle, tracker } =
    await registration();

  const response = Promise.withResolvers<Response>();

  worker.use(
    http.post(sessionEndpoint, () => new HttpResponse(null, { status: 503 }), {
      once: true,
    }),
    http.post(sessionEndpoint, async () => response.promise),
  );

  tracker.start();

  await settle();

  await vi.advanceTimersByTimeAsync(5000);

  await arrived(2);

  tracker.sessionId();

  await vi.advanceTimersByTimeAsync(0);

  expect(requests).toHaveLength(2);

  expect(registrations).toHaveBeenCalledTimes(2);

  response.resolve(Response.json({ ok: true }));

  await settle();

  await vi.advanceTimersByTimeAsync(60_000);

  expect(requests).toHaveLength(2);

  expect(registrations).toHaveBeenCalledTimes(2);
});
