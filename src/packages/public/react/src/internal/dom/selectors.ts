import { createLogger } from "../../util/log.js";

const log = createLogger("privacy");

function isClosed(selector: string) {
  let depth = 0;
  let quote = "";

  for (let index = 0; index < selector.length; index += 1) {
    const char = selector[index];

    if (char === "\\") {
      index += 1;
    } else if (quote) {
      quote = char === quote ? "" : quote;
    } else if (char === '"' || char === "'") {
      quote = char;
    } else if (char === "(" || char === "[") {
      depth += 1;
    } else if (char === ")" || char === "]") {
      depth -= 1;
    }
  }

  return depth === 0 && !quote;
}

function isValidSelector(selector: string) {
  if (!isClosed(selector)) {
    return false;
  }

  try {
    document.createDocumentFragment().querySelector(selector);

    return true;
  } catch {
    return false;
  }
}

export function validSelectors(list: string | undefined, option: string) {
  const parts = (list ?? "").split(",");
  const valid: string[] = [];
  let start = 0;

  while (start < parts.length) {
    let end = parts.length;

    while (end > start && !isValidSelector(parts.slice(start, end).join(","))) {
      end -= 1;
    }

    if (end > start) {
      valid.push(parts.slice(start, end).join(",").trim());

      start = end;
    } else {
      const invalid = parts[start]?.trim();

      if (invalid) {
        log.warn("ignoring invalid %s selector %o", option, invalid);
      }

      start += 1;
    }
  }

  return valid.join(", ") || undefined;
}
