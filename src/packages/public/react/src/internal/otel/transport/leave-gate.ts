interface SpanGate {
  release: () => void;
  wait: () => Promise<true>;
}

const FALLBACK_TICKS = 64;

const LAST_RESORT_MS = 1000;

const gates = new Set<SpanGate>();

const state = {
  latest: undefined as SpanGate | undefined,
  logsExporting: false,
};

/** Holds the spans of the next leave-time trace export, other than exceptions, until the logs have reserved their keepalive budget. With no logs to export it waits a few microtasks; a page that stays alive also gets a timer, so held spans are never stranded. An unloading page may not run a timer, so the decision releases them first. */
export function holdSpansForLogs() {
  let open: ((value: true) => void) | undefined;

  const held = new Promise<true>((resolve) => {
    open = resolve;
  });

  state.logsExporting = false;

  const current = {
    release: () => {
      gates.delete(current);

      if (state.latest === current) {
        state.latest = undefined;
      }

      open?.(true);
    },
    wait: async () => held,
  };

  gates.add(current);

  state.latest = current;

  setTimeout(current.release, LAST_RESORT_MS);

  void (async () => {
    for (let tick = 0; tick < FALLBACK_TICKS; tick += 1) {
      // oxlint-disable-next-line no-await-in-loop -- Each tick is one microtask turn; the loop bounds the wait without a timer.
      await Promise.resolve();
    }

    if (!state.logsExporting) {
      current.release();
    }
  })();
}

/** The gate the latest leave-time flush set, or undefined outside one. */
export function currentSpanGate() {
  return state.latest;
}

/** The log export has started, so held spans keep waiting for its reservation. */
export function noteLogsExporting() {
  state.logsExporting = gates.size > 0;
}

/** The log export has reserved keepalive budget, been refused it, or finished, so every held span export may take what is left. */
export function noteLogsDecided() {
  state.logsExporting = false;

  for (const gate of gates) {
    gate.release();
  }
}
