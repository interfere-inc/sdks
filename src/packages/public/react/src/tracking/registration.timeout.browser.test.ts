import { HttpResponse, http } from "msw";
import { expect, onTestFinished, test, vi } from "vitest";

import { worker } from "../../test/msw.js";
import { registration, sessionEndpoint } from "./registration.fixture.js";

test("retries after the generated client's ten-second timeout aborts the transport", async () => {
  const { arrived, registrations, requests, settle, tracker } =
    await registration();

  const fetches = vi.spyOn(globalThis, "fetch");
  const response = Promise.withResolvers<Response>();

  worker.use(
    http.post(sessionEndpoint, async () => response.promise, { once: true }),
  );

  onTestFinished(() => response.resolve(HttpResponse.json({ ok: true })));

  tracker.start();

  await arrived(1);

  const signal = fetches.mock.calls[0]?.[1]?.signal;

  expect(signal).toBeInstanceOf(AbortSignal);

  await vi.advanceTimersByTimeAsync(9999);

  expect(signal?.aborted).toBe(false);
  expect(requests).toHaveLength(1);
  expect(registrations).toHaveBeenCalledTimes(1);

  await vi.advanceTimersByTimeAsync(1);

  await settle();

  expect(signal?.aborted).toBe(true);

  await vi.advanceTimersByTimeAsync(5000);

  await settle();

  expect(requests).toHaveLength(2);

  expect(registrations).toHaveBeenCalledTimes(2);

  await vi.advanceTimersByTimeAsync(60_000);

  expect(requests).toHaveLength(2);

  expect(registrations).toHaveBeenCalledTimes(2);
});

test.each([408, 429])(
  "retries transient status %i without another getter",
  async (status) => {
    const { registrations, requests, settle, tracker } = await registration();

    worker.use(
      http.post(sessionEndpoint, () => new HttpResponse(null, { status }), {
        once: true,
      }),
    );

    tracker.start();

    await settle();

    await vi.advanceTimersByTimeAsync(5000);

    await settle();

    expect(requests).toHaveLength(2);

    expect(registrations).toHaveBeenCalledTimes(2);
  },
);
