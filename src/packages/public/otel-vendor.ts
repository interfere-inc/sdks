/// <reference types="node" />
import {
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  realpathSync,
  writeFileSync,
} from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";
import type { Rolldown, TsdownHooks, UserConfig } from "tsdown";
import { z } from "zod";

const NODE_MODULES = "/node_modules/";

const API = "@opentelemetry/api";

const compatModule = fileURLToPath(
  new URL("otel-api-compat.ts", import.meta.url),
);

export const otelVendored = /^@opentelemetry\/(?!api$)/u;

const declaration = /\.d\.[cm]?ts$/u;

type ImportTarget = string | { [condition: string]: ImportTarget };

const importTargetSchema: z.ZodType<ImportTarget> = z.lazy(() =>
  z.union([z.string(), z.record(z.string(), importTargetSchema)]),
);

const importsSchema = z.record(z.string(), importTargetSchema);

type ImportsMap = z.infer<typeof importsSchema>;

const manifestSchema = z.object({ imports: importsSchema.default({}) });

const vendorPackage = (file: string) => {
  const index = file.lastIndexOf(NODE_MODULES);

  if (index === -1) {
    return;
  }

  const [scope = "", name = ""] = file
    .slice(index + NODE_MODULES.length)
    .split("/");

  const packageName = scope.startsWith("@") ? `${scope}/${name}` : scope;

  return {
    name: packageName,
    root: file.slice(0, index + NODE_MODULES.length) + packageName,
  };
};

const readImports = (root: string) =>
  manifestSchema.parse(
    JSON.parse(readFileSync(join(root, "package.json"), "utf8")),
  ).imports;

const esmTarget: (target: ImportTarget | undefined) => string | undefined = (
  target,
) => {
  if (target === undefined) {
    return;
  }

  const leaf = z.string().safeParse(target);

  if (leaf.success) {
    return leaf.data;
  }

  const conditions = importsSchema.parse(target);

  return esmTarget(conditions["import"] ?? conditions["default"]);
};

const platformTargets = (target: ImportTarget | undefined) => {
  const conditions = importsSchema.safeParse(target);

  if (!conditions.success) {
    return;
  }

  const node = esmTarget(conditions.data["node"]);
  const browser = esmTarget(conditions.data["default"]);

  return node && browser ? { browser, node } : undefined;
};

const vendorName = (name: string) => {
  const index = name.lastIndexOf("node_modules/");
  const path = name.slice(index + "node_modules/".length);

  return index !== -1 &&
    otelVendored.test(path.split("/").slice(0, 2).join("/"))
    ? `vendor/${path}`
    : undefined;
};

const vendorFileName = (
  pattern: NonNullable<Rolldown.OutputOptions["entryFileNames"]>,
) => {
  const renamed: NonNullable<Rolldown.OutputOptions["entryFileNames"]> = (
    chunk,
  ) => {
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Rolldown types file-name patterns as a string or a function.
    const template = typeof pattern === "string" ? pattern : pattern(chunk);

    const name =
      chunk.facadeModuleId === compatModule
        ? "vendor/otel-api-compat"
        : vendorName(chunk.name);

    return name ? template.replace("[name]", name) : template;
  };

  return renamed;
};

const sameImports = (expected: ImportsMap, actual: ImportsMap) =>
  Object.entries(expected).every(
    ([key, value]) => JSON.stringify(actual[key]) === JSON.stringify(value),
  ) &&
  Object.keys(actual)
    .filter((key) => key.startsWith("#otel/"))
    .every((key) => key in expected);

const typesManifestSchema = z.object({
  exports: z.record(z.string(), importTargetSchema).default({}),
  types: z.string().optional(),
});

