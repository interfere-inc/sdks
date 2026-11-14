/* oxlint-disable import/no-default-export */
import type { Plugin } from "./lib/types.js";

const devicePlugin: Plugin = {
  name: "device",

  setup(ctx) {
    return ctx.startDeviceInfoCapture() ?? undefined;
  },
};

export default devicePlugin;
