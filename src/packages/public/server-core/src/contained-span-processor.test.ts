import {
  BasicTracerProvider,
  InMemorySpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { afterEach, expect, test } from "vitest";

import { containSpanProcessor } from "./contained-span-processor.js";

const providers: BasicTracerProvider[] = [];

afterEach(async () => {
  await Promise.all(providers.map(async (provider) => provider.shutdown()));

  providers.length = 0;
});

test("a throwing processor neither breaks span creation nor starves later processors", () => {
  const exporter = new InMemorySpanExporter();

  const provider = new BasicTracerProvider({
    spanProcessors: [
      containSpanProcessor({
        async forceFlush() {
          await Promise.resolve();
        },
        onEnd() {
          throw new Error("injected onEnd failure");
        },
        onEnding() {
          throw new Error("injected onEnding failure");
        },
        onStart() {
          throw new Error("injected onStart failure");
        },
        async shutdown() {
          await Promise.resolve();
        },
      }),
      new SimpleSpanProcessor(exporter),
    ],
  });

  providers.push(provider);

  expect(() => {
    provider.getTracer("test").startSpan("request").end();
  }).not.toThrow();

  expect(exporter.getFinishedSpans().map((span) => span.name)).toStrictEqual([
    "request",
  ]);
});
