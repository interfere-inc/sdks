/** Length and breadth caps applied to captured exception payloads so one event fits a keepalive export. */
export const EXCEPTION_LIMITS = {
  messageLength: 1024,
  serializedLength: 4096,
  stackFrames: 50,
  stackLength: 6144,
} as const;

const ELLIPSIS = "…";

const FRAME_LINE_RE = /^\s*at\s|^[^@\n]*@\S*:\d+(?::\d+)?\s*$/u;

const HIGH_SURROGATE_RE = /[\uD800-\uDBFF]$/u;

/** True when a raw stack line is a frame in V8 (`at …`) or Gecko/JavaScriptCore (`fn@url:line:col`) form. */
export function isStackFrameLine(line: string) {
  return FRAME_LINE_RE.test(line);
}

/** Clips text to `max` characters, ending with an ellipsis and never splitting a surrogate pair. */
export function truncateText(text: string, max: number) {
  if (text.length <= max) {
    return text;
  }

  const head = text.slice(0, Math.max(0, max - ELLIPSIS.length));

  return `${HIGH_SURROGATE_RE.test(head) ? head.slice(0, -1) : head}${ELLIPSIS}`;
}

/** Caps an exception message at `EXCEPTION_LIMITS.messageLength` characters. */
export function truncateExceptionMessage(message: string) {
  return truncateText(message, EXCEPTION_LIMITS.messageLength);
}

/** Caps a raw stack's message line, frame count and total length. */
export function truncateExceptionStack(stack: string, message?: string) {
  let bounded = stack;

  if (message && message.length > EXCEPTION_LIMITS.messageLength) {
    const truncated = truncateExceptionMessage(message);

    bounded = bounded.replace(message, () => truncated);
  }

  if (bounded.length <= EXCEPTION_LIMITS.stackLength) {
    const lines = bounded.split("\n");

    if (
      lines.filter((line) => isStackFrameLine(line)).length <=
      EXCEPTION_LIMITS.stackFrames
    ) {
      return bounded;
    }
  }

  const kept: string[] = [];
  let frames = 0;
  let length = 0;

  for (const line of bounded.split("\n")) {
    const isFrame = isStackFrameLine(line);

    if (isFrame && frames >= EXCEPTION_LIMITS.stackFrames) {
      break;
    }

    const cost = line.length + (kept.length > 0 ? 1 : 0);

    if (length + cost > EXCEPTION_LIMITS.stackLength) {
      if (kept.length === 0) {
        kept.push(truncateText(line, EXCEPTION_LIMITS.stackLength));
      }

      break;
    }

    kept.push(line);

    length += cost;

    if (isFrame) {
      frames += 1;
    }
  }

  return kept.join("\n");
}
