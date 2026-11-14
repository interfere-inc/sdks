import { isNonErrorException } from "@interfere/types/sdk/errors";
import type { NonErrorException } from "@interfere/types/sdk/errors";
import type { ErrorFilterOptions } from "@interfere/types/sdk/options/filter-options";
import {
  matchesDeniedUrl,
  matchesIgnoredError,
  topFrameUrl,
} from "@interfere/types/sdk/options/filter";

import { createLogger } from "../../util/log.js";
import type { RecordExceptionOpts } from "./exception-span.js";

const log = createLogger("error-filters");

function exceptionUrl(
  value: Error | NonErrorException,
  opts: RecordExceptionOpts,
) {
  const fromStack =
    !isNonErrorException(value) && value.stack
      ? topFrameUrl(value.stack)
      : null;

  return fromStack ?? opts.fallbackFrames?.[0]?.file ?? null;
}

function runBeforeSend(
  beforeSend: NonNullable<ErrorFilterOptions["beforeSend"]>,
  value: Error | NonErrorException,
  opts: RecordExceptionOpts,
) {
  try {
    const result = beforeSend(value, {
      mechanism: opts.mechanism,
      originalException: value,
    });

    if (
      result === null ||
      result instanceof Error ||
      isNonErrorException(result)
    ) {
      return result;
    }

    log.warn(
      "beforeSend returned neither an exception nor null; sending the exception it received.",
    );
  } catch (error) {
    log.warn("beforeSend threw; sending the original exception.", error);
  }

  return value;
}

/** Applies ignoreErrors, denyUrls, errors.sampleRate and beforeSend; null means drop. */
export function filterException(
  filters: ErrorFilterOptions,
  value: Error | NonErrorException,
  opts: RecordExceptionOpts,
) {
  const { beforeSend, denyUrls, errors, ignoreErrors } = filters;

  if (ignoreErrors?.length && matchesIgnoredError(value, ignoreErrors)) {
    return null;
  }

  if (
    denyUrls?.length &&
    matchesDeniedUrl(exceptionUrl(value, opts), denyUrls)
  ) {
    return null;
  }

  const sampleRate = errors?.sampleRate;

  if (
    sampleRate !== undefined &&
    Number.isFinite(sampleRate) &&
    Math.random() >= sampleRate
  ) {
    return null;
  }

  return beforeSend ? runBeforeSend(beforeSend, value, opts) : value;
}
