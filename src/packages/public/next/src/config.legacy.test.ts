import { afterEach, expect, test, vi } from "vitest";
import { PHASE_PRODUCTION_BUILD } from "next/constants";
import webpack from "webpack";
import type { Configuration } from "webpack";

import { withTempProject } from "../test/fixtures/build.js";
import { webpackContext } from "../test/fixtures/webpack.js";
import { withInterfere } from "./config.js";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import type { ProductionCompileContext } from "./config.js";

afterEach(() => {
  vi.restoreAllMocks();

  vi.unstubAllEnvs();
});

test("next 15 preserves customer hooks and publishes from the client compiler with a custom distDir", async () => {
  expect.hasAssertions();

  vi.stubEnv("INTERFERE_API_KEY", undefined);

  vi.stubEnv("INTERFERE_PUBLIC_KEY", undefined);

  vi.stubEnv("NEXT_PUBLIC_INTERFERE_PUBLIC_KEY", undefined);

  await withTempProject("next15-compile-", async (projectDir) => {
    process.chdir(projectDir);

    mkdirSync(join(projectDir, "node_modules/next"), { recursive: true });

    writeFileSync(
      join(projectDir, "node_modules/next/package.json"),
      JSON.stringify({ version: "15.3.9" }),
    );

    const compiled: ProductionCompileContext[] = [];

    const customerCompile = async (context: ProductionCompileContext) => {
      await Promise.resolve();

      compiled.push(context);
    };

    let webpackCalls = 0;

    const customerWebpack = (config: Configuration) => {
      webpackCalls += 1;

      return config;
    };

    const config = await withInterfere({
      distDir: "custom-output",
      compiler: {
        removeConsole: true,
        runAfterProductionCompile: customerCompile,
      },
      webpack: customerWebpack,
    })(PHASE_PRODUCTION_BUILD);

    expect(config.compiler).toStrictEqual({ removeConsole: true });

    const clientConfig: Configuration = { mode: "none" };

    const options = { ...(await webpackContext(projectDir)), dev: false };

    config.webpack?.(clientConfig, options);

    expect(webpackCalls).toBe(1);

    const compiler = webpack(clientConfig);

    await compiler.hooks.done.promise(
      new webpack.Stats(
        compiler.newCompilation(compiler.newCompilationParams()),
      ),
    );

    expect(compiled).toStrictEqual([
      {
        projectDir,
        distDir: "custom-output",
      },
    ]);
  });
});

test("next 15.4 and later publish from the production compile hook so Turbopack builds upload", async () => {
  expect.hasAssertions();

  vi.stubEnv("INTERFERE_API_KEY", undefined);

  vi.stubEnv("INTERFERE_PUBLIC_KEY", undefined);

  vi.stubEnv("NEXT_PUBLIC_INTERFERE_PUBLIC_KEY", undefined);

  await withTempProject("next15-turbopack-compile-", async (projectDir) => {
    process.chdir(projectDir);

    mkdirSync(join(projectDir, "node_modules/next"), { recursive: true });

    writeFileSync(
      join(projectDir, "node_modules/next/package.json"),
      JSON.stringify({ version: "15.5.28" }),
    );

    const compiled: ProductionCompileContext[] = [];

    const customerCompile = async (context: ProductionCompileContext) => {
      await Promise.resolve();

      compiled.push(context);
    };

    const config = await withInterfere({
      distDir: "custom-output",
      compiler: { runAfterProductionCompile: customerCompile },
    })(PHASE_PRODUCTION_BUILD);

    const clientConfig: Configuration = { mode: "none" };

    config.webpack?.(clientConfig, {
      ...(await webpackContext(projectDir)),
      dev: false,
    });

    expect(clientConfig.plugins ?? []).toHaveLength(0);

    await config.compiler?.runAfterProductionCompile?.({
      projectDir,
      distDir: "custom-output",
    });

    expect(compiled).toStrictEqual([
      {
        projectDir,
        distDir: "custom-output",
      },
    ]);
  });
});
