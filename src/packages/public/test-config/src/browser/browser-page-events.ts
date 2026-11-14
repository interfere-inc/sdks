import { afterAll, beforeEach } from "vitest";

import { commands } from "./browser-platform.ts";

beforeEach(async () => {
  await commands.observePageEvents(true);
});

afterAll(async () => {
  await commands.observePageEvents(false);
});
