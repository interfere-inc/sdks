import { HttpResponse, http } from "msw";
import { worker } from "../../test/msw.js";
import { expect, test, vi } from "vitest";

import { registration, sessionEndpoint } from "./registration.fixture.js";

test.each(["dispose", "discardDeviceIdentity"] as const)(
  "cancels pending retry on %s",
  async (action) => {
    const { registrations, requests, settle, tracker } = await registration();

    worker.use(
      http.post(
        sessionEndpoint,
        () => new HttpResponse(null, { status: 503 }),
        { once: true },
      ),
    );

    tracker.startDeviceIdentity();

    tracker.start();

    await settle();

    expect(requests).toHaveLength(1);

    expect(registrations).toHaveBeenCalledTimes(1);

    tracker[action]();

    await vi.advanceTimersByTimeAsync(60_000);

    expect(requests).toHaveLength(1);

    expect(registrations).toHaveBeenCalledTimes(1);
  },
);

test("reset registers current identity and cancels the old scheduled retry", async () => {
  const { registrations, requests, settle, tracker } = await registration();

  worker.use(
    http.post(sessionEndpoint, () => new HttpResponse(null, { status: 503 }), {
      once: true,
    }),
  );

  tracker.startDeviceIdentity();

  tracker.start();

  await settle();

  const previous = tracker.getDeviceId();

  tracker.reset();

  await settle();

  expect(tracker.getDeviceId()).not.toBe(previous);
  expect(requests).toHaveLength(2);
  expect(registrations).toHaveBeenCalledTimes(2);

  await expect(requests[1]?.text()).resolves.toContain(tracker.getDeviceId());

  await vi.advanceTimersByTimeAsync(60_000);

  expect(requests).toHaveLength(2);

  expect(registrations).toHaveBeenCalledTimes(2);
});

test.each(["dispose", "discardDeviceIdentity"] as const)(
  "late failure cannot restore a retry after %s",
  async (action) => {
    const { arrived, registrations, requests, settle, tracker } =
      await registration();

    const response = Promise.withResolvers<Response>();

    worker.use(
      http.post(sessionEndpoint, async () => response.promise, { once: true }),
    );

    tracker.startDeviceIdentity();

    tracker.start();

    await vi.advanceTimersByTimeAsync(0);

    await arrived(1);

    tracker[action]();

    response.resolve(new HttpResponse(null, { status: 503 }));

    await settle();

    await vi.advanceTimersByTimeAsync(60_000);

    expect(requests).toHaveLength(1);

    expect(registrations).toHaveBeenCalledTimes(1);
  },
);

test("late success from before reset cannot clear the current registration retry", async () => {
  const { arrived, registrations, requests, settle, tracker } =
    await registration();

  const response = Promise.withResolvers<Response>();

  worker.use(
    http.post(sessionEndpoint, async () => response.promise, { once: true }),
    http.post(sessionEndpoint, () => new HttpResponse(null, { status: 503 }), {
      once: true,
    }),
  );

  tracker.startDeviceIdentity();

  tracker.start();

  await vi.advanceTimersByTimeAsync(0);

  await arrived(1);

  tracker.reset();

  await arrived(2);

  response.resolve(Response.json({ ok: true }));

  await settle();

  await vi.advanceTimersByTimeAsync(5000);

  await settle();

  expect(requests).toHaveLength(3);

  expect(registrations).toHaveBeenCalledTimes(3);

  await expect(requests[2]?.text()).resolves.toContain(tracker.getDeviceId());
});
