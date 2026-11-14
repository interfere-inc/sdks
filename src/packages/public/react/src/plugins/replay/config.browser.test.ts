import {
  INTERFERE_BLOCK_CLASS,
  INTERFERE_IGNORE_CLASS,
  INTERFERE_MASK_TEXT_CLASS,
} from "../../internal/dom/privacy-classes.js";
import { toRecordOptions } from "./config.js";
import { describe, expect, test } from "vitest";

describe(toRecordOptions, () => {
  test("always applies the interfere mask/block/ignore classes", () => {
    const options = toRecordOptions({});

    expect(options.maskTextClass).toBe(INTERFERE_MASK_TEXT_CLASS);
    expect(options.blockClass).toBe(INTERFERE_BLOCK_CLASS);
    expect(options.ignoreClass).toBe(INTERFERE_IGNORE_CLASS);
  });

  test("does not enable input masking by default", () => {
    expect(toRecordOptions({}).maskAllInputs).toBeUndefined();
  });

  test("keeps the interfere classes even when the customer sets their own", () => {
    const options = toRecordOptions({
      blockClass: "card",
      ignoreClass: "hush",
      maskTextClass: "secret",
    });

    expect(options.maskTextClass).toBe(INTERFERE_MASK_TEXT_CLASS);
    expect(options.blockClass).toBe(INTERFERE_BLOCK_CLASS);
    expect(options.ignoreClass).toBe(INTERFERE_IGNORE_CLASS);
  });

  test("passes through customer selectors alongside the interfere classes", () => {
    const options = toRecordOptions({
      blockSelector: ".card",
      maskTextSelector: ".secret",
    });

    expect(options.maskTextSelector).toBe(".secret");
    expect(options.blockSelector).toBe(".card");
    expect(options.maskTextClass).toBe(INTERFERE_MASK_TEXT_CLASS);
    expect(options.blockClass).toBe(INTERFERE_BLOCK_CLASS);
  });

  test("passes through explicit masking options", () => {
    const options = toRecordOptions({
      maskAllInputs: true,
      maskInputOptions: { email: true },
      recordCanvas: true,
    });

    expect(options.maskAllInputs).toBe(true);
    expect(options.maskInputOptions).toStrictEqual({ email: true });
    expect(options.recordCanvas).toBe(true);
  });
});
