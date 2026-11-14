import { isNonErrorException } from "../errors.ts";
import type { NonErrorException } from "../errors.ts";
import type { ErrorFilterPattern } from "./filter-options.ts";

const V8_FRAME_RE = /^\s*at\s/u;

const LOCATION_SUFFIX_RE = /:\d+(?::\d+)?$/u;

function matchesAny(value: string, patterns: readonly ErrorFilterPattern[]) {
  return patterns.some((pattern) => {
    if (pattern instanceof RegExp) {
      pattern.lastIndex = 0;

      return pattern.test(value);
    }

    return value.includes(pattern);
  });
}

function frameLocation(line: string, v8: boolean) {
  const trimmed = line.trim();
  let location: string;

  if (v8) {
    const body = trimmed.replace(V8_FRAME_RE, "");
    const open = body.lastIndexOf("(");

    location =
      open === -1
        ? body
        : body.slice(open + 1, body.endsWith(")") ? -1 : undefined);
  } else {
    location = trimmed.slice(trimmed.lastIndexOf("@") + 1);
  }

  return LOCATION_SUFFIX_RE.test(location)
    ? location.replace(LOCATION_SUFFIX_RE, "")
    : undefined;
}

/** The script URL of the innermost stack frame that carries a source location. */
export function topFrameUrl(stack: string) {
  const lines = stack.split("\n");
  const v8 = lines.some((line) => V8_FRAME_RE.test(line));

  for (const line of lines) {
    if (v8 ? !V8_FRAME_RE.test(line) : !line.includes("@")) {
      continue;
    }

    const url = frameLocation(line, v8);

    if (url) {
      return url;
    }
  }

  return null;
}

/** True when the exception's message, or `Type: message`, matches any pattern. */
export function matchesIgnoredError(
  exception: Error | NonErrorException,
  patterns: readonly ErrorFilterPattern[],
) {
  const [type, message] = isNonErrorException(exception)
    ? [exception.type, exception.value]
    : [exception.name, exception.message];

  return (
    matchesAny(message, patterns) || matchesAny(`${type}: ${message}`, patterns)
  );
}

/** True when the script URL matches any pattern. */
export function matchesDeniedUrl(
  url: string | null,
  patterns: readonly ErrorFilterPattern[],
) {
  return url !== null && matchesAny(url, patterns);
}
