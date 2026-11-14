import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";

import { containAsync } from "@interfere/server-core/contain";

import { captureError } from "../capture/capture.js";
import { flushServerTelemetry } from "../exporters/server-flush.js";

const SITE = "withInterfereAfter()";

/**
 * Wraps a task passed to Next.js `after()` so a throw or rejection is captured
 * by Interfere (mechanism `auto.function.nextjs.after`) and then re-thrown
 * unchanged. Next does not report `after()` failures to `onRequestError`, so
 * without this wrapper they reach only the server console. The wrapper runs
 * after the response is sent and never delays it.
 *
 * ```ts
 * import { after } from "next/server";
 * import { withInterfereAfter } from "@interfere/next/server";
 *
 * after(withInterfereAfter(async () => {
 *   await recordAnalytics();
 * }));
 * ```
 */
export function withInterfereAfter<T>(task: Promise<T>): Promise<T>;
export function withInterfereAfter<T>(
  task: () => T | Promise<T>,
): () => Promise<T>;
export function withInterfereAfter<T>(
  task: Promise<T> | (() => T | Promise<T>),
) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- after() accepts a callback or any thenable, including promises from another realm.
  if (typeof task === "function") {
    return async () => observe(task);
  }

  return observe(async () => task);
}

async function observe<T>(task: () => T | Promise<T>) {
  try {
    return await task();
  } catch (error) {
    captureError(error, undefined, {
      mechanism: { handled: false, type: MECHANISM_TYPE.nextjs.after },
    });

    await containAsync(SITE, flushServerTelemetry);

    throw error;
  }
}
