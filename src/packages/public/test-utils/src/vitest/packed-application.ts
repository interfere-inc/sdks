/* oxlint-disable node/no-process-env */
import { execFile, spawn } from "node:child_process";
import { once } from "node:events";
import {
  cp,
  mkdir,
  mkdtemp,
  readFile,
  realpath,
  rm,
  symlink,
  writeFile,
} from "node:fs/promises";
import { createRequire } from "node:module";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { promisify } from "node:util";
import { z } from "zod";

const exec = promisify(execFile);

const metadata = z.object({
  name: z.string(),
  dependencies: z.record(z.string(), z.string()).default({}),
  peerDependencies: z.record(z.string(), z.string()).default({}),
  peerDependenciesMeta: z
    .record(z.string(), z.object({ optional: z.boolean().optional() }))
    .default({}),
});

function childEnvironment(overrides: Record<string, string | undefined>) {
  return Object.fromEntries([
    ["PATH", process.env["PATH"]],
    ["NODE_ENV", "production"],
    ["NEXT_TELEMETRY_DISABLED", "1"],
    ...Object.entries(overrides),
  ]) as NodeJS.ProcessEnv;
}

const endsWithParent = `data:text/javascript,${encodeURIComponent(
  'process.once("disconnect", () => process.kill(process.pid, "SIGTERM"));\nprocess.channel?.unref();',
)}`;

export function childArguments(script: string) {
  return ["--import", endsWithParent, script];
}

export type PackageSelection = string | { as: string; from: string };

export interface ApplicationOptions {
  /** Pre-built tarballs by package name; a listed `@interfere/*` package is installed from that file instead of being packed from the workspace. */
  readonly tarballs?: Readonly<Record<string, string>>;
}

export async function application(
  packageRoot: string,
  packages: PackageSelection[],
  options: ApplicationOptions = {},
) {
  const root = await mkdtemp(join(tmpdir(), "interfere-packed-application-"));

  const installed = new Set<string>();
  const require = createRequire(join(packageRoot, "package.json"));

  async function install(name: string, from: string, as = name) {
    if (installed.has(as)) {
      return;
    }

    installed.add(as);

    const modules = join(from, "node_modules", name);

    const owner = metadata.parse(
      JSON.parse(await readFile(join(from, "package.json"), "utf8")),
    );

    const source = owner.name === name ? from : await realpath(modules);

    const manifest = metadata.parse(
      JSON.parse(await readFile(join(source, "package.json"), "utf8")),
    );

    const destination = join(root, "node_modules", as);

    await mkdir(dirname(destination), { recursive: true });

    if (!name.startsWith("@interfere/")) {
      await symlink(source, destination, "dir");

      return;
    }

    const tarball = join(root, `${name.replaceAll("/", "-")}.tgz`);

    const prebuilt = options.tarballs?.[name];

    if (prebuilt === undefined) {
      await exec(
        "bun",
        ["pm", "pack", "--ignore-scripts", "--quiet", "--filename", tarball],
        { cwd: source },
      );
    } else {
      await cp(prebuilt, tarball);
    }

    await mkdir(destination, { recursive: true });

    await exec("tar", [
      "-xzf",
      tarball,
      "--strip-components=1",
      "-C",
      destination,
    ]);

    const requiredPeers = Object.keys(manifest.peerDependencies).filter(
      (peer) => manifest.peerDependenciesMeta[peer]?.optional !== true,
    );

    await Promise.all(
      [...Object.keys(manifest.dependencies), ...requiredPeers].map(
        async (dependency) => install(dependency, source),
      ),
    );
  }

  try {
    await writeFile(
      join(root, "package.json"),
      JSON.stringify({ private: true, type: "module" }),
    );

    await Promise.all(
      packages.map(async (selection) =>
        selection instanceof Object
          ? install(selection.from, packageRoot, selection.as)
          : install(selection, packageRoot),
      ),
    );

    return {
      root,
      async copyFixture(name: string) {
        await cp(join(packageRoot, "test/fixtures", name), root, {
          recursive: true,
        });
      },
      async run(
        script: string,
        env: Record<string, string | undefined> = {},
        runtime = process.execPath,
      ) {
        return exec(runtime, [script], {
          cwd: root,
          timeout: 30_000,
          maxBuffer: 5 * 1024 * 1024,
          env: childEnvironment(env),
        });
      },
      async build(env: Record<string, string | undefined>, cwd = root) {
        return exec(
          process.execPath,
          [
            join(dirname(require.resolve("vite/package.json")), "bin/vite.js"),
            "build",
          ],
          {
            cwd,
            timeout: 90_000,
            maxBuffer: 5 * 1024 * 1024,
            env: childEnvironment(env),
          },
        );
      },
      async buildNext(
        env: Record<string, string | undefined>,
        bundler?: "webpack" | "turbopack",
        cwd = root,
      ) {
        const installedNext = join(
          root,
          "node_modules",
          "next",
          "dist",
          "bin",
          "next",
        );

        const bin = installed.has("next")
          ? installedNext
          : require.resolve("next/dist/bin/next");

        return exec(
          process.execPath,
          [bin, "build", ...(bundler ? [`--${bundler}`] : [])],
          {
            cwd,
            timeout: 150_000,
            maxBuffer: 10 * 1024 * 1024,
            env: childEnvironment(env),
          },
        );
      },
      async start(
        script: string,
        env: Record<string, string | undefined>,
        cwd = root,
      ) {
        const child = spawn(process.execPath, childArguments(script), {
          cwd,
          env: childEnvironment(env),
          stdio: ["ignore", "pipe", "pipe", "ipc"],
        });

        let output = "";

        child.stdout?.on("data", (chunk: Buffer) => {
          output += chunk.toString();
        });

        child.stderr?.on("data", (chunk: Buffer) => {
          output += chunk.toString();
        });

        const exited = once(child, "exit");

        try {
          const messages: unknown[] = await once(child, "message", {
            signal: AbortSignal.timeout(30_000),
          });

          const url = z.url().parse(messages[0]);

          return {
            url,
            output: () => output,
            async close() {
              child.kill("SIGTERM");

              await exited;
            },
          };
        } catch (error) {
          child.kill("SIGKILL");

          await exited;

          throw new Error(`Application startup failed: ${output}`, {
            cause: error,
          });
        }
      },
      async close() {
        await rm(root, { recursive: true, force: true });
      },
    };
  } catch (error) {
    await rm(root, { recursive: true, force: true });

    throw error;
  }
}
