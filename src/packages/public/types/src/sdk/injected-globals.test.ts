import {
  API_URL_GLOBAL,
  BUILD_MARKER_GLOBAL,
  ENVIRONMENT_GLOBAL,
  FORCE_ENABLE_GLOBAL,
  INGEST_URL_GLOBAL,
  PUBLIC_KEY_GLOBAL,
  RELEASE_SLUG_GLOBAL,
  SDK_STACK_GLOBAL,
} from "./injected-globals.ts";
import { describe, expect, test } from "vitest";

describe("injected-globals wire ABI", () => {
  test("pins each global name to its exact build/runtime wire string", () => {
    expect(API_URL_GLOBAL).toBe("__INTERFERE_API_URL__");
    expect(BUILD_MARKER_GLOBAL).toBe("__INTERFERE_BUILD__");
    expect(ENVIRONMENT_GLOBAL).toBe("__INTERFERE_ENVIRONMENT__");
    expect(FORCE_ENABLE_GLOBAL).toBe("__INTERFERE_FORCE_ENABLE__");
    expect(INGEST_URL_GLOBAL).toBe("__INTERFERE_INGEST_URL__");
    expect(PUBLIC_KEY_GLOBAL).toBe("__INTERFERE_PUBLIC_KEY__");
    expect(RELEASE_SLUG_GLOBAL).toBe("__INTERFERE_RELEASE_SLUG__");
    expect(SDK_STACK_GLOBAL).toBe("__INTERFERE_SDK_STACK__");
  });

  test("keeps every injected global name distinct so writers never clobber each other", () => {
    const names = [
      API_URL_GLOBAL,
      BUILD_MARKER_GLOBAL,
      ENVIRONMENT_GLOBAL,
      FORCE_ENABLE_GLOBAL,
      INGEST_URL_GLOBAL,
      PUBLIC_KEY_GLOBAL,
      RELEASE_SLUG_GLOBAL,
      SDK_STACK_GLOBAL,
    ];

    expect(new Set(names).size).toBe(names.length);
  });
});
