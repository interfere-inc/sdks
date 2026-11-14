// oxlint-disable anti-slop/no-runtime-typeof
interface SourceMapRichness {
  hasSourcesContent: boolean;
  hasNames: boolean;
  hasFile: boolean;
  mappingsPresent: boolean;
}

const EMPTY_RICHNESS: SourceMapRichness = {
  hasSourcesContent: false,
  hasNames: false,
  hasFile: false,
  mappingsPresent: false,
};

// oxlint-disable-next-line anti-slop/no-object-parameters -- Source-map inspection accepts partial JSON objects and reads optional fields without claiming a validated source-map contract.
function leafRichness(map: object) {
  const sourcesContent =
    "sourcesContent" in map ? map.sourcesContent : undefined;

  const names = "names" in map ? map.names : undefined;
  const file = "file" in map ? map.file : undefined;
  const mappings = "mappings" in map ? map.mappings : undefined;

  return {
    hasSourcesContent:
      Array.isArray(sourcesContent) && sourcesContent.length > 0,
    hasNames: Array.isArray(names) && names.length > 0,
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Parsed source-map file fields may be missing, empty, or non-string JSON values.
    hasFile: typeof file === "string" && file.length > 0,
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Only nonempty mapping strings count; other JSON values must not advertise mappings.
    mappingsPresent: typeof mappings === "string" && mappings.length > 0,
  } satisfies SourceMapRichness;
}

export function extractRichness(content: string) {
  let parsed: unknown;

  try {
    parsed = JSON.parse(content);
  } catch {
    return EMPTY_RICHNESS;
  }

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Valid JSON primitives are not source-map containers and must report no richness.
  if (!(parsed !== null && typeof parsed === "object")) {
    return EMPTY_RICHNESS;
  }

  const sections = "sections" in parsed ? parsed.sections : undefined;

  if (!Array.isArray(sections)) {
    return leafRichness(parsed);
  }

  const merged: SourceMapRichness = {
    hasSourcesContent: false,
    hasNames: false,
    hasFile:
      "file" in parsed &&
      // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Indexed maps may contain a malformed root file field independently of their sections.
      typeof parsed.file === "string" &&
      parsed.file.length > 0,
    mappingsPresent: false,
  };

  const indexedSections: readonly unknown[] = sections;

  for (const section of indexedSections) {
    if (
      section === null ||
      typeof section !== "object" ||
      !("map" in section)
    ) {
      continue;
    }

    const inner: unknown = section.map;

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- A section's map field is arbitrary JSON until checked; malformed sections must not discard valid sibling richness.
    if (!(inner !== null && typeof inner === "object")) {
      continue;
    }

    const r = leafRichness(inner);

    merged.hasSourcesContent ||= r.hasSourcesContent;
    merged.hasNames ||= r.hasNames;
    merged.mappingsPresent ||= r.mappingsPresent;
  }

  return merged;
}
