import { register as registerInterfere } from "@interfere/next/instrumentation";

export { onRequestError } from "@interfere/next/instrumentation";

export async function register() {
  if (
    process.env["NEXT_RUNTIME"] === "nodejs" &&
    process.env["DD_TRACE_OTEL_ENABLED"] === "true"
  ) {
    const { default: tracer } = await import("dd-trace");

    new tracer.TracerProvider().register();
  }

  await registerInterfere({ propagateTo: [process.env["OUTBOUND_URL"] ?? ""] });
}
