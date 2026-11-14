import type { UrlRedactionOption } from "@interfere/types/sdk/url-redaction";

import { createLogger } from "../../util/log.js";
import {
  INTERFERE_BLOCK_CLASS,
  INTERFERE_MASK_TEXT_CLASS,
} from "./privacy-classes.js";

const log = createLogger("privacy");

export type PrivacyLevel = "blocked" | "masked" | "public";

/** What the SDK never records. Selectors apply to session replay and to click, submit and rage-click records from the first capture. */
export interface PrivacyOptions {
  /**
   * CSS selector for elements recorded only as placeholders. Replay shows a
   * same-size box; interaction records keep the event, tag and field label.
   * Matches the element and everything inside it.
   */
  blockSelector?: string;
  /**
   * CSS selector for elements whose text is masked in replay and omitted from
   * interaction records. Matches the element and everything inside it.
   */
  maskTextSelector?: string;
  /** Query values and fragments to redact in recorded URLs. Defaults to built-in sensitive parameter names. */
  urls?: UrlRedactionOption;
}

export interface PrivacyPolicy {
  readonly blockAll: boolean;
  readonly blockSelector: string;
  readonly maskSelector: string;
}

const LABEL_ATTRIBUTE = "data-interfere-label";

const FORM_FIELD_SELECTOR = "input, select, textarea";

const TEXT_LIMIT = 120;

export const DEFAULT_PRIVACY_POLICY: PrivacyPolicy = {
  blockAll: false,
  blockSelector: `.${INTERFERE_BLOCK_CLASS}`,
  maskSelector: `.${INTERFERE_MASK_TEXT_CLASS}`,
};

export function resolvePrivacyPolicy({
  blockSelector,
  maskTextSelector,
}: PrivacyOptions = {}) {
  const policy: PrivacyPolicy = {
    blockAll: false,
    blockSelector: [DEFAULT_PRIVACY_POLICY.blockSelector, blockSelector]
      .filter(Boolean)
      .join(", "),
    maskSelector: [DEFAULT_PRIVACY_POLICY.maskSelector, maskTextSelector]
      .filter(Boolean)
      .join(", "),
  };

  try {
    document.querySelector(`${policy.blockSelector}, ${policy.maskSelector}`);

    return policy;
  } catch {
    log.warn("invalid privacy selector; interaction details are blocked");

    return { ...policy, blockAll: true } satisfies PrivacyPolicy;
  }
}

export function privacyLevel(element: Element, policy: PrivacyPolicy) {
  if (policy.blockAll || element.closest(policy.blockSelector)) {
    return "blocked";
  }

  return (
    element.closest(policy.maskSelector) ? "masked" : "public"
  ) satisfies PrivacyLevel;
}

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
