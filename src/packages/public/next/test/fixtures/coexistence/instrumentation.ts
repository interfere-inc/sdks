import { trace } from "@opentelemetry/api";
import { TracerProvider } from "@opentelemetry/sdk-trace";
import { register as registerInterfere } from "@interfere/next/instrumentation";

export { onRequestError } from "@interfere/next/instrumentation";

export async function register() {
  trace.setGlobalTracerProvider(new TracerProvider());

  await registerInterfere();

  await registerInterfere();
}
