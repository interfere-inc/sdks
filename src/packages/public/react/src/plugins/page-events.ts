/* oxlint-disable import/no-default-export */
import type { Plugin } from "./lib/types.js";

const pageEventsPlugin: Plugin = {
  name: "pageEvents",

  setup(ctx) {
    return ctx.startPageEventsCapture() ?? undefined;
  },
};

export default pageEventsPlugin;
