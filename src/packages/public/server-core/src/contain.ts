const reported = new Set<string>();

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Contained failures are arbitrary thrown values from SDK internals or third-party OTel code.
function reportContainedFailure(site: string, error: unknown) {
  if (reported.has(site)) {
    return;
  }

  reported.add(site);

  try {
    console.warn(
      `[interfere] ${site} failed inside the Interfere SDK. Your app keeps running; Interfere skipped this step and may send less telemetry.`,
      error,
    );
  } catch {
    return;
  }
}

export function contain<T>(site: string, run: () => T) {
  try {
    return run();
  } catch (error) {
    reportContainedFailure(site, error);

    return null;
  }
}

export async function containAsync<T>(site: string, run: () => Promise<T>) {
  try {
    return await run();
  } catch (error) {
    reportContainedFailure(site, error);

    return null;
  }
}
