const TEXT_ENTRY_INPUT_TYPES: ReadonlySet<string> = new Set([
  "date",
  "datetime-local",
  "email",
  "month",
  "number",
  "password",
  "search",
  "tel",
  "text",
  "time",
  "url",
  "week",
]);

const REPEAT_CLICK_WINDOW_MS = 1000;

const LABEL_FORWARD_WINDOW_MS = 100;

function isTextEntry(element: Element) {
  if (element instanceof HTMLTextAreaElement) {
    return true;
  }

  if (element instanceof HTMLInputElement) {
    return TEXT_ENTRY_INPUT_TYPES.has(element.type);
  }

  return element instanceof HTMLElement && element.isContentEditable;
}

/**
 * Decides which actionable clicks become spans. Skips focusing a text field,
 * the control click a `<label>` forwards, and repeat clicks on the same
 * element within a second; rage clicks are recorded separately.
 */
export function createClickSpanGate() {
  let previous: { element: Element; at: number } | undefined;

  return (element: Element) => {
    const at = performance.now();

    const last = previous;

    previous = { element, at };

    if (isTextEntry(element)) {
      return false;
    }

    if (last === undefined) {
      return true;
    }

    const elapsed = at - last.at;

    if (last.element === element && elapsed < REPEAT_CLICK_WINDOW_MS) {
      return false;
    }

    return !(
      last.element instanceof HTMLLabelElement &&
      last.element.control === element &&
      elapsed < LABEL_FORWARD_WINDOW_MS
    );
  };
}