const specifierPattern =
  /(?<prefix>\bfrom\s*|\bimport\s*\(\s*|\bimport\s+)(?<quote>["'])(?<specifier>[^"']+)\k<quote>/gu;

const typesTarget: (
  target: ImportTarget | undefined,
  condition: "import" | "require",
) => string | undefined = (target, condition) => {
  const conditions = importsSchema.safeParse(target);

  if (!conditions.success) {
    return;
  }

  const types = z.string().safeParse(conditions.data["types"]);

  if (types.success) {
    return types.data;
  }

  return typesTarget(
    conditions.data[condition] ?? conditions.data["default"],
    condition,
  );
};

const packageRoot = (name: string, from: string) => {
  let directory = from;

  while (!existsSync(join(directory, "node_modules", name, "package.json"))) {
    const parent = dirname(directory);

    if (parent === directory) {
      throw new Error(`Cannot find ${name} from ${from}`);
    }

    directory = parent;
  }

  return realpathSync(join(directory, "node_modules", name));
};

const declarationFile = (file: string) =>
  file
    .replace(/\.mjs$/u, ".d.mts")
    .replace(/\.cjs$/u, ".d.cts")
    .replace(/\.js$/u, ".d.ts");

interface TypesOrigin {
  fileName: string;
  root: string;
  name?: string;
  path?: string;
}

interface TypesCopy {
  fileName: string;
  name: string;
  path: string;
  root: string;
  source?: string;
}

const vendorTypes = () => {
  const copies = new Map<string, TypesCopy>();
  const queue: TypesCopy[] = [];

  const schedule = (path: string, name: string, root: string) => {
    const existing = copies.get(path);

    if (existing || !existsSync(path)) {
      return existing?.fileName;
    }

    const copy = {
      fileName: `vendor/${name}/${relative(root, path)}`,
      name,
      path,
      root,
    };

    copies.set(path, copy);

    queue.push(copy);

    return copy.fileName;
  };

  const locate = (
    specifier: string,
    from: string,
    condition: "import" | "require",
  ) => {
    const [scope = "", name = "", ...rest] = specifier.split("/");
    const packageName = `${scope}/${name}`;
    const root = packageRoot(packageName, from);

    const manifest = typesManifestSchema.parse(
      JSON.parse(readFileSync(join(root, "package.json"), "utf8")),
    );

    const subpath = rest.length > 0 ? `./${rest.join("/")}` : ".";

    const entry =
      typesTarget(manifest.exports[subpath], condition) ??
      (subpath === "." ? manifest.types : undefined);

    if (!entry) {
      throw new Error(`${specifier} has no ${condition} types`);
    }

    return schedule(join(root, entry), packageName, root);
  };

  const rewrite = (code: string, from: TypesOrigin) => {
    const condition = from.fileName.endsWith(".d.cts") ? "require" : "import";

    return code.replaceAll(
      specifierPattern,
      (match, prefix: string, quote: string, specifier: string) => {
        if (specifier.startsWith(".")) {
          if (from.path && from.name) {
            schedule(
              declarationFile(join(dirname(from.path), specifier)),
              from.name,
              from.root,
            );
          }

          return match;
        }

        const [scope = "", name = ""] = specifier.split("/");

        if (!otelVendored.test(`${scope}/${name}`)) {
          return match;
        }

        const target = locate(specifier, from.root, condition);

        if (!target) {
          return match;
        }

        const path = relative(dirname(from.fileName), target)
          .replace(/\.d\.mts$/u, ".mjs")
          .replace(/\.d\.cts$/u, ".cjs")
          .replace(/\.d\.ts$/u, ".js");

        return `${prefix}${quote}${path.startsWith(".") ? path : `./${path}`}${quote}`;
      },
    );
  };

  const files = () => {
    for (let copy = queue.shift(); copy; copy = queue.shift()) {
      copy.source = rewrite(readFileSync(copy.path, "utf8"), copy);
    }

    return [...copies.values()].map(({ fileName, source = "" }) => ({
      fileName,
      source,
    }));
  };

  const packages = () =>
    new Map([...copies.values()].map(({ name, root }) => [name, root]));

  return { files, packages, rewrite };
};

const vendorPlugin = () => {
  const required: ImportsMap = {};

  const plugin: Rolldown.Plugin = {
    name: "interfere:otel-vendor",
    resolveId: {
      order: "pre",
      handler(source, importer) {
        if (!importer || declaration.test(importer)) {
          return null;
        }

        if (source === API) {
          return importer === compatModule ? null : compatModule;
        }

        const owner = vendorPackage(importer);

        if (
          !owner ||
          !otelVendored.test(owner.name) ||
          !source.startsWith("#")
        ) {
          return null;
        }

        const targets = platformTargets(readImports(owner.root)[source]);

        if (!targets) {
          return this.error(
            `${owner.name} imports ${source} without node and default targets`,
          );
        }

        const specifier = `#otel/${owner.name.slice("@opentelemetry/".length)}/${source.slice(1)}`;

        const files = {
          default: `vendor/${owner.name}/${source.slice(1)}/browser.mjs`,
          node: `vendor/${owner.name}/${source.slice(1)}/node.mjs`,
        };

        this.emitFile({
          fileName: files.node,
          id: join(owner.root, targets.node),
          type: "chunk",
        });

        this.emitFile({
          fileName: files.default,
          id: join(owner.root, targets.browser),
          type: "chunk",
        });

        required[specifier] = {
          node: `./dist/${files.node}`,
          default: `./dist/${files.default}`,
        };

        return { external: true, id: specifier };
      },
    },
    buildEnd(error) {
      if (error) {
        return;
      }

      const actual = readImports(process.cwd());

      if (!sameImports(required, actual)) {
        const expected = Object.fromEntries([
          ...Object.entries(actual).filter(
            ([key]) => !key.startsWith("#otel/"),
          ),
          ...Object.entries(required),
        ]);

        this.error(
          `package.json imports must map the vendored OpenTelemetry platform modules:\n${JSON.stringify(expected, null, 2)}`,
        );
      }
    },
    generateBundle() {
      const roots = new Map<string, string>();

      for (const id of this.getModuleIds()) {
        const owner = vendorPackage(id);

        if (owner && otelVendored.test(owner.name)) {
          roots.set(owner.name, owner.root);
        }
      }

      for (const [name, root] of roots) {
        const license = join(root, "LICENSE");

        if (existsSync(license)) {
          this.emitFile({
            fileName: `vendor/${name}/LICENSE`,
            source: readFileSync(license),
            type: "asset",
          });
        }
      }
    },
  };

  return plugin;
};

const writeFile = (
  outDir: string,
  fileName: string,
  source: string | Buffer,
) => {
  mkdirSync(dirname(join(outDir, fileName)), { recursive: true });

  writeFileSync(join(outDir, fileName), source);
};

const vendorDeclarations: TsdownHooks["build:done"] = ({ options }) => {
  const types = vendorTypes();

  for (const fileName of readdirSync(options.outDir, {
    encoding: "utf8",
    recursive: true,
  })) {
    if (declaration.test(fileName) && !fileName.startsWith("vendor/")) {
      const code = readFileSync(join(options.outDir, fileName), "utf8");

      const rewritten = types.rewrite(code, { fileName, root: options.cwd });

      if (rewritten !== code) {
        writeFile(options.outDir, fileName, rewritten);
      }
    }
  }

  for (const { fileName, source } of types.files()) {
    writeFile(options.outDir, fileName, source);
  }

  for (const [name, root] of types.packages()) {
    const license = join(root, "LICENSE");

    if (existsSync(license)) {
      writeFile(
        options.outDir,
        `vendor/${name}/LICENSE`,
        readFileSync(license),
      );
    }
  }
};

export const otelNeverBundle: (
  id: string,
  importer: string | undefined,
) => boolean = (id, importer) =>
  id.startsWith("#") && !importer?.includes(NODE_MODULES);

export const vendorOpenTelemetry: (
  config: Omit<UserConfig, "hooks" | "outputOptions">,
) => UserConfig = (config) => ({
  ...config,
  deps: {
    ...config.deps,
    dts: { ...config.deps?.dts, neverBundle: [otelVendored] },
  },
  outputOptions: (options) => ({
    ...options,
    chunkFileNames: vendorFileName(
      options.chunkFileNames ?? "[name]-[hash].js",
    ),
    entryFileNames: vendorFileName(options.entryFileNames ?? "[name].js"),
  }),
  hooks: { "build:done": vendorDeclarations },
  plugins: [vendorPlugin(), ...[config.plugins ?? []].flat()],
});
