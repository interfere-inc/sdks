import { once } from "node:events";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { trace } from "@opentelemetry/api";
import {
  installGlobalHandlers,
  uninstallGlobalHandlers,
} from "../../src/internal/server/capture/global-handlers";

const exporter = new InMemorySpanExporter();

const provider = new BasicTracerProvider({
  spanProcessors: [new SimpleSpanProcessor(exporter)],
});

const mode = process.argv.at(2);

trace.setGlobalTracerProvider(provider);

function write(line: string) {
  process.stdout.write(`${line}\n`);
}

function exceptionCount() {
  return exporter
    .getFinishedSpans()
    .flatMap((span) => span.events)
    .filter((event) => event.name === "exception").length;
}

async function deliver() {
  const released = once(process.stdin, "data");

  write("flush:waiting");

  await released;

  await provider.forceFlush();

  write("flush:done");

  process.stdin.pause();

  if (
    mode === "rejection" ||
    mode === "host-listener" ||
    mode === "before-exit"
  ) {
    setImmediate(() => {
      write(`alive:${exceptionCount()}`);

      if (mode !== "before-exit") {
        uninstallGlobalHandlers();
      }
    });
  }
}

if (mode === "host-listener") {
  process.on("uncaughtException", () => {
    write("host:logged");
  });
}

if (mode === "sigterm-host") {
  process.once("SIGTERM", () => {
    write("host:shutdown");

    setImmediate(() => {
      process.stdout.write("host:exit\n", () => {
        process.exit(0);
      });
    });
  });
}

installGlobalHandlers({
  captureUncaughtException: true,
  captureUnhandledRejection: true,
  // oxlint-disable-next-line typescript/promise-function-async -- The broken-flush mode must throw synchronously before any promise exists.
  flush: () => {
    write("flush:started");

    if (mode === "broken-flush") {
      throw new Error("flush threw synchronously");
    }

    if (mode === "sigterm-stuck") {
      return new Promise<void>(() => {
        process.stdin.resume();
      });
    }

    return deliver();
  },
});

if (
  mode === "uncaught" ||
  mode === "broken-flush" ||
  mode === "host-listener"
) {
  setImmediate(() => {
    throw new Error("native uncaught error");
  });
} else if (mode === "rejection") {
  void Promise.reject(new Error("native unhandled rejection"));
} else if (
  mode === "sigterm" ||
  mode === "sigterm-host" ||
  mode === "sigterm-stuck"
) {
  setInterval(() => {
    write("serving");
  }, 60_000);

  setImmediate(() => {
    process.kill(process.pid, "SIGTERM");
  });
} else if (mode !== "before-exit") {
  throw new Error("Unknown handler fixture mode");
}
