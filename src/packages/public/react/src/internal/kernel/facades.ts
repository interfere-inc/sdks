import { clearExportCarryover } from "../otel/export/carryover.js";
import type { SessionTracker } from "../../tracking/api.js";
import type { StartupExceptionBuffer } from "../capture/startup-buffer.js";
import type { PluginRuntime } from "../plugins/plugin-runtime.js";
import type {
  KernelConsent,
  KernelDevice,
  KernelIdentity,
  KernelSession,
} from "./handles.js";

interface ConsentState {
  requireRuntime: () => PluginRuntime;
  startup: StartupExceptionBuffer;
  tracker: SessionTracker;
}

interface TrackerState {
  tracker: SessionTracker;
}

export function createKernelConsent(kernel: ConsentState) {
  return {
    get: () => kernel.requireRuntime().getConsent(),
    set: (decision) => {
      const runtime = kernel.requireRuntime();
      const previous = runtime.getConsent();

      runtime.setConsent(decision);

      const current = runtime.getConsent();

      kernel.tracker.setPersistent(current.analytics);

      if (previous.analytics && !current.analytics) {
        clearExportCarryover();
      } else if (previous.replay && !current.replay) {
        clearExportCarryover("replay:");
      }

      kernel.startup.releaseDeferred();
    },
  } satisfies KernelConsent;
}

export function createKernelIdentity(kernel: TrackerState) {
  return {
    clear: () => {
      clearExportCarryover();

      kernel.tracker.reset();
    },
    get: () => kernel.tracker.getIdentity(),
    set: async (params) => kernel.tracker.identify(params),
  } satisfies KernelIdentity;
}

export function createKernelDevice(kernel: TrackerState) {
  return {
    getDeviceId: () => kernel.tracker.getDeviceId(),
    getFpHash: () => kernel.tracker.getFpHash(),
  } satisfies KernelDevice;
}

export function createKernelSession(kernel: TrackerState) {
  return {
    getId: () => kernel.tracker.sessionId(),
    getWindowId: () => kernel.tracker.windowId(),
  } satisfies KernelSession;
}
