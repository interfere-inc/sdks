import { extractRichness } from "@interfere/types/sdk/source-map-richness";
import { expect, test } from "vitest";

const empty = {
  hasSourcesContent: false,
  hasNames: false,
  hasFile: false,
  mappingsPresent: false,
};

test.each(["", "{", "null", "true", "42", '"source"', "[]", "{}"])(
  "reports no richness for %j",
  (content) => {
    expect(extractRichness(content)).toStrictEqual(empty);
  },
);

test("recognizes populated leaf fields", () => {
  expect(
    extractRichness(
      JSON.stringify({
        version: 3,
        sourcesContent: ["const answer = 42;"],
        names: ["answer"],
        file: "app.js",
        mappings: "AAAA",
      }),
    ),
  ).toStrictEqual({
    hasSourcesContent: true,
    hasNames: true,
    hasFile: true,
    mappingsPresent: true,
  });
});

test.each([
  { sourcesContent: [], names: [], file: "", mappings: "" },
  { sourcesContent: "source", names: "name", file: 1, mappings: true },
  { sourcesContent: null, names: null, file: null, mappings: null },
  { sourcesContent: {}, names: {}, file: [], mappings: {} },
])("does not advertise malformed or empty fields in %j", (map) => {
  expect(extractRichness(JSON.stringify(map))).toStrictEqual(empty);
});

test("preserves presence semantics for arrays containing unavailable entries", () => {
  expect(
    extractRichness(JSON.stringify({ sourcesContent: [null], names: [null] })),
  ).toStrictEqual({ ...empty, hasSourcesContent: true, hasNames: true });
});

test.each([null, false, 1, "sections", {}])(
  "treats non-array sections %j as a leaf map",
  (sections) => {
    expect(
      extractRichness(JSON.stringify({ sections, mappings: "AAAA" })),
    ).toStrictEqual({ ...empty, mappingsPresent: true });
  },
);

test("merges valid indexed sections without dropping siblings of malformed sections", () => {
  expect(
    extractRichness(
      JSON.stringify({
        file: "app.js",
        sections: [
          null,
          true,
          1,
          "section",
          {},
          { map: null },
          { map: false },
          { map: 1 },
          { map: "map" },
          { map: { sourcesContent: ["const answer = 42;"] } },
          { map: { names: ["answer"] } },
          { map: { mappings: "AAAA" } },
        ],
      }),
    ),
  ).toStrictEqual({
    hasSourcesContent: true,
    hasNames: true,
    hasFile: true,
    mappingsPresent: true,
  });
});

test.each([null, 1, false, "", [], {}])(
  "does not inherit an indexed map file from inner sections when root file is %j",
  (file) => {
    expect(
      extractRichness(
        JSON.stringify({ file, sections: [{ map: { file: "inner.js" } }] }),
      ),
    ).toStrictEqual(empty);
  },
);

test("uses section fields rather than leaf fields on indexed maps", () => {
  expect(
    extractRichness(
      JSON.stringify({
        sourcesContent: ["ignored root source"],
        names: ["ignored root name"],
        mappings: "AAAA",
        sections: [],
      }),
    ),
  ).toStrictEqual(empty);
});
