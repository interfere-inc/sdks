/* oxlint-disable anti-slop/no-runtime-typeof, anti-slop/no-unknown-parameters -- DOM Events arrive as thrown values or rejection reasons of any shape, and their host fields may be absent or overwritten. */
function hostTag(value: unknown) {
  return Object.prototype.toString.call(value).slice(8, -1);
}

function text(field: unknown) {
  return typeof field === "string" && field.length > 0 ? field : undefined;
}

/** The serialized target of a DOM Event captured as an error. */
export interface SerializedTarget {
  id?: string;
  kind?: string;
  tag?: string;
  url?: string;
}

function describeTarget(target: unknown) {
  if (target === null || typeof target !== "object") {
    return null;
  }

  const tagName = "tagName" in target ? text(target.tagName) : undefined;

  if (!tagName) {
    const kind = hostTag(target);

    return { description: kind, serialized: { kind } };
  }

  const tag = tagName.toLowerCase();
  const id = "id" in target ? text(target.id) : undefined;

  const url =
    ("currentSrc" in target ? text(target.currentSrc) : undefined) ??
    ("src" in target ? text(target.src) : undefined) ??
    ("href" in target ? text(target.href) : undefined);

  const serialized: SerializedTarget = { tag };

  if (id) {
    serialized.id = id;
  }

  if (url) {
    serialized.url = url;
  }

  return {
    description: `${tag}${id ? `#${id}` : ""}${url ? ` ${url}` : ""}`,
    serialized,
  };
}

/** Describes a DOM Event used as an error by its class, type and target, or returns null for any other value. */
export function describeDomEvent(value: unknown) {
  if (value === null || typeof value !== "object") {
    return null;
  }

  const kind = hostTag(value);
  const type = "type" in value ? text(value.type) : undefined;

  if (!kind.endsWith("Event") || !type) {
    return null;
  }

  const target = describeTarget("target" in value ? value.target : null);

  return {
    kind: "non-error",
    serialized: { target: target?.serialized ?? null, type },
    type: kind,
    value: target
      ? `${kind} '${type}' on ${target.description}`
      : `${kind} '${type}'`,
  } as const;
}
