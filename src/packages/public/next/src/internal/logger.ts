import chalk from "chalk";
import { writeConsoleLines } from "./console-output";

type LogLevel = "info" | "warn" | "error" | "fatal";

const styles = {
  error: {
    content: chalk.redBright,
    prefix: `${chalk.red.bold("⨯")} `,
    prependLevel: true,
    text: chalk.red.bold,
  },
  fatal: {
    content: chalk.redBright,
    prefix: `${chalk.red.bold("⨯")} `,
    prependLevel: true,
    text: chalk.red.bold,
  },
  info: {
    content: chalk.white,
    prefix: chalk.whiteBright.bold("❖"),
    prependLevel: false,
    text: chalk.cyan.bold,
  },
  warn: {
    content: chalk.yellowBright,
    prefix: `${chalk.yellow.bold("⚠")} `,
    prependLevel: false,
    text: chalk.yellow.bold,
  },
} satisfies Record<
  LogLevel,
  {
    prefix: string;
    text: typeof chalk.bold;
    content: typeof chalk;
    prependLevel: boolean;
  }
>;

const consoleMethods = {
  error: "error",
  fatal: "error",
  info: "log",
  warn: "warn",
} satisfies Record<LogLevel, keyof Console>;

function isTestEnv() {
  // oxlint-disable-next-line node/no-process-env
  return Boolean(process.env["VITEST"] || process.env["VITEST_WORKER_ID"]);
}

/**
 * Thrown by `log.fatal` in test environments where `process.exit` is not
 * appropriate. Tagged so callers' catch blocks can re-throw fatal errors and
 * keep test-env behavior aligned with the production hard-exit.
 */
export class FatalError extends Error {
  override readonly name = "FatalError";
}

function emit(level: LogLevel, title: string, lines: string[]) {
  if (isTestEnv()) {
    return;
  }

  const style = styles[level];
  const method = consoleMethods[level];
  const fn = globalThis.console[method];

  const prependLevel = style.prependLevel
    ? `[${style.text(level.toUpperCase())}] `
    : "";

  const output = [
    `${prependLevel} ${chalk.white("Interfere →")} ${style.text(title)}`,
    ...lines.map((line, index) => {
      const connector = index === lines.length - 1 ? "└" : "├";

      return `${prependLevel} ${connector} ${style.content(line)}`;
    }),
  ];

  writeConsoleLines(fn, output);
}

export const log = {
  error: (title: string, lines: string[]) => {
    emit("error", title, lines);
  },
  fatal(title: string, lines: string[]): never {
    emit("fatal", title, lines);

    if (isTestEnv()) {
      throw new FatalError(title);
    }

    // oxlint-disable-next-line unicorn/no-process-exit
    process.exit(1);
  },
  info: (title: string, lines: string[]) => {
    emit("info", title, lines);
  },
  warn: (title: string, lines: string[]) => {
    emit("warn", title, lines);
  },
};
