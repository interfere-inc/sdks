import { mkdtemp, realpath, rm, symlink, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { beforeAll, describe, expect, test } from "vitest";
import type { ViteUserConfig } from "vitest/config";
import { createVitest } from "vitest/node";
import { browser } from "./browser";

const PHASE_DEADLINE_MS = 30_000;

// WARNING: the nested run is Chromium + Vite + a test file inside one hook, so a bare hook timeout cannot say which step stalled; every phase races a named deadline instead.
async function phase<T>(name: string, work: Promise<T>) {
  let timer: NodeJS.Timeout | undefined;

  const expired = new Promise<never>((_resolve, reject) => {
    timer = setTimeout(() => {
      reject(
        new Error(
          `Nested browser run phase "${name}" did not finish within ${PHASE_DEADLINE_MS}ms`,
        ),
      );
    }, PHASE_DEADLINE_MS);
  });

  try {
    return await Promise.race([work, expired]);
  } finally {
    clearTimeout(timer);
  }
}

async function runBrowserProject() {
  const directory = await realpath(
    await mkdtemp(join(tmpdir(), "browser-startup-")),
  );

  const { exitCode } = process;

  try {
    await symlink(
      fileURLToPath(
        new URL("../../../test-utils/node_modules", import.meta.url),
      ),
      join(directory, "node_modules"),
      "dir",
    );

    await writeFile(
      join(directory, "setup.ts"),
      `
import { createServer } from "node:http";
export default async function setup(project) {
  const server = createServer((request, response) => {
    response.setHeader("access-control-allow-origin", "*");
    response.setHeader("content-type", "application/json");
    response.end(JSON.stringify({ name: "ready" }));
  });
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  project.provide("setupEndpoint", "http://127.0.0.1:" + server.address().port);
  return () => new Promise((resolve, reject) => server.close((error) => error ? reject(error) : resolve()));
}
`,
    );

    await writeFile(
      join(directory, "isolation.browser.test.ts"),
      `
import { expect, test } from "vitest";
import { fillStorageQuota } from "@interfere/test-config/browser-storage";
const namespaced = /^vt[0-9a-f]{8}_shared-key$/u;
test("namespaces storage and cookies under a per-file prefix", () => {
  localStorage.setItem("shared-key", "a");
  sessionStorage.setItem("shared-key", "b");
  document.cookie = "shared-cookie=1; path=/";
  expect(localStorage.getItem("shared-key")).toBe("a");
  expect(sessionStorage.getItem("shared-key")).toBe("b");
  expect(localStorage.length).toBe(1);
  expect(localStorage.key(0)).toBe("shared-key");
  expect(Object.keys(localStorage).filter((key) => namespaced.test(key))).toHaveLength(1);
  expect(Object.keys(localStorage)).not.toContain("shared-key");
  expect(document.cookie).toBe("shared-cookie=1");
});
test("restores what the previous test wrote once it finished", async () => {
  expect(localStorage.getItem("shared-key")).toBeNull();
  expect(sessionStorage.getItem("shared-key")).toBeNull();
  expect(document.cookie).toBe("");
  await expect(cookieStore.get("shared-cookie")).resolves.toBeNull();
  await cookieStore.set("store-cookie", "2");
  await expect(cookieStore.get("store-cookie")).resolves.toMatchObject({ name: "store-cookie", value: "2" });
  expect(document.cookie).toBe("store-cookie=2");
});
test("restores Cookie Store writes too", async () => {
  await expect(cookieStore.getAll()).resolves.toStrictEqual([]);
  expect(document.cookie).toBe("");
});
test("simulates an exhausted quota inside this file only", () => {
  fillStorageQuota(sessionStorage);
  expect(() => sessionStorage.setItem("shared-key", "c")).toThrow(DOMException);
  expect(() => sessionStorage.setItem("shared-key", "c")).toThrow(expect.objectContaining({ name: "QuotaExceededError" }));
  expect(sessionStorage.getItem("shared-key")).toBeNull();
  localStorage.setItem("shared-key", "d");
  expect(localStorage.getItem("shared-key")).toBe("d");
});
test("lifts the simulated quota with the test that filled it", () => {
  sessionStorage.setItem("shared-key", "e");
  expect(sessionStorage.getItem("shared-key")).toBe("e");
  expect(localStorage.getItem("shared-key")).toBeNull();
});
`,
    );

    await writeFile(
      join(directory, "gc.browser.test.ts"),
      `
import { expect, test } from "vitest";
import { commands } from "@interfere/test-config/browser-platform";
test("collectGarbage frees an object nothing references", async () => {
  const collected = (() => new WeakRef({ payload: new Array(1024).fill(0) }))();
  await commands.collectGarbage();
  expect(collected.deref()).toBeUndefined();
});
`,
    );

    await writeFile(
      join(directory, "startup.browser.test.ts"),
      `
import { expect, inject, test } from "vitest";
import { page } from "vitest/browser";
const payload = await fetch(inject("setupEndpoint")).then(response => response.json());
test("imports after global setup has provided its endpoint", async () => {
  expect(payload).toEqual({ name: "ready" });
  expect({ width: innerWidth, height: innerHeight }).toEqual({ width: 640, height: 360 });
  document.body.innerHTML = '<label>Status<input aria-label="Status"></label>';
  const input = page.getByRole("textbox", { name: "Status" });
  await input.fill(payload.name);
  await expect.element(input).toHaveValue("ready");
});
`,
    );

    const runner = await phase(
      "createVitest",
      createVitest({
        root: directory,
        config: false,
        watch: false,
        reporters: [],
        fsModuleCache: false,
        coverage: { enabled: false },
        globalSetup: [join(directory, "setup.ts")],
        projects: [
          browser({
            root: directory,
            test: {
              name: "startup-browser",
              fsModuleCache: false,
              browser: { viewport: { width: 640, height: 360 } },
            },
          }),
        ],
      }),
    );

    try {
      const result = await phase("start", runner.start());

      return {
        unhandledErrors: result.unhandledErrors,
        modules: result.testModules
          .map((module) => ({
            name: module.moduleId.slice(directory.length + 1),
            state: module.state(),
            errors: module.errors().map(({ message }) => message),
            tests: [...module.children.allTests()].length,
          }))
          .toSorted((left, right) => left.name.localeCompare(right.name)),
        exitCode: process.exitCode ?? 0,
      };
    } finally {
      await phase("close", runner.close());
    }
  } finally {
    process.exitCode = exitCode;

    await rm(directory, { recursive: true, force: true });
  }
}

let run: Awaited<ReturnType<typeof runBrowserProject>>;

beforeAll(async () => {
  run = await runBrowserProject();
}, 60_000);

test("collects the browser test after global setup and applies viewport overrides", () => {
  expect(run.unhandledErrors).toStrictEqual([]);

  expect(run.modules).toStrictEqual([
    {
      name: "gc.browser.test.ts",
      state: "passed",
      errors: [],
      tests: 1,
    },
    {
      name: "isolation.browser.test.ts",
      state: "passed",
      errors: [],
      tests: 5,
    },
    {
      name: "startup.browser.test.ts",
      state: "passed",
      errors: [],
      tests: 1,
    },
  ]);

  expect(run.exitCode).toBe(0);
});

test("does not prewarm the browser before global setup", () => {
  const project: ViteUserConfig = browser();

  expect(project.test?.browser?.provider).not.toHaveProperty("prewarm");
});

describe.each([
  { cli: {}, expected: ["plain.browser.test.ts"] },
  { cli: { cliExclude: ["**/*.browser.test.*"] }, expected: [] },
])(
  "collects plain browser tests but not visual ones, honouring CLI excludes $cli",
  ({ cli, expected }) => {
    let collected: string[];

    beforeAll(async () => {
      const directory = await realpath(
        await mkdtemp(join(tmpdir(), "browser-include-")),
      );

      try {
        await Promise.all(
          [
            "plain.browser.test.ts",
            "frame.visual.browser.test.tsx",
            "unit.test.ts",
          ].map(async (name) => writeFile(join(directory, name), "")),
        );

        const runner = await createVitest({
          ...cli,
          root: directory,
          config: false,
          watch: false,
          reporters: [],
          coverage: { enabled: false },
          projects: [browser({ root: directory, test: { name: "collect" } })],
        });

        try {
          const specifications = await runner.globTestSpecifications();

          collected = specifications.map(({ moduleId }) =>
            moduleId.slice(directory.length + 1),
          );
        } finally {
          await runner.close();
        }
      } finally {
        await rm(directory, { recursive: true, force: true });
      }
    });

    test("collects the configured files", () => {
      expect(collected).toStrictEqual(expected);
    });
  },
);
