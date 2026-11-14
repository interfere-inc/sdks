import { omitUndefined } from "@interfere/helpers/omit-undefined";
import type { SetRequired } from "@interfere/helpers/set-required";

import {
  INTERFERE_BLOCK_CLASS,
  INTERFERE_IGNORE_CLASS,
  INTERFERE_MASK_TEXT_CLASS,
} from "../../internal/dom/privacy-classes.js";
import type { PrivacyPolicy } from "../../internal/dom/privacy.js";

/** Session replay recording options passed through to the bundled rrweb recorder. */
export interface ReplayConfig {
  blockClass?: string | RegExp;
  blockSelector?: string;
  ignoreClass?: string;
  ignoreSelector?: string;
  maskAllInputs?: boolean;
  maskInputFn?: (text: string, element: HTMLElement) => string;
  maskInputOptions?: Partial<
    Record<
      | "color"
      | "date"
      | "datetime-local"
      | "email"
      | "month"
      | "number"
      | "password"
      | "range"
      | "search"
      | "select"
      | "tel"
      | "text"
      | "textarea"
      | "time"
      | "url"
      | "week",
      boolean
    >
  >;
  maskTextClass?: string | RegExp;
  maskTextFn?: (text: string, element: HTMLElement | null) => string;
  maskTextSelector?: string;
  recordCanvas?: boolean;
}

export type ReplayOption = boolean | ReplayConfig;

export type ReplayRecordOptions = SetRequired<
  ReplayConfig,
  "maskTextClass" | "blockClass" | "ignoreClass"
>;

function joinSelectors(...selectors: (string | undefined)[]) {
  return selectors.filter(Boolean).join(", ") || undefined;
}

export function toRecordOptions(config: ReplayConfig, privacy?: PrivacyPolicy) {
  const options: ReplayRecordOptions = omitUndefined({
    ...config,
    blockClass: INTERFERE_BLOCK_CLASS,
    blockSelector: joinSelectors(privacy?.blockSelector, config.blockSelector),
    ignoreClass: INTERFERE_IGNORE_CLASS,
    maskTextClass: INTERFERE_MASK_TEXT_CLASS,
    maskTextSelector: joinSelectors(
      privacy?.maskSelector,
      config.maskTextSelector,
    ),
  });

  return options;
}
