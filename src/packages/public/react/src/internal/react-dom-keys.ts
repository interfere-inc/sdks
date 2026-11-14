import { createElement } from "react";
import { flushSync } from "react-dom";
import { createRoot } from "react-dom/client";

let primed = false;

export function primeReactDomKeys() {
  if (primed || typeof document === "undefined") {
    return;
  }

  primed = true;

  try {
    // WARNING: The container must belong to a throwaway document; React marks a root's ownerDocument as listening, and a later hydrateRoot(document) then attaches no event handlers.
    const inert = document.implementation.createHTMLDocument("");
    const root = createRoot(inert.createElement("div"));

    flushSync(() => {
      root.render(createElement("span"));
    });

    root.unmount();
  } catch {
    return;
  }
}
