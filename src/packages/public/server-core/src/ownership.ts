import type { TracingOwnership } from "@interfere/types/sdk/remote-config/tracing-ownership";
import { ProxyTracerProvider, trace } from "@opentelemetry/api";
import type { TracerProvider } from "@opentelemetry/api";
import { logs } from "@opentelemetry/api-logs";
import type { LoggerProvider } from "@opentelemetry/api-logs";

/** Logged once when managed `register()` finds a tracer provider another owner registered first. */
export const EXISTING_PROVIDER_WARNING =
  '[interfere] Another OpenTelemetry tracer provider is already registered (for example by Sentry or @vercel/otel), so Interfere captures server errors through its own exporter and does not own request tracing. If that provider is yours, call register({ tracing: "application" }) and export its traces to Interfere over standard OTLP.';

/** Logged once when managed `register()` owns the tracer provider but another owner registered the propagator first. */
export const EXISTING_PROPAGATOR_WARNING =
  '[interfere] Another OpenTelemetry propagator already owns trace headers, so Interfere will not inject its own. If that propagator is yours, call register({ tracing: "application" }).';

// oxlint-disable-next-line typescript/no-deprecated -- OTel 1.x exposes the registered provider only through its proxy delegate; a fresh proxy yields the shared no-op delegate.
const UNREGISTERED_TRACER_PROVIDER = new ProxyTracerProvider().getDelegate();

/** The tracer provider another owner registered globally, or `null` while the API still serves its no-op delegate. */
export function registeredTracerProvider() {
  const provider = trace.getTracerProvider();

  const delegate =
    // oxlint-disable-next-line typescript/no-deprecated -- OTel 1.x exposes the registered provider only through its proxy delegate.
    provider instanceof ProxyTracerProvider ? provider.getDelegate() : provider;

  return (
    delegate === UNREGISTERED_TRACER_PROVIDER ? null : delegate
  ) satisfies TracerProvider | null;
}

const ADOPTED_PROVIDER_WARNING =
  '[interfere] Adopted the OpenTelemetry tracer provider this application registered first (for example Sentry or dd-trace). Server errors still reach Interfere; request traces reach Interfere only if that provider exports them over OTLP. Call register({ tracing: "application" }) to confirm this setup and silence this message.';

const DATADOG_TRACER_WARNING =
  "[interfere] dd-trace is also tracing this process without its OpenTelemetry bridge, so Datadog and Interfere record separate traces and dd-trace owns outbound trace headers. Set DD_TRACE_OTEL_ENABLED=true and register dd-trace's TracerProvider before Interfere to keep one trace.";

let adoptionLogged = false;
let datadogLogged = false;

/**
 * The tracing owner for this process: the explicit `register({ tracing })`
 * option, else auto, which adopts an already registered provider and
 * otherwise manages tracing.
 */
export function resolveTracingMode(
  requested: TracingOwnership | "auto" = "auto",
) {
  let mode: TracingOwnership;

  if (requested === "auto") {
    mode = registeredTracerProvider() === null ? "managed" : "application";
  } else {
    mode = requested;
  }

  if (requested === "auto" && mode === "application" && !adoptionLogged) {
    adoptionLogged = true;

    console.warn(ADOPTED_PROVIDER_WARNING);
  }

  if (mode === "managed" && "_ddtrace" in globalThis && !datadogLogged) {
    datadogLogged = true;

    console.warn(DATADOG_TRACER_WARNING);
  }

  return mode satisfies TracingOwnership;
}

/**
 * The logger provider another owner registered globally, or `null` while the
 * logs API still serves its proxy. The API exposes no public query for this.
 *
 * WARNING: `_getDelegate` is the proxy's internal marker; a registered
 * provider is returned directly and has no such method. Re-check when
 * `@opentelemetry/api-logs` changes its proxy shape.
 */
export function registeredLoggerProvider() {
  const provider = logs.getLoggerProvider();

  return (
    "_getDelegate" in provider ? null : provider
  ) satisfies LoggerProvider | null;
}
