import { runInNewContext } from "node:vm";
import { expect, test } from "vitest";
import { releaseSlugSchema } from "@interfere/types/releases/slug";

import type { ResolvedBuildConfig } from "../../config.js";
import { FatalError } from "../logger.js";
import { interfereCompilerConfig } from "./production-publication.js";

const context = { projectDir: process.cwd(), distDir: ".next" };

const config = {
  apiKey: "interfere_ak_fixture",
  apiUrl: "https://in.example.com",
  buildId: "a".repeat(40),
  environment: "preview",
  purgeSourceMaps: true,
  releaseSlug: releaseSlugSchema.parse("rel_aaaaaaaaaaaaaaaa"),
  uploadSourceMaps: true,
} satisfies ResolvedBuildConfig;

const foreignFailure: unknown = runInNewContext(
  'new Error("upload response lost")',
);

if (!Error.isError(foreignFailure)) {
  throw new TypeError("Expected native foreign error fixture");
}

test.each([new Error("upload response lost"), foreignFailure])(
  "unexpected publication rejection preserves the customer build: %s",
  async (failure) => {
    expect.hasAssertions();

    const order: string[] = [];

    const compiler = interfereCompilerConfig(
      {
        runAfterProductionCompile: async () => {
          await Promise.resolve();

          order.push("customer");
        },
      },
      config,
      async (receivedContext, receivedConfig) => {
        expect(receivedContext).toBe(context);
        expect(receivedConfig).toBe(config);

        order.push("publication");

        await Promise.reject(failure);
      },
    );

    await expect(
      compiler.runAfterProductionCompile(context),
    ).resolves.toBeUndefined();

    expect(order).toStrictEqual(["customer", "publication"]);
  },
);

test("fatal publication failures remain fatal", async () => {
  const failure = new FatalError("invalid release configuration");

  const compiler = interfereCompilerConfig(undefined, config, async () => {
    await Promise.resolve();

    throw failure;
  });

  await expect(compiler.runAfterProductionCompile(context)).rejects.toBe(
    failure,
  );
});

test("a rejected customer hook prevents publication and remains visible", async () => {
  const failure = new Error("customer compiler validation failed");
  let publications = 0;

  const compiler = interfereCompilerConfig(
    {
      runAfterProductionCompile: async () => {
        await Promise.resolve();

        throw failure;
      },
    },
    config,
    async () => {
      await Promise.resolve();

      publications += 1;
    },
  );

  await expect(compiler.runAfterProductionCompile(context)).rejects.toBe(
    failure,
  );

  expect(publications).toBe(0);
});
