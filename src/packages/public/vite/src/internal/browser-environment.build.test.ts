import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { runInNewContext } from "node:vm";
import { build } from "vite";
import { beforeAll, expect, test, vi } from "vitest";

const RESULT_GLOBAL = "interfereEnvironment";

async function bundleForBrowser() {
  const directory = await mkdtemp(path.join(tmpdir(), "interfere-browser-"));

  const entry = path.join(directory, "entry.ts");

  const resourceAttributes = path.join(
    import.meta.dirname,
    "../../../react/src/internal/capture/resource-attributes.ts",
  );

  await writeFile(
    entry,
    [
      `import { buildResourceAttributes } from ${JSON.stringify(resourceAttributes)};`,
      `globalThis.${RESULT_GLOBAL} = buildResourceAttributes({ sdkStack: [] })["deployment.environment.name"];`,
    ].join("\n"),
  );

  try {
    const output = await build({
      configFile: false,
      logLevel: "silent",
      mode: "production",
      root: directory,
      build: {
        minify: false,
        rolldownOptions: { input: entry, output: { format: "iife" } },
        write: false,
      },
    });

    const results = Array.isArray(output) ? output : [output];

    return results
      .flatMap((result) => ("output" in result ? result.output : []))
      .flatMap((file) =>
        file.type === "chunk" && file.isEntry ? [file.code] : [],
      )
      .join("\n");
  } finally {
    await rm(directory, { force: true, recursive: true });
  }
}

let code: string;

beforeAll(async () => {
  vi.stubEnv("NODE_ENV", "production");

  code = await bundleForBrowser();

  vi.unstubAllEnvs();
}, 60_000);

test("a production browser bundle reports production on a page with no process global", () => {
  expect(runInNewContext("typeof process", {})).toBe("undefined");

  expect(runInNewContext(`${code}\n${RESULT_GLOBAL}`, { console })).toBe(
    "production",
  );
});
