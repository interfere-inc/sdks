import { omitUndefined } from "@interfere/helpers/omit-undefined";
import { MECHANISM_TYPE, toException } from "@interfere/types/sdk/errors";
import type { NonErrorException } from "@interfere/types/sdk/errors";
import type { ErrorMechanism } from "@interfere/types/sdk/plugins/payload/errors";
import type { Attributes, TracerProvider } from "@opentelemetry/api";
import type { Instrumentation } from "next";
import type { CapturePolicy } from "@interfere/server-core/runtime";
import { contain, containAsync } from "@interfere/server-core/contain";
import { isEnabledOnServer } from "../../env.js";
import { isPluginEnabled } from "../remote-config.js";
import { readServerAction } from "../actions/server-action-marker.js";
import { controlFlowOutcome } from "./control-flow.js";
import { claimException } from "./next-span-exceptions.js";
import { captureState } from "./shared-state.js";
import { flushServerTelemetry } from "../exporters/server-flush.js";
import type { CaptureErrorContext, OnRequestErrorContext } from "../types.js";
import { runtime } from "../runtime.js";

export type ErrorDelivery = CapturePolicy["errors"];

export function setIsolatedCaptureProvider(
  provider: TracerProvider,
  options: CapturePolicy,
) {
  runtime.configureCapture(provider, options);
}

const ON_REQUEST_ERROR_MECHANISM: ErrorMechanism = {
  handled: false,
  synthetic: false,
  type: MECHANISM_TYPE.nextjs.onRequestError,
};

const SERVER_ACTION_MECHANISM: ErrorMechanism = {
  handled: false,
  synthetic: false,
  type: MECHANISM_TYPE.nextjs.serverAction,
};

const PROXY_MECHANISM: ErrorMechanism = {
  handled: false,
  synthetic: false,
  type: MECHANISM_TYPE.nextjs.proxy,
};

const ROUTE_HANDLER_MECHANISM: ErrorMechanism = {
  handled: false,
  synthetic: false,
  type: MECHANISM_TYPE.nextjs.routeHandler,
};

const DEFAULT_CAPTURE_ERROR_MECHANISM: ErrorMechanism = {
  handled: true,
  type: MECHANISM_TYPE.nextjs.captureError,
};

const UNHANDLED_ROUTE_HANDLER_STATUS = 500;

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Capture receives arbitrary thrown values.
function isNextControlFlowError(error: unknown) {
  return controlFlowOutcome(error) !== undefined;
}

const MAX_PENDING_PROXY_THROWS = 64;

const { pendingProxyThrows, seen } = captureState();

