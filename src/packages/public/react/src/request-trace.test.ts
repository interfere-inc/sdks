import { expect, test, vi } from "vitest";
import { correlateRequestTrace } from "./request-trace.js";
import { observeRequestTrace } from "./internal/otel/request-observer/request-trace-correlation.js";

const TRACE = "1234567890abcdef1234567890abcdef";
const PARENT = `00-${TRACE}-1234567890abcdef-00`;

test.each(["before", "after"])(
  "correlates the exact response when headers arrive %s observation",
  (order) => {
    const response = new Response();
    const receiver = vi.fn<(...args: unknown[]) => void>();
    const unrelated = vi.fn<(...args: unknown[]) => void>();

    observeRequestTrace(new Response(), unrelated);

    const deliver = () =>
      correlateRequestTrace(response, { traceparent: PARENT });

    if (order === "before") {
      deliver();
    }

    observeRequestTrace(response, receiver);

    if (order === "after") {
      deliver();
    }

    deliver();

    expect(receiver).toHaveBeenCalledTimes(1);

    expect(receiver).toHaveBeenCalledWith(
      expect.objectContaining({ traceId: TRACE, traceFlags: 0 }),
    );

    expect(unrelated).not.toHaveBeenCalled();
  },
);

test.each([
  "",
  `${PARENT}, ${PARENT}`,
  "00-00000000000000000000000000000000-1234567890abcdef-01",
  "00-1234567890abcdef1234567890abcdef-0000000000000000-01",
])("rejects invalid final traceparent %s", (traceparent) => {
  const response = new Response();
  const receiver = vi.fn<(...args: unknown[]) => void>();

  observeRequestTrace(response, receiver);

  correlateRequestTrace(response, { traceparent });

  expect(receiver).not.toHaveBeenCalled();
});

test("missing headers do not prevent later valid correlation", () => {
  const response = new Response();
  const receiver = vi.fn<(...args: unknown[]) => void>();

  observeRequestTrace(response, receiver);

  correlateRequestTrace(response, {});

  expect(receiver).not.toHaveBeenCalled();

  correlateRequestTrace(response, new Headers({ traceparent: PARENT }));

  expect(receiver).toHaveBeenCalledTimes(1);
});
