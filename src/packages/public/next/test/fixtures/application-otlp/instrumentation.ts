import { registerOTel } from "@vercel/otel";
import { register as registerInterfere } from "@interfere/next/instrumentation";

export { onRequestError } from "@interfere/next/instrumentation";

export async function register() {
  registerOTel({ serviceName: "storefront" });

  await registerInterfere({ tracing: "application" });
}
