import { MECHANISM_TYPE } from "@interfere/types/sdk/errors";

import {
  SpanKind,
  SpanStatusCode,
  context,
  propagation,
  trace,
} from "@opentelemetry/api";
import type { Span } from "@opentelemetry/api";

import { captureError } from "./capture";
import { isEnabledOnServer } from "./env";
import { runtime } from "./runtime";

const TRACER_NAME = "@interfere/vite/tanstack-start";

const PROPAGATED_FIELDS = ["traceparent", "tracestate", "baggage"] as const;

export interface TraceMetaDescriptor {
  content: string;
  name: string;
}

export function traceMeta() {
  const span = trace.getActiveSpan();

  if (!span || runtime.mode === "application") {
    return [] satisfies TraceMetaDescriptor[];
  }

  const carrier: Record<string, string> = {};

  try {
    propagation.inject(context.active(), carrier);
  } catch {
    return [] satisfies TraceMetaDescriptor[];
  }

  const meta: TraceMetaDescriptor[] = [];

  for (const name of PROPAGATED_FIELDS) {
    const content = carrier[name];

    if (content) {
      meta.push({ content, name });
    }
  }

  return meta;
}

export async function withRequestSpan<T>(
  request: Request,
  next: () => Promise<T>,
) {
  if (!isEnabledOnServer() || runtime.mode === "application") {
    return next();
  }

  return context.with(extractParent(request), async () =>
    trace
      .getTracer(TRACER_NAME)
      .startActiveSpan(
        spanName(request),
        { attributes: requestAttributes(request), kind: SpanKind.SERVER },
        async (span) => {
          let result: T;

          try {
            result = await next();
          } catch (error) {
            fail(span, request, error);

            span.end();

            throw error;
          }

          return endWithBody(span, request, result);
        },
      ),
  );
}

function extractParent(request: Request) {
  try {
    return propagation.extract(
      context.active(),
      headersToCarrier(request.headers),
    );
  } catch {
    return context.active();
  }
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Request handlers and response streams can throw any value.
function fail(span: Span, request: Request, error: unknown) {
  span.setStatus({ code: SpanStatusCode.ERROR });

  context.with(trace.setSpan(context.active(), span), () => {
    captureError(error, {
      mechanism: { handled: false, type: MECHANISM_TYPE.node.captureError },
      request: {
        method: request.method,
        path: new URL(request.url).pathname,
        headers: request.headers,
      },
    });
  });
}

function endWithBody<T>(span: Span, request: Request, result: T) {
  const response = responseOf(result);

  if (!response?.body) {
    span.end();

    return result;
  }

  const observed = observeBody(response, response.body, span, request);

  // oxlint-disable-next-line typescript/no-unsafe-type-assertion -- The observed copy replaces the response with the same shape the caller returned.
  return (
    result instanceof Response ? observed : { ...result, response: observed }
  ) as T;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- A request middleware result is either a Response or a framework context carrying one.
function responseOf(result: unknown) {
  if (result instanceof Response) {
    return result;
  }

  if (
    result instanceof Object &&
    "response" in result &&
    result.response instanceof Response
  ) {
    return result.response;
  }

  return null;
}

function observeBody(
  response: Response,
  body: ReadableStream<Uint8Array>,
  span: Span,
  request: Request,
) {
  const reader = body.getReader();

  let open = true;

  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Streams reject with any value.
  const finish = (error?: unknown) => {
    if (!open) {
      return;
    }

    open = false;

    if (error !== undefined) {
      fail(span, request, error);
    }

    span.end();
  };

  const observed = new Response(
    new ReadableStream<Uint8Array>({
      async pull(controller) {
        try {
          const chunk = await reader.read();

          if (chunk.done) {
            finish();

            controller.close();

            return;
          }

          controller.enqueue(chunk.value);
        } catch (error) {
          finish(error);

          controller.error(error);
        }
      },
      async cancel(reason) {
        finish();

        await reader.cancel(reason);
      },
    }),
    {
      headers: response.headers,
      status: response.status,
      statusText: response.statusText,
    },
  );

  for (const key of Object.getOwnPropertySymbols(response)) {
    const descriptor = Object.getOwnPropertyDescriptor(response, key);

    if (descriptor) {
      Object.defineProperty(observed, key, descriptor);
    }
  }

  return observed;
}

function headersToCarrier(headers: Headers) {
  const carrier: Record<string, string> = {};

  for (const [key, value] of headers) {
    carrier[key] = value;
  }

  return carrier;
}

function spanName(request: Request) {
  return `${request.method} ${new URL(request.url).pathname}`;
}

function requestAttributes(request: Request) {
  const url = new URL(request.url);

  return {
    "http.request.method": request.method,
    "server.address": url.host,
    "url.full": url.href,
    "url.path": url.pathname,
  };
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Server functions can throw any value.
export function captureServerFunctionError(error: unknown) {
  if (isRouterControlFlow(error)) {
    return;
  }

  captureError(error, {
    mechanism: { handled: false, type: MECHANISM_TYPE.node.captureError },
  });
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- Router control flow is recognized structurally from any thrown value.
function isRouterControlFlow(error: unknown) {
  return (
    error instanceof Response ||
    (error instanceof Object &&
      "isNotFound" in error &&
      error.isNotFound === true)
  );
}
