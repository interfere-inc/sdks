/** A request header Interfere can write: W3C trace context and its state, or the session baggage. */
export type TraceHeaderName = "baggage" | "traceparent" | "tracestate";

/** An explicit propagation origin with the header names it receives. */
export interface PropagateToEntry {
  headers: TraceHeaderName[];
  origin: string;
}

/** `false` writes no header anywhere, page origin included; a list restricts every origin to those headers. */
export type TraceHeadersOption = TraceHeaderName[] | false;

/** The header names one origin receives; empty keeps the origin tracked for observation and failed-request capture with nothing written. */
export type HeaderSet = ReadonlySet<TraceHeaderName>;

/** One allowed origin: its header set and whether it was discovered from workspace configuration rather than asserted by the page or the customer. */
export interface OriginPolicy {
  discovered: boolean;
  headers: HeaderSet;
}

/** Exact origins allowed to receive trace headers, each with its policy. */
export type PropagationPolicy = ReadonlyMap<string, OriginPolicy>;
