import { normalizePath } from "./normalize-path.js";
import { describe, expect, test } from "vitest";

describe(normalizePath, () => {
  test("masks uuid segments", () => {
    expect(
      normalizePath("/api/projects/0af76519-16cd-43dd-8448-eb211c80319c/tags"),
    ).toBe("/api/projects/{id}/tags");
  });

  test("masks long hex runs", () => {
    expect(normalizePath("/assets/0af7651916cd43dd8448/chunk")).toBe(
      "/assets/{hash}/chunk",
    );
  });

  test("masks digit runs of two or more", () => {
    expect(normalizePath("/orders/12345/items/7")).toBe("/orders/{n}/items/7");
  });

  test("applies uuid masking before hex and digits", () => {
    expect(
      normalizePath(
        "/a/0af76519-16cd-43dd-8448-eb211c80319c/b/deadbeefdeadbeef/c/42",
      ),
    ).toBe("/a/{id}/b/{hash}/c/{n}");
  });

  test.each([
    ["/assets/button-B2ivX0_I.js", "/assets/{asset}.js"],
    ["/assets/src-iT_QcnsK.js", "/assets/{asset}.js"],
    ["/assets/lazyRouteComponent-RxpN1wl_.js", "/assets/{asset}.js"],
    ["/assets/config-DapTbKhs-ByCqvPbu.js", "/assets/{asset}.js"],
    ["/assets/log-D6Ht--tv-BgoWiVF_.js", "/assets/{asset}.js"],
    ["/assets/button-variants-ByFC14Yv.js", "/assets/{asset}.js"],
    ["/assets/index-DapTbKhs.css", "/assets/{asset}.css"],
    ["/assets/index-B2ivX0_I.js.map", "/assets/{asset}.js.map"],
    ["/assets/inter-latin-B2ivX0_I.woff2", "/assets/{asset}.woff2"],
    ["/static/js/main.3f2a1b9c.js", "/static/js/{asset}.js"],
    ["/assets/B2ivX0_I.mjs", "/assets/{asset}.mjs"],
    ["/assets/button-{hash}.js", "/assets/{asset}.js"],
    ["/v12/assets/button-B2ivX0_I.js", "/v{n}/assets/{asset}.js"],
  ])("collapses the hashed asset %s", (path, expected) => {
    expect(normalizePath(path)).toBe(expected);
  });

  test.each([
    ["/assets/account-Settings.js", "/assets/account-Settings.js"],
    ["/assets/hooks-useState.js", "/assets/hooks-useState.js"],
    ["/assets/page-PageMenu.js", "/assets/page-PageMenu.js"],
    ["/assets/variants-abcdefgh.js", "/assets/variants-abcdefgh.js"],
    ["/assets/variants-ABCDEFGH.js", "/assets/variants-ABCDEFGH.js"],
    ["/fonts/inter-latin-400.woff2", "/fonts/inter-latin-{n}.woff2"],
    ["/img/icon-192x192.png", "/img/icon-{n}x{n}.png"],
    ["/api/build-B2ivX0_I", "/api/build-B2ivX0_I"],
    ["/api/build-B2ivX0_I.json", "/api/build-B2ivX0_I.json"],
    ["/assets/button-B2ivX0_I.js/extra", "/assets/button-B2ivX0_I.js/extra"],
    ["/assets/buttonB2ivX0_I.js", "/assets/buttonB2ivX0_I.js"],
  ])("leaves the ordinary path %s alone", (path, expected) => {
    expect(normalizePath(path)).toBe(expected);
  });

  test("leaves template-free paths untouched", () => {
    expect(normalizePath("/api/checkout/submit")).toBe("/api/checkout/submit");
  });
});
