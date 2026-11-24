import type { Kernel } from "../kernel.js";
import { createCaptureOnlySink } from "../capture/capture-only-sink.js";
import type { CaptureOnlySinkInput } from "../capture/capture-only-sink.js";
import type { TelemetrySink } from "../capture/sink.js";
import { installViewCorrelation } from "../otel/correlation.js";

type CaptureOnlyInput = Omit<
  CaptureOnlySinkInput,
  "getActionId" | "getDeviceId" | "getSessionId" | "getViewId" | "urlRedaction"
>;

export function createCaptureOnlyCorrelation(
  kernel: Kernel,
  input: CaptureOnlyInput,
) {
  const sink: TelemetrySink = createCaptureOnlySink({
    ...input,
    getActionId: () => kernel.views.actionId(),
    getDeviceId: () => kernel.device.getDeviceId(),
    getSessionId: () => kernel.session.getId(),
    getViewId: () => kernel.views.viewId(),
    urlRedaction: kernel.urlRedaction,
  });

  const correlation = installViewCorrelation({
    documentContext: null,
    kernel,
    loggerProvider: {
      getLogger: () => ({
        emit: (record) => {
          sink.emitLog(record);
        },
      }),
    },
  });

  return { correlation, sink };
}
