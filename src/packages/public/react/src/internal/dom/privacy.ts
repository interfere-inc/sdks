import type { UrlRedactionOption } from "@interfere/types/sdk/url-redaction";

import {
  INTERFERE_BLOCK_CLASS,
  INTERFERE_MASK_TEXT_CLASS,
} from "./privacy-classes.js";
import { validSelectors } from "./selectors.js";

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
  readonly blockSelector: string;
  readonly maskSelector: string;
}

export const DEFAULT_PRIVACY_POLICY: PrivacyPolicy = {
  blockSelector: `.${INTERFERE_BLOCK_CLASS}`,
  maskSelector: `.${INTERFERE_MASK_TEXT_CLASS}`,
};

export function resolvePrivacyPolicy({
  blockSelector,
  maskTextSelector,
}: PrivacyOptions = {}) {
  const policy: PrivacyPolicy = {
    blockSelector: [
      DEFAULT_PRIVACY_POLICY.blockSelector,
      validSelectors(blockSelector, "privacy.blockSelector"),
    ]
      .filter(Boolean)
      .join(", "),
    maskSelector: [
      DEFAULT_PRIVACY_POLICY.maskSelector,
      validSelectors(maskTextSelector, "privacy.maskTextSelector"),
    ]
      .filter(Boolean)
      .join(", "),
  };

  return policy;
}

export function privacyLevel(element: Element, policy: PrivacyPolicy) {
  if (element.closest(policy.blockSelector)) {
    return "blocked";
  }

  return (
    element.closest(policy.maskSelector) ? "masked" : "public"
  ) satisfies PrivacyLevel;
}
