import { setupServer } from "msw/node";
import { afterAll, afterEach, beforeAll, beforeEach } from "vitest";

const REQUEST_CONTEXT = Symbol.for("@next/request-context");

export const network = setupServer();
export const requests: Request[] = [];

const waitUntil: Promise<unknown>[] = [];

/** Settles every delivery the SDK handed to the Edge request's waitUntil. */
export async function delivered() {
  await Promise.all(waitUntil.splice(0));
}

beforeAll(() => {
  network.events.on("request:start", ({ request }) =>
    requests.push(request.clone()),
  );

  network.listen({ onUnhandledRequest: "error" });
});

beforeEach(() => {
  requests.length = 0;

  waitUntil.length = 0;

  network.resetHandlers();

  Object.assign(globalThis, {
    [REQUEST_CONTEXT]: {
      get: () => ({
        waitUntil: (promise: Promise<unknown>) => {
          waitUntil.push(promise);
        },
      }),
    },
  });
});

afterEach(() => {
  Reflect.deleteProperty(globalThis, REQUEST_CONTEXT);
});

afterAll(() => network.close());
