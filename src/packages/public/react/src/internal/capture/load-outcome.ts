import { omitUndefined } from "@interfere/helpers/omit-undefined";

const LOAD_FAILURE = {
  httpStatus: "http_status",
  noStatus: "no_status",
  noTimingEntry: "no_timing_entry",
  offline: "offline",
  rejectedResponse: "rejected_response",
} as const;

const HTTP_PROTOCOL_VERSIONS = new Map([
  ["http/0.9", "0.9"],
  ["http/1.0", "1.0"],
  ["http/1.1", "1.1"],
  ["h2", "2"],
  ["h2c", "2"],
  ["h3", "3"],
]);

const OBSERVED_ENTRY_CAP = 100;

export type LoadOutcome = ReturnType<typeof outcomeOf>;

export interface LoadOutcomeReader {
  dispose: () => void;
  read: (url: string) => LoadOutcome;
}

function latestResourceEntry(url: string) {
  try {
    const last = globalThis.performance
      .getEntriesByName(url, "resource")
      .at(-1);

    return last instanceof PerformanceResourceTiming ? last : null;
  } catch {
    return null;
  }
}

function isOffline() {
  // oxlint-disable-next-line typescript/no-unnecessary-condition -- Some embedded hosts have no navigator.
  return !(globalThis.navigator?.onLine ?? true);
}

function failureOf(entry: PerformanceResourceTiming | null, status: number) {
  if (isOffline()) {
    return LOAD_FAILURE.offline;
  }

  if (!entry) {
    return LOAD_FAILURE.noTimingEntry;
  }

  if (status >= 400) {
    return LOAD_FAILURE.httpStatus;
  }

  return status > 0 ? LOAD_FAILURE.rejectedResponse : LOAD_FAILURE.noStatus;
}

function outcomeOf(entry: PerformanceResourceTiming | null) {
  const status = entry?.responseStatus ?? 0;

  const version = entry
    ? HTTP_PROTOCOL_VERSIONS.get(entry.nextHopProtocol)
    : undefined;

  return omitUndefined({
    "http.response.status_code": status > 0 ? status : undefined,
    "interfere.load.duration_ms": entry
      ? Math.round(entry.duration)
      : undefined,
    "interfere.load.failure": failureOf(entry, status),
    "interfere.load.transfer_size": entry?.transferSize,
    "network.protocol.name": version ? ("http" as const) : undefined,
    "network.protocol.version": version,
  });
}

/**
 * Reads what the browser recorded about a failed load: its Resource Timing
 * status, size, duration and protocol, and whether the device was offline.
 * Entries come from an observer because the page's resource buffer stops
 * accepting entries once full, which long-lived pages reach.
 */
export function createLoadOutcomeReader() {
  const entries = new Map<string, PerformanceResourceTiming>();

  const remember = (list: readonly PerformanceEntry[]) => {
    for (const entry of list) {
      if (entry instanceof PerformanceResourceTiming) {
        entries.delete(entry.name);

        entries.set(entry.name, entry);
      }
    }

    for (const name of entries.keys()) {
      if (entries.size <= OBSERVED_ENTRY_CAP) {
        break;
      }

      entries.delete(name);
    }
  };

  let observer: PerformanceObserver | null = null;

  try {
    observer = new PerformanceObserver((list) => {
      remember(list.getEntries());
    });

    observer.observe({ type: "resource" });
  } catch {
    observer = null;
  }

  return {
    dispose: () => {
      observer?.disconnect();

      entries.clear();
    },
    read: (url: string) => {
      if (observer) {
        remember(observer.takeRecords());
      }

      return outcomeOf(entries.get(url) ?? latestResourceEntry(url));
    },
  } satisfies LoadOutcomeReader;
}
