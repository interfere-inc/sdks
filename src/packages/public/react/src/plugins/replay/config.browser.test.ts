import {
  INTERFERE_BLOCK_CLASS,
  INTERFERE_IGNORE_CLASS,
  INTERFERE_MASK_TEXT_CLASS,
} from "../../internal/dom/privacy-classes.js";
import { toRecordOptions } from "./config.js";
import type { ReplayConfig } from "./config.js";
import { setLogLevel } from "../../util/log.js";
import { describe, expect, test, vi } from "vitest";

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

  test("honours customer classes alongside the interfere classes", () => {
    const options = toRecordOptions({
      blockClass: "card",
      blockSelector: "[data-card]",
      ignoreClass: "hush",
      maskTextClass: "secret",
    });

    expect(options.maskTextClass).toBe(INTERFERE_MASK_TEXT_CLASS);
    expect(options.blockClass).toBe(INTERFERE_BLOCK_CLASS);
    expect(options.ignoreClass).toBe(INTERFERE_IGNORE_CLASS);
    expect(options.blockSelector).toBe(".card, [data-card]");
    expect(options.ignoreSelector).toBe(".hush");
    expect(options.maskTextSelector).toBe(".secret");
  });

  test("matches customer class patterns and the interfere class", () => {
    const options = toRecordOptions({
      blockClass: /^card-/gu,
      maskTextClass: /secret/iu,
    });

    const { blockClass, maskTextClass } = options;

    if (!(blockClass instanceof RegExp && maskTextClass instanceof RegExp)) {
      throw new TypeError("test invariant: expected class patterns");
    }

    expect(blockClass.flags).toBe("u");

    expect(
      ["card-number", "card-number", INTERFERE_BLOCK_CLASS, "cardholder"].map(
        (name) => blockClass.test(name),
      ),
    ).toStrictEqual([true, true, true, false]);

    expect(maskTextClass.test("SECRET-value")).toBe(true);
    expect(maskTextClass.test(INTERFERE_MASK_TEXT_CLASS)).toBe(true);
    expect(options.blockSelector).toBeUndefined();
  });

  test("drops only the invalid customer selectors and warns", () => {
    setLogLevel("warn");

    const warn = vi.spyOn(console, "warn");

    const options = toRecordOptions({
      blockSelector: '[data-card], [, :is(.a, .b), [data-x="1,2"]',
      ignoreSelector: "::nope(",
      maskTextSelector: ".secret, >>",
    });

    expect(options.blockSelector).toBe(
      '[data-card], :is(.a, .b), [data-x="1,2"]',
    );

    expect(options.ignoreSelector).toBeUndefined();
    expect(options.maskTextSelector).toBe(".secret");
    expect(warn).toHaveBeenCalledTimes(3);
  });

  test("passes only known rrweb options", () => {
    const config = {
      checkoutEveryNms: 1,
      emit: () => undefined,
      maskAllInputs: true,
    } as ReplayConfig;

    expect(Object.keys(toRecordOptions(config)).toSorted()).toStrictEqual([
      "blockClass",
      "ignoreClass",
      "maskAllInputs",
      "maskTextClass",
    ]);
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
