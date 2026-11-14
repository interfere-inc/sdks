import { beforeEach } from "vitest";

import { commands } from "./browser-platform.ts";

// WARNING: Chromium re-hovers whatever renders under the pointer the previous test left behind (the page outlives every test iframe), so park it off-document before each test.
beforeEach(async () => {
  await commands.parkPointer();
});
