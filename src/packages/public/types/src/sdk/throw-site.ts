import { isNonErrorException } from "#sdk/errors";
import type { NonErrorException } from "#sdk/errors";

const MAX_THROW_SITE_LINES = 30;

const MAX_THROW_SITE_LINE_LENGTH = 1024;

const V8_FRAME_RE =
  /^\s*at\s+(?:(?<fn>.+?)\s+\()?(?<location>.+?):(?<line>\d+)(?::\d+)?\)?\s*$/u;

const GECKO_FRAME_RE =
  /^(?<fn>[^@]*)@(?<location>.+?):(?<line>\d+)(?::\d+)?\s*$/u;

const FOREIGN_LOCATION_RE =
  /(?:^|\/)node_modules\/|^(?:native|<anonymous>|\[native code\])$|\beval at\s|^(?:chrome|moz|safari-web|safari|ms-browser|webkit-masked)-(?:extension|url):/u;

const ANONYMOUS_FN_RE = /^(?:<anonymous>|Object\.<anonymous>|global code)?$/u;

function basename(location: string) {
  const trimmed = location.replace(/[?#].*$/u, "");
  const last = trimmed.lastIndexOf("/");

  return (last === -1 ? trimmed : trimmed.slice(last + 1)) || trimmed;
}

function throwSiteLabel(stack: string) {
  for (const rawLine of stack.split("\n", MAX_THROW_SITE_LINES)) {
    const line = rawLine.slice(0, MAX_THROW_SITE_LINE_LENGTH);

    const groups = (V8_FRAME_RE.exec(line) ?? GECKO_FRAME_RE.exec(line))
      ?.groups;

    if (!groups?.["location"] || !groups["line"]) {
      continue;
    }

    const location = groups["location"].trim();

    if (FOREIGN_LOCATION_RE.test(location)) {
      continue;
    }

    const fn = (groups["fn"] ?? "").replace(/^(?:async|new)\s+/u, "").trim();
    const site = `${basename(location)}:${groups["line"]}`;

    return ANONYMOUS_FN_RE.test(fn) ? site : `${fn} (${site})`;
  }

  return null;
}

/**
 * Name for the span the SDK opens to carry a captured exception when no
 * span is active: the top in-app frame of the raw stack as `fn (file:line)`
 * (`file:line` for anonymous frames; `node_modules`, native and
 * browser-extension frames are skipped), or the error's type when the
 * stack has no usable frame or host code replaced it with a non-string.
 * Non-Error rejections carry no stack and use their structural type. The
 * label mirrors the raw text the SDK ships; symbolication of minified
 * names stays server-side.
 */
export function exceptionSpanName(value: Error | NonErrorException) {
  if (isNonErrorException(value)) {
    return value.type;
  }

  const stack: string | undefined = value.stack;

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Host code can overwrite `error.stack` with a non-string; the name must never throw at capture time.
  const label = typeof stack === "string" ? throwSiteLabel(stack) : null;

  return label ?? (value.name || "Error");
}
