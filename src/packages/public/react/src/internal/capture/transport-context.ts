import {
  NETWORK_UNAVAILABLE,
  TRANSPORT_CONTEXT_ATTRIBUTE,
  TRANSPORT_FAILURE_ERROR_TYPE,
  isTransportFailureMessage,
} from "@interfere/types/sdk/errors/network";

import type { Attributes } from "@opentelemetry/api";

import { isPageLeaving } from "../page-lifecycle/unloading.js";

const MAX_CAUSE_DEPTH = 5;

function hasTransportFailure(error: Error) {
  let current: unknown = error;

  for (let depth = 0; depth < MAX_CAUSE_DEPTH; depth += 1) {
    if (!(current instanceof Error)) {
      return false;
    }

    if (isTransportFailureMessage(current.message)) {
      return true;
    }

    current = current.cause;
  }

  return false;
}

export function transportContextAttrs(online?: boolean) {
  const attrs: Attributes = {
    [TRANSPORT_CONTEXT_ATTRIBUTE.errorType]: TRANSPORT_FAILURE_ERROR_TYPE,
    [TRANSPORT_CONTEXT_ATTRIBUTE.pageLeaving]: isPageLeaving(),
  };

  // oxlint-disable-next-line typescript/no-unnecessary-condition -- Server and worker hosts have no document.
  if (globalThis.document !== undefined) {
    attrs[TRANSPORT_CONTEXT_ATTRIBUTE.visibility] =
      globalThis.document.visibilityState;
  }

  if (online !== undefined && !online) {
    attrs[TRANSPORT_CONTEXT_ATTRIBUTE.connectionType] = NETWORK_UNAVAILABLE;
  }

  return attrs;
}

export function transportFailureAttrs(error: Error) {
  if (!hasTransportFailure(error)) {
    return null;
  }

  // oxlint-disable-next-line typescript/no-unnecessary-condition -- Some embedded hosts have no navigator.
  return transportContextAttrs(globalThis.navigator?.onLine);
}
