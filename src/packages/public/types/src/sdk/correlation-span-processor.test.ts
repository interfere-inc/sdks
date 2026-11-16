import { ROOT_CONTEXT, propagation } from "@opentelemetry/api";
import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { expect, test } from "vitest";

import { correlationSpanProcessor } from "./correlation-span-processor.js";

test("stamps only interfere.* baggage from the parent context", async () => {
  const exporter = new InMemorySpanExporter();

  const provider = new BasicTracerProvider({
    spanProcessors: [
      correlationSpanProcessor(),
      new SimpleSpanProcessor(exporter),
    ],
  });

  const parent = propagation.setBaggage(
    ROOT_CONTEXT,
    propagation.createBaggage({
      "interfere.session.id": { value: "019f4cac-2178-768b-ac0a-5b98b34d4c58" },
      "session.id": { value: "019f4cac-2178-768b-ac0a-5b98b34d4c59" },
      "device.id": { value: "9d1e3f2a-8c4b-4f6e-9a7d-1b2c3d4e5f60" },
      "user.id": { value: "42" },
    }),
  );

  provider.getTracer("test").startSpan("request", {}, parent).end();

  expect(
    exporter.getFinishedSpans().map((span) => span.attributes),
  ).toStrictEqual([
    { "interfere.session.id": "019f4cac-2178-768b-ac0a-5b98b34d4c58" },
  ]);

  await provider.shutdown();
});
