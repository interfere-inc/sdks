import {
  EXCEPTION_LIMITS,
  truncateExceptionMessage,
  truncateText,
} from "./truncate.ts";

const MAX_DEPTH = 4;

const MAX_ENTRIES = 50;

const MAX_STRING_LENGTH = 512;

const MARKER_RESERVE = 256;

const DEPTH_LIMIT_SENTINEL = "[depth-limit]";

const CIRCULAR_SENTINEL = "[circular]";

const TRUNCATED_SENTINEL = "[truncated]";

type SerializedException =
  | null
  | undefined
  | string
  | number
  | boolean
  | SerializedException[]
  | { [key: string]: SerializedException };

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Exception grouping classifies every JavaScript value, including primitives.
export function classifyNonError(value: unknown) {
  if (value === null) {
    return "null";
  }

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- The primitive typeof label is the exported exception grouping category.
  const t = typeof value;

  if (t !== "object") {
    return t;
  }

  const tag = Object.prototype.toString.call(value).slice(8, -1);

  return tag === "Object" ? "PlainObject" : tag;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Exception descriptions preserve arbitrary thrown primitives and object keys.
function describe(value: unknown) {
  if (value === null || value === undefined) {
    return `Non-Error captured: ${String(value)}`;
  }

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Object descriptions list keys without serializing potentially cyclic values.
  if (typeof value === "object") {
    const keys = Object.keys(value).toSorted();

    if (keys.length === 0) {
      return "Non-Error captured: empty object";
    }

    return `Non-Error captured with keys: ${keys.join(", ")}`;
  }

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Thrown functions use Function.prototype.toString rather than executing them.
  if (typeof value === "function") {
    return `Non-Error captured: ${Function.prototype.toString.call(value)}`;
  }

  if (
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Thrown primitive descriptions preserve their JavaScript value without parsing.
    typeof value === "string" ||
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Thrown primitive descriptions preserve their JavaScript value without parsing.
    typeof value === "number" ||
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Thrown primitive descriptions preserve their JavaScript value without parsing.
    typeof value === "boolean" ||
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Thrown primitive descriptions preserve their JavaScript value without parsing.
    typeof value === "bigint" ||
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Thrown primitive descriptions preserve their JavaScript value without parsing.
    typeof value === "symbol"
  ) {
    return `Non-Error captured: ${String(value)}`;
  }

  return "Non-Error captured";
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Exception descriptions accept every JavaScript thrown value.
export function describeNonError(value: unknown) {
  return truncateExceptionMessage(describe(value));
}

/**
 * Serializes a thrown value bounded by depth, entries per container, string
 * length and total JSON length, so `JSON.stringify` of the result stays within
 * `EXCEPTION_LIMITS.serializedLength`.
 */
// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Exception payload serialization accepts arbitrary cyclic JavaScript values.
export function serializeNonError(value: unknown) {
  const seen = new WeakSet<object>();
  let remaining = EXCEPTION_LIMITS.serializedLength - MARKER_RESERVE;

  const leaf = (text: string) => {
    const clipped = truncateText(
      text,
      Math.max(0, Math.min(MAX_STRING_LENGTH, remaining - 2)),
    );

    remaining -= JSON.stringify(clipped).length + 1;

    return remaining >= 0 ? clipped : TRUNCATED_SENTINEL;
  };

  // oxlint-disable-next-line anti-slop/no-unknown-parameters, landmines/no-explicit-return-types -- Each recursively visited exception property may contain any JavaScript value.
  const walk = (current: unknown, depth: number): SerializedException => {
    if (current === null) {
      remaining -= 5;

      return null;
    }

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- String leaves are clipped to the remaining budget.
    if (typeof current === "string") {
      return leaf(current);
    }

    if (
      // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Serializable primitive leaves retain their original type.
      typeof current === "number" ||
      // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Serializable primitive leaves retain their original type.
      typeof current === "boolean"
    ) {
      remaining -= String(current).length + 1;

      return current;
    }

    if (current === undefined) {
      return;
    }

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Function-valued exception properties are rendered as text rather than invoked.
    if (typeof current === "function") {
      return leaf(current.toString());
    }

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Non-JSON primitive leaves are rendered as strings without executing them.
    if (typeof current === "symbol" || typeof current === "bigint") {
      return leaf(String(current));
    }

    if (depth > MAX_DEPTH) {
      return leaf(DEPTH_LIMIT_SENTINEL);
    }

    if (seen.has(current)) {
      return leaf(CIRCULAR_SENTINEL);
    }

    seen.add(current);

    remaining -= 2;

    if (Array.isArray(current)) {
      const items: SerializedException[] = [];

      for (const item of current) {
        if (items.length >= MAX_ENTRIES || remaining <= 0) {
          items.push(TRUNCATED_SENTINEL);

          break;
        }

        items.push(walk(item, depth + 1));
      }

      return items;
    }

    const out: Record<string, SerializedException> = {};
    let entries = 0;

    for (const [key, entry] of Object.entries(current)) {
      remaining -= JSON.stringify(key).length + 1;

      if (entries >= MAX_ENTRIES || remaining <= 0) {
        out[TRUNCATED_SENTINEL] = TRUNCATED_SENTINEL;

        break;
      }

      out[key] = walk(entry, depth + 1);

      entries += 1;
    }

    return out;
  };

  return walk(value, 0);
}
