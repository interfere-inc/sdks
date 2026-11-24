// oxlint-disable-next-line import/no-namespace -- Older API versions lack some exports, so they are read off the namespace.
import * as hostApi from "@opentelemetry/api";
import type { Logger, LoggerProvider } from "@opentelemetry/api";

// oxlint-disable-next-line import/export -- The import plugin cannot list the exports of the API package entry.
export * from "@opentelemetry/api";

type LogsApi = Pick<
  typeof hostApi.logs,
  "disable" | "getLogger" | "getLoggerProvider" | "setGlobalLoggerProvider"
>;

interface LogsSlot {
  provider?: LoggerProvider;
}

const host: Partial<
  Pick<typeof hostApi, "SeverityNumber" | "createNoopLogger" | "logs">
> = { ...hostApi };

const noopLogger: Logger = { emit: () => undefined, enabled: () => false };

const severityNumbers = {
  UNSPECIFIED: 0,
  TRACE: 1,
  TRACE2: 2,
  TRACE3: 3,
  TRACE4: 4,
  DEBUG: 5,
  DEBUG2: 6,
  DEBUG3: 7,
  DEBUG4: 8,
  INFO: 9,
  INFO2: 10,
  INFO3: 11,
  INFO4: 12,
  WARN: 13,
  WARN2: 14,
  WARN3: 15,
  WARN4: 16,
  ERROR: 17,
  ERROR2: 18,
  ERROR3: 19,
  ERROR4: 20,
  FATAL: 21,
  FATAL2: 22,
  FATAL3: 23,
  FATAL4: 24,
} as const satisfies Record<keyof typeof hostApi.SeverityNumber, number>;

const logsSlot = () => {
  const key = Symbol.for("@interfere/otel-api-compat/logs");
  const global: Record<symbol, LogsSlot | undefined> = globalThis;

  global[key] ??= {};

  return global[key];
};

const logsRegistry = () => {
  const slot = logsSlot();

  const proxy: LoggerProvider & {
    _getDelegate: () => LoggerProvider | undefined;
  } = {
    _getDelegate: () => slot.provider,
    getLogger: (...args) => ({
      emit: (record) => {
        (slot.provider?.getLogger(...args) ?? noopLogger).emit(record);
      },
      enabled: (options) =>
        slot.provider?.getLogger(...args).enabled(options) ?? false,
    }),
  };

  const registry: LogsApi = {
    disable: () => {
      delete slot.provider;
    },
    getLogger: (...args) => (slot.provider ?? proxy).getLogger(...args),
    getLoggerProvider: () => slot.provider ?? proxy,
    setGlobalLoggerProvider: (provider) => {
      if (slot.provider) {
        return false;
      }

      slot.provider = provider;

      return true;
    },
  };

  return registry;
};

export const SeverityNumber:
  | typeof hostApi.SeverityNumber
  | typeof severityNumbers = host.SeverityNumber ?? severityNumbers;

export const createNoopLogger: () => Logger =
  host.createNoopLogger ?? (() => noopLogger);

export const logs: LogsApi = host.logs ?? logsRegistry();
