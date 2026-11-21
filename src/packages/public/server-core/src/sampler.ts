import type { Attributes, Context, Link, SpanKind } from "@opentelemetry/api";
import { getStringFromEnv } from "@opentelemetry/core";
import {
  AlwaysOffSampler,
  AlwaysOnSampler,
  ParentBasedSampler,
  SamplingDecision,
  TraceIdRatioBasedSampler,
} from "@opentelemetry/sdk-trace";
import type { Sampler, SamplingResult } from "@opentelemetry/sdk-trace";

export const CAPTURE_KIND_ATTRIBUTE = "interfere.capture.kind";

function ratio() {
  const value = Number(getStringFromEnv("OTEL_TRACES_SAMPLER_ARG") ?? "1");

  return Number.isFinite(value) && value >= 0 && value <= 1 ? value : 1;
}

const SAMPLERS = new Map<string, () => Sampler>([
  ["always_on", () => new AlwaysOnSampler()],
  ["always_off", () => new AlwaysOffSampler()],
  ["traceidratio", () => new TraceIdRatioBasedSampler(ratio())],
  [
    "parentbased_always_off",
    () => new ParentBasedSampler({ root: new AlwaysOffSampler() }),
  ],
  [
    "parentbased_traceidratio",
    () =>
      new ParentBasedSampler({ root: new TraceIdRatioBasedSampler(ratio()) }),
  ],
]);

export function configuredSampler() {
  const build = SAMPLERS.get(getStringFromEnv("OTEL_TRACES_SAMPLER") ?? "");

  return build
    ? build()
    : new ParentBasedSampler({ root: new AlwaysOnSampler() });
}

class ExceptionSampler implements Sampler {
  private readonly delegate: Sampler;

  constructor(delegate: Sampler) {
    this.delegate = delegate;
  }

  shouldSample(
    parent: Context,
    traceId: string,
    name: string,
    kind: SpanKind,
    attributes: Attributes,
    links: Link[],
  ) {
    if (attributes[CAPTURE_KIND_ATTRIBUTE] === "exception") {
      return {
        decision: SamplingDecision.RECORD_AND_SAMPLED,
      } satisfies SamplingResult;
    }

    return this.delegate.shouldSample(
      parent,
      traceId,
      name,
      kind,
      attributes,
      links,
    );
  }

  toString() {
    return `ExceptionSampler{${this.delegate.toString()}}`;
  }
}

/** Applies the OTEL_TRACES_SAMPLER head-sampling policy to ordinary spans while always keeping Interfere exception spans. */
export function exceptionSampler(delegate: Sampler) {
  return new ExceptionSampler(delegate);
}
