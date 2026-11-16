import { omitUndefined } from "@interfere/helpers/omit-undefined";
import type { SetRequired } from "@interfere/helpers/set-required";

import {
  INTERFERE_BLOCK_CLASS,
  INTERFERE_IGNORE_CLASS,
  INTERFERE_MASK_TEXT_CLASS,
} from "../../internal/dom/privacy-classes.js";
import { validSelectors } from "../../internal/dom/selectors.js";
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

function classSelector(className: string | RegExp | undefined) {
  if (
    className === undefined ||
    className instanceof RegExp ||
    className === ""
  ) {
    return;
  }

  return `.${CSS.escape(className)}`;
}

function classPattern(own: string, custom: string | RegExp | undefined) {
  return custom instanceof RegExp
    ? new RegExp(
        `^${own}$|(?:${custom.source})`,
        custom.flags.replaceAll(/[gy]/gu, ""),
      )
    : own;
}

export function toRecordOptions(config: ReplayConfig, privacy?: PrivacyPolicy) {
  const options: ReplayRecordOptions = omitUndefined({
    blockClass: classPattern(INTERFERE_BLOCK_CLASS, config.blockClass),
    blockSelector: joinSelectors(
      privacy?.blockSelector,
      classSelector(config.blockClass),
      validSelectors(config.blockSelector, "replay.blockSelector"),
    ),
    ignoreClass: INTERFERE_IGNORE_CLASS,
    ignoreSelector: joinSelectors(
      classSelector(config.ignoreClass),
      validSelectors(config.ignoreSelector, "replay.ignoreSelector"),
    ),
    maskAllInputs: config.maskAllInputs,
    maskInputFn: config.maskInputFn,
    maskInputOptions: config.maskInputOptions,
    maskTextClass: classPattern(
      INTERFERE_MASK_TEXT_CLASS,
      config.maskTextClass,
    ),
    maskTextFn: config.maskTextFn,
    maskTextSelector: joinSelectors(
      privacy?.maskSelector,
      classSelector(config.maskTextClass),
      validSelectors(config.maskTextSelector, "replay.maskTextSelector"),
    ),
    recordCanvas: config.recordCanvas,
  });

  return options;
}
