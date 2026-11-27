import { InterfereClient } from "@interfere/javascript";
import { HttpResponse, http } from "msw";
import { onTestFinished, vi } from "vitest";
import type { MockInstance } from "vitest";

import { worker } from "../../test/msw.js";
import { SessionTracker } from "./api.js";
import { DeviceManager } from "./device.js";

export const sessionEndpoint = "https://registration.invalid/v3/session";

interface RegistrationFixture {
  arrived: (count: number) => Promise<void>;
  registrations: MockInstance<InterfereClient["sessions"]["createSession"]>;
  requests: Request[];
  settle: () => Promise<void>;
  tracker: SessionTracker;
}

export async function registration() {
  const device = new DeviceManager();

  device.init();

  await device.whenFingerprintReady();

  vi.useFakeTimers({ toFake: ["Date", "setTimeout", "clearTimeout"] });

  const requests: Request[] = [];

  const client = new InterfereClient({
    baseUrl: "https://registration.invalid",
    releaseTokenHeader: { apiKey: "test" },
  });

  // oxlint-disable-next-line typescript/no-unsafe-assignment
  const prototype: InterfereClient["sessions"] = Object.getPrototypeOf(
    client.sessions,
  );

  const registrations = vi.spyOn(prototype, "createSession");

  worker.use(http.post(sessionEndpoint, () => HttpResponse.json({ ok: true })));

  function recordRequest({ request }: { request: Request }) {
    if (request.url === sessionEndpoint) {
      requests.push(request.clone());
    }
  }

  worker.events.on("request:start", recordRequest);

  const tracker = new SessionTracker({
    device,
    target: {
      baseUrl: "https://registration.invalid",
      releaseTokenHeader: { apiKey: "test" },
    },
  });

  async function settle() {
    await vi.advanceTimersByTimeAsync(0);

    await Promise.allSettled(
      registrations.mock.results.flatMap((result) =>
        result.type === "return" ? [result.value] : [],
      ),
    );
  }

  async function arrived(count: number) {
    const deadline = performance.now() + 10_000;

    await new Promise<void>((resolve, reject) => {
      function inspect() {
        if (requests.length >= count) {
          resolve();

          return;
        }

        if (performance.now() > deadline) {
          reject(
            new Error(
              `Expected ${count} registration requests, received ${requests.length}`,
            ),
          );

          return;
        }

        requestAnimationFrame(inspect);
      }

      inspect();
    });
  }

  onTestFinished(() => {
    tracker.dispose();

    tracker.discardDeviceIdentity();

    worker.events.removeListener("request:start", recordRequest);

    for (const key of [
      "interfere:session_id",
      "interfere:last_activity",
      "interfere:session_started",
    ]) {
      localStorage.removeItem(key);
    }

    vi.useRealTimers();
  });

  const fixture: RegistrationFixture = {
    arrived,
    registrations,
    requests,
    settle,
    tracker,
  };

  return fixture;
}
