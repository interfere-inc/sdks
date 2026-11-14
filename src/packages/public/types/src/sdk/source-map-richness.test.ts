import { extractRichness } from "./source-map-richness";
import { describe, expect, test } from "vitest";

describe(extractRichness, () => {
  test("returns all-false on invalid JSON", () => {
    expect(extractRichness("not json")).toStrictEqual({
      hasFile: false,
      hasNames: false,
      hasSourcesContent: false,
      mappingsPresent: false,
    });
  });

  test("returns all-false on non-object JSON", () => {
    expect(extractRichness("42")).toStrictEqual({
      hasFile: false,
      hasNames: false,
      hasSourcesContent: false,
      mappingsPresent: false,
    });
  });

  test("reads a flat (webpack-style) source map", () => {
    const map = JSON.stringify({
      file: "chunk.js",
      mappings: "AAAA",
      names: ["foo"],
      sources: ["a.ts", "b.ts"],
      sourcesContent: ["x", "y"],
      version: 3,
    });

    expect(extractRichness(map)).toStrictEqual({
      hasFile: true,
      hasNames: true,
      hasSourcesContent: true,
      mappingsPresent: true,
    });
  });

  test("reads an indexed (turbopack-style) source map by descending into sections", () => {
    const map = JSON.stringify({
      file: "chunk.js",
      sections: [
        {
          offset: { line: 0, column: 0 },
          map: {
            version: 3,
            sources: ["a.ts"],
            sourcesContent: ["x"],
            names: ["foo"],
            mappings: "AAAA",
          },
        },
      ],
      sources: [],
      version: 3,
    });

    expect(extractRichness(map)).toStrictEqual({
      hasFile: true,
      hasNames: true,
      hasSourcesContent: true,
      mappingsPresent: true,
    });
  });

  test("oRs richness across multiple sections (any-section semantics)", () => {
    const map = JSON.stringify({
      sections: [
        {
          offset: { line: 0, column: 0 },
          map: {
            version: 3,
            sources: ["a.ts"],
            mappings: "AAAA",
          },
        },
        {
          offset: { line: 1, column: 0 },
          map: {
            version: 3,
            sources: ["b.ts"],
            sourcesContent: ["y"],
            names: ["bar"],
            mappings: "AAAA",
          },
        },
      ],
      version: 3,
    });

    expect(extractRichness(map)).toStrictEqual({
      hasFile: false,
      hasNames: true,
      hasSourcesContent: true,
      mappingsPresent: true,
    });
  });

  test("treats empty sourcesContent / names arrays as missing", () => {
    const map = JSON.stringify({
      file: "chunk.js",
      mappings: "AAAA",
      names: [],
      sources: ["a.ts"],
      sourcesContent: [],
      version: 3,
    });

    expect(extractRichness(map)).toStrictEqual({
      hasFile: true,
      hasNames: false,
      hasSourcesContent: false,
      mappingsPresent: true,
    });
  });

  test("ignores malformed sections entries", () => {
    const map = JSON.stringify({
      sections: [
        null,
        { offset: { line: 0, column: 0 } },
        { offset: { line: 0, column: 0 }, map: "not an object" },
        {
          offset: { line: 0, column: 0 },
          map: { version: 3, sources: ["a.ts"], mappings: "AAAA" },
        },
      ],
      version: 3,
    });

    expect(extractRichness(map)).toStrictEqual({
      hasFile: false,
      hasNames: false,
      hasSourcesContent: false,
      mappingsPresent: true,
    });
  });
});
