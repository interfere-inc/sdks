import { expect, test } from "vitest";
import { z } from "zod";
import config from "../tsdown.config";

test("passes glob entries to rolldown in a stable order", async () => {
  const entries = {
    provider: "/src/provider.tsx",
    "internal/otel/exporter": "/src/internal/otel/exporter.ts",
    api: "/src/api.ts",
    "internal/kernel": "/src/internal/kernel.ts",
  };

  const order = async (input: Record<string, string>) => {
    const inputOptions = z
      .function({
        input: [
          z.object({ input: z.record(z.string(), z.string()) }),
          z.literal("es"),
          z.object({ cjsDts: z.boolean() }),
        ],
      })
      .parse(config.inputOptions);

    const options = z
      .object({ input: z.record(z.string(), z.string()) })
      .parse(await inputOptions({ input }, "es", { cjsDts: false }));

    return Object.keys(options.input);
  };

  const expected = [
    "api",
    "internal/kernel",
    "internal/otel/exporter",
    "provider",
  ];

  await expect(order(entries)).resolves.toStrictEqual(expected);

  await expect(
    order(Object.fromEntries(Object.entries(entries).toReversed())),
  ).resolves.toStrictEqual(expected);
});
