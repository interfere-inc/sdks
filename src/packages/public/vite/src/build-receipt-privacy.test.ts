import {
  mkdir,
  mkdtemp,
  readFile,
  rm,
  symlink,
  writeFile,
} from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { build } from "vite";
import { afterEach, beforeEach, expect, test, vi } from "vitest";
import { interfere } from "./plugin";

let root: string;

beforeEach(async () => {
  root = await mkdtemp(path.join(tmpdir(), "vite-receipt-privacy-"));

  await writeFile(path.join(root, "entry.js"), "export const value = 1;");
});

afterEach(async () => {
  vi.unstubAllEnvs();

  await rm(root, { recursive: true, force: true });
});

async function compile(destination: string) {
  await build({
    root,
    configFile: false,
    logLevel: "silent",
    plugins: [interfere({ buildReceipt: destination })],
    build: { ssr: path.join(root, "entry.js"), outDir: "dist" },
  });
}

test.each([
  "dist/receipt.json",
  "public/receipt.json",
  "dist/..receipt.json",
  "public/..receipt.json",
])("refuses receipts in served directory %s", async (destination) => {
  await mkdir(path.join(root, "public"));

  await expect(compile(destination)).rejects.toThrow(
    "outside public and build output directories",
  );

  await expect(readFile(path.join(root, destination))).rejects.toMatchObject({
    code: "ENOENT",
  });
});

test("refuses a symlink into served output", async () => {
  await mkdir(path.join(root, "public"));

  await symlink(path.join(root, "public"), path.join(root, "private-link"));

  await expect(compile("private-link/receipt.json")).rejects.toThrow(
    "outside public and build output directories",
  );
});

test("refuses receipts outside the project", async () => {
  await expect(compile("../outside-receipt.json")).rejects.toThrow(
    "inside the project directory",
  );
});
