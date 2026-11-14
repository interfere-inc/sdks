import { expect, test, vi } from "vitest";

test("invalid origins never become propagation targets", async () => {
  const { serverPropagationTargets } = await import("./propagation-targets.js");

  const warn = vi.spyOn(console, "warn");

  expect(
    serverPropagationTargets([
      "http://api.example.com",
      "https://*.example.com",
      "https://api.example.com/path",
      "https://user:pass@api.example.com",
      "https://api.example.com?query=1",
      "https://api.example.com/#fragment",
      String.raw`https://api.example.com\evil`,
      "not-an-origin",
    ]),
  ).toStrictEqual([]);

  expect(warn).toHaveBeenCalledTimes(8);
});
