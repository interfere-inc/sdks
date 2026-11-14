let depth = 0;

/** Runs `fn` so console calls it makes reach the host console but no subscriber; the SDK's own diagnostics must never be captured as customer telemetry. */
export function runUnobserved(fn: () => void) {
  depth += 1;

  try {
    fn();
  } finally {
    depth -= 1;
  }
}

/** Whether console calls made now are hidden from subscribers. */
export function isUnobserved() {
  return depth > 0;
}
