const FRAME_LOCATION = /:\d+:\d+\)?$/u;

function throwSite(stack: string | undefined) {
  return (
    stack
      ?.split("\n")
      .find((line) => FRAME_LOCATION.test(line.trim()))
      ?.trim() ?? ""
  );
}

/** Identifies one throw site, so separate Error instances React creates when it retries a render compare equal. */
export function errorFingerprint(error: Error) {
  return `${error.name}|${error.message}|${throwSite(error.stack)}`;
}

/** Drops an Error that a different capture path already recorded from the same throw site during the current synchronous turn. */
export function createTurnDedupe() {
  const recorded = new Map<string, string>();

  return ((fingerprint: string, mechanismType: string) => {
    const previous = recorded.get(fingerprint);

    if (previous !== undefined) {
      return previous === mechanismType;
    }

    if (recorded.size === 0) {
      queueMicrotask(() => {
        recorded.clear();
      });
    }

    recorded.set(fingerprint, mechanismType);

    return true;
  }) satisfies (fingerprint: string, mechanismType: string) => boolean;
}
