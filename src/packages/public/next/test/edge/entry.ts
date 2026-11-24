import { trace } from "@opentelemetry/api";
import { register } from "../../src/instrumentation.edge.ts";

Reflect.set(globalThis, "qualify", async () => {
  await register({ tracing: "managed" });

  return trace
    .getTracer("edge-qualification")
    .startSpan("edge-initialized")
    .isRecording();
});
