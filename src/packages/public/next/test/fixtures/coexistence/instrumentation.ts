import { trace } from "@opentelemetry/api";
import { BasicTracerProvider } from "@opentelemetry/sdk-trace-base";
import { register as registerInterfere } from "@interfere/next/instrumentation";

export { onRequestError } from "@interfere/next/instrumentation";

export async function register() {
  trace.setGlobalTracerProvider(new BasicTracerProvider());

  await registerInterfere();

  await registerInterfere();
}
