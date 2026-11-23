import { relative } from "node:path";
import type {
  Reporter,
  TestModule,
  TestModuleState,
  TestRunEndReason,
} from "vitest/node";

type PhaseEvent =
  | { phase: "discovered"; project: string; file: string }
  | {
      phase: "module";
      project: string;
      file: string;
      state: TestModuleState;
      milliseconds: {
        environment: number;
        prepare: number;
        collect: number;
        setup: number;
        testsAndHooks: number;
      };
      workerId: number;
      concurrencyId: number;
    }
  | {
      phase: "finished";
      files: number;
      unhandledErrors: number;
      reason: TestRunEndReason;
    };

function pad(value: number, width: number) {
  return String(value).padStart(width, "0");
}

function isoTimestamp(instant: Date) {
  return `${pad(instant.getUTCFullYear(), 4)}-${pad(instant.getUTCMonth() + 1, 2)}-${pad(instant.getUTCDate(), 2)}T${pad(instant.getUTCHours(), 2)}:${pad(instant.getUTCMinutes(), 2)}:${pad(instant.getUTCSeconds(), 2)}.${pad(instant.getUTCMilliseconds(), 3)}Z`;
}

// oxlint-disable-next-line import/no-default-export
export default class ModuleStartReporter implements Reporter {
  private readonly execution = {
    runId: process.env["GITHUB_RUN_ID"] ?? null,
    runAttempt: process.env["GITHUB_RUN_ATTEMPT"] ?? null,
    suite: process.env["TEST_SUITE"] ?? null,
    startedAt: isoTimestamp(new Date()),
  };

  private readonly write: (line: string) => void;

  constructor(options: { write?: (line: string) => void } = {}) {
    this.write =
      options.write ??
      ((line) => {
        process.stdout.write(line);
      });
  }

  private emit(event: PhaseEvent) {
    this.write(
      `vitest-phase ${JSON.stringify({ ...this.execution, ...event })}\n`,
    );
  }

  onTestRunStart(
    ...[specifications]: Parameters<NonNullable<Reporter["onTestRunStart"]>>
  ) {
    const discovered = specifications
      .map(({ project, moduleId }) => ({
        project: project.name,
        file: relative(project.config.root, moduleId),
      }))
      .toSorted((a, b) =>
        JSON.stringify([a.project, a.file]).localeCompare(
          JSON.stringify([b.project, b.file]),
        ),
      );

    for (const { project, file } of discovered) {
      this.emit({ phase: "discovered", project, file });
    }
  }

  onTestModuleStart(testModule: TestModule) {
    this.write(`▶ ${testModule.project.name} ${testModule.relativeModuleId}\n`);
  }

  onTestModuleEnd(testModule: TestModule) {
    const diagnostic = testModule.diagnostic();

    this.emit({
      phase: "module",
      project: testModule.project.name,
      file: testModule.relativeModuleId,
      state: testModule.state(),
      milliseconds: {
        environment: diagnostic.environmentSetupDuration,
        prepare: diagnostic.prepareDuration,
        collect: diagnostic.collectDuration,
        setup: diagnostic.setupDuration,
        testsAndHooks: diagnostic.duration,
      },
      workerId: diagnostic.workerId,
      concurrencyId: diagnostic.concurrencyId,
    });
  }

  onTestRunEnd(
    ...[modules, errors, reason]: Parameters<
      NonNullable<Reporter["onTestRunEnd"]>
    >
  ) {
    this.emit({
      phase: "finished",
      files: modules.length,
      unhandledErrors: errors.length,
      reason,
    });
  }
}
