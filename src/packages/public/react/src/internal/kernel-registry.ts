import { createLogger } from "../util/log.js";
import type { Kernel } from "./kernel.js";

const log = createLogger("kernel-registry");

let active: Kernel | null = null;

export function registerKernel(kernel: Kernel) {
  if (active && active !== kernel) {
    log.warn(
      "init() called twice with different kernels; later one wins. The public capture()/span() helpers route only to the latest kernel — multi-surface apps should call kernel.recordException() directly.",
    );
  }

  active = kernel;
}

export function unregisterKernel(kernel: Kernel) {
  if (active === kernel) {
    active = null;
  }
}

export function activeKernel() {
  return active;
}
