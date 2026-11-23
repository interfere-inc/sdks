import { setupErrorsPlugin } from "../../../test/fixtures/errors.test-utils.js";
import { onConsoleCall, resetConsolePatchForTests } from "./console-patch.js";
import type { ConsoleHandler } from "./console-patch.js";
import { afterEach, expect, onTestFinished, test, vi } from "vitest";

afterEach(() => {
  resetConsolePatchForTests();

  vi.restoreAllMocks();
});

function revokedProxy() {
  const { proxy, revoke } = Proxy.revocable({}, {});

  revoke();

  return proxy;
}

function prototypeTrapProxy() {
  return new Proxy(
    {},
    {
      getPrototypeOf() {
        throw new Error("prototype trap");
      },
    },
  );
}

function throwingMessageError() {
  const error = new Error("hidden");

  Object.defineProperty(error, "message", {
    get() {
      throw new Error("message getter");
    },
  });

  return error;
}

function throwingNameError() {
  const error = new Error("hidden");

  Object.defineProperty(error, "name", {
    get() {
      throw new Error("name getter");
    },
  });

  return error;
}

test.each([
  { label: "a revoked proxy", make: revokedProxy },
  { label: "a proxy whose prototype trap throws", make: prototypeTrapProxy },
  { label: "an Error whose message getter throws", make: throwingMessageError },
  { label: "an Error whose name getter throws", make: throwingNameError },
])(
  "console.error($label) never throws into the caller while errors are captured",
  ({ make }) => {
    const hostError = console.error;

    // oxlint-disable-next-line landmines/no-test-doubles -- Vitest's browser console formatter throws on revoked proxies before SDK code runs; a host console that does not format arguments isolates the SDK subscriber.
    console.error = () => undefined;

    onTestFinished(() => {
      console.error = hostError;
    });

    setupErrorsPlugin();

    const value = make();

    expect(() => {
      console.error("context:", value);
    }).not.toThrow();
  },
);

test("a throwing subscriber neither throws into the caller nor starves later subscribers", () => {
  vi.spyOn(console, "warn");

  const later = vi.fn<ConsoleHandler>();

  onConsoleCall(() => {
    throw new Error("subscriber failure");
  });

  onConsoleCall(later);

  expect(() => {
    console.warn("host warning");
  }).not.toThrow();

  expect(later).toHaveBeenCalledExactlyOnceWith("warn", ["host warning"]);
});
