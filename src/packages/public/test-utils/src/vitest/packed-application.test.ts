import {
  mkdir,
  mkdtemp,
  readFile,
  readdir,
  rm,
  writeFile,
} from "node:fs/promises";
import { execFile } from "node:child_process";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { promisify } from "node:util";
import { afterEach, beforeEach, expect, test, vi } from "vitest";
import { z } from "zod";
import { application } from "./packed-application";

let directory: string;
let scratch: string;

beforeEach(async () => {
  directory = await mkdtemp(join(tmpdir(), "packed-application-contract-"));

  scratch = join(directory, "scratch");

  await mkdir(scratch);

  vi.stubEnv("TMPDIR", scratch);

  await writeFile(
    join(directory, "package.json"),
    JSON.stringify({
      name: "@interfere/fixture",
      version: "1.0.0",
      files: ["index.cjs"],
      exports: "./index.cjs",
      dependencies: { external: "1.0.0" },
    }),
  );

  await writeFile(
    join(directory, "index.cjs"),
    'module.exports = require("external") + 1;',
  );

  await mkdir(join(directory, "node_modules/external"), { recursive: true });

  await writeFile(
    join(directory, "node_modules/external/package.json"),
    JSON.stringify({ name: "external", main: "index.cjs" }),
  );

  await writeFile(
    join(directory, "node_modules/external/index.cjs"),
    "module.exports = 41;",
  );
});

afterEach(async () => {
  vi.unstubAllEnvs();

  await rm(directory, { recursive: true, force: true });
});

test("runs an isolated packed consumer with explicit environment and cleans its files", async () => {
  vi.stubEnv("PACKED_PARENT_SECRET", "must-not-leak");

  const fixture = join(directory, "test/fixtures/customer");

  await mkdir(fixture, { recursive: true });

  await writeFile(
    join(fixture, "consumer.cjs"),
    'console.log(JSON.stringify({ value: require("@interfere/fixture"), secret: process.env.PACKED_PARENT_SECRET, explicit: process.env.EXPLICIT, mode: process.env.NODE_ENV }));',
  );

  const app = await application(directory, [
    "@interfere/fixture",
    "@interfere/fixture",
  ]);

  try {
    await app.copyFixture("customer");

    await writeFile(join(directory, "index.cjs"), "module.exports = 0;");

    const result = await app.run("consumer.cjs", { EXPLICIT: "supplied" });

    expect(JSON.parse(result.stdout)).toStrictEqual({
      value: 42,
      explicit: "supplied",
      mode: "production",
    });

    await expect(
      readFile(
        join(app.root, "node_modules/@interfere/fixture/index.cjs"),
        "utf8",
      ),
    ).resolves.toContain('require("external")');

    await expect(app.run("missing.cjs")).rejects.toThrow("Cannot find module");
  } finally {
    await app.close();
  }

  await expect(readdir(scratch)).resolves.toStrictEqual([]);
});

test("starts a real HTTP child and terminates it after use", async () => {
  const app = await application(directory, []);

  try {
    await writeFile(
      join(app.root, "server.cjs"),
      'const server = require("node:http").createServer((req, res) => { console.error("request served"); res.end("ready"); }); server.listen(0, "127.0.0.1", () => { console.log("server ready"); process.send("http://127.0.0.1:" + server.address().port); }); process.on("SIGTERM", () => server.close(() => process.exit(0)));',
    );

    const running = await app.start("server.cjs", {});

    try {
      await expect.poll(running.output).toContain("server ready");

      await expect((await fetch(running.url)).text()).resolves.toBe("ready");

      await expect.poll(running.output).toContain("request served");
    } finally {
      await running.close();
    }

    await expect(fetch(running.url)).rejects.toThrow("fetch failed");
  } finally {
    await app.close();
  }
});

test("qualification tarballs resolve with workspace conditions without changing the workspace", async () => {
  const file = join(directory, "package.json");

  const original = z
    .record(z.string(), z.json())
    .parse(JSON.parse(await readFile(file, "utf8")));

  original["exports"] = {
    monorepo: "./src/missing.ts",
    default: "./index.cjs",
  };

  original["imports"] = {
    "#entry": { monorepo: "./src/missing.ts", default: "./index.cjs" },
  };

  await writeFile(file, JSON.stringify(original));

  const app = await application(directory, ["@interfere/fixture"]);

  try {
    const tarball = join(app.root, "@interfere-fixture.tgz");

    const packed = await promisify(execFile)("tar", [
      "-xOzf",
      tarball,
      "package/package.json",
    ]);

    const manifest = z
      .object({ exports: z.json(), imports: z.json() })
      .parse(JSON.parse(packed.stdout));

    expect(manifest.exports).toStrictEqual({ default: "./index.cjs" });

    expect(manifest.imports).toStrictEqual({
      "#entry": { default: "./index.cjs" },
    });

    await writeFile(
      join(app.root, "consumer.cjs"),
      'console.log(require("@interfere/fixture"));',
    );

    expect(
      (
        await app.run("consumer.cjs", { NODE_OPTIONS: "--conditions=monorepo" })
      ).stdout.trim(),
    ).toBe("42");

    expect(JSON.parse(await readFile(file, "utf8"))).toStrictEqual(original);
  } finally {
    await app.close();
  }
});

test("rejects invalid startup messages and cleans failed dependency installations", async () => {
  const app = await application(directory, []);

  try {
    await writeFile(
      join(app.root, "invalid.cjs"),
      'console.log("startup output"); console.error("startup error"); process.send("invalid-url"); setInterval(() => {}, 1000);',
    );

    await expect(app.start("invalid.cjs", {})).rejects.toThrow(
      "Application startup failed:",
    );
  } finally {
    await app.close();
  }

  await expect(application(directory, ["@interfere/missing"])).rejects.toThrow(
    "ENOENT",
  );

  await expect(readdir(scratch)).resolves.toStrictEqual([]);
});

test("installs a listed package from a prebuilt tarball instead of packing it", async () => {
  const prebuilt = join(directory, "prebuilt");

  await mkdir(prebuilt);

  await writeFile(
    join(prebuilt, "package.json"),
    JSON.stringify({
      name: "@interfere/fixture",
      version: "9.9.9",
      files: ["index.cjs"],
      exports: "./index.cjs",
    }),
  );

  await writeFile(join(prebuilt, "index.cjs"), "module.exports = 99;");

  const tarball = join(directory, "prebuilt.tgz");

  await promisify(execFile)(
    "bun",
    ["pm", "pack", "--ignore-scripts", "--quiet", "--filename", tarball],
    { cwd: prebuilt },
  );

  const app = await application(directory, ["@interfere/fixture"], {
    tarballs: { "@interfere/fixture": tarball },
  });

  try {
    await expect(
      readFile(
        join(app.root, "node_modules/@interfere/fixture/package.json"),
        "utf8",
      ),
    ).resolves.toContain('"version": "9.9.9"');

    const suppliedTarball = await readFile(tarball);

    await expect(
      readFile(join(app.root, "@interfere-fixture.tgz")),
    ).resolves.toStrictEqual(suppliedTarball);

    await expect(
      readFile(
        join(app.root, "node_modules/@interfere/fixture/index.cjs"),
        "utf8",
      ),
    ).resolves.toBe("module.exports = 99;");
  } finally {
    await app.close();
  }
});
