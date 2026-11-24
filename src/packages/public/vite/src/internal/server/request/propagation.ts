import { trace } from "@opentelemetry/api";
import type {
  Context,
  Span,
  TextMapPropagator,
  TextMapSetter,
} from "@opentelemetry/api";

/** Applies outbound URL policy to request spans while preserving inbound and document propagation. */
export class RequestPropagation implements TextMapPropagator {
  private readonly destinations = new WeakMap<Span, string>();
  private readonly propagator: TextMapPropagator;
  private readonly urls: readonly (string | RegExp)[];

  constructor(
    propagator: TextMapPropagator,
    urls: readonly (string | RegExp)[],
  ) {
    this.propagator = propagator;
    this.urls = urls;
  }

  observe(span: Span, url: string) {
    this.destinations.set(span, url);
  }

  inject<Carrier>(
    ctx: Context,
    carrier: Carrier,
    setter: TextMapSetter<Carrier>,
  ) {
    const span = trace.getSpan(ctx);
    const destination = span && this.destinations.get(span);

    if (
      destination !== undefined &&
      !this.urls.some((pattern) =>
        pattern instanceof RegExp
          ? new RegExp(pattern.source, pattern.flags).test(destination)
          : destination.includes(pattern),
      )
    ) {
      return;
    }

    this.propagator.inject(ctx, carrier, setter);
  }

  extract: TextMapPropagator["extract"] = (ctx, carrier, getter) =>
    this.propagator.extract(ctx, carrier, getter);

  fields() {
    return this.propagator.fields();
  }
}
