import { parseRetryAfter } from "./after.js";

export const TELEMETRY_EXPORT_TIMEOUT_MS = 65_000;

function retryDelay(response: Response) {
  const delay = parseRetryAfter(response.headers.get("retry-after")) ?? 30_000;

  return delay + Math.floor(Math.random() * 1000);
}

async function wait(ms: number, signal: AbortSignal) {
  signal.throwIfAborted();

  await new Promise<void>((resolve, reject) => {
    const abort = () => {
      clearTimeout(timer);

      reject(new Error("Telemetry export aborted", { cause: signal.reason }));
    };

    const timer = setTimeout(() => {
      signal.removeEventListener("abort", abort);

      resolve();
    }, ms);

    signal.addEventListener("abort", abort, { once: true });
  });
}

export async function retryTelemetry(
  send: (signal: AbortSignal) => Promise<Response>,
  parentSignal?: AbortSignal | null,
) {
  const controller = new AbortController();
  const abortFromParent = () => controller.abort(parentSignal?.reason);

  const signal =
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Older Edge runtimes expose AbortController but omit the optional AbortSignal.any API.
    parentSignal && typeof AbortSignal.any === "function"
      ? AbortSignal.any([parentSignal, controller.signal])
      : controller.signal;

  if (parentSignal && signal === controller.signal) {
    if (parentSignal.aborted) {
      abortFromParent();
    } else {
      parentSignal.addEventListener("abort", abortFromParent, { once: true });
    }
  }

  const timer = setTimeout(
    () => controller.abort(new Error("Telemetry export deadline exceeded")),
    TELEMETRY_EXPORT_TIMEOUT_MS,
  );

  const deadline = Date.now() + TELEMETRY_EXPORT_TIMEOUT_MS;

  const attempt: (index: number) => Promise<Response> = async (index) => {
    signal.throwIfAborted();

    const response = await send(signal);

    if (
      index === 2 ||
      (response.status !== 429 &&
        ![500, 502, 503, 504].includes(response.status))
    ) {
      return response;
    }

    const delay = retryDelay(response);

    if (Date.now() + delay >= deadline) {
      return response;
    }

    await response.body?.cancel();

    await wait(delay, signal);

    return attempt(index + 1);
  };

  try {
    return await attempt(0);
  } finally {
    clearTimeout(timer);

    parentSignal?.removeEventListener("abort", abortFromParent);
  }
}