/** Lets Next's proxy error report skip a non-Error throw that the proxy wrapper already captured. */
export function rememberProxyThrow(
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Proxies can throw any JavaScript value; identity is compared, never inspected.
  value: unknown,
) {
  if (value instanceof Error) {
    return;
  }

  pendingProxyThrows.push(value);

  if (pendingProxyThrows.length > MAX_PENDING_PROXY_THROWS) {
    pendingProxyThrows.shift();
  }
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Compared by identity only.
function takeProxyThrow(value: unknown) {
  const index = pendingProxyThrows.findIndex((pending) =>
    Object.is(pending, value),
  );

  if (index === -1) {
    return false;
  }

  pendingProxyThrows.splice(index, 1);

  return true;
}

function isProxyRoute(routeType: string | undefined) {
  return routeType === "proxy" || routeType === "middleware";
}

interface RecordExceptionInput {
  context?: CaptureErrorContext | undefined;
  value: Error | NonErrorException;
  mechanism: ErrorMechanism;
}

function addNextjsAttrs(
  attrs: Attributes,
  nextjs: CaptureErrorContext["nextjs"],
  handled: boolean | undefined,
) {
  if (!nextjs) {
    return;
  }

  if (nextjs.errorDigest) {
    attrs["interfere.error.digest"] = nextjs.errorDigest;
  }

  if (nextjs.requestMethod) {
    attrs["http.request.method"] = nextjs.requestMethod;
  }

  if (nextjs.requestPath) {
    attrs["url.path"] = nextjs.requestPath;
  }

  if (nextjs.routePath) {
    attrs["http.route"] = nextjs.routePath;
  }

  const routeContext = {
    "interfere.nextjs.render_source": nextjs.renderSource,
    "interfere.nextjs.render_type": nextjs.renderType,
    "interfere.nextjs.revalidate_reason": nextjs.revalidateReason,
    "interfere.nextjs.route_type": nextjs.routeType,
    "interfere.nextjs.router_kind": nextjs.routerKind,
  };

  for (const [key, value] of Object.entries(routeContext)) {
    if (value) {
      attrs[key] = value;
    }
  }

  if (nextjs.routeType === "route" && !handled) {
    attrs["http.response.status_code"] = UNHANDLED_ROUTE_HANDLER_STATUS;
  }
}

function addRequestContextAttrs(
  attrs: Attributes,
  context: CaptureErrorContext | undefined,
) {
  if (context?.serverAction) {
    attrs["interfere.server_action.name"] = context.serverAction.name;

    if (context.serverAction.argsShape) {
      attrs["interfere.server_action.args_shape"] =
        context.serverAction.argsShape;
    }
  }

  if (context?.requestHeaders) {
    for (const [name, headerValue] of Object.entries(context.requestHeaders)) {
      attrs[`http.request.header.${name}`] = headerValue;
    }
  }
}

function buildAttrs(
  mechanism: ErrorMechanism,
  context: CaptureErrorContext | undefined,
) {
  const attrs: Attributes = {
    "interfere.exception.handled": mechanism.handled,
    "interfere.exception.mechanism": mechanism.type,
  };

  addNextjsAttrs(attrs, context?.nextjs, mechanism.handled);

  addRequestContextAttrs(attrs, context);

  return attrs;
}

function recordException({ value, mechanism, context }: RecordExceptionInput) {
  runtime.recordException(value, mechanism, buildAttrs(mechanism, context));
}

export function captureError(
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- The public capture API accepts primitive and structured JavaScript throw values before normalization.
  error: unknown,
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- This compatibility argument accepts customer framework request values without inspecting or capturing them.
  _request?: unknown,
  context?: CaptureErrorContext,
) {
  contain("captureError()", () => {
    if (
      !isEnabledOnServer() ||
      !isPluginEnabled("errors") ||
      isNextControlFlowError(error)
    ) {
      return;
    }

    if (error instanceof Error) {
      if (seen.has(error)) {
        return;
      }

      seen.add(error);
    }

    claimException(error);

    recordException({
      context,
      value: toException(error),
      mechanism: context?.mechanism ?? DEFAULT_CAPTURE_ERROR_MECHANISM,
    });
  });
}

const CAPTURED_REQUEST_HEADERS: readonly string[] = [
  "accept",
  "accept-language",
  "content-length",
  "content-type",
  "host",
  "referer",
  "user-agent",
  "x-forwarded-for",
  "x-forwarded-host",
  "x-forwarded-proto",
  "x-vercel-id",
];

type RequestErrorParameters = Parameters<Instrumentation.onRequestError>;

function pickAllowedHeaders(
  headers: RequestErrorParameters[1]["headers"] | undefined,
) {
  if (!headers) {
    return;
  }

  const picked: Record<string, string> = {};

  for (const name of CAPTURED_REQUEST_HEADERS) {
    const value = headers[name];

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- JavaScript request adapters may supply invalid header values; capture only strings and string arrays.
    if (typeof value === "string") {
      picked[name] = value;
    } else if (
      Array.isArray(value) &&
      // oxlint-disable-next-line anti-slop/no-runtime-typeof -- JavaScript adapters can supply mixed arrays outside Next's header contract.
      value.every((part) => typeof part === "string")
    ) {
      picked[name] = value.join(", ");
    }
  }

  return (Object.keys(picked).length > 0 ? picked : undefined) satisfies
    | Record<string, string>
    | undefined;
}

export async function onRequestError(
  error: RequestErrorParameters[0],
  request: Partial<RequestErrorParameters[1]> | undefined,
  context: RequestErrorParameters[2] | OnRequestErrorContext,
) {
  await containAsync("onRequestError()", async () => {
    if (isNextControlFlowError(error)) {
      return;
    }

    if (isProxyRoute(context.routeType) && takeProxyThrow(error)) {
      return;
    }

    if (error instanceof Error) {
      if (seen.has(error)) {
        return;
      }

      seen.add(error);
    }

    if (!isEnabledOnServer() || !isPluginEnabled("errors")) {
      return;
    }

    const action = readServerAction(error);

    const digest =
      error instanceof Error && "digest" in error ? error.digest : undefined;

    let mechanism = ON_REQUEST_ERROR_MECHANISM;

    if (action || context.routeType === "action") {
      mechanism = SERVER_ACTION_MECHANISM;
    } else if (context.routeType === "route") {
      mechanism = ROUTE_HANDLER_MECHANISM;
    } else if (isProxyRoute(context.routeType)) {
      mechanism = PROXY_MECHANISM;
    }

    claimException(error);

    recordException({
      context: omitUndefined({
        mechanism,
        nextjs: omitUndefined({
          ...context,
          requestMethod: request?.method || undefined,
          requestPath: request?.path || undefined,
          errorDigest:
            // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Thrown values may carry a digest outside Next's string contract.
            typeof digest === "string" ? digest || undefined : undefined,
        }),
        requestHeaders: pickAllowedHeaders(request?.headers),
        serverAction: action ?? undefined,
      }),
      value:
        // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Next can report string throws; preserve them as non-Error captures without a synthetic stack.
        typeof error === "string"
          ? {
              kind: "non-error",
              serialized: error,
              type: "string",
              value: error,
            }
          : toException(error),
      mechanism,
    });

    await flushServerTelemetry();
  });
}
