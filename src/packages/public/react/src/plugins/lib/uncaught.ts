import type { IngestedFrame } from "@interfere/types/data/frame";
import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { toException } from "@interfere/types/sdk/errors";
import { recoverError } from "@interfere/types/sdk/errors/recover";

const MESSAGE_ONLY_NOISE =
  /^(?:Uncaught )?(?:Script error\.?|ResizeObserver loop (?:limit exceeded|completed with undelivered notifications)\.?)$/u;

export interface UncaughtSite {
  readonly column: unknown;
  readonly line: unknown;
  readonly message: string | Event;
  readonly source: unknown;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Uncaught handlers receive any thrown JavaScript value.
export function normalizeThrown(value: unknown) {
  const exception = toException(value);

  return {
    exception,
    synthetic: recoverError(value) === undefined,
  };
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Uncaught handlers receive any thrown JavaScript value.
export function uncaughtException(site: UncaughtSite, error: unknown) {
  if (error !== undefined && error !== null) {
    return normalizeThrown(error);
  }

  if (
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Window.onerror receives either an error-message string or an Event.
    typeof site.message !== "string" ||
    site.message === "" ||
    MESSAGE_ONLY_NOISE.test(site.message)
  ) {
    return null;
  }

  return { exception: new Error(site.message), synthetic: true };
}

export function siteFallbackFrame(site: UncaughtSite) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Window.onerror may omit the source URL, in which case no fallback stack frame exists.
  if (typeof site.source !== "string" || site.source === "") {
    return;
  }

  return omitUndefined({
    file: site.source,
    id: "fallback-0",
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Window.onerror line numbers are optional host callback arguments.
    line: typeof site.line === "number" ? site.line : undefined,
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Window.onerror column numbers are optional host callback arguments.
    column: typeof site.column === "number" ? site.column : undefined,
  }) satisfies IngestedFrame;
}

export function siteKey(site: UncaughtSite) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Window.onerror receives either an error-message string or an Event, whose type supplies the repeat key.
  return `${typeof site.message === "string" ? site.message : site.message.type}|${String(site.source)}|${String(site.line)}|${String(site.column)}`;
}
