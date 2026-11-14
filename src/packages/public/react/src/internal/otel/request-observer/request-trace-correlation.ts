import type { SpanContext } from "@opentelemetry/api";
import { readTraceContext } from "../trace-headers.js";

type Receiver = (trace: SpanContext) => void;

const receivers = new WeakMap<Response, Receiver>();
const contexts = new WeakMap<Response, SpanContext>();
const completed = new WeakSet<Response>();

export function observeRequestTrace(response: Response, receiver: Receiver) {
  if (completed.has(response)) {
    return;
  }

  const trace = contexts.get(response);

  if (trace) {
    completed.add(response);

    contexts.delete(response);

    receiver(trace);
  } else {
    receivers.set(response, receiver);
  }
}

export function correlateResponseTrace(response: Response, headers: Headers) {
  if (completed.has(response)) {
    return;
  }

  const trace = readTraceContext(headers);

  if (!trace) {
    return;
  }

  const receiver = receivers.get(response);

  if (receiver) {
    completed.add(response);

    receivers.delete(response);

    receiver(trace);
  } else {
    contexts.set(response, trace);
  }
}
