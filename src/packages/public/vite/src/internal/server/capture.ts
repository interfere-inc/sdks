import { MECHANISM_TYPE, toException } from "@interfere/types/sdk/errors";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import type { CapturePolicy } from "@interfere/server-core/runtime";
import { resolveUrlRedaction } from "@interfere/types/sdk/url-redaction";
import type { TracerProvider } from "@opentelemetry/api";
import { isEnabledOnServer } from "./env";
import { isPluginEnabled } from "./remote-config";
import { runtime } from "./runtime";
import type { CaptureErrorContext } from "./types";

const seen = new WeakSet<Error>();

export function setIsolatedCaptureProvider(
  provider: TracerProvider,
  options: Omit<CapturePolicy, "urlRedaction"> &
    Partial<Pick<CapturePolicy, "urlRedaction">>,
) {
  runtime.configureCapture(provider, {
    ...options,
    urlRedaction: options.urlRedaction ?? resolveUrlRedaction(),
  });
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Public capture normalizes arbitrary JavaScript throws.
export function captureError(error: unknown, ctx?: CaptureErrorContext) {
  if (!isEnabledOnServer() || !isPluginEnabled("errors")) {
    return;
  }

  if (error instanceof Error) {
    if (seen.has(error)) {
      return;
    }

    seen.add(error);
  }

  runtime.recordException(
    toException(error),
    ctx?.mechanism ?? { handled: true, type: MECHANISM_TYPE.node.captureError },
    omitUndefined({
      "http.request.method": ctx?.request?.method,
      "url.path": ctx?.request?.path,
      "http.request.header.x-forwarded-for":
        ctx?.request?.headers?.get("x-forwarded-for") ?? undefined,
    }),
  );
}
