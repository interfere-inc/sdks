import { ROOT_CONTEXT, propagation, trace } from "@opentelemetry/api";
import {
  InMemorySpanExporter,
  SimpleSpanProcessor,
  TracerProvider,
} from "@opentelemetry/sdk-trace";
import { expect, test } from "vitest";

import { correlationSpanProcessor } from "./instrumentation.js";

test("host providers correlate request and child spans from interfere.* baggage only", async () => {
  const processor = correlationSpanProcessor();

  expect(correlationSpanProcessor()).not.toBe(processor);

  const exporter = new InMemorySpanExporter();

  const provider = new TracerProvider({
    spanProcessors: [processor, new SimpleSpanProcessor({ exporter })],
  });

  const parent = propagation.setBaggage(
    ROOT_CONTEXT,
    propagation.createBaggage({
      "interfere.session.id": { value: "019f4cac-2178-768b-ac0a-5b98b34d4c58" },
      "interfere.device.id": { value: "019f4cac-2178-768b-ac0a-5b98b34d4c59" },
      "session.id": { value: "vendor-session" },
      "user.id": { value: "private-user" },
    }),
  );

  try {
    const tracer = provider.getTracer("host");
    const request = tracer.startSpan("request", {}, parent);

    tracer.startSpan("child", {}, trace.setSpan(parent, request)).end();

    request.end();

    expect(exporter.getFinishedSpans()[0]?.attributes).toStrictEqual({
      "interfere.session.id": "019f4cac-2178-768b-ac0a-5b98b34d4c58",
      "interfere.device.id": "019f4cac-2178-768b-ac0a-5b98b34d4c59",
    });

    expect(exporter.getFinishedSpans()[1]?.attributes).toStrictEqual(
      exporter.getFinishedSpans()[0]?.attributes,
    );

    expect(propagation.getBaggage(parent)?.getEntry("session.id")?.value).toBe(
      "vendor-session",
    );

    const legacy = propagation.setBaggage(
      ROOT_CONTEXT,
      propagation.createBaggage({
        "session.id": { value: "019f4cac-2178-768b-ac0a-5b98b34d4c58" },
      }),
    );

    tracer.startSpan("legacy-request", {}, legacy).end();

    expect(exporter.getFinishedSpans()[2]?.attributes).toStrictEqual({});
  } finally {
    await provider.shutdown();
  }
});
