import { detectActiveBundler } from "./detect-bundler.js";
import { describe, expect, test } from "vitest";

describe(detectActiveBundler, () => {
  test.each([
    { name: "no nextMajor", nextMajor: null },
    { name: "nextMajor 15", nextMajor: 15 },
    { name: "nextMajor 16", nextMajor: 16 },
  ])("returns turbopack for TURBOPACK=1 with $name", ({ nextMajor }) => {
    expect(
      detectActiveBundler(
        { TURBOPACK: "1" },
        ["node", "next", "build"],
        nextMajor,
      ),
    ).toBe("turbopack");
  });

  test.each([
    {
      expected: "webpack",
      name: "'' with nextMajor null",
      nextMajor: null,
      value: "",
    },
    {
      expected: "webpack",
      name: "'0' with nextMajor null",
      nextMajor: null,
      value: "0",
    },
    {
      expected: "webpack",
      name: "'true' with nextMajor 15",
      nextMajor: 15,
      value: "true",
    },
    {
      expected: "turbopack",
      name: "'true' with nextMajor 16",
      nextMajor: 16,
      value: "true",
    },
  ])(
    "ignores TURBOPACK=$name and follows the version default",
    ({ value, nextMajor, expected }) => {
      expect(
        detectActiveBundler(
          { TURBOPACK: value },
          ["node", "next", "build"],
          nextMajor,
        ),
      ).toBe(expected);
    },
  );

  test.each([
    { flag: "--turbo", name: "--turbo with nextMajor null", nextMajor: null },
    { flag: "--turbo", name: "--turbo with nextMajor 15", nextMajor: 15 },
    {
      flag: "--turbopack",
      name: "--turbopack with nextMajor null",
      nextMajor: null,
    },
    {
      flag: "--turbopack",
      name: "--turbopack with nextMajor 15",
      nextMajor: 15,
    },
  ])("returns turbopack for $name", ({ flag, nextMajor }) => {
    expect(
      detectActiveBundler({}, ["node", "next", "build", flag], nextMajor),
    ).toBe("turbopack");
  });

  test.each([
    {
      argv: ["node", "next", "build", "--webpack"],
      env: {},
      name: "--webpack alone with nextMajor null",
      nextMajor: null,
    },
    {
      argv: ["node", "next", "build", "--webpack"],
      env: {},
      name: "--webpack with nextMajor 16 default",
      nextMajor: 16,
    },
    {
      argv: ["node", "next", "build", "--webpack"],
      env: {},
      name: "--webpack with nextMajor 17 default",
      nextMajor: 17,
    },
    {
      argv: ["node", "next", "build", "--turbopack", "--webpack"],
      env: {},
      name: "--webpack alongside --turbopack",
      nextMajor: null,
    },
    {
      argv: ["node", "next", "build", "--turbo", "--webpack"],
      env: {},
      name: "--webpack alongside --turbo",
      nextMajor: null,
    },
    {
      argv: ["node", "next", "build", "--webpack"],
      env: { TURBOPACK: "1" },
      name: "--webpack alongside TURBOPACK=1",
      nextMajor: null,
    },
    {
      argv: ["node", "next", "build", "--turbopack", "--webpack"],
      env: { TURBOPACK: "1" },
      name: "--webpack beats TURBOPACK=1 and nextMajor 16",
      nextMajor: 16,
    },
  ])(
    "returns webpack for explicit opt-out: $name",
    ({ env, argv, nextMajor }) => {
      expect(detectActiveBundler(env, argv, nextMajor)).toBe("webpack");
    },
  );

  test.each([
    { expected: "turbopack", name: "nextMajor 16", nextMajor: 16 },
    { expected: "turbopack", name: "nextMajor 17", nextMajor: 17 },
    { expected: "webpack", name: "nextMajor 15", nextMajor: 15 },
    { expected: "webpack", name: "nextMajor null", nextMajor: null },
    { expected: "webpack", name: "nextMajor undefined", nextMajor: undefined },
  ])(
    "resolves via the version default when no explicit signal is present: $name",
    ({ nextMajor, expected }) => {
      expect(
        detectActiveBundler({}, ["node", "next", "build"], nextMajor),
      ).toBe(expected);
    },
  );

  test.each([
    {
      expected: "webpack",
      name: "nextMajor null follows webpack default",
      nextMajor: null,
    },
    {
      expected: "turbopack",
      name: "nextMajor 16 follows turbopack default",
      nextMajor: 16,
    },
  ])(
    "does not treat --turbo-extra as a turbopack flag: $name",
    ({ nextMajor, expected }) => {
      expect(
        detectActiveBundler(
          {},
          ["node", "next", "build", "--turbo-extra"],
          nextMajor,
        ),
      ).toBe(expected);
    },
  );
});
