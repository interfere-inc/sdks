import webpack from "webpack";
import type { Configuration } from "webpack";
import { afterEach, expect, test, vi } from "vitest";

import { webpackContext } from "../../../test/fixtures/webpack.js";
import { withLegacyWebpackPublication } from "./legacy-webpack-publication.js";

const context = { projectDir: process.cwd(), distDir: "custom-output" };

afterEach(() => {
  vi.restoreAllMocks();
});

test("awaits publication once after customer completion hooks", async () => {
  expect.hasAssertions();

  const order: string[] = [];
  const pending = Promise.withResolvers<undefined>();
  let calls = 0;

  const publish = async () => {
    calls += 1;

    order.push("publish");

    await pending.promise;

    order.push("cleanup");
  };

  const customerConfig: Configuration = {
    mode: "none",
    plugins: [
      {
        apply(compiler) {
          compiler.hooks.done.tapPromise("Customer", async () => {
            await Promise.resolve();

            order.push("customer");
          });
        },
      },
    ],
  };

  const customer = () => customerConfig;
  const hook = withLegacyWebpackPublication(customer, context, publish);

  expect(
    hook({}, { ...(await webpackContext(context.projectDir)), dev: false }),
  ).toBe(customerConfig);

  const compiler = webpack(customerConfig);

  const stats = new webpack.Stats(
    compiler.newCompilation(compiler.newCompilationParams()),
  );

  const completion = compiler.hooks.done.promise(stats);

  await vi.waitFor(() => expect(calls).toBe(1));

  expect(order).toStrictEqual(["customer", "publish"]);

  pending.resolve(undefined);

  await completion;

  expect(order).toStrictEqual(["customer", "publish", "cleanup"]);

  await compiler.hooks.done.promise(stats);

  expect(calls).toBe(1);
});

test.each([
  { dev: true, isServer: false },
  { dev: false, isServer: true, nextRuntime: "nodejs" as const },
  { dev: false, isServer: true, nextRuntime: "edge" as const },
])(
  "does not publish from $nextRuntime server or development compilers",
  async (options) => {
    expect.hasAssertions();

    let calls = 0;

    const publish = async () => {
      await Promise.resolve();

      calls += 1;
    };

    const config: Configuration = {};
    const hook = withLegacyWebpackPublication(undefined, context, publish);

    expect(
      hook(config, {
        ...(await webpackContext(context.projectDir)),
        ...options,
      }),
    ).toBe(config);

    expect(config.plugins).toBeUndefined();
    expect(calls).toBe(0);
  },
);

test("does not publish failed client output", async () => {
  expect.hasAssertions();

  let calls = 0;

  const publish = async () => {
    await Promise.resolve();

    calls += 1;
  };

  const config: Configuration = { mode: "none" };
  const hook = withLegacyWebpackPublication(undefined, context, publish);

  hook(config, { ...(await webpackContext(context.projectDir)), dev: false });

  const compiler = webpack(config);
  const compilation = compiler.newCompilation(compiler.newCompilationParams());

  compilation.errors.push(new webpack.WebpackError("Compilation failed"));

  await compiler.hooks.done.promise(new webpack.Stats(compilation));

  expect(calls).toBe(0);
});

test.each([null, [], "invalid"])(
  "rejects an invalid customer webpack configuration before publication: %j",
  async (configured) => {
    let publications = 0;

    const hook = withLegacyWebpackPublication(
      () => configured,
      context,
      async () => {
        await Promise.resolve();

        publications += 1;
      },
    );

    const options = {
      ...(await webpackContext(context.projectDir)),
      dev: false,
    };

    expect(() => hook({}, options)).toThrow(
      "The webpack hook must return a configuration object.",
    );

    expect(publications).toBe(0);
  },
);

test("rejects a malformed customer plugin collection without replacing it", async () => {
  let publications = 0;
  const configured = { plugins: { customer: true } };

  const hook = withLegacyWebpackPublication(
    () => configured,
    context,
    async () => {
      await Promise.resolve();

      publications += 1;
    },
  );

  const options = {
    ...(await webpackContext(context.projectDir)),
    dev: false,
  };

  expect(() => hook({}, options)).toThrow("Webpack plugins must be an array.");
  expect(configured.plugins).toStrictEqual({ customer: true });
  expect(publications).toBe(0);
});
