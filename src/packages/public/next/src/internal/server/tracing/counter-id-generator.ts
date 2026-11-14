import { randomBytes } from "node:crypto";
import type { IdGenerator } from "@opentelemetry/sdk-trace-base";

const TRACE_ID_HEX_LEN = 32;

const SPAN_ID_HEX_LEN = 16;

const TRACE_PREFIX_HEX_LEN = 24;

const SPAN_PREFIX_HEX_LEN = 12;

const TRACE_COUNTER_HEX_LEN = TRACE_ID_HEX_LEN - TRACE_PREFIX_HEX_LEN;

const SPAN_COUNTER_HEX_LEN = SPAN_ID_HEX_LEN - SPAN_PREFIX_HEX_LEN;

const TRACE_COUNTER_MODULUS = 2 ** (TRACE_COUNTER_HEX_LEN * 4);

const SPAN_COUNTER_MODULUS = 2 ** (SPAN_COUNTER_HEX_LEN * 4);

export class CounterIdGenerator implements IdGenerator {
  private readonly tracePrefix: string;
  private readonly spanPrefix: string;
  private traceCounter = 0;
  private spanCounter = 0;

  constructor() {
    const seed = randomBytes((TRACE_PREFIX_HEX_LEN + SPAN_PREFIX_HEX_LEN) / 2);

    this.tracePrefix = seed
      .subarray(0, TRACE_PREFIX_HEX_LEN / 2)
      .toString("hex");

    this.spanPrefix = seed.subarray(TRACE_PREFIX_HEX_LEN / 2).toString("hex");
  }

  generateTraceId() {
    this.traceCounter = (this.traceCounter + 1) % TRACE_COUNTER_MODULUS;

    return (
      this.tracePrefix +
      this.traceCounter.toString(16).padStart(TRACE_COUNTER_HEX_LEN, "0")
    );
  }

  generateSpanId() {
    this.spanCounter = (this.spanCounter + 1) % SPAN_COUNTER_MODULUS;

    return (
      this.spanPrefix +
      this.spanCounter.toString(16).padStart(SPAN_COUNTER_HEX_LEN, "0")
    );
  }
}
