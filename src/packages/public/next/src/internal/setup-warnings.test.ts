import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

import {
  readNextMajorVersion,
  supportsProductionCompileHook,
} from "./setup-warnings.js";
import { afterEach, expect, test, vi } from "vitest";

afterEach(() => {
  vi.restoreAllMocks();
});

test.each([
  { manifest: null, expected: null },
  { manifest: false, expected: null },
  { manifest: 16, expected: null },
  { manifest: "16.0.0", expected: null },
  { manifest: {}, expected: null },
  { manifest: { version: null }, expected: null },
  { manifest: { version: 16 }, expected: null },
  { manifest: { version: [] }, expected: null },
  { manifest: { version: "invalid" }, expected: null },
  { manifest: { version: "" }, expected: null },
  { manifest: { version: "16.0.0" }, expected: 16 },
  { manifest: { version: "15.3.0-canary.1" }, expected: 15 },
])(
  "reads an installed Next manifest: $manifest",
  async ({ manifest, expected }) => {
    const project = await mkdtemp(join(tmpdir(), "next-version-test-"));

    const packageDirectory = join(project, "node_modules", "next");

    try {
      await mkdir(packageDirectory, { recursive: true });

      await writeFile(
        join(packageDirectory, "package.json"),
        JSON.stringify(manifest),
      );

      expect(readNextMajorVersion(project)).toBe(expected);
    } finally {
      await rm(project, { recursive: true, force: true });
    }
  },
);

test("returns null outside a Next installation", async () => {
  const project = await mkdtemp(join(tmpdir(), "next-missing-test-"));

  try {
    expect(readNextMajorVersion(project)).toBeNull();
  } finally {
    await rm(project, { recursive: true, force: true });
  }
});

test.each([
  { version: "14.2.30", expected: false },
  { version: "15.3.9", expected: false },
  { version: "15.4.0", expected: true },
  { version: "15.5.28", expected: true },
  { version: "16.0.0", expected: true },
])(
  "detects the production compile hook in Next $version",
  async ({ version, expected }) => {
    const project = await mkdtemp(join(tmpdir(), "next-hook-test-"));

    const packageDirectory = join(project, "node_modules", "next");

    try {
      await mkdir(packageDirectory, { recursive: true });

      await writeFile(
        join(packageDirectory, "package.json"),
        JSON.stringify({ version }),
      );

      expect(supportsProductionCompileHook(project)).toBe(expected);
    } finally {
      await rm(project, { recursive: true, force: true });
    }
  },
);

test("assumes the production compile hook outside a Next installation", async () => {
  const project = await mkdtemp(join(tmpdir(), "next-hook-missing-test-"));

  try {
    expect(supportsProductionCompileHook(project)).toBe(true);
  } finally {
    await rm(project, { recursive: true, force: true });
  }
});
