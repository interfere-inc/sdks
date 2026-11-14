import { createHash } from "node:crypto";
import { relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { JUnitReporter } from "vitest/node";
import type { JUnitOptions } from "vitest/node";

const repository = resolve(
  fileURLToPath(new URL("../../../../../..", import.meta.url)),
);

function attributeBytes(value: string) {
  return Buffer.byteLength(
    value.replaceAll(/[&"'<>]/gu, (character) => {
      if (character === "&") {
        return "&amp;";
      }

      if (character === '"') {
        return "&quot;";
      }

      if (character === "'") {
        return "&apos;";
      }

      return character === "<" ? "&lt;" : "&gt;";
    }),
  );
}

function boundedName(name: string, classname: string) {
  const limit = 1000 - Buffer.byteLength(`${classname} > `);

  if (attributeBytes(name) <= limit) {
    return name;
  }

  const suffix = ` [${createHash("sha256").update(name).digest("hex")}]`;
  let prefix = "";
  let bytes = attributeBytes(suffix);

  for (const character of name) {
    bytes += attributeBytes(character);

    if (bytes > limit) {
      break;
    }

    prefix += character;
  }

  return `${prefix}${suffix}`;
}

function identity({
  filepath,
  displayName,
}: {
  filepath: string;
  displayName: string;
}) {
  return `${relative(repository, filepath).replaceAll("\\", "/")} [${displayName}]`;
}

// oxlint-disable-next-line import/no-default-export
export default class RepositoryJUnitReporter extends JUnitReporter {
  private projectRoot = repository;

  private readonly unhandledPositions = new Map<string, number>();

  constructor(options: JUnitOptions = {}) {
    super({
      ...options,
      addFileAttribute: true,
      classnameTemplate: identity,
      suiteNameTemplate: identity,
    });
  }

  override async onInit(context: Parameters<JUnitReporter["onInit"]>[0]) {
    this.projectRoot = context.config.root;

    await super.onInit(context);
  }

  override async writeElement(
    name: string,
    inputAttributes: Parameters<JUnitReporter["writeElement"]>[1] & {
      name?: string;
      classname?: string;
      file?: string | undefined;
    },
    children: () => Promise<void>,
  ) {
    let attributes = inputAttributes;

    const scope =
      relative(
        repository,
        this.projectRoot === repository ? process.cwd() : this.projectRoot,
      ).replaceAll("\\", "/") || ".";

    if (name === "testsuite" && attributes.name === "vitest unhandled errors") {
      this.unhandledPositions.clear();

      await super.writeElement(
        name,
        { ...attributes, name: `${scope} > vitest unhandled errors` },
        children,
      );

      return;
    }

    if (
      name === "testcase" &&
      attributes.classname === "vitest unhandled errors"
    ) {
      const file = attributes.file
        ? relative(
            repository,
            resolve(this.projectRoot, attributes.file),
          ).replaceAll("\\", "/")
        : undefined;

      const classname = `${file ?? scope} > vitest unhandled errors`;

      const key = JSON.stringify([classname, attributes.name]);
      const position = (this.unhandledPositions.get(key) ?? 0) + 1;

      this.unhandledPositions.set(key, position);

      attributes = {
        ...attributes,
        file,
        classname,
        name: `${attributes.name} [occurrence ${position}]`,
      };
    }

    if (name === "testcase" && attributes.name && attributes.classname) {
      const fullName = attributes.name;
      const compactName = boundedName(fullName, attributes.classname);

      if (compactName !== fullName) {
        await super.writeElement(
          name,
          { ...attributes, name: compactName },
          async () => {
            await super.writeElement("properties", {}, async () => {
              await super.writeElement(
                "property",
                { name: "vitest.fullName", value: fullName },
                async () => await Promise.resolve(),
              );
            });

            await children();
          },
        );

        return;
      }
    }

    await super.writeElement(name, attributes, children);
  }

  override async writeTasks(
    tasks: Parameters<JUnitReporter["writeTasks"]>[0],
    _filename: string,
    fileAbsPath: string,
  ) {
    const counts = new Map<string, number>();
    const positions = new Map<string, number>();

    for (const task of tasks) {
      counts.set(task.name, (counts.get(task.name) ?? 0) + 1);
    }

    const distinct = tasks.map((task) => {
      let fullName = task.name;

      if (counts.get(task.name) !== 1) {
        let position = (positions.get(task.name) ?? 0) + 1;

        while (counts.has(`${task.name} [case ${position}]`)) {
          position += 1;
        }

        positions.set(task.name, position);

        fullName = `${task.name} [case ${position}]`;
      }

      const name =
        task.type === "test"
          ? boundedName(
              fullName,
              identity({
                filepath: fileAbsPath,
                // oxlint-disable-next-line typescript/no-unnecessary-condition
                displayName: task.file?.projectName ?? "",
              }),
            )
          : fullName;

      if (name !== fullName && task.type === "test") {
        return {
          ...task,
          name,
          annotations: [
            ...task.annotations,
            { type: "vitest.fullName", message: fullName },
          ],
        };
      }

      return { ...task, name };
    });

    await super.writeTasks(
      distinct,
      relative(repository, fileAbsPath).replaceAll("\\", "/"),
      fileAbsPath,
    );
  }
}
