import { privacyLevel } from "./privacy.js";
import type { PrivacyPolicy } from "./privacy.js";

const LABEL_ATTRIBUTE = "data-interfere-label";

const FORM_FIELD_SELECTOR = "input, select, textarea";

const TEXT_LIMIT = 120;

export function publicText(element: Element, policy: PrivacyPolicy) {
  if (element.matches(FORM_FIELD_SELECTOR)) {
    return "";
  }

  const hidden = `${policy.blockSelector}, ${policy.maskSelector}, ${FORM_FIELD_SELECTOR}`;

  const walker = document.createTreeWalker(
    element,
    // oxlint-disable-next-line no-bitwise -- TreeWalker filters are bit masks.
    NodeFilter.SHOW_ELEMENT | NodeFilter.SHOW_TEXT,
    {
      acceptNode(node) {
        if (!(node instanceof Element)) {
          return NodeFilter.FILTER_ACCEPT;
        }

        return node.matches(hidden)
          ? NodeFilter.FILTER_REJECT
          : NodeFilter.FILTER_SKIP;
      },
    },
  );

  let text = "";

  for (
    let node = walker.nextNode();
    node && text.trimStart().length < TEXT_LIMIT;
    node = walker.nextNode()
  ) {
    text += node.nodeValue ?? "";
  }

  return text.trim().slice(0, TEXT_LIMIT);
}

function labelElements(element: Element) {
  const labels: Element[] =
    element instanceof HTMLButtonElement ||
    element instanceof HTMLInputElement ||
    element instanceof HTMLSelectElement ||
    element instanceof HTMLTextAreaElement
      ? [...(element.labels ?? [])]
      : [];

  for (const id of element.getAttribute("aria-labelledby")?.split(/\s+/u) ??
    []) {
    const labelledBy = id
      ? element.ownerDocument.querySelector(`#${CSS.escape(id)}`)
      : null;

    if (labelledBy) {
      labels.push(labelledBy);
    }
  }

  return labels;
}

export function fieldLabel(element: Element, policy: PrivacyPolicy) {
  const explicit = element
    .closest(`[${LABEL_ATTRIBUTE}]`)
    ?.getAttribute(LABEL_ATTRIBUTE)
    ?.trim();

  if (explicit) {
    return explicit.slice(0, TEXT_LIMIT);
  }

  return labelElements(element)
    .filter((label) => privacyLevel(label, policy) === "public")
    .map((label) => publicText(label, policy))
    .filter(Boolean)
    .join(" ")
    .slice(0, TEXT_LIMIT);
}
