const MARKER = "__interfere_server_action__";

export interface ServerActionMarker {
  readonly argumentSummary?: string;
  readonly name: string;
}

export function markServerAction(
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Server actions may throw any JavaScript value; primitive throws must pass through without annotation.
  error: unknown,
  info: ServerActionMarker,
) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Error annotation requires an object; primitive throws cannot carry marker properties.
  if (error === null || typeof error !== "object") {
    return;
  }

  Object.defineProperty(error, MARKER, {
    configurable: true,
    enumerable: false,
    value: info,
    writable: true,
  });
}

export function readServerAction(
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Request error hooks receive arbitrary thrown values, not exclusively Error instances.
  error: unknown,
) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Primitive throws have no marker properties to inspect.
  if (error === null || typeof error !== "object") {
    return;
  }

  if (!(MARKER in error)) {
    return;
  }

  const marker = error[MARKER];

  if (
    marker === null || // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Customer-thrown objects may already contain a non-object value at the marker key.
    typeof marker !== "object" ||
    !("name" in marker) ||
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Only string marker names are valid action identities.
    typeof marker.name !== "string"
  ) {
    return;
  }

  if (!("argumentSummary" in marker) || marker.argumentSummary === undefined) {
    return { name: marker.name } satisfies ServerActionMarker | undefined;
  }

  // oxlint-disable-next-line anti-slop/no-runtime-typeof
  if (typeof marker.argumentSummary !== "string") {
    return;
  }

  return {
    name: marker.name,
    argumentSummary: marker.argumentSummary,
  } satisfies ServerActionMarker;
}
