import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { log } from "../../logger.js";
import type { SourceMapFile } from "./discover.js";
import { reportUnmappedChunks } from "./guard.js";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

function makeFile(overrides: Partial<SourceMapFile> = {}) {
  return {
    absolute: "/tmp/project/.next/static/chunks/app/page.js.map",
    chunkAbsolutes: ["/tmp/project/.next/static/chunks/app/page.js"],
    chunkUrl: "_next/static/chunks/app/page.js.map",
    content: '{"version":3,"sources":["src/app.ts"],"mappings":"AAAA"}',
    debugId: "6a8b0c1d-2e3f-4a5b-6c7d-8e9f0a1b2c3d",
    hash: "deadbeefdeadbeef",
    path: ".next/static/chunks/app/page.js.map",
    ...overrides,
  } satisfies SourceMapFile;
}

function stampedChunk(debugId: string) {
  return `function chunk(){}\n//# debugId=${debugId}\n`;
}

const UNSTAMPED_CHUNK = "function runtime(){}";

let projectDir: string;

let chunksDir: string;

beforeEach(async () => {
  projectDir = await mkdtemp(join(tmpdir(), "guard-test-"));

  chunksDir = join(projectDir, ".next", "static", "chunks");

  await mkdir(chunksDir, { recursive: true });
});

afterEach(async () => {
  vi.restoreAllMocks();

  await rm(projectDir, { force: true, recursive: true });
});

describe(reportUnmappedChunks, () => {
  test("accepts every chunk paired to a shared discovered map", async () => {
    const error = vi.spyOn(log, "error");

    await writeFile(
      join(chunksDir, "main.js"),
      stampedChunk("6a8b0c1d-2e3f-4a5b-6c7d-8e9f0a1b2c3d"),
    );

    const discovered = [
      makeFile({
        chunkAbsolutes: [
          join(chunksDir, "main.js"),
          join(chunksDir, "alias.js"),
        ],
      }),
    ];

    await writeFile(
      join(chunksDir, "alias.js"),
      stampedChunk("6a8b0c1d-2e3f-4a5b-6c7d-8e9f0a1b2c3d"),
    );

    await reportUnmappedChunks({
      discovered,
      distDir: ".next",
      projectDir,
    });

    expect(error).not.toHaveBeenCalled();
  });

  test("does not count runtime chunks with no source-map markers", async () => {
    const error = vi.spyOn(log, "error");

    await writeFile(join(chunksDir, "main.js"), UNSTAMPED_CHUNK);

    await reportUnmappedChunks({
      discovered: [],
      distDir: ".next",
      projectDir,
    });

    expect(error).not.toHaveBeenCalled();
  });

  test("does not treat an embedded worker template's map marker as a chunk directive", async () => {
    const error = vi.spyOn(log, "error");

    await writeFile(
      join(chunksDir, "main.js"),
      "const worker = `code();\n//# sourceMappingURL=worker.js.map\n`;",
    );

    await reportUnmappedChunks({
      discovered: [],
      distDir: ".next",
      projectDir,
    });

    expect(error).not.toHaveBeenCalled();
  });

  test("logs an error without failing when stamped client chunks have no discovered maps", async () => {
    const error = vi.spyOn(log, "error");

    await writeFile(
      join(chunksDir, "main.js"),
      stampedChunk("6a8b0c1d-2e3f-4a5b-6c7d-8e9f0a1b2c3d"),
    );

    await writeFile(
      join(chunksDir, "framework.js"),
      stampedChunk("1b2c3d4e-5f6a-7b8c-9d0e-1f2a3b4c5d6e"),
    );

    await expect(
      reportUnmappedChunks({
        discovered: [],
        distDir: ".next",
        projectDir,
      }),
    ).resolves.toBeUndefined();

    expect(error).toHaveBeenCalledWith(
      "Source-map coverage is incomplete",
      expect.arrayContaining([expect.stringContaining("2 client chunk(s)")]),
    );
  });

  test("does not accept server-map metadata as coverage for stamped client chunks", async () => {
    const error = vi.spyOn(log, "error");

    await writeFile(
      join(chunksDir, "main.js"),
      stampedChunk("6a8b0c1d-2e3f-4a5b-6c7d-8e9f0a1b2c3d"),
    );

    await reportUnmappedChunks({
      discovered: [makeFile({ chunkUrl: "server/app/page.js.map" })],
      distDir: ".next",
      projectDir,
    });

    expect(error).toHaveBeenCalledWith(
      "Source-map coverage is incomplete",
      expect.any(Array),
    );
  });

  test("reports partial client and server coverage even when one client map exists", async () => {
    const error = vi.spyOn(log, "error");

    const serverDir = join(projectDir, ".next", "server", "chunks");

    await mkdir(serverDir, { recursive: true });

    await writeFile(join(chunksDir, "covered.js"), stampedChunk("abc-def"));

    await writeFile(join(chunksDir, "lazy.js"), stampedChunk("abc-def"));

    await writeFile(
      join(serverDir, "action.js"),
      "action();\n//# sourceMappingURL=action.js.map",
    );

    await reportUnmappedChunks({
      discovered: [
        makeFile({ chunkAbsolutes: [join(chunksDir, "covered.js")] }),
      ],
      distDir: ".next",
      projectDir,
    });

    expect(error).toHaveBeenCalledWith(
      "Source-map coverage is incomplete",
      expect.arrayContaining([
        expect.stringContaining("1 client chunk(s) and 1 server chunk(s)"),
        "static/chunks/lazy.js",
        "server/chunks/action.js",
      ]),
    );
  });
});
