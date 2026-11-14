import { expect, test, vi } from "vitest";
import { primeReactDomKeys } from "./react-dom-keys.js";

const listeningMarkers = () =>
  Object.keys(document).filter((key) => key.startsWith("_reactListening"));

test("mounts the prime root in a throwaway document so hydrateRoot(document) still attaches event listeners", () => {
  const createHTMLDocument = vi.spyOn(
    document.implementation,
    "createHTMLDocument",
  );

  expect(listeningMarkers()).toStrictEqual([]);

  primeReactDomKeys();

  primeReactDomKeys();

  expect(listeningMarkers()).toStrictEqual([]);

  expect(createHTMLDocument).toHaveBeenCalledTimes(1);

  expect(createHTMLDocument.mock.results[0]?.value).not.toBe(document);
});
